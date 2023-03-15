// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use self::page::FlashPage;
use crate::flash::header::BlockHeader;
use crate::FLAG_BYTES;
use buddy_allocator::{BinaryBuddyImpl, BuddyAllocator};
use core::fmt::Formatter;

// Re-export abi structures
pub use abi::flash::BlockType;

#[cfg(feature = "swap")]
use crate::swap::SwapStartType;

/// NOTE: the header must be a multiple of both 4 and FLAG_BYTES, where FLAG_BYTES represents the mininum
/// write granularity. For simplicity, we assume FLAG_BYTES a multiple of 4.
///
/// So the total size of the header becomes:
///     FLAG_BYTES*3    + 4      + (FLAG_BYTES*3 + 4) % 4 + (FLAG_BYTES*3 + 4 + (FLAG_BYTES*3 + 4) % 4) % FLAG_BYTES
///     ------------  ---------    ----------------------   ---------------------------------------------------------
///        flags       2x16bits         align to 4                                align to FLAG_BYTES
///                     fields
/// i.e. 16bit write granularity: 2*3 + 4 + (6 + 4) % 4 + (10 + 2) % 2 = 10 + 2 + 0 = 12
/// i.e. 64bit write granularity: 8*3 + 4 + (24 + 4) % 4 + (28 + 0) % 8 = 28 + 0 + 4 = 32
pub const HEADER_SIZE: usize = FLAG_BYTES * 3
    + 4
    + (FLAG_BYTES * 3 + 4) % 4
    + (FLAG_BYTES * 3 + 4 + (FLAG_BYTES * 3 + 4) % 4) % FLAG_BYTES;

/**
 * --------------------------------
 *      Exported interfaces
 * --------------------------------
 */
/// Interface for read/write interaction with flash memory.
pub trait FlashMethods<'a> {
    /// Reads a slice of len bytes starting from the specified offset,
    /// saving the resulting data in the supplied buffer
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()>;
    /// Retrieve the page number from the supplied address
    fn page_from_address(&self, address: u32) -> Option<FlashPage>;
    /// Retrieve the page from a page number
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage>;
    /// Retrieve the prev. page from the number of page
    fn prev_page(&self, page_num: u16) -> Option<FlashPage>;
    /// Writes a byte to the corresponding offset
    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()>;
    /// In case writes to flash memory are buffered, forces the synchronization.
    /// Otherwise, is a nop.
    fn flush_write_buffer(&mut self) -> Result<(), ()>;
    /// Erase a page number
    fn erase(&mut self, page_num: u16) -> Result<(), ()>;
    #[cfg(feature = "swap")]
    /// Launch the swap procedure
    fn launch_swap(
        &mut self,
        page_number: u16,
        start_type: crate::swap::SwapStartType,
        start_size: usize,
    ) -> crate::swap::SwapResult;
}

/// Interface offered to the storage component, to mask the actual
/// implementation with its generics parameters.
pub trait FlashAllocator<'a> {
    /// Allocate the requested size. If enough space is available, the block is provisioned
    /// and the base address of the block is returned.
    ///
    /// The method returns an object containing the base address of the block (pointing to usable space)
    /// and the provisioned size. This can be more than the requested, as we need to reserve
    /// space for the block header.
    fn allocate(&mut self, size: u32, block_type: BlockType) -> Result<FlashBlock, ()>;
    /// Deallocate the block that starts at the provided base address.
    /// This method is save, as the address is validated to be a correct block base address,
    /// by scanning the whole memory from the beginning.
    fn deallocate(&mut self, base_addr: u32) -> Result<(), ()>;

    /// Refresh the metadata of the block object
    fn refresh(&self, block: &mut FlashBlock);

    /// Dumps the state of the allocator
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error>;
}

/// Manages the block header
pub mod header {
    use super::{FLAG_BYTES, HEADER_SIZE};
    use abi::flash::BlockType;

    fn make_array<A, T>(slice: &[T]) -> A
    where
        A: Sized + Default + AsMut<[T]>,
        T: Copy,
    {
        let mut a = Default::default();
        // the type cannot be inferred!
        // a.as_mut().copy_from_slice(slice);
        <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
        a
    }

    /**
     * As the header is very small, and fields are read mutiple times,
     * this implementation copies the header of the block in SRAM when
     * this structure is constructed.
     */
    /// This structure is a cache of the block header in SRAM, to separate
    /// the physical layout from the logical data needed by the algorithm
    #[allow(dead_code)]
    pub struct BlockHeader {
        /// The block was once allocated
        allocated: bool,
        /// At some point, the erase process was launched
        dismissed: bool,
        /// The block contains only valid data
        finalized: bool,
        /// The following field is correct also for unallocated block,
        /// representing the level of smallest blocks
        block_level: u16,
        /// The type of data contained by the block
        block_type: BlockType,
    }

    impl BlockHeader {
        pub fn new(header_buffer: &[u8; HEADER_SIZE], smallest_block_level: u16) -> Self {
            // Construct the structure
            let allocated_flag: [u8; FLAG_BYTES] = header_buffer[0..FLAG_BYTES].try_into().unwrap();
            let dismissed_flag: [u8; FLAG_BYTES] = header_buffer[FLAG_BYTES..FLAG_BYTES * 2]
                .try_into()
                .unwrap();
            let finalized_flag: [u8; FLAG_BYTES] = header_buffer[FLAG_BYTES * 2..FLAG_BYTES * 3]
                .try_into()
                .unwrap();
            // Remember there is alignment space in between
            let block_level_offset: usize = HEADER_SIZE - 4;

            let block_level_bytes = make_array::<[u8; 2], u8>(
                &header_buffer[block_level_offset..block_level_offset + 2],
            );
            let block_level: u16 = u16::from_le_bytes(block_level_bytes);
            let block_type_offset: usize = block_level_offset + 2;
            let block_type_bytes =
                make_array::<[u8; 2], u8>(&header_buffer[block_type_offset..block_type_offset + 2]);
            let block_type: u16 = u16::from_le_bytes(block_type_bytes);
            // It's more reliable to check != 0xFF than all 0x00, as it could help
            // in unexpected reboot, where we have written only part of the header.
            let allocated = allocated_flag != [0xFF; FLAG_BYTES];
            Self {
                allocated: allocated,
                dismissed: dismissed_flag != [0xFF; FLAG_BYTES],
                finalized: finalized_flag != [0xFF; FLAG_BYTES],
                block_level: match allocated {
                    true => block_level,
                    false => smallest_block_level,
                },
                block_type: BlockType::from(block_type),
            }
        }
        fn write_flag(dest_buffer: &mut [u8], offset: usize, flag: bool) {
            for i in 0..FLAG_BYTES {
                dest_buffer[offset + i] = match flag {
                    true => 0x00,
                    false => 0xFF,
                }
            }
        }
        pub fn write_buffer(
            allocated: bool,
            dismissed: bool,
            finalized: bool,
            block_level: u16,
            block_type: BlockType,
        ) -> [u8; HEADER_SIZE] {
            let mut buffer: [u8; HEADER_SIZE] = [0xFF; HEADER_SIZE];
            Self::write_flag(&mut buffer, 0, allocated);
            Self::write_flag(&mut buffer, FLAG_BYTES, dismissed);
            Self::write_flag(&mut buffer, FLAG_BYTES * 2, finalized);

            // Remember there is alignment space in between
            let block_level_offset: usize = HEADER_SIZE - 4;

            buffer[block_level_offset] = block_level.to_le_bytes()[0];
            buffer[block_level_offset + 1] = block_level.to_le_bytes()[1];
            let block_type_offset: usize = block_level_offset + 2;
            let block_type_u: u16 = block_type.into();
            buffer[block_type_offset] = block_type_u.to_le_bytes()[0];
            buffer[block_type_offset + 1] = block_type_u.to_le_bytes()[1];
            buffer
        }

        pub fn is_allocated(&self) -> bool {
            self.allocated
        }
        pub fn is_dismissed(&self) -> bool {
            self.dismissed
        }
        pub fn is_finalized(&self) -> bool {
            self.finalized
        }

        pub fn block_level(&self) -> u16 {
            self.block_level
        }
        pub fn block_type(&self) -> BlockType {
            self.block_type
        }
    }
}

/// Structure used as input data for provided flash methods
pub mod page {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub struct FlashPage {
        page_num: u16,
        base_address: u32,
        size: u16,
    }
    impl FlashPage {
        pub const fn new(page_num: u16, base_address: u32, size: u16) -> Self {
            Self {
                page_num: page_num,
                base_address: base_address,
                size: size,
            }
        }
        pub const fn contains_addr(&self, address: u32) -> bool {
            (address >= self.base_address) && (address < self.base_address + self.size as u32)
        }
        pub const fn page_number(&self) -> u16 {
            self.page_num
        }
        pub const fn base_address(&self) -> u32 {
            self.base_address
        }
        pub const fn end_address(&self) -> u32 {
            self.base_address + self.size as u32 - 1
        }
        pub const fn size(&self) -> u32 {
            self.size as u32
        }
    }
}

/// Utility to traverse a flash region as an iterator among allocated blocks.
/// Useful for scanning the flash in the kernel or in the allocator component.
pub mod walker {
    use super::{utils, FlashBlock, FlashMethods};

    // Currently rust does not support trait upcasting coercion when "dyn" is involved.
    // The fact is that we would like to require for walker only reading access to flash,
    // and then we cannot pass a "read/write" instance directly.

    pub struct FlashWalkerImpl<
        'a,
        'b,
        const START_ADDR: u32,      // Allocator start address
        const END_ADDR: u32,        // Allocator end address
        const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
        const SMALLEST_BLOCK_LEVEL: usize,
    > {
        current_offset: usize,
        flash: &'a mut dyn FlashMethods<'b>,
    }
    impl<
            'a,
            'b,
            const START_ADDR: u32,      // Allocator start address
            const END_ADDR: u32,        // Allocator end address
            const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
            const SMALLEST_BLOCK_LEVEL: usize,
        > FlashWalkerImpl<'a, 'b, START_ADDR, END_ADDR, START_SCAN_ADDR, SMALLEST_BLOCK_LEVEL>
    {
        const ALLOCATOR_SIZE: usize = (END_ADDR - START_ADDR + 1) as usize;
        const START_SCAN_OFFSET: usize = (START_SCAN_ADDR - START_ADDR) as usize;

        pub fn new(flash: &'a mut dyn FlashMethods<'b>) -> Self {
            Self {
                flash: flash,
                current_offset: Self::START_SCAN_OFFSET,
            }
        }

        fn reset(&mut self) {
            self.current_offset = Self::START_SCAN_OFFSET;
        }
    }

    impl<
            'a,
            'b,
            const START_ADDR: u32,      // Allocator start address
            const END_ADDR: u32,        // Allocator end address
            const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
            const SMALLEST_BLOCK_LEVEL: usize,
        > Iterator
        for FlashWalkerImpl<'a, 'b, START_ADDR, END_ADDR, START_SCAN_ADDR, SMALLEST_BLOCK_LEVEL>
    {
        type Item = FlashBlock;
        fn next(&mut self) -> Option<Self::Item> {
            // Scan for next valid block
            while self.current_offset < Self::ALLOCATOR_SIZE {
                // Read header of the next block
                let block_header = utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
                    self.flash,
                    self.current_offset as u32,
                );
                // Skip deallocated blocks
                if block_header.is_allocated() && !block_header.is_dismissed() {
                    // Construct result
                    let result = FlashBlock {
                        block_base_address: START_ADDR
                            + self.current_offset as u32,
                        block_type: block_header.block_type(),
                        block_size: utils::get_block_size::<START_ADDR, END_ADDR>(&block_header)
                            as u32,
                        finalized: block_header.is_finalized(),
                    };
                    // Prepare for the next call
                    self.current_offset +=
                        utils::get_block_size::<START_ADDR, END_ADDR>(&block_header);
                    return Some(result);
                } else {
                    // Skip the block
                    self.current_offset +=
                        utils::get_block_size::<START_ADDR, END_ADDR>(&block_header);
                }
            }
            return None;
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.current_offset = Self::START_SCAN_OFFSET;
            let mut count: usize = 0;
            let mut block: Option<FlashBlock> = self.next();
            while count < n {
                block = self.next();
                count += 1;
            }
            return block;
        }
    }

    impl<
            'a,
            'b,
            const START_ADDR: u32,      // Allocator start address
            const END_ADDR: u32,        // Allocator end address
            const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
            const SMALLEST_BLOCK_LEVEL: usize,
        > FlashMethods<'a>
        for FlashWalkerImpl<'a, 'b, START_ADDR, END_ADDR, START_SCAN_ADDR, SMALLEST_BLOCK_LEVEL>
    {
        fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
            self.flash.read(address, buffer)
        }

        fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
            self.flash.write(address, data)
        }

        fn flush_write_buffer(&mut self) -> Result<(), ()> {
            self.flash.flush_write_buffer()
        }

        fn page_from_address(&self, address: u32) -> Option<super::page::FlashPage> {
            self.flash.page_from_address(address)
        }

        fn page_from_number(&self, page_num: u16) -> Option<super::page::FlashPage> {
            self.flash.page_from_number(page_num)
        }

        fn prev_page(&self, page_num: u16) -> Option<super::page::FlashPage> {
            self.flash.prev_page(page_num)
        }

        fn erase(&mut self, page_num: u16) -> Result<(), ()> {
            self.flash.erase(page_num)
        }

        #[cfg(feature = "swap")]
        fn launch_swap(
            &mut self,
            page_number: u16,
            start_type: crate::swap::SwapStartType,
            start_size: usize,
        ) -> crate::swap::SwapResult {
            self.flash.launch_swap(page_number, start_type, start_size)
        }
    }

    pub trait FlashWalker: Iterator<Item = FlashBlock> {
        fn reset(&mut self);
    }
    impl<
            'a,
            'b,
            const START_ADDR: u32,      // Allocator start address
            const END_ADDR: u32,        // Allocator end address
            const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
            const SMALLEST_BLOCK_LEVEL: usize,
        > FlashWalker
        for FlashWalkerImpl<'a, 'b, START_ADDR, END_ADDR, START_SCAN_ADDR, SMALLEST_BLOCK_LEVEL>
    {
        fn reset(&mut self) {
            self.reset()
        }
    }
}

/// Utilities to interact with flash blocks
pub mod utils {
    use super::HEADER_SIZE;
    use super::{header::BlockHeader, FlashBlock, FlashMethods};

    /// Reads a block header from a given buffer
    pub fn read_block_header<'a, const START_ADDR: u32, const SMALLEST_BLOCK_LEVEL: usize>(
        flash: &dyn FlashMethods<'a>,
        offset: u32,
    ) -> BlockHeader {
        let mut header_buffer: [u8; HEADER_SIZE] = [0x00; HEADER_SIZE];

        flash.read(START_ADDR + offset, &mut header_buffer).unwrap();
        let block_header: BlockHeader =
            BlockHeader::new(&header_buffer, SMALLEST_BLOCK_LEVEL as u16);
        return block_header;
    }

    /// Returns the size of a block given its header
    pub fn get_block_size<'a, const START_ADDR: u32, const END_ADDR: u32>(
        block_header: &BlockHeader,
    ) -> usize {
        let size = (END_ADDR - START_ADDR + 1) as usize;
        // This is valid also for blocks not allocated, as it's already
        // included in the block header.
        size >> block_header.block_level()
    }

    /// Reads a flash block at the given address
    pub fn get_flash_block<
        'a,
        const START_ADDR: u32,
        const END_ADDR: u32,
        const START_SCAN_ADDR: u32,
        const SMALLEST_BLOCK_LEVEL: usize,
    >(
        flash: &dyn FlashMethods<'a>,
        mut base_address: u32,
        is_base_exact: bool,
    ) -> Option<FlashBlock> {
        let size = (END_ADDR - START_ADDR + 1) as usize;
        let start_scan_offset: usize = (START_SCAN_ADDR - START_ADDR) as usize;
        // Convert address if its not already pointing to the exact start
        if !is_base_exact {
            base_address -= HEADER_SIZE as u32;
        }
        let mut process_index: usize = start_scan_offset;
        while process_index < size {
            // Read header
            let block_header =
                read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(flash, process_index as u32);
            // Allocated blocks
            if block_header.is_allocated() && !block_header.is_dismissed() {
                // Check this is the block
                let block_size = get_block_size::<START_ADDR, END_ADDR>(&block_header);
                if process_index as u32 + START_ADDR == base_address {
                    return Some(FlashBlock {
                        block_base_address: base_address,
                        finalized: block_header.is_finalized(),
                        block_type: block_header.block_type(),
                        block_size: block_size as u32,
                    });
                }
                // Countinue scanning
                process_index += block_size;
            } else {
                // Continue scanning
                process_index += size >> SMALLEST_BLOCK_LEVEL;
            }
        }
        return None;
    }

    /// Marks the block as closed. From this point on, the system will consider
    /// this block complete.
    pub fn finalize_block<'a, const START_ADDR: u32, const SMALLEST_BLOCK_LEVEL: usize>(
        flash: &mut dyn FlashMethods<'a>,
        block: FlashBlock,
    ) -> Result<(), ()> {
        // Read again the header to be safe
        let block_base_addr = block.get_nominal_base_address();
        let curr_header = self::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
            flash,
            block_base_addr - START_ADDR,
        );

        // Check if this operation is possible
        if !curr_header.is_allocated() || curr_header.is_dismissed() || curr_header.is_finalized() {
            return Err(());
        }

        // Generating the new header
        let header = BlockHeader::write_buffer(
            true,
            false,
            true,
            curr_header.block_level(),
            curr_header.block_type(),
        );
        // Writing new header
        flash.write(block_base_addr, &header).unwrap();
        // Always flush after an header or flag
        flash.flush_write_buffer().unwrap();

        Ok(())
    }

    /// Marks the block as dismissed, this is done during the removal procedure
    pub unsafe fn mark_block_dismissed<
        'a,
        const START_ADDR: u32,
        const SMALLEST_BLOCK_LEVEL: usize,
    >(
        flash: &mut dyn FlashMethods<'a>,
        block: FlashBlock,
    ) -> Result<(), ()> {
        // Read again the header to be safe
        let block_base_addr = block.get_nominal_base_address();
        let curr_header = self::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
            flash,
            block_base_addr - START_ADDR,
        );

        // Check if this operation is possible
        if !curr_header.is_allocated() {
            return Err(());
        }
        if curr_header.is_dismissed() {
            return Ok(()); // Already done
        }

        // Generating the new header
        let header = BlockHeader::write_buffer(
            true,
            true,
            curr_header.is_finalized(),
            curr_header.block_level(),
            curr_header.block_type(),
        );
        // Writing new header
        flash.write(block_base_addr, &header).unwrap();
        // Always flush after an header or flag
        flash.flush_write_buffer().unwrap();

        Ok(())
    }
}

/// This structure represents a flash block. It's an abstraction used
/// to return data about an allocation, and because its initializer is private
/// to the crate, this ensures the current crate is the only that can generate an instance.
/// Receiving an instance of this structure is a guarantee of the validity of the fields.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FlashBlock {
    block_base_address: u32,
    finalized: bool,
    block_type: BlockType,
    block_size: u32,
}
impl<'a> FlashBlock {
    pub fn get_nominal_base_address(&self) -> u32 {
        self.block_base_address
    }
    pub fn get_base_address(&self) -> u32 {
        self.block_base_address + HEADER_SIZE as u32
    }
    pub fn get_type(&self) -> BlockType {
        self.block_type
    }
    pub fn get_nominal_size(&self) -> u32 {
        return self.block_size;
    }
    pub fn get_size(&self) -> u32 {
        match self.block_type {
            BlockType::COMPONENT => self.block_size - HEADER_SIZE as u32 - 8,
            _ => self.block_size - HEADER_SIZE as u32,
        }
    }
    pub fn is_finalized(&self) -> bool {
        self.finalized
    }
    /// Gets data of the block as a slice, without copying into memory.
    /// Unsafe as it attempts to read the data directly, without passing through
    /// the interface. If the component does not have read access, this call will make
    /// the component crash, instead of giving error
    pub unsafe fn get_data(&self) -> &'a [u8] {
        let base_address = match self.block_type {
            BlockType::COMPONENT => self.block_base_address + 8,
            _ => self.block_base_address,
        };
        core::slice::from_raw_parts(base_address as *const u8, self.get_size() as usize)
    }
}

pub struct FlashAllocatorImpl<
    'a,
    const START_ADDR: u32,      // Allocator start address
    const END_ADDR: u32,        // Allocator end address
    const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
    const BLOCK_SIZE: usize,    // Minimum granularity of the allocator
    const NUM_BLOCKS: usize,
    const SMALLEST_BLOCK_LEVEL: usize,
    const TREE_NUM_NODES: usize,
> {
    buddy_allocator: BinaryBuddyImpl<
        START_ADDR,
        END_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        SMALLEST_BLOCK_LEVEL,
        TREE_NUM_NODES,
    >,
    flash: &'a mut dyn FlashMethods<'a>,
}

impl<
        'a,
        const START_ADDR: u32,      // Allocator start address
        const END_ADDR: u32,        // Allocator end address
        const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const SMALLEST_BLOCK_LEVEL: usize,
        const TREE_NUM_NODES: usize,
    >
    FlashAllocatorImpl<
        'a,
        START_ADDR,
        END_ADDR,
        START_SCAN_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        SMALLEST_BLOCK_LEVEL,
        TREE_NUM_NODES,
    >
{
    const ALLOCATOR_SIZE: usize = (END_ADDR - START_ADDR + 1) as usize;
    const START_SCAN_OFFSET: usize = (START_SCAN_ADDR - START_ADDR) as usize;

    pub fn analyze_storage(flash: &mut dyn FlashMethods<'a>, remove_unfinalized_blocks: bool) {
        let mut process_index: usize = Self::START_SCAN_OFFSET;
        while process_index < Self::ALLOCATOR_SIZE {
            // Read header
            let block_header = utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
                flash,
                process_index as u32,
            );
            // Allocated blocks
            if block_header.is_allocated() {
                let block_size = utils::get_block_size::<START_ADDR, END_ADDR>(&block_header);
                // Check if a freed_block exists
                if block_header.is_dismissed()
                    || (remove_unfinalized_blocks & !block_header.is_finalized())
                {
                    // Launch the erase again
                    let mut _block_level_out: u16 = 0;
                    Self::deallocate_procedure(
                        flash,
                        START_ADDR + process_index as u32,
                        &mut _block_level_out,
                    );
                }
                // Continue scanning
                process_index += block_size;
                continue;
            } else {
                assert!(!block_header.is_dismissed());
                // Continue scanning
                process_index += BLOCK_SIZE;
            }
        }
    }

    fn deallocate_procedure(
        flash: &mut dyn FlashMethods<'a>,
        addr: u32,
        block_level_out: &mut u16,
    ) {
        // Check position
        assert!(addr >= START_SCAN_ADDR && addr <= END_ADDR);
        let offset: usize = (addr - START_ADDR) as usize;
        // Read header
        let block_header =
            utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(flash, offset as u32);
        let block_level = block_header.block_level();
        // Generating the new header
        let header = BlockHeader::write_buffer(
            true,
            true,
            block_header.is_finalized(),
            block_level,
            block_header.block_type(),
        );
        drop(block_header); // Needed to release flash object
                            // Write the new header
        flash.write(addr, &header).unwrap();
        // Always flush after an header or flag
        flash.flush_write_buffer().unwrap();
        // Erase block
        Self::block_erase_procedure(flash, addr, block_level as usize);
        // Pass parameters
        *block_level_out = block_level;
    }

    #[cfg(not(feature = "swap"))]
    fn block_erase_procedure(
        flash: &mut dyn FlashMethods<'a>,
        block_start_addr: u32,
        block_level: usize,
    ) {
        // Start erasing from the last page
        let block_size = (Self::ALLOCATOR_SIZE >> block_level) as u32;
        let mut current_addr = block_start_addr + block_size - 1; // -1 or we will enter the next page
        while current_addr >= block_start_addr {
            // Get the page
            let page = flash.page_from_address(current_addr).unwrap();
            // Erase the page
            flash.erase(page.page_number()).unwrap();
            // Move one page down
            current_addr -= page.size();
        }
    }

    #[cfg(feature = "swap")]
    fn block_erase_procedure(
        flash: &mut dyn FlashMethods<'a>,
        block_start_addr: u32,
        block_level: usize,
    ) {
        let block_size = Self::ALLOCATOR_SIZE >> block_level;
        // Point 2
        // - Get PS
        let ps = flash.page_from_address(block_start_addr).unwrap();
        // - Find PREV_HEADER
        let mut curr_offset: u32 = Self::START_SCAN_OFFSET as u32;
        let mut prev_header_addr: u32 = 0; // It is never a valid address
        let mut prev_block_size: u32 = 0;
        let mut prev_allocated: bool = false;
        loop {
            let header =
                utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(flash, curr_offset);
            let block_size = utils::get_block_size::<START_ADDR, END_ADDR>(&header) as u32;
            let block_page = flash.page_from_address(START_ADDR + curr_offset).unwrap();
            if block_page.page_number() == ps.page_number() {
                // We found the prev. header the prev. iteration
                break;
            } else {
                // Save this data
                prev_header_addr = START_ADDR + curr_offset;
                prev_block_size = block_size;
                prev_allocated = header.is_allocated();
                // Jump and scan next header
                curr_offset += block_size;
            }
        }
        // TODO: fix a possible run over END_ADDR problem
        // Point 3
        // - pages of the block, get the last and the first to see if at least two
        let block_end_addr = block_start_addr + block_size as u32 - 1;
        let pe = flash.page_from_address(block_end_addr).unwrap();
        if pe.page_number() != ps.page_number() {
            // The block contains at least two pages
            // 3.1 - call swap on the last page
            flash.launch_swap(
                pe.page_number(),
                SwapStartType::DiscardStart,
                block_start_addr as usize + block_size - pe.base_address() as usize,
            );
            // 3.2 - erase every intermediate page
            let mut page = flash.prev_page(pe.page_number()).unwrap();
            while page.page_number() > ps.page_number() {
                // Just erase the page
                flash.erase(page.page_number()).unwrap();
                // Go to prev. page
                page = flash.prev_page(page.page_number()).unwrap(); // Surely exists as > PS
            }
        }
        // Point 4
        if prev_header_addr == 0 {
            // PS is the first page, just swap
            flash.launch_swap(
                ps.page_number(),
                SwapStartType::ValidHeader,
                0, // Don't care
            );
        } else {
            let prev_block_end: u32 = prev_header_addr + prev_block_size;
            if prev_block_end == ps.base_address() {
                // Case 1
                flash.launch_swap(ps.page_number(), SwapStartType::ValidHeader, 0);
            } else if prev_block_end > ps.base_address() {
                if !prev_allocated {
                    // Case 2
                    flash.launch_swap(
                        ps.page_number(),
                        SwapStartType::DiscardStart,
                        (prev_block_end - ps.base_address()) as usize,
                    );
                } else {
                    // Case 3
                    flash.launch_swap(
                        ps.page_number(),
                        SwapStartType::PreserveStart,
                        (prev_block_end - ps.base_address()) as usize,
                    );
                }
            }
        }
    }

    pub fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        f.write_str("\n------- Block Status -------\n")?;
        let mut block_index = Self::START_SCAN_OFFSET;
        while block_index < ((Self::ALLOCATOR_SIZE - Self::START_SCAN_OFFSET) / BLOCK_SIZE) {
            let offset: usize = block_index * BLOCK_SIZE;
            let block_header = utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
                self.flash,
                offset as u32,
            );
            f.write_fmt(format_args!(
                "[{}] (allocated: {}, dismissed: {}, level: {}, type: {:?})\n",
                block_index,
                block_header.is_allocated(),
                block_header.is_dismissed(),
                block_header.block_level(),
                block_header.block_type()
            ))?;
            let block_size = utils::get_block_size::<START_ADDR, END_ADDR>(&block_header);
            let num_blocks = block_size / BLOCK_SIZE;
            block_index += num_blocks;
        }
        f.write_str("\n------- Allocator free_list -------\n")?;
        self.buddy_allocator.dump(f)?;
        Ok(())
    }
    pub fn allocate(&mut self, size: u32, block_type: BlockType) -> Result<FlashBlock, ()> {
        // Get block
        let actual_size = size + HEADER_SIZE as u32;
        let addr_result = self.buddy_allocator.alloc(actual_size as usize);
        if addr_result.is_none() {
            return Err(());
        }
        let addr = addr_result.unwrap();
        let level: u16 = self
            .buddy_allocator
            .size_to_level(actual_size as usize)
            .unwrap() as u16;
        // Generate header
        let header = BlockHeader::write_buffer(true, false, false, level, block_type);
        // Write header
        self.flash.write(addr, &header).unwrap();
        // Always flush after an header or flag
        self.flash.flush_write_buffer().unwrap();
        // Return only a pointer to the usable space
        return Ok(FlashBlock {
            block_base_address: addr,
            block_type: block_type,
            block_size: Self::ALLOCATOR_SIZE as u32 >> level,
            finalized: false,
        });
    }
    pub fn deallocate(&mut self, base_addr: u32) -> Result<(), ()> {
        // Get back the original start address of the block
        let addr = base_addr - (HEADER_SIZE as u32);
        // Check that this is a valid base_addr
        if utils::get_flash_block::<START_ADDR, END_ADDR, START_SCAN_ADDR, SMALLEST_BLOCK_LEVEL>(
            self.flash, addr, true,
        )
        .is_none()
        {
            return Err(());
        }
        // Deallocate
        let mut block_level: u16 = 0;
        Self::deallocate_procedure(self.flash, addr, &mut block_level);
        // Recollect block as free
        let block_size = (Self::ALLOCATOR_SIZE >> block_level) as u32;
        let num_blocks = block_size / BLOCK_SIZE as u32;
        let first_block = (addr - START_ADDR) / BLOCK_SIZE as u32;
        for b in first_block..first_block + num_blocks {
            self.buddy_allocator.add_free_block(b as usize);
        }
        return Ok(());
    }

    pub fn from_flash(
        flash: &'a mut dyn FlashMethods<'a>,
        skip_storage_analysis: bool,
        remove_unfinalized_blocks: bool,
    ) -> Self {
        // Some asserts
        assert!(START_SCAN_ADDR >= START_ADDR && START_SCAN_ADDR < END_ADDR);
        // Create a new allocator
        let mut allocator = BinaryBuddyImpl::<
            START_ADDR,
            END_ADDR,
            BLOCK_SIZE,
            NUM_BLOCKS,
            SMALLEST_BLOCK_LEVEL,
            TREE_NUM_NODES,
        >::new(true);
        // Check for recovery
        if !skip_storage_analysis {
            Self::analyze_storage(flash, remove_unfinalized_blocks);
        }
        // Scan to reconstruct state
        // Steps of 1 blocks
        let mut process_index: usize = Self::START_SCAN_OFFSET;
        while process_index < Self::ALLOCATOR_SIZE {
            // Read header
            let block_header = utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
                flash,
                process_index as u32,
            );
            // Skip allocated blocks
            if block_header.is_allocated() {
                let block_size = Self::ALLOCATOR_SIZE >> block_header.block_level();
                process_index += block_size;
                continue;
            } else {
                assert!(!block_header.is_dismissed());
                // Add the block
                let block_num = process_index / BLOCK_SIZE / 1;
                allocator.add_free_block(block_num as usize).unwrap();
                process_index += BLOCK_SIZE;
            }
        }
        Self {
            buddy_allocator: allocator,
            flash: flash,
        }
    }

    fn refresh_block(&self, block: &mut FlashBlock) {
        // Read header again
        let block_header = utils::read_block_header::<START_ADDR, SMALLEST_BLOCK_LEVEL>(
            self.flash,
            block.block_base_address - START_ADDR,
        );
        // This are the only fields that change change
        block.block_type = block_header.block_type();
        block.finalized = block_header.is_finalized();
    }
}

impl<
        'a,
        const START_ADDR: u32,      // Allocator start address
        const END_ADDR: u32,        // Allocator end address
        const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const SMALLEST_BLOCK_LEVEL: usize,
        const TREE_NUM_NODES: usize,
    > FlashAllocator<'a>
    for FlashAllocatorImpl<
        'a,
        START_ADDR,
        END_ADDR,
        START_SCAN_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        SMALLEST_BLOCK_LEVEL,
        TREE_NUM_NODES,
    >
{
    fn allocate(&mut self, size: u32, block_type: BlockType) -> Result<FlashBlock, ()> {
        self.allocate(size, block_type)
    }

    fn deallocate(&mut self, addr: u32) -> Result<(), ()> {
        self.deallocate(addr)
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }

    fn refresh(&self, block: &mut FlashBlock) {
        self.refresh_block(block);
    }
}

use abi::flash::BlockType;

use self::page::FlashPage;
use crate::flash::header::BlockHeader;
use buddy_allocator::{BuddyAllocator, BuddyAllocatorImpl};
use core::fmt::Formatter;

#[cfg(feature = "swap")]
use crate::swap::SwapStartType;

pub mod header {
    use core::marker::PhantomData;

    use abi::flash::BlockType;

    /**
     * As the header is very small, and fields are read mutiple times,
     * this implementation copies the header of the block in SRAM when
     * this structure is constructed.
     *
     * It's also possible to avoid this behavior as in hbf_rs
     */
    #[allow(dead_code)]
    pub struct BlockHeader<'a, const FLAG_BYTES: usize> {
        allocated: bool,
        dismissed: bool,
        finalized: bool,
        block_level: u16,
        block_type: BlockType,
        ph: &'a PhantomData<u8>, // Needed to force the lifetime
    }

    impl<'a, const FLAG_BYTES: usize> BlockHeader<'a, FLAG_BYTES> {
        pub const HEADER_SIZE: usize = FLAG_BYTES * 4 + 2 + 2;

        pub fn new(header_address: &'a [u8], max_level: u16) -> Self {
            let ptr = header_address.as_ptr();
            // Construct the structure
            let ptr_flag = ptr as *const [u8; FLAG_BYTES];
            let allocated_flag: [u8; FLAG_BYTES] = unsafe { ptr_flag.read_unaligned() }.into();
            let dismissed_flag: [u8; FLAG_BYTES] =
                unsafe { ptr_flag.add(1).read_unaligned() }.into();
            let finalized_flag: [u8; FLAG_BYTES] =
                unsafe { ptr_flag.add(2).read_unaligned() }.into();
            let block_level_ptr = unsafe { ptr_flag.add(4) } as *const u16;
            let block_level: u16 = unsafe { block_level_ptr.read_unaligned() }.into();
            let block_type_ptr = unsafe { block_level_ptr.add(1) } as *const u16;
            let block_type: u16 = unsafe { block_type_ptr.read_unaligned() }.into();
            let allocated = allocated_flag == [0x00; FLAG_BYTES];
            Self {
                allocated: allocated,
                dismissed: dismissed_flag == [0x00; FLAG_BYTES],
                finalized: finalized_flag == [0x00; FLAG_BYTES],
                block_level: match allocated {
                    true => block_level,
                    false => max_level,
                },
                block_type: BlockType::from(block_type),
                ph: &PhantomData,
            }
        }
        fn write_flag<'b>(dest_buffer: &'b mut [u8], offset: usize, flag: bool) {
            for i in 0..FLAG_BYTES {
                dest_buffer[offset + i] = match flag {
                    true => 0x00,
                    false => 0xFF,
                }
            }
        }
        pub fn write_buffer<'b>(
            allocated: bool,
            dismissed: bool,
            finalized: bool,
            block_level: u16,
            block_type: BlockType,
        ) -> [u8; FLAG_BYTES * 4 + 2 + 2]
        where
            [u8; FLAG_BYTES * 4 + 2 + 2]: Sized,
        {
            let mut buffer: [u8; FLAG_BYTES * 4 + 2 + 2] = [0xFF; FLAG_BYTES * 4 + 2 + 2];
            Self::write_flag(&mut buffer, 0, allocated);
            Self::write_flag(&mut buffer, FLAG_BYTES, dismissed);
            Self::write_flag(&mut buffer, FLAG_BYTES * 2, finalized);
            let level_offset = FLAG_BYTES * 4;
            buffer[level_offset] = block_level.to_le_bytes()[0];
            buffer[level_offset + 1] = block_level.to_le_bytes()[1];
            let flags_offset = level_offset + 2;
            let block_type_u: u16 = block_type.into();
            buffer[flags_offset] = block_type_u.to_le_bytes()[0];
            buffer[flags_offset + 1] = block_type_u.to_le_bytes()[1];
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

pub mod page {
    #[derive(Clone, Copy)]
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

pub mod walker {
    use super::utils;
    use super::{header::BlockHeader, FlashBlock, FlashMethods};

    pub struct FlashWalkerImpl<
        'a,
        'b,
        const START_ADDR: u32,      // Allocator start address
        const END_ADDR: u32,        // Allocator end address
        const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
        const NUM_SLOTS: usize,
        const BLOCK_SIZE: usize,
        const FLAG_BYTES: usize,
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
            const NUM_SLOTS: usize,
            const BLOCK_SIZE: usize,
            const FLAG_BYTES: usize,
        >
        FlashWalkerImpl<
            'a,
            'b,
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            NUM_SLOTS,
            BLOCK_SIZE,
            FLAG_BYTES,
        >
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
            const NUM_SLOTS: usize,
            const BLOCK_SIZE: usize,
            const FLAG_BYTES: usize,
        > Iterator
        for FlashWalkerImpl<
            'a,
            'b,
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            NUM_SLOTS,
            BLOCK_SIZE,
            FLAG_BYTES,
        >
    {
        type Item = FlashBlock;
        fn next(&mut self) -> Option<Self::Item> {
            // Scan for next valid block
            while self.current_offset < Self::ALLOCATOR_SIZE {
                // Read header of the next block
                let block_header = utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
                    self.flash,
                    self.current_offset as u32,
                );
                // Skip deallocated blocks
                if block_header.is_allocated() && !block_header.is_dismissed() {
                    // Construct result
                    let result = FlashBlock {
                        block_base_address: START_ADDR
                            + self.current_offset as u32
                            + (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32),
                        block_type: block_header.block_type(),
                        block_size: utils::get_block_size::<
                            START_ADDR,
                            END_ADDR,
                            BLOCK_SIZE,
                            FLAG_BYTES,
                        >(&block_header) as u32
                            - (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32),
                        finalized: block_header.is_finalized(),
                    };
                    // Prepare for the next call
                    self.current_offset +=
                        utils::get_block_size::<START_ADDR, END_ADDR, BLOCK_SIZE, FLAG_BYTES>(
                            &block_header,
                        );
                    return Some(result);
                } else {
                    // Skip the block
                    self.current_offset +=
                        utils::get_block_size::<START_ADDR, END_ADDR, BLOCK_SIZE, FLAG_BYTES>(
                            &block_header,
                        );
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
            const NUM_SLOTS: usize,
            const BLOCK_SIZE: usize,
            const FLAG_BYTES: usize,
        > FlashMethods<'a>
        for FlashWalkerImpl<
            'a,
            'b,
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            NUM_SLOTS,
            BLOCK_SIZE,
            FLAG_BYTES,
        >
    {
        fn read(&self, address: u32, len: usize) -> Result<&'a [u8], ()> {
            self.flash.read(address, len)
        }

        fn write(&mut self, address: u32, value: u8) -> Result<(), ()> {
            self.flash.write(address, value)
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
            const NUM_SLOTS: usize,
            const BLOCK_SIZE: usize,
            const FLAG_BYTES: usize,
        > FlashWalker
        for FlashWalkerImpl<
            'a,
            'b,
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            NUM_SLOTS,
            BLOCK_SIZE,
            FLAG_BYTES,
        >
    {
        fn reset(&mut self) {
            self.reset()
        }
    }
}

pub mod utils {
    use abi::flash::BlockType;

    use super::{header::BlockHeader, FlashBlock, FlashMethods};

    pub fn read_block_header<
        'a,
        const FLAG_BYTES: usize,
        const START_ADDR: u32,
        const NUM_SLOTS: usize,
    >(
        flash: &dyn FlashMethods<'a>,
        offset: u32,
    ) -> BlockHeader<'a, FLAG_BYTES> {
        let header_buffer = flash
            .read(START_ADDR + offset, BlockHeader::<FLAG_BYTES>::HEADER_SIZE)
            .unwrap();
        let block_header: BlockHeader<FLAG_BYTES> = BlockHeader::<FLAG_BYTES>::new(
            header_buffer,
            buddy_allocator::get_max_level::<NUM_SLOTS>() as u16,
        );
        return block_header;
    }

    pub fn get_block_size<
        'a,
        const START_ADDR: u32,
        const END_ADDR: u32,
        const BLOCK_SIZE: usize,
        const FLAG_BYTES: usize,
    >(
        block_header: &BlockHeader<'a, FLAG_BYTES>,
    ) -> usize {
        let size = (END_ADDR - START_ADDR + 1) as usize;
        if block_header.is_allocated() {
            size >> block_header.block_level()
        } else {
            BLOCK_SIZE
        }
    }

    pub fn get_flash_block<
        'a,
        const START_ADDR: u32,
        const END_ADDR: u32,
        const START_SCAN_ADDR: u32,
        const NUM_SLOTS: usize,
        const BLOCK_SIZE: usize,
        const FLAG_BYTES: usize,
    >(
        flash: &dyn FlashMethods<'a>,
        mut base_address: u32,
        is_base_exact: bool,
    ) -> Option<FlashBlock> {
        let size = (END_ADDR - START_ADDR + 1) as usize;
        let start_scan_offset: usize = (START_SCAN_ADDR - START_ADDR) as usize;
        // Convert address if its not already pointing to the exact start
        if !is_base_exact {
            base_address -= BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32;
        }
        let mut process_index: usize = start_scan_offset;
        while process_index < size {
            // Read header
            let block_header =
                read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(flash, process_index as u32);
            // Allocated blocks
            if block_header.is_allocated() && !block_header.is_dismissed() {
                // Check this is the block
                if process_index as u32 + START_ADDR == base_address {
                    return Some(FlashBlock {
                        block_base_address: base_address
                            + (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32),
                        finalized: block_header.is_finalized(),
                        block_type: block_header.block_type(),
                        block_size: (size as u32 >> block_header.block_level())
                            - (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32),
                    });
                }
                // Countinue scanning
                let block_size = size >> block_header.block_level();
                process_index += block_size;
            } else {
                // Continue scanning
                process_index += BLOCK_SIZE;
            }
        }
        return None;
    }

    /// Sets the flag on a flash block
    pub fn mark_block<'a, const START_ADDR: u32, const NUM_SLOTS: usize, const FLAG_BYTES: usize>(
        flash: &mut dyn FlashMethods<'a>,
        block: FlashBlock,
        block_type: BlockType,
    ) -> Result<(), ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Read again the header to be safe
        let block_base_addr =
            block.get_base_address() - BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32;
        let curr_header = self::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
            flash,
            block_base_addr - START_ADDR,
        );

        // Check if this operation is possible
        if !curr_header.is_allocated() || curr_header.is_dismissed() || curr_header.is_finalized() {
            return Err(());
        }
        if curr_header.block_type() != BlockType::NONE {
            return Err(());
        }

        // Generating the new header
        let header = BlockHeader::<FLAG_BYTES>::write_buffer(
            true,
            false,
            false,
            curr_header.block_level(),
            block_type,
        );
        // Writing new header
        for i in 0..header.len() {
            flash.write(block_base_addr + i as u32, header[i]).unwrap();
        }
        // Always flush after an header or flag
        flash.flush_write_buffer().unwrap();

        Ok(())
    }

    /// Sets the flag on a flash block
    pub fn finalize_block<'a, const START_ADDR: u32, const NUM_SLOTS: usize, const FLAG_BYTES: usize>(
        flash: &mut dyn FlashMethods<'a>,
        block: FlashBlock
    ) -> Result<(), ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Read again the header to be safe
        let block_base_addr =
            block.get_base_address() - BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32;
        let curr_header = self::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
            flash,
            block_base_addr - START_ADDR,
        );

        // Check if this operation is possible
        if !curr_header.is_allocated() || curr_header.is_dismissed() || curr_header.is_finalized() {
            return Err(());
        }

        // Generating the new header
        let header = BlockHeader::<FLAG_BYTES>::write_buffer(
            true,
            false,
            true,
            curr_header.block_level(),
            curr_header.block_type(),
        );
        // Writing new header
        for i in 0..header.len() {
            flash.write(block_base_addr + i as u32, header[i]).unwrap();
        }
        // Always flush after an header or flag
        flash.flush_write_buffer().unwrap();

        Ok(())
    }

    /// Sets the flag on a flash block
    pub unsafe fn mark_block_dismissed<'a, const START_ADDR: u32, const NUM_SLOTS: usize, const FLAG_BYTES: usize>(
        flash: &mut dyn FlashMethods<'a>,
        block: FlashBlock
    ) -> Result<(), ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Read again the header to be safe
        let block_base_addr =
            block.get_base_address() - BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32;
        let curr_header = self::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
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
        let header = BlockHeader::<FLAG_BYTES>::write_buffer(
            true,
            true,
            curr_header.is_finalized(),
            curr_header.block_level(),
            curr_header.block_type(),
        );
        // Writing new header
        for i in 0..header.len() {
            flash.write(block_base_addr + i as u32, header[i]).unwrap();
        }
        // Always flush after an header or flag
        flash.flush_write_buffer().unwrap();

        Ok(())
    }
}
/// Interface for interacting with flash memory.
pub trait FlashMethods<'a> {
    /// Reads a slice of len bytes starting from the specified offset
    fn read(&self, address: u32, len: usize) -> Result<&'a [u8], ()>;
    /// Writes a byte to the corresponding offset
    fn write(&mut self, address: u32, value: u8) -> Result<(), ()>;
    /// In case writes to flash memory are buffered, forces the synchronization.
    /// Otherwise, is a nop.
    fn flush_write_buffer(&mut self) -> Result<(), ()>;
    /// Retrieve the page number from the offset
    fn page_from_address(&self, address: u32) -> Option<FlashPage>;
    /// Retrieve the page from a page number
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage>;
    /// Retrieve the prev. page from the offset
    fn prev_page(&self, page_num: u16) -> Option<FlashPage>;
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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FlashBlock {
    block_base_address: u32,
    finalized: bool,
    block_type: BlockType,
    block_size: u32,
}
impl<'a> FlashBlock {
    pub fn get_base_address(&self) -> u32 {
        self.block_base_address
    }
    pub fn get_type(&self) -> BlockType {
        self.block_type
    }
    pub fn get_size(&self) -> u32 {
        match self.block_type {
            BlockType::COMPONENT => self.block_size - 8,
            _ => self.block_size
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
            _ => self.block_base_address
        };
        core::slice::from_raw_parts(
            base_address as *const u8,
            self.get_size() as usize,
        )
    }
}

/// Interface offered to the storage component
pub trait FlashAllocator<'a, const FLAG_BYTES: usize> {
    /// Allocate the requested size. If enough space is available,
    /// the block is provisioned and the base address of the block is returned.
    /// N.B.: The base address of the block already points to usable space.
    ///       The actual available size is less that the requested, as the block starts with the header.
    ///       To get the actual size, use the get_size method below.
    fn allocate(&mut self, size: u32) -> Result<FlashBlock, ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized;
    /// Deallocate the block that starts at the provided base address
    fn deallocate(&mut self, base_addr: u32) -> Result<(), ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized;

    /// Calculate the actual size of the block (nominal block size - header size)
    fn refresh(&self, block: &mut FlashBlock);

    /// Dumps the state of the allocator
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error>;
}

pub struct FlashAllocatorImpl<
    'a,
    const START_ADDR: u32,      // Allocator start address
    const END_ADDR: u32,        // Allocator end address
    const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
    const BLOCK_SIZE: usize,    // Minimum granularity of the allocator
    const NUM_BLOCKS: usize,
    const NUM_SLOTS: usize,
    const FLAG_BYTES: usize, // Number of bytes to reserve for each flag
> {
    buddy_allocator: BuddyAllocatorImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>,
    flash: &'a mut dyn FlashMethods<'a>,
}

impl<
        'a,
        const START_ADDR: u32,      // Allocator start address
        const END_ADDR: u32,        // Allocator end address
        const START_SCAN_ADDR: u32, // Position of the first block (>= START_ADDR)
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const NUM_SLOTS: usize,
        const FLAG_BYTES: usize,
    >
    FlashAllocatorImpl<
        'a,
        START_ADDR,
        END_ADDR,
        START_SCAN_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        NUM_SLOTS,
        FLAG_BYTES,
    >
{
    const ALLOCATOR_SIZE: usize = (END_ADDR - START_ADDR + 1) as usize;
    const START_SCAN_OFFSET: usize = (START_SCAN_ADDR - START_ADDR) as usize;

    pub fn analyze_storage(flash: &mut dyn FlashMethods<'a>)
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        let mut process_index: usize = Self::START_SCAN_OFFSET;
        while process_index < Self::ALLOCATOR_SIZE {
            // Read header
            let block_header = utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
                flash,
                process_index as u32,
            );
            // Allocated blocks
            if block_header.is_allocated() {
                let block_size = Self::ALLOCATOR_SIZE >> block_header.block_level();
                // Check if a freed_block exists
                if block_header.is_dismissed() {
                    // Launch the erase again
                    let mut block_level_out: u16 = 0;
                    Self::deallocate_procedure(
                        flash,
                        START_ADDR + process_index as u32,
                        &mut block_level_out,
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

    fn deallocate_procedure(flash: &mut dyn FlashMethods<'a>, addr: u32, block_level_out: &mut u16)
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Check position
        assert!(addr >= START_SCAN_ADDR && addr <= END_ADDR);
        let offset: usize = (addr - START_ADDR) as usize;
        // Read header
        let block_header =
            utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(flash, offset as u32);
        let block_level = block_header.block_level();
        // Generating the new header
        let header = BlockHeader::<FLAG_BYTES>::write_buffer(
            true,
            true,
            block_header.is_finalized(),
            block_level,
            block_header.block_type(),
        );
        drop(block_header); // Needed to release flash object
                            // Write the new header
        for i in 0..header.len() {
            flash.write(addr + i as u32, header[i]).unwrap();
        }
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
                utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(flash, curr_offset);
            let block_size = (Self::ALLOCATOR_SIZE >> header.block_level()) as u32;
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
            let block_header = utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
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
            let block_size = (Self::ALLOCATOR_SIZE >> block_header.block_level()) as usize;
            let num_blocks = block_size / BLOCK_SIZE;
            block_index += num_blocks;
        }
        f.write_str("\n------- Allocator free_list -------\n")?;
        self.buddy_allocator.dump(f)?;
        Ok(())
    }
    pub fn allocate(&mut self, size: u32) -> Result<FlashBlock, ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Get block
        let addr_result = self.buddy_allocator.alloc(size as usize);
        if addr_result.is_none() {
            return Err(());
        }
        let addr = addr_result.unwrap();
        let level: u16 = self.buddy_allocator.size_to_level(size as usize).unwrap() as u16;
        // Generate header
        let header =
            BlockHeader::<FLAG_BYTES>::write_buffer(true, false, false, level, BlockType::NONE);
        // Write header
        for i in 0..header.len() {
            self.flash.write(addr + i as u32, header[i]).unwrap();
        }
        // Always flush after an header or flag
        self.flash.flush_write_buffer().unwrap();
        // Return only a pointer to the usable space
        return Ok(FlashBlock {
            block_base_address: addr + (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32),
            block_type: BlockType::NONE,
            block_size: (Self::ALLOCATOR_SIZE as u32 >> level)
                - (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32),
            finalized: false,
        });
    }
    pub fn deallocate(&mut self, base_addr: u32) -> Result<(), ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Get back the original start address of the block
        let addr = base_addr - (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32);
        // Check that this is a valid base_addr
        if utils::get_flash_block::<
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            NUM_SLOTS,
            BLOCK_SIZE,
            FLAG_BYTES,
        >(self.flash, addr, true)
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
            self.buddy_allocator.add_free_block(b as u8);
        }
        return Ok(());
    }

    pub fn from_flash(flash: &'a mut dyn FlashMethods<'a>, skip_storage_analysis: bool) -> Self
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Some asserts
        assert!(START_SCAN_ADDR >= START_ADDR && START_SCAN_ADDR < END_ADDR);
        // Create a new allocator
        let mut allocator =
            BuddyAllocatorImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>::new(
                true,
            );
        // Check for recovery
        if !skip_storage_analysis {
            Self::analyze_storage(flash);
        }
        // Scan to reconstruct state
        // Steps of 1 blocks
        let mut process_index: usize = Self::START_SCAN_OFFSET;
        while process_index < Self::ALLOCATOR_SIZE {
            // Read header
            let block_header = utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
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
                allocator.add_free_block(block_num as u8).unwrap();
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
        let block_header = utils::read_block_header::<FLAG_BYTES, START_ADDR, NUM_SLOTS>(
            self.flash,
            block.block_base_address - (BlockHeader::<FLAG_BYTES>::HEADER_SIZE as u32) - START_ADDR,
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
        const NUM_SLOTS: usize,
        const FLAG_BYTES: usize,
    > FlashAllocator<'a, FLAG_BYTES>
    for FlashAllocatorImpl<
        'a,
        START_ADDR,
        END_ADDR,
        START_SCAN_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        NUM_SLOTS,
        FLAG_BYTES,
    >
{
    fn allocate(&mut self, size: u32) -> Result<FlashBlock, ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        self.allocate(size)
    }

    fn deallocate(&mut self, addr: u32) -> Result<(), ()>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        self.deallocate(addr)
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }

    fn refresh(&self, block: &mut FlashBlock) {
        self.refresh_block(block);
    }
}

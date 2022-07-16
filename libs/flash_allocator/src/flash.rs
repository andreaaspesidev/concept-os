use abi::flash::{BlockHeader, BlockHeaderGen};

use crate::buddy::{BuddyAllocator, BuddyAllocatorImpl};
use core::fmt::Formatter;

#[cfg(feature = "swap")]
use crate::swap::SwapStartType;

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

/// Interface for interacting with flash memory.
pub trait FlashMethods<'a> {
    /// Reads a slice of len bytes starting from the specified offset
    fn read(&self, address: u32, len: usize) -> &'a [u8];
    /// Writes a byte to the corresponding offset
    fn write(&mut self, address: u32, value: u8);
    /// Retrieve the page number from the offset
    fn page_from_address(&self, address: u32) -> Option<FlashPage>;
    /// Retrieve the page from a page number
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage>;
    /// Retrieve the prev. page from the offset
    fn prev_page(&self, page_num: u16) -> Option<FlashPage>;
    /// Erase a page number
    fn erase(&mut self, page_num: u16);

    #[cfg(feature = "swap")]
    /// Launch the swap procedure
    fn launch_swap(
        &mut self,
        page_number: u16,
        start_type: SwapStartType,
        start_size: usize,
    ) -> crate::swap::SwapResult;
}

pub trait FlashAllocator<'a, const FLAG_BYTES: usize> {
    fn allocate(&mut self, size: u32) -> Option<u32>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized;
    fn deallocate(&mut self, base_addr: u32)
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized;
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error>;
}

pub struct FlashAllocatorImpl<
    'a,
    const START_ADDR: u32,
    const END_ADDR: u32,
    const BLOCK_SIZE: usize,
    const NUM_BLOCKS: usize,
    const NUM_SLOTS: usize,
    const FLAG_BYTES: usize,
> {
    buddy_allocator: BuddyAllocatorImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>,
    flash: &'a mut dyn FlashMethods<'a>,
}

impl<
        'a,
        const START_ADDR: u32,
        const END_ADDR: u32,
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const NUM_SLOTS: usize,
        const FLAG_BYTES: usize,
    > FlashAllocatorImpl<'a, START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS, FLAG_BYTES>
{
    const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize;

    pub fn new(flash: &'a mut dyn FlashMethods<'a>) -> Self {
        Self {
            buddy_allocator: BuddyAllocatorImpl::<
                START_ADDR,
                END_ADDR,
                BLOCK_SIZE,
                NUM_BLOCKS,
                NUM_SLOTS,
            >::new(false),
            flash: flash,
        }
    }
    fn check_for_recovery(flash: &mut dyn FlashMethods<'a>, max_level: usize)
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        let mut process_index: usize = 0;
        while process_index < Self::SIZE {
            // Read header
            let block_header = Self::read_block_header(flash, process_index as u32, max_level);
            // Allocated blocks
            if block_header.is_allocated() {
                // Check if a freed_block exits
                if block_header.is_dismissed() {
                    // Launch the erase again
                    let mut block_level_out: u16 = 0;
                    Self::deallocate_procedure(
                        flash,
                        max_level,
                        START_ADDR + process_index as u32,
                        &mut block_level_out,
                    );
                    return;
                }
                // Otherwise continue scanning
                let block_size = Self::SIZE >> block_header.block_level();
                process_index += block_size;
                continue;
            } else {
                assert!(!block_header.is_dismissed());
                // Continue scanning
                process_index += BLOCK_SIZE;
            }
        }
    }
    fn read_block_header(
        flash: &dyn FlashMethods<'a>,
        offset: u32,
        max_level: usize,
    ) -> impl BlockHeader<'a, FLAG_BYTES> {
        let header_buffer = flash.read(
            START_ADDR + offset,
            BlockHeaderGen::<FLAG_BYTES>::HEADER_SIZE,
        );
        //for i in (0..header_buffer.len()).step_by(4) {
        //    println!("[RAW {}] {:#010x}", i, u32_from_arr(&header_buffer[i..i+4]));
        //}
        let block_header: BlockHeaderGen<FLAG_BYTES> =
            BlockHeaderGen::<FLAG_BYTES>::new(header_buffer, max_level as u16);
        return block_header;
    }

    fn deallocate_procedure(
        flash: &mut dyn FlashMethods<'a>,
        max_level: usize,
        addr: u32,
        block_level_out: &mut u16,
    ) where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Check position
        assert!(addr >= START_ADDR && addr <= END_ADDR);
        let offset: usize = (addr - START_ADDR) as usize;
        // Read header
        let block_header = Self::read_block_header(flash, offset as u32, max_level);
        let block_level = block_header.block_level();
        // Generating the new header
        let header = BlockHeaderGen::<FLAG_BYTES>::write_buffer(
            true,
            true,
            false,
            block_level,
            block_header.block_type(),
        );
        drop(block_header); // Needed to release flash object
                            // Write the new header
        for i in 0..header.len() {
            flash.write(addr + i as u32, header[i]);
        }
        // Erase block
        Self::block_erase_procedure(flash, addr, block_level as usize, max_level);
        // Pass parameters
        *block_level_out = block_level;
    }

    #[cfg(not(feature = "swap"))]
    fn block_erase_procedure(
        flash: &mut dyn FlashMethods<'a>,
        block_start_addr: u32,
        block_level: usize,
        _max_level: usize,
    ) {
        // Start erasing from the last page
        let block_size = (Self::SIZE >> block_level) as u32;
        let mut current_addr = block_start_addr + block_size - 1; // -1 or we will enter the next page
        while current_addr >= block_start_addr {
            // Get the page
            let page = flash.page_from_address(current_addr).unwrap();
            // Erase the page
            flash.erase(page.page_num);
            // Move one page down
            current_addr -= page.size();
        }
    }

    #[cfg(feature = "swap")]
    fn block_erase_procedure(
        flash: &mut dyn FlashMethods<'a>,
        block_start_addr: u32,
        block_level: usize,
        max_level: usize,
    ) {
        let block_size = Self::SIZE >> block_level;
        // Point 2
        // - Get PS
        let ps = flash.page_from_address(block_start_addr).unwrap();
        // - Find PREV_HEADER
        let mut curr_offset: u32 = 0;
        let mut prev_header_addr: u32 = 0; // It is never a valid address
        let mut prev_block_size: u32 = 0;
        let mut prev_allocated: bool = false;
        loop {
            let header = Self::read_block_header(flash, curr_offset, max_level);
            let block_size = (Self::SIZE >> header.block_level()) as u32;
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
                flash.erase(page.page_number());
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
        let mut block_index = 0;
        while block_index < (Self::SIZE / BLOCK_SIZE) {
            let offset: usize = block_index * BLOCK_SIZE;
            let block_header = Self::read_block_header(
                self.flash,
                offset as u32,
                self.buddy_allocator.max_level(),
            );
            f.write_fmt(format_args!(
                "[{}] (allocated: {}, dismissed: {}, level: {}, type: {:?})\n",
                block_index,
                block_header.is_allocated(),
                block_header.is_dismissed(),
                block_header.block_level(),
                block_header.block_type()
            ))?;
            let block_size = (Self::SIZE >> block_header.block_level()) as usize;
            let num_blocks = block_size / BLOCK_SIZE;
            block_index += num_blocks;
        }
        f.write_str("\n------- Allocator free_list -------\n")?;
        self.buddy_allocator.dump(f)?;
        Ok(())
    }
    pub fn allocate(&mut self, size: u32) -> Option<u32>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Get block
        let addr_result = self.buddy_allocator.alloc(size as usize);
        if addr_result.is_none() {
            return None;
        }
        let addr = addr_result.unwrap();
        let level: u16 = self.buddy_allocator.size_to_level(size as usize).unwrap() as u16;
        // Generate header
        let header = BlockHeaderGen::<FLAG_BYTES>::write_buffer(
            true,
            false,
            false,
            level,
            abi::flash::BlockType::UNKNOWN(0xFFFF),
        );
        // Write header
        for i in 0..header.len() {
            self.flash.write(addr + i as u32, header[i]);
        }
        // Return only a pointer to the usable space
        return Some(addr + (BlockHeaderGen::<FLAG_BYTES>::HEADER_SIZE as u32));
    }
    pub fn deallocate(&mut self, base_addr: u32)
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Get back the original start address of the block
        let addr = base_addr - (BlockHeaderGen::<FLAG_BYTES>::HEADER_SIZE as u32);
        // Deallocate
        let mut block_level: u16 = 0;
        Self::deallocate_procedure(
            self.flash,
            self.buddy_allocator.max_level(),
            addr,
            &mut block_level,
        );
        // Recollect block as free
        let block_size = (Self::SIZE >> block_level) as u32;
        let num_blocks = block_size / BLOCK_SIZE as u32;
        let first_block = (addr - START_ADDR) / BLOCK_SIZE as u32;
        for b in first_block..first_block + num_blocks {
            self.buddy_allocator.add_free_block(b as u8);
        }
    }

    pub fn from_flash(flash: &'a mut dyn FlashMethods<'a>) -> Self
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Create a new allocator
        let mut allocator =
            BuddyAllocatorImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>::new(
                true,
            );
        // Check for recovery
        Self::check_for_recovery(flash, allocator.max_level());
        // Scan to reconstruct state
        // Steps of 1 blocks
        let mut process_index: usize = 0;
        while process_index < Self::SIZE {
            // Read header
            let block_header =
                Self::read_block_header(flash, process_index as u32, allocator.max_level());
            // Skip allocated blocks
            if block_header.is_allocated() {
                let block_size = Self::SIZE >> block_header.block_level();
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
}

impl<
        'a,
        const START_ADDR: u32,
        const END_ADDR: u32,
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const NUM_SLOTS: usize,
        const FLAG_BYTES: usize,
    > FlashAllocator<'a, FLAG_BYTES>
    for FlashAllocatorImpl<'a, START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS, FLAG_BYTES>
{
    fn allocate(&mut self, size: u32) -> Option<u32>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        self.allocate(size)
    }

    fn deallocate(&mut self, addr: u32)
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        self.deallocate(addr)
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }
}

/*
    SWAP Procedure
*/

use crate::flash::{header::BlockHeader, FlashMethods};

/// u32 from big endian bytes
/// 0x12345678
/// [0] 0x12
/// [1] 0x34
/// [2] 0x56
/// [3] 0x78
fn u32_be_from_array(arr: &[u8]) -> u32 {
    ((arr[3] as u32) << 0)
        + ((arr[2] as u32) << 8)
        + ((arr[1] as u32) << 16)
        + ((arr[0] as u32) << 24)
}

/// u16 from big endian bytes
/// 0x1234
/// [0] 0x12
/// [1] 0x34
fn u16_be_from_array(arr: &[u8]) -> u16 {
    ((arr[1] as u16) << 0) + ((arr[0] as u16) << 8)
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SwapStartType {
    ValidHeader = 0,
    PreserveStart = 1,
    DiscardStart = 2,
}

pub enum SwapResult {
    Success = 0,
    UnknownPage = 1,
    SwapSmall = 2,
}

pub trait Swapper<'a> {
    /// Performs the swap operation
    fn swap_procedure(
        &mut self,
        page_number: u16,
        start_type: SwapStartType,
        start_size: u32,
    ) -> SwapResult;
    /// Must be called at system start-up to recover from pending swapping operations
    fn recovery_procedure(&mut self);
}

pub struct SwapperImpl<
    'a,
    const BLOCK_SIZE: usize,
    const BLOCK_MAX_LEVEL: u16,
    const ALLOCATOR_SIZE: usize,
    const FLAG_BYTES: usize,
    const SWAP_PAGE_NUM: u16,
> {
    flash: &'a mut dyn FlashMethods<'a>,
    swap_init: bool,
    current_position: u32,
}

impl<
        'a,
        const BLOCK_SIZE: usize,
        const BLOCK_MAX_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const FLAG_BYTES: usize,
        const SWAP_PAGE_NUM: u16,
    > SwapperImpl<'a, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_BYTES, SWAP_PAGE_NUM>
{
    pub fn new(flash: &'a mut dyn FlashMethods<'a>) -> Self {
        Self {
            flash: flash,
            swap_init: false,
            current_position: 0,
        }
    }

    fn read_block_header(&self, address: u32) -> BlockHeader<'a, FLAG_BYTES> {
        let header_buffer = self
            .flash
            .read(address, BlockHeader::<FLAG_BYTES>::HEADER_SIZE);
        let block_header: BlockHeader<FLAG_BYTES> =
            BlockHeader::<FLAG_BYTES>::new(header_buffer, BLOCK_MAX_LEVEL);
        return block_header;
    }

    fn init_swap(&mut self, page_number: u16) {
        // Skip if already done
        if self.swap_init {
            return;
        }
        // Write PAGE_NUM
        let swap_page = self.flash.page_from_number(SWAP_PAGE_NUM).unwrap();
        self.flash
            .write(swap_page.base_address(), (page_number >> 8) as u8); // High word
        self.flash
            .write(swap_page.base_address() + 1, (page_number & 0xFF) as u8); // Low word
                                                                              // Mark completed
        self.current_position += 2 + FLAG_BYTES as u32; // The header size
        self.swap_init = true;
    }

    fn add_fragment(
        &mut self,
        frgm_target: u32,
        frgm_size: u32,
        page_start_addr: u32,
        swap_start_addr: u32,
    ) {
        // Write header
        let mut buff: [u8; 4];
        // 1 - write target address
        buff = frgm_target.to_be_bytes();
        for b in buff {
            self.flash.write(swap_start_addr + self.current_position, b);
            self.current_position += 1;
        }
        // 2 - write fragment size
        buff = frgm_size.to_be_bytes();
        for b in buff {
            self.flash.write(swap_start_addr + self.current_position, b);
            self.current_position += 1;
        }
        // 3 - copy fragment data
        let mut read_pos: u32 = page_start_addr + frgm_target;
        let frgm_end_addr = read_pos + frgm_size;
        while read_pos < frgm_end_addr {
            // todo: check if included or not
            let data = self.flash.read(read_pos, 1);
            assert!(data.len() == 1);
            self.flash
                .write(swap_start_addr + self.current_position, data[0]);
            self.current_position += 1;
            read_pos += 1;
        }
    }

    fn contains_allocated_blocks(&self, scan_start_addr: u32, scan_end_addr: u32) -> bool {
        let mut curr_pos = scan_start_addr;
        while curr_pos <= scan_end_addr {
            let header = self.read_block_header(curr_pos);
            if header.is_allocated() && !header.is_dismissed() {
                return true;
            }
            curr_pos += (ALLOCATOR_SIZE >> header.block_level()) as u32;
        }
        return false;
    }

    fn read_u32(&self, address: u32) -> u32 {
        let buff = self.flash.read(address, 4);
        return u32_be_from_array(buff);
    }
    fn read_u16(&self, address: u32) -> u16 {
        let buff = self.flash.read(address, 2);
        return u16_be_from_array(buff);
    }
    fn read_flag(&self, address: u32) -> bool {
        let buff = self.flash.read(address, FLAG_BYTES);
        return buff == [0x00; FLAG_BYTES];
    }

    fn copy_back(&mut self, target_page_start_addr: u32) {
        // Write back each fragment
        let swap_page = self.flash.page_from_number(SWAP_PAGE_NUM).unwrap();
        // Start from the first fragment
        let mut curr_pos = swap_page.base_address() + 2 + FLAG_BYTES as u32; // The header size
        let mut frgm_target = self.read_u32(curr_pos);
        curr_pos += 4;
        let mut frgm_size = self.read_u32(curr_pos);
        curr_pos += 4;
        while frgm_size < 0xFFFF_FFFF {
            // Empty header, finished fragments
            // Copy back fragment
            for i in 0..frgm_size {
                let data = self.flash.read(curr_pos + i, 1);
                self.flash
                    .write(target_page_start_addr + frgm_target + i, data[0]);
            }
            curr_pos += frgm_size;
            if curr_pos + 4 >= swap_page.end_address() {
                break; // We reached the end of the swap, surely no fragment after this
            }
            // Read again
            frgm_target = self.read_u32(curr_pos);
            curr_pos += 4;
            frgm_size = self.read_u32(curr_pos);
            curr_pos += 4;
        }
    }
}

impl<
        'a,
        const BLOCK_SIZE: usize,
        const BLOCK_MAX_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const FLAG_BYTES: usize,
        const SWAP_PAGE_NUM: u16,
    > Swapper<'a>
    for SwapperImpl<'a, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_BYTES, SWAP_PAGE_NUM>
{
    fn swap_procedure(
        &mut self,
        page_number: u16,
        start_type: SwapStartType,
        start_size: u32,
    ) -> SwapResult {
        let swap_page = self.flash.page_from_number(SWAP_PAGE_NUM).unwrap();
        // Check this page exists
        let search_page = self.flash.page_from_number(page_number);
        if search_page.is_none() {
            return SwapResult::UnknownPage;
        }
        let page = search_page.unwrap();
        // Check page size
        if page.size() > swap_page.size() {
            return SwapResult::SwapSmall;
        }
        // 2.1 - get the first valid header
        let mut curr_pos: u32 = page.base_address();
        if start_type == SwapStartType::PreserveStart {
            // Case 2 - add the fragment
            self.init_swap(page_number);
            self.add_fragment(0, start_size, page.base_address(), swap_page.base_address());
            curr_pos += start_size;
        } else if start_type == SwapStartType::DiscardStart {
            // Case 3
            curr_pos += start_size;
        }
        // 2.2 - check if swap not needed
        if start_type != SwapStartType::PreserveStart {
            if !self.contains_allocated_blocks(curr_pos, page.end_address()) {
                // Just erase the page
                self.flash.erase(page.page_number());
                return SwapResult::Success;
            }
        }
        // 2.3 - copy all fragments
        self.init_swap(page_number);
        while curr_pos <= page.end_address() {
            let header = self.read_block_header(curr_pos);
            let block_size = (ALLOCATOR_SIZE >> header.block_level()) as u32;
            if header.is_allocated() & !header.is_dismissed() {
                let page_remaining = page.end_address() - curr_pos + 1;
                self.add_fragment(
                    curr_pos - page.base_address(),
                    core::cmp::min(block_size, page_remaining),
                    page.base_address(),
                    swap_page.base_address(),
                );
            }
            curr_pos += block_size;
        }
        // 3 - mark copy completed, erase target page
        for i in 0..(FLAG_BYTES as u32) {
            self.flash.write(swap_page.base_address() + 2 + i, 0x00);
        }
        self.flash.erase(page_number);
        // 4 - copy back
        self.copy_back(page.base_address());
        // 5 - erase swap
        self.flash.erase(SWAP_PAGE_NUM);

        return SwapResult::Success;
    }

    fn recovery_procedure(&mut self) {
        let swap_page = self.flash.page_from_number(SWAP_PAGE_NUM).unwrap();
        let page_number = self.read_u16(swap_page.base_address());
        if page_number == 0xFFFF {
            // Everything okay, skip
            return;
        }
        let page = self.flash.page_from_number(page_number).unwrap();
        // Read flag
        let copy_completed = self.read_flag(swap_page.base_address() + 2);
        if !copy_completed {
            // Safe to erase and return
            self.flash.erase(SWAP_PAGE_NUM);
            return;
        }
        // Erase again target page
        self.flash.erase(page_number);
        // 4 - copy back
        self.copy_back(page.base_address());
        // 5 - erase swap
        self.flash.erase(SWAP_PAGE_NUM);
    }
}

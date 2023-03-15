// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/*
    SWAP Procedure
*/

use crate::flash_allocator::{
    flash::{
        header::{self, BlockHeader},
        FlashMethods, HEADER_SIZE,
    },
    FLAG_BYTES,
};

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
    FlashError = 3,
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
    const SMALLEST_BLOCK_LEVEL: u16,
    const ALLOCATOR_SIZE: usize,
    const SWAP_PAGE_NUM: u16,
> {
    flash: &'a mut dyn FlashMethods<'a>,
    swap_init: bool,
    current_position: u32,
}

impl<
        'a,
        const BLOCK_SIZE: usize,
        const SMALLEST_BLOCK_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const SWAP_PAGE_NUM: u16,
    > SwapperImpl<'a, BLOCK_SIZE, SMALLEST_BLOCK_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM>
{
    pub fn new(flash: &'a mut dyn FlashMethods<'a>) -> Self {
        Self {
            flash: flash,
            swap_init: false,
            current_position: 0,
        }
    }

    fn read_block_header(&self, address: u32) -> BlockHeader {
        let mut header_buffer: [u8; HEADER_SIZE] = [0x00; HEADER_SIZE];
        self.flash.read(address, &mut header_buffer).unwrap();
        let block_header: BlockHeader = BlockHeader::new(&header_buffer, SMALLEST_BLOCK_LEVEL);
        return block_header;
    }

    fn init_swap(&mut self, page_number: u16) {
        // Skip if already done
        if self.swap_init {
            return;
        }
        // Write PAGE_NUM (in little endian), but at the beginning of the space.
        // Then to support higher granularity, padd with free space to reach that granularity.
        let swap_page = self.flash.page_from_number(SWAP_PAGE_NUM).unwrap();
        self.flash
            .write(
                swap_page.base_address(),
                &[
                    (page_number & 0xFF) as u8, // Low word
                    (page_number >> 8) as u8,   // High word
                ],
            )
            .unwrap();
        // After an header, always flush to ensure data is written
        self.flash.flush_write_buffer().unwrap();
        self.current_position += 2 * FLAG_BYTES as u32; // The header size
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
        buff = frgm_target.to_le_bytes();
        self.flash
            .write(swap_start_addr + self.current_position, &buff)
            .unwrap();
        self.current_position += buff.len() as u32;

        // 2 - write fragment size
        buff = frgm_size.to_le_bytes();
        self.flash
            .write(swap_start_addr + self.current_position, &buff)
            .unwrap();
        self.current_position += buff.len() as u32;
        // 3 - copy fragment data
        let mut read_pos: u32 = page_start_addr + frgm_target;
        let frgm_end_addr = read_pos + frgm_size;
        while read_pos < frgm_end_addr {
            // TODO: move data in chunks
            let mut data_buff: [u8; 1] = [0x00; 1];
            self.flash.read(read_pos, &mut data_buff).unwrap();
            self.flash
                .write(swap_start_addr + self.current_position, &data_buff)
                .unwrap();
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
        let mut buff: [u8; 4] = [0x00; 4];
        self.flash.read(address, &mut buff).unwrap();
        return u32::from_le_bytes(buff);
    }
    fn read_u16(&self, address: u32) -> u16 {
        let mut buff: [u8; 2] = [0x00; 2];
        self.flash.read(address, &mut buff).unwrap();
        return u16::from_le_bytes(buff);
    }
    fn read_flag(&self, address: u32) -> bool {
        let mut buff: [u8; FLAG_BYTES] = [0x00; FLAG_BYTES];
        self.flash.read(address, &mut buff).unwrap();
        // Safer != 0xFF than == 0x00
        return buff != [0xFF; FLAG_BYTES];
    }

    fn copy_back(&mut self, target_page_start_addr: u32) {
        // Write back each fragment
        let swap_page = self.flash.page_from_number(SWAP_PAGE_NUM).unwrap();
        // Start from the first fragment
        let mut curr_pos = swap_page.base_address() + 2 * FLAG_BYTES as u32; // The header size
        let mut frgm_target = self.read_u32(curr_pos);
        curr_pos += 4;
        let mut frgm_size = self.read_u32(curr_pos);
        curr_pos += 4;
        while frgm_size < 0xFFFF_FFFF {
            // Empty header, finished fragments
            // Copy back fragment
            for i in 0..frgm_size {
                let mut data_buff: [u8; 1] = [0x00; 1];
                self.flash.read(curr_pos + i, &mut data_buff).unwrap();
                self.flash
                    .write(target_page_start_addr + frgm_target + i, &data_buff)
                    .unwrap();
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
        const SWAP_PAGE_NUM: u16,
    > Swapper<'a> for SwapperImpl<'a, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM>
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
                self.flash.erase(page.page_number()).unwrap();
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
        let buff: [u8; FLAG_BYTES] = [0x00; FLAG_BYTES];
        self.flash
            .write(swap_page.base_address() + FLAG_BYTES as u32, &buff)
            .unwrap();
        self.flash.flush_write_buffer().unwrap();
        self.flash.erase(page_number).unwrap();
        // 4 - copy back
        self.copy_back(page.base_address());
        self.flash.flush_write_buffer().unwrap(); // Always flush the flash after completing!
                                                  // 5 - erase swap
        self.flash.erase(SWAP_PAGE_NUM).unwrap();

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
        let copy_completed = self.read_flag(swap_page.base_address() + FLAG_BYTES as u32);
        if !copy_completed {
            // Safe to erase and return
            self.flash.erase(SWAP_PAGE_NUM).unwrap();
            return;
        }
        // Erase again target page
        self.flash.erase(page_number).unwrap();
        // 4 - copy back
        self.copy_back(page.base_address());
        // 5 - erase swap
        self.flash.erase(SWAP_PAGE_NUM).unwrap();
    }
}

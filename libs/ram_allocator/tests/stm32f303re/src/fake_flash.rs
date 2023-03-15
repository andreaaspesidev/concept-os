// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use flash_allocator::flash::page::FlashPage;
use flash_allocator::flash::FlashMethods;

/*
    Fake flash memory interface,
    backed with a simple vector.
*/
pub struct Flash<
    'b,
    const BLOCK_SIZE: usize,
    const ALLOCATOR_SIZE: usize,
> {
    content: &'b mut [u8],
    start_addr: u32,
    page_mapping: &'static [FlashPage],
}

impl<
        'b,
        const BLOCK_SIZE: usize,
        const ALLOCATOR_SIZE: usize,
    > Flash<'b, BLOCK_SIZE, ALLOCATOR_SIZE>
{
    pub fn new(start_addr: u32, page_mapping: &'static [FlashPage], content: &'b mut [u8]) -> Self {
        Self {
            content: content,
            start_addr: start_addr,
            page_mapping: page_mapping,
        }
    }
    fn page_from_num(&self, page_num: u16) -> Option<&FlashPage> {
        for p in self.page_mapping {
            if p.page_number() == page_num {
                return Some(p);
            }
        }
        None
    }
    fn write_u8(&mut self, address: u32, value: u8) {
        // In case flash memory requires an higher granularity for writing
        // this method must enforce it by buffering data and make a single write
        let offset = (address - self.start_addr) as usize;
        assert!(self.content[offset] == 0xFF || value == 0x00 || self.content[offset] == value);
        self.content[offset] = value;
    }
}

impl<
        'a,
        'b,
        const BLOCK_SIZE: usize,
        const ALLOCATOR_SIZE: usize
    > FlashMethods<'a>
    for Flash<'b, BLOCK_SIZE, ALLOCATOR_SIZE>
{
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(),()> {
        let offset = (address - self.start_addr) as usize;
        for i in 0..buffer.len() {
            buffer[i] = self.content[offset + i]
        }
        Ok(())
    }
    fn write(&mut self, address: u32, data: &[u8]) -> Result<(),()> {
        for i in 0..data.len() {
            self.write_u8(address + i as u32, data[i]);
        }
        Ok(())
    }
    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        Ok(())
    }
    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        for p in self.page_mapping {
            if p.contains_addr(address) {
                return Some(*p);
            }
        }
        None
    }
    fn erase(&mut self, page_num: u16) -> Result<(),()> {
        let page = self.page_from_num(page_num).unwrap();
        let offset_start = (page.base_address() - self.start_addr) as usize;
        let offset_end = offset_start + page.size() as usize;
        for i in offset_start..offset_end {
            self.content[i] = 0xFF; // Erase byte
        }
        Ok(())
    }

    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        for p in self.page_mapping {
            if p.page_number() == page_num {
                return Some(*p);
            }
        }
        None
    }

    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
        let prev_num = page_num - 1;
        self.page_from_number(prev_num)
    }
}

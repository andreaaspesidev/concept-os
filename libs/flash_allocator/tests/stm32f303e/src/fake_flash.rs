use flash_allocator::flash::{page::FlashPage, FlashMethods};


fn u16_from_arr(arr: &[u8]) -> u16 {
    ((arr[0] as u16) << 0)
    + ((arr[1] as u16) << 8)
}

/*
    Fake flash memory interface,
    backed with a simple vector.
*/


pub struct Flash<const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
{
    content: Vec<u8>,
    write_buffer: [u8; 2],
    target_address: u32,
}

impl<const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
    Flash<FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    pub fn new() -> Self {
        let size: usize = (FLASH_END_ADDRESS - FLASH_START_ADDRESS + 1) as usize;
        Self {
            content: vec![0xFF; size],
            write_buffer: [0xFF; 2],
            target_address: 0,
        }
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        let offset = (self.target_address - FLASH_START_ADDRESS) as usize;
        let data: u16 = u16::from_le_bytes(self.write_buffer);
        let actual_data: u16 = u16_from_arr(&self.content[offset..offset+2]);
        // Check if no change, do not write just skip
        if data == actual_data {
            // Reset status
            self.target_address = 0;
            return Ok(());
        }
        // Check if we would hard-fauld continuing
        if data != 0x0000 && actual_data != 0xFFFF {
            return Err(());
        }
        self.content[offset] = (data & 0xFF) as u8;
        self.content[offset+1] = (data >> 8) as u8;
        
        // Reset status
        self.target_address = 0;
        Ok(())
    }
    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), ()> {
        // In STM32F303, we must write 16bits at a time. Half writes or other "tricks" does
        // not work, as the flash controller checks the whole word is 0xFFFF before proceding
        // with the write. It's always possible to write 0x0000 in any situation, as the only exception.

        // Every write is then bufferized, and then the buffer flushed automatically whenever possible
        // (on the high byte of the word)
        let is_high_byte: bool = address % 2 > 0;
        let base_address = address - is_high_byte as u32; // Realign base address

        // Check whether we already have a buffer filling up
        if self.target_address > 0 {
            // Check whether we are changing base
            if self.target_address != base_address {
                // In this case force an automatic flush, but then the high byte of the
                // word cannot be written before the next erase. Maybe simply fail?
                self.flush_write_buffer()?;
            }
        }

        // Buffer empty, populate for this write
        if self.target_address == 0 {
            // Fill the buffer with the current data
            let offset: usize = (base_address - FLASH_START_ADDRESS) as usize;
            let current_word: u16 = u16_from_arr(&self.content[offset..offset+2]);
            self.write_buffer[0] = (current_word & 0xFF) as u8;
            self.write_buffer[1] = (current_word >> 8) as u8;
        }

        // Check whether this operation is possible
        if self.write_buffer[is_high_byte as usize] != 0xFF {
            // We can only admit a 0x0000 or the same value (we will skip the write)
            if value != 0x00 && value != self.write_buffer[is_high_byte as usize] {
                return Err(());
            }
        }

        // Set the base
        self.target_address = base_address;
        // Set the new byte
        self.write_buffer[is_high_byte as usize] = value;
        // Automatic flush when we get enough data
        if is_high_byte {
            self.flush_write_buffer()?;
        }
        Ok(())
    }
}

impl<'a, const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
    FlashMethods<'a> for Flash<FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(),()> {
        // Validate read address
        if address < FLASH_START_ADDRESS || (address + (buffer.len() as u32) > FLASH_END_ADDRESS) {
            return Err(());
        }
        // Negate write if this includes pending writes
        // TODO: maybe read considering the buffer? How to compose the abstraction?
        if self.target_address > 0 {
            if self.target_address >= address && self.target_address <= address + (buffer.len() as u32) {
                return Err(());
            }
        }
        let offset: usize = (address - FLASH_START_ADDRESS) as usize;
        // Actually perform the operation
        for i in 0..buffer.len() {
            buffer[i] = self.content[offset + i];
        }
        Ok(())
    }
    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
        for i in 0..data.len() {
            self.write_u8(address + i as u32, data[i])?;
        }
        Ok(())
    }
    fn flush_write_buffer(&mut self) -> Result<(),()> {
        // Nothing to flush
        if self.target_address == 0 {
            return Ok(());
        }
        // Perform flush
        self.flush_write_buffer()
    }
    fn erase(&mut self, page_num: u16) -> Result<(), ()> {
        // Check the page exists
        let page = self.page_from_number(page_num).ok_or(())?;

        let offset_start = (page.base_address() - FLASH_START_ADDRESS) as usize;
        let offset_end = offset_start + page.size() as usize;
        for i in offset_start..offset_end {
            self.content[i] = 0xFF; // Erase byte
        }
        Ok(())
    }

    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        if address <= FLASH_END_ADDRESS {
            let offset = address - FLASH_START_ADDRESS;
            let page_num = offset / PAGE_SIZE;
            let base_addr = FLASH_START_ADDRESS + page_num * PAGE_SIZE;
            return Some(FlashPage::new(page_num as u16, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        let max_num: u32 = (FLASH_END_ADDRESS - FLASH_START_ADDRESS + 1) / PAGE_SIZE;
        if page_num < max_num as u16 {
            let base_addr = FLASH_START_ADDRESS + page_num as u32 * PAGE_SIZE;
            return Some(FlashPage::new(page_num, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
        let prev_num = page_num - 1;
        self.page_from_number(prev_num)
    }
}

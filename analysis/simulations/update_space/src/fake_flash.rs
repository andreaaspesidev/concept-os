use flash_allocator::flash::{page::FlashPage, FlashMethods};


/*
    Fake flash memory interface,
    backed with a simple vector.
*/
pub struct Flash<
    'b,
    const FLASH_START_ADDRESS: u32,
    const PAGE_SIZE: u32,
    const FLASH_END_ADDRESS: u32,
> {
    content: &'b mut [u8],
    write_buffer: [u8; 8],
    target_address: u32,
}

impl<'b, const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
    Flash<'b, FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    pub fn new(content: &'b mut [u8]) -> Self {
        Self {
            content: content,
            write_buffer: [0xFF; 8],
            target_address: 0,
        }
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        let offset = (self.target_address - FLASH_START_ADDRESS) as usize;
        let data: u64 = u64::from_le_bytes(self.write_buffer);
        let actual_data: u64 = u64::from_le_bytes(self.content[offset..offset+8].try_into().unwrap());
        // Check if no change, do not write just skip
        if data == actual_data {
            // Reset status
            self.target_address = 0;
            return Ok(());
        }
        // Check if we would hard-fauld continuing
        if data != 0x0000_0000_0000_0000 && actual_data != 0xFFFF_FFFF_FFFF_FFFF {
            self.target_address = 0;
            return Err(());
        }
        // Write the data in the buffer
        let offset = (self.target_address - FLASH_START_ADDRESS) as usize;
        for i in 0..self.write_buffer.len() {
            self.content[offset + i] = self.write_buffer[i];
        }
        // Reset status
        self.target_address = 0;
        Ok(())
    }

    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), ()> {
        let byte_number = address as usize % 8;
        let base_address = address - byte_number as u32; // Realign base address

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
            let offset = (base_address - FLASH_START_ADDRESS) as usize;
            let len = self.write_buffer.len();
            self.write_buffer.copy_from_slice(&self.content[offset..offset+len]);
        }

        // Check whether this operation is possible
        if self.write_buffer[byte_number] != 0xFF {
            // We can only admit a 0x0000 or the same value (we will skip the write)
            if value != 0x00 && value != self.write_buffer[byte_number] {
                return Err(());
            }
        }

        // Set the base
        self.target_address = base_address;
        // Set the new byte
        self.write_buffer[byte_number] = value;
        // Automatic flush when we get enough data
        if byte_number == self.write_buffer.len() - 1 {
            self.flush_write_buffer()?;
        }
        Ok(())
    }
    pub fn page_from_address(address: u32) -> Option<FlashPage> {
        if address <= FLASH_END_ADDRESS {
            let offset = address - FLASH_START_ADDRESS;
            let page_num = offset / PAGE_SIZE;
            let base_addr = FLASH_START_ADDRESS + page_num * PAGE_SIZE;
            return Some(FlashPage::new(page_num as u16, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    pub fn page_from_number(page_num: u16) -> Option<FlashPage> {
        let max_num: u32 = (FLASH_END_ADDRESS - FLASH_START_ADDRESS + 1) / PAGE_SIZE;
        if page_num < max_num as u16 {
            let base_addr = FLASH_START_ADDRESS + page_num as u32 * PAGE_SIZE;
            return Some(FlashPage::new(page_num, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    pub fn prev_page(page_num: u16) -> Option<FlashPage> {
        let prev_num = page_num - 1;
        Self::page_from_number(prev_num)
    }
}

impl<
        'a,
        'b,
        const FLASH_START_ADDRESS: u32,
        const PAGE_SIZE: u32,
        const FLASH_END_ADDRESS: u32,
    > FlashMethods<'a> for Flash<'b, FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
        // Validate read address
        if address < FLASH_START_ADDRESS || address + (buffer.len() as u32) > FLASH_END_ADDRESS {
            return Err(());
        }
        // Negate write if this includes pending writes
        // TODO: maybe read considering the buffer? How to compose the abstraction?
        if self.target_address > 0 {
            if self.target_address >= address
                && self.target_address <= address + (buffer.len() as u32)
            {
                return Err(());
            }
        }
        // Actually perform the operation
        let offset = (address - FLASH_START_ADDRESS) as usize;
        buffer.copy_from_slice(&self.content[offset..offset+buffer.len()]);
        Ok(())
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
        // Write the bytes singularly, using the legacy method.
        // In this way we easily take into account odd lengths
        for i in 0..data.len() {
            self.write_u8(address + i as u32, data[i])?;
        }
        Ok(())
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
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

        // Clear the corresponding slice
        let offset = (page.base_address() - FLASH_START_ADDRESS) as usize;
        for i in 0..(PAGE_SIZE as usize) {
            self.content[offset + i] = 0xFF;
        }
        Ok(())
    }

    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        Self::page_from_address(address)
    }
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        Self::page_from_number(page_num)
    }
    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
        Self::prev_page(page_num)
    }
}

use crate::flash_allocator::flash::page::FlashPage;
use crate::flash_allocator::flash::FlashMethods;
use crate::flash_allocator::swap::{SwapResult, SwapStartType, Swapper, SwapperImpl};

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

/*
    Fake flash memory interface,
    backed with a simple vector.
*/
/*pub struct Flash<
    'b,
    const BLOCK_SIZE: usize,
    const BLOCK_MAX_LEVEL: u16,
    const ALLOCATOR_SIZE: usize,
    const SWAP_PAGE_NUM: u16,
    const FLAG_BYTES: usize,
> {
    content: &'b mut [u8],
    start_addr: u32,
    page_mapping: &'static [FlashPage],
}

impl<
        'b,
        const BLOCK_SIZE: usize,
        const BLOCK_MAX_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const SWAP_PAGE_NUM: u16,
        const FLAG_BYTES: usize,
    > Flash<'b, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>
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
    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), ()> {
        // In case flash memory requires an higher granularity for writing
        // this method must enforce it by buffering data and make a single write
        let offset = (address - self.start_addr) as usize;
        if !(self.content[offset] == 0xFF || value == 0x00 || self.content[offset] == value) {
            return Err(());
        }
        self.content[offset] = value;
        Ok(())
    }
}*/

pub struct Flash<
    'b,
    const FLASH_START_ADDRESS: u32,
    const FLASH_END_ADDRESS: u32,
    const BLOCK_SIZE: usize,
    const BLOCK_MAX_LEVEL: u16,
    const ALLOCATOR_SIZE: usize,
    const SWAP_PAGE_NUM: u16,
    const FLAG_BYTES: usize,
> {
    content: &'b mut [u8],
    write_buffer: [u8; FLAG_BYTES],
    target_address: u32,
    page_mapping: &'static [FlashPage],
}

impl<
        'b,
        const FLASH_START_ADDRESS: u32,
        const FLASH_END_ADDRESS: u32,
        const BLOCK_SIZE: usize,
        const BLOCK_MAX_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const SWAP_PAGE_NUM: u16,
        const FLAG_BYTES: usize,
    >
    Flash<
        'b,
        FLASH_START_ADDRESS,
        FLASH_END_ADDRESS,
        BLOCK_SIZE,
        BLOCK_MAX_LEVEL,
        ALLOCATOR_SIZE,
        SWAP_PAGE_NUM,
        FLAG_BYTES,
    >
{
    pub fn new(page_mapping: &'static [FlashPage], content: &'b mut [u8]) -> Self {
        Self {
            content: content,
            write_buffer: [0xFF; FLAG_BYTES],
            target_address: 0,
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

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        let offset = (self.target_address - FLASH_START_ADDRESS) as usize;
        let actual_data: [u8; FLAG_BYTES] = self.content[offset..offset + FLAG_BYTES]
            .try_into()
            .unwrap();
        // Check if no change, do not write just skip
        if self.write_buffer == actual_data {
            // Reset status
            self.target_address = 0;
            return Ok(());
        }
        // Check if we would hard-fauld continuing
        if self.write_buffer != [0x00; FLAG_BYTES] && actual_data != [0xFF; FLAG_BYTES] {
            return Err(());
        }
        for i in 0..FLAG_BYTES {
            self.content[offset + i] = self.write_buffer[i];
        }

        // Reset status
        self.target_address = 0;
        Ok(())
    }
    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), ()> {
        // We have to use the flash controller minimum write granularity. If for instance this is 16bits,
        // the target region must be 0xFFFF before proceding with the write.
        // It's always possible to write 0x0000 in any situation, as the only exception.

        // Every write is then bufferized, and then the buffer flushed automatically whenever possible
        // (on the high byte of the buffer)
        let byte_number = address as usize % FLAG_BYTES;
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
            let offset: usize = (base_address - FLASH_START_ADDRESS) as usize;
            self.write_buffer
                .copy_from_slice(&self.content[offset..offset + FLAG_BYTES]);
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
        if byte_number == FLAG_BYTES - 1 {
            self.flush_write_buffer()?;
        }
        Ok(())
    }
}

impl<
        'a,
        'b,
        const FLASH_START_ADDRESS: u32,
        const FLASH_END_ADDRESS: u32,
        const BLOCK_SIZE: usize,
        const BLOCK_MAX_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const SWAP_PAGE_NUM: u16,
        const FLAG_BYTES: usize,
    > FlashMethods<'a>
    for Flash<
        'b,
        FLASH_START_ADDRESS,
        FLASH_END_ADDRESS,
        BLOCK_SIZE,
        BLOCK_MAX_LEVEL,
        ALLOCATOR_SIZE,
        SWAP_PAGE_NUM,
        FLAG_BYTES,
    >
where
    [(); FLAG_BYTES * 3
        + 4
        + (FLAG_BYTES * 3 + 4) % 4
        + (FLAG_BYTES * 3 + 4 + (FLAG_BYTES * 3 + 4) % 4) % FLAG_BYTES]: Sized,
{
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
        let offset = (address - FLASH_START_ADDRESS) as usize;
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
    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        for p in self.page_mapping {
            if p.contains_addr(address) {
                return Some(*p);
            }
        }
        None
    }
    fn erase(&mut self, page_num: u16) -> Result<(), ()> {
        let page = self.page_from_num(page_num).ok_or(())?;
        let offset_start = (page.base_address() - FLASH_START_ADDRESS) as usize;
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

    fn launch_swap(
        &mut self,
        page_number: u16,
        start_type: SwapStartType,
        start_size: usize,
    ) -> SwapResult {
        let mut swapper = SwapperImpl::<
            BLOCK_SIZE,
            BLOCK_MAX_LEVEL,
            ALLOCATOR_SIZE,
            SWAP_PAGE_NUM
        >::new(self);
        swapper.swap_procedure(page_number, start_type, start_size as u32)
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        // If we have sth in the flash
        if self.target_address > 0 {
            // Actually perform the operation
            return self.flush_write_buffer();
        }
        // Otherwise ignore
        Ok(())
    }
}

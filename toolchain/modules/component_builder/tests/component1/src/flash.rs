use flash_allocator::flash::{FlashMethods, FlashPage};

/*
    Fake flash memory interface,
    backed with a simple vector.
*/
pub struct Flash<const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
{}

impl<const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
    Flash<FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
    FlashMethods<'a> for Flash<FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    fn read(&self, address: u32, len: usize) -> &'a [u8] {
        todo!();
    }
    fn write(&mut self, address: u32, value: u8) {
        // In case flash memory requires an higher granularity for writing
        // this method must enforce it by buffering data and make a single write
        todo!();
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
    fn erase(&mut self, page_num: u16) {
        todo!();
    }

    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        let MAX_NUMBER: u32 = (FLASH_END_ADDRESS - FLASH_START_ADDRESS + 1) / PAGE_SIZE;
        if page_num < MAX_NUMBER as u16 {
            let base_addr = FLASH_END_ADDRESS + page_num as u32 * PAGE_SIZE;
            return Some(FlashPage::new(page_num, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
        let prev_num = page_num - 1;
        self.page_from_number(prev_num)
    }
}

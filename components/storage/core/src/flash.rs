use flash_allocator::flash::{page::FlashPage, FlashMethods};
use userlib::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "board_stm32f303re")] {
        use stm32f303re as device;
        use stm32f303re::{FLASH_START_ADDR,FLASH_PAGE_SIZE,FLASH_END_ADDR};
    } else {
        compile_error!("Missing board configuration");
    }
}


/// Flash Interface, backed by kernel
pub struct FlashInterface {}

impl FlashInterface {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> FlashMethods<'a> for FlashInterface {
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
        // Directly read from flash
        buffer.copy_from_slice(unsafe {
            core::slice::from_raw_parts(address as *const u8, buffer.len())
        });
        Ok(())
    }
    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
        kipc::write_flash(address, data)
    }
    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        kipc::flash_flush_buffer()
    }
    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        device::Flash::<FLASH_START_ADDR,FLASH_PAGE_SIZE,FLASH_END_ADDR>::page_from_address(address)
    }
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        device::Flash::<FLASH_START_ADDR,FLASH_PAGE_SIZE,FLASH_END_ADDR>::page_from_number(page_num)
    }
    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
        device::Flash::<FLASH_START_ADDR,FLASH_PAGE_SIZE,FLASH_END_ADDR>::prev_page(page_num)
    }
    fn erase(&mut self, page_num: u16) -> Result<(), ()> {
        kipc::flash_erase(page_num)
    }
}

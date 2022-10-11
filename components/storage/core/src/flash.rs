use flash_allocator::flash::{FlashMethods, page::FlashPage};
use userlib::*;

/// Flash Interface, backed by kernel
pub struct FlashInterface {}

impl FlashInterface {
    pub fn new() -> Self {
        Self {  }
    }
}

impl <'a> FlashMethods<'a> for FlashInterface {
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
        kipc::read_flash(address, buffer)
    }
    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
        kipc::write_flash(address, data)
    }
    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        kipc::flash_flush_buffer()
    }
    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        kipc::flash_page_from_address(address)
    }
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        kipc::flash_page_from_number(page_num)
    }
    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
       kipc::flash_prev_page(page_num) 
    }
    fn erase(&mut self, page_num: u16) -> Result<(), ()> {
        kipc::flash_erase(page_num)
    }
}
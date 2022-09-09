use flash_allocator::flash::FlashMethods;

macro_rules! sys_log {
    ($s:expr) => {
        unsafe {
            let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[0];
            cortex_m::iprintln!(stim, $s);
        }
    };
    ($s:expr, $($tt:tt)*) => {
        unsafe {
            let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[0];
            cortex_m::iprintln!(stim, $s, $($tt)*);
        }
    };
}

pub struct FlashReader<
    const ALLOCATED_FLASH_START: u32,
    const ALLOCATED_FLASH_END: u32,
> {}

impl<const ALLOCATED_FLASH_START: u32, const ALLOCATED_FLASH_END: u32>
    FlashReader<ALLOCATED_FLASH_START, ALLOCATED_FLASH_END>
{
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, const ALLOCATED_FLASH_START: u32, const ALLOCATED_FLASH_END: u32>
    FlashMethods<'a>
    for FlashReader<ALLOCATED_FLASH_START, ALLOCATED_FLASH_END>
{
    fn read(&self, address: u32, len: usize) -> Result<&'a [u8], ()> {
        // Just read whatever in the range
        if address < ALLOCATED_FLASH_START
            || address + len as u32 > ALLOCATED_FLASH_END
        {
            return Err(());
        }
        // Read as raw slice
        unsafe { Ok(core::slice::from_raw_parts(address as *const u8, len)) }
    }
    fn write(&mut self, _: u32, _: u8) -> Result<(), ()> {
        panic!("Operation not supported!");
    }
    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        panic!("Operation not supported!");
    }
    fn page_from_address(
        &self,
        _: u32,
    ) -> Option<flash_allocator::flash::page::FlashPage> {
        panic!("Operation not supported!");
    }
    fn page_from_number(
        &self,
        _: u16,
    ) -> Option<flash_allocator::flash::page::FlashPage> {
        panic!("Operation not supported!");
    }
    fn prev_page(
        &self,
        _: u16,
    ) -> Option<flash_allocator::flash::page::FlashPage> {
        panic!("Operation not supported!");
    }
    fn erase(&mut self, _: u16) -> Result<(), ()> {
        panic!("Operation not supported!");
    }
}

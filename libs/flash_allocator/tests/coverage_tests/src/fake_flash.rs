use crate::flash_allocator::flash::{FlashMethods, FlashPage};
use crate::flash_allocator::swap::{SwapResult, SwapStartType, Swapper, SwapperImpl};

/*
    Fake flash memory interface,
    backed with a simple vector.
*/
pub struct Flash<
    'b,
    const BLOCK_SIZE: usize,
    const BLOCK_MAX_LEVEL: u16,
    const ALLOCATOR_SIZE: usize,
    const FLAG_BYTES: usize,
    const SWAP_PAGE_NUM: u16,
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
        const FLAG_BYTES: usize,
        const SWAP_PAGE_NUM: u16,
    > Flash<'b, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_BYTES, SWAP_PAGE_NUM>
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
}

impl<
        'a,
        'b,
        const BLOCK_SIZE: usize,
        const BLOCK_MAX_LEVEL: u16,
        const ALLOCATOR_SIZE: usize,
        const FLAG_BYTES: usize,
        const SWAP_PAGE_NUM: u16,
    > FlashMethods<'a>
    for Flash<'b, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_BYTES, SWAP_PAGE_NUM>
{
    fn read(&self, address: u32, len: usize) -> &'a [u8] {
        let offset = (address - self.start_addr) as usize;
        unsafe {
            // Needed as for testing now we are using vectors in heap, that would outlive the lifetime 'a
            core::slice::from_raw_parts(&self.content[offset], len)
        }
    }
    fn write(&mut self, address: u32, value: u8) {
        // In case flash memory requires an higher granularity for writing
        // this method must enforce it by buffering data and make a single write
        let offset = (address - self.start_addr) as usize;
        assert!(self.content[offset] == 0xFF || value == 0x00 || self.content[offset] == value);
        self.content[offset] = value;
    }
    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        for p in self.page_mapping {
            if p.contains_addr(address) {
                return Some(*p);
            }
        }
        None
    }
    fn erase(&mut self, page_num: u16) {
        let page = self.page_from_num(page_num).unwrap();
        let offset_start = (page.base_address() - self.start_addr) as usize;
        let offset_end = offset_start + page.size() as usize;
        for i in offset_start..offset_end {
            self.content[i] = 0xFF; // Erase byte
        }
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
            FLAG_BYTES,
            SWAP_PAGE_NUM,
        >::new(self);
        swapper.swap_procedure(page_number, start_type, start_size as u32)
    }
}

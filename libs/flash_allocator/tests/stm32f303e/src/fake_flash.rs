use flash_allocator::flash::{page::FlashPage, FlashMethods};

fn u32_from_arr(arr: &[u8]) -> u32 {
    ((arr[0] as u32) <<  0) +
    ((arr[1] as u32) <<  8) +
    ((arr[2] as u32) << 16) +
    ((arr[3] as u32) << 24)
}

/*
    Fake flash memory interface,
    backed with a simple vector.
*/
pub struct Flash {
    content: Vec<u8>,
    start_addr: u32,
    page_mapping: &'static [FlashPage]
}

impl Flash {
    pub fn new(size: usize, start_addr: u32, page_mapping: &'static [FlashPage]) -> Self {
        Self {
            content: vec![0xFF; size],
            start_addr: start_addr,
            page_mapping: page_mapping
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
    pub fn memory_dump(&self, max_page: usize) {
        for page in 0..max_page {
            let page_start = self.page_mapping[page].base_address() - self.start_addr;
            println!("[Page #{}]", page);
            for i in 0..4 {
                let offset_start: usize = (page_start) as usize + i*4;
                let word = u32_from_arr(&self.content[offset_start..offset_start+4]);
                println!("\t[{}] {:#010x}", i, word);
            }
        }
    }
}

impl <'a> FlashMethods<'a> for Flash {
    fn read(&self, address: u32, len: usize) -> &'a [u8] {
        let offset = (address - self.start_addr) as usize;
        unsafe{
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
}
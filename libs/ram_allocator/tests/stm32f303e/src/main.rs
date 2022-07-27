mod fake_flash;
mod flash_structure;

#[cfg(test)]
mod tests {
    use crate::{fake_flash::Flash, flash_structure::FLASH_PAGES};
    use flash_allocator::flash::{
        FlashAllocator, FlashAllocatorImpl, FlashMethods, walker::FlashWalkerImpl,
    };
    use ram_allocator::{RAMAllocatorImpl, RAMAllocator};
    use std::fmt;

    /*
        Used to get a formatter instance
    */
    pub struct Fmt<F>(pub F)
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result;
    impl<F> fmt::Debug for Fmt<F>
    where
        F: Fn(&mut fmt::Formatter) -> fmt::Result,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            (self.0)(f)
        }
    }

    // Flash: 0x0800 0000 - 0x0807 FFFF
    // Size: 512Kb
    const FLASH_START_ADDR: u32 = 0x0800_0000;
    const FLASH_END_ADDR: u32 = 0x0807_FFFF;
    const FLASH_SIZE: usize = (FLASH_END_ADDR - FLASH_START_ADDR + 1) as usize; // 0x80000 -> 2^19 -> 524288
    
    const FLASH_ALLOCATOR_START_ADDR: u32 = 0x0800_0000;
    const FLASH_ALLOCATOR_END_ADDR: u32 = 0x0807_FFFF; 
    const FLASH_ALLOCATOR_SIZE: usize = (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1) as usize;
    const FLASH_ALLOCATOR_START_SCAN_ADDR: u32 = 0x0800_1000; // ALLOCATOR_START_SCAN_ADDR - ALLOCATOR_START_ADDR MUST BE A POWER OF 2

    const FLASH_BLOCK_SIZE: usize = 4096;
    const FLASH_NUM_BLOCKS: usize = FLASH_ALLOCATOR_SIZE / FLASH_BLOCK_SIZE as usize; // 128
    const FLASH_NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1
    const FLAG_SIZE: usize = 2;

    const SRAM_START_ADDR: u32 = 0x2000_0000;
    const SRAM_END_ADDR: u32 = 0x2000_FFFF;
    const SRAM_SIZE: usize = (SRAM_END_ADDR - SRAM_START_ADDR + 1) as usize; // 64Kb

    const SRAM_RESERVED: u32 = 4096;
    const SRAM_BLOCK_SIZE: usize = 512;
    const SRAM_NUM_BLOCKS: usize = SRAM_SIZE / SRAM_BLOCK_SIZE as usize; // 128
    const SRAM_NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1

    fn mark_component(flash: &mut [u8], start_addr: u32) {
        let header_start: usize = start_addr as usize - FLASH_ALLOCATOR_START_ADDR as usize - 12;
        flash[header_start + 10] = 0xFE;
        flash[header_start + 11] = 0xFF;
    }

    #[test]
    fn test() {
        const BLOCK_MAX_LEVEL: u16 = (FLASH_NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<FLASH_BLOCK_SIZE, BLOCK_MAX_LEVEL, FLASH_ALLOCATOR_SIZE, FLAG_SIZE>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );
        let mut flash_allocator = FlashAllocatorImpl::<
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_BLOCK_SIZE,
            FLASH_NUM_BLOCKS,
            FLASH_NUM_SLOTS,
            FLAG_SIZE,
        >::from_flash(&mut flash);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Allocation 1
        let flash_alloc1 = flash_allocator.allocate(FLASH_BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", flash_alloc1.get_base_address());
        // Allocate 2
        let flash_alloc2 = flash_allocator.allocate(3 * FLASH_BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", flash_alloc2.get_base_address());
        // Allocate 3
        let flash_alloc3 = flash_allocator.allocate(4 * FLASH_BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", flash_alloc3.get_base_address());
        
        drop(flash_allocator);
        
        let mut sram_allocator = RAMAllocatorImpl::<
            SRAM_START_ADDR, 
            SRAM_END_ADDR, 
            SRAM_BLOCK_SIZE, 
            SRAM_NUM_BLOCKS, 
            SRAM_NUM_SLOTS, 
            SRAM_RESERVED,
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_NUM_SLOTS,
            FLASH_BLOCK_SIZE,
            FLAG_SIZE
            >::from_flash(&mut flash);
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));
        // Allocate 1
        let ram_alloc1 = sram_allocator.allocate(flash_alloc1.get_base_address(), 512).unwrap();
        println!("Allocated at: {:#010x}", ram_alloc1.get_base_address());
        mark_component(&mut shadow_copy, ram_alloc1.get_flash_position().get_base_address());
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));
        // Allocate 2
        let ram_alloc2 = sram_allocator.allocate(flash_alloc2.get_base_address(), 1024).unwrap();
        println!("Allocated at: {:#010x}", ram_alloc2.get_base_address());
        mark_component(&mut shadow_copy, ram_alloc2.get_flash_position().get_base_address());
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));
        // Allocate 3
        let ram_alloc3 = sram_allocator.allocate(flash_alloc3.get_base_address(), 4096).unwrap();
        println!("Allocated at: {:#010x}", ram_alloc3.get_base_address());
        mark_component(&mut shadow_copy, ram_alloc3.get_flash_position().get_base_address());
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));

        drop(sram_allocator);

        // Recreate
        let sram_allocator_rec = RAMAllocatorImpl::<
            SRAM_START_ADDR, 
            SRAM_END_ADDR, 
            SRAM_BLOCK_SIZE, 
            SRAM_NUM_BLOCKS, 
            SRAM_NUM_SLOTS, 
            SRAM_RESERVED,
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_NUM_SLOTS,
            FLASH_BLOCK_SIZE,
            FLAG_SIZE
            >::from_flash(&mut flash);
        println!("{:?}", &Fmt(|f| sram_allocator_rec.dump(f)));    
    }
}

fn main() {}

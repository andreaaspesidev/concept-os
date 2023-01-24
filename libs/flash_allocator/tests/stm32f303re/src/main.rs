mod fake_flash;

#[cfg(test)]
mod tests {
    use crate::fake_flash::Flash;
    use flash_allocator::{flash::{
        FlashAllocator, FlashAllocatorImpl, FlashMethods, BlockType
    }};
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
    const START_ADDR: u32 = 0x0800_0000;
    const PAGE_SIZE: u32 = 2048;
    const START_SCAN_ADDR: u32 = 0x0800_1000; // ALLOCATOR_START_SCAN_ADDR - ALLOCATOR_START_ADDR MUST BE A POWER OF 2
    const END_ADDR: u32 = 0x0807_FFFF;
    const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x80000 -> 2^19 -> 524288
    
    const BLOCK_SIZE: usize = 2048; // Single page size

    fn init_stm32f303e<'a>(
        flash: &'a mut dyn FlashMethods<'a>
    ) -> impl FlashAllocator<'a> {
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 256
        const TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
        const NUM_NODES: usize = 2 * NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

        let bd= FlashAllocatorImpl::<
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            BLOCK_SIZE,
            NUM_BLOCKS,
            TREE_MAX_LEVEL,
            NUM_NODES
        >::from_flash(flash, false, false);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                FlashAllocatorImpl::<
                    START_ADDR,
                    END_ADDR,
                    START_SCAN_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    TREE_MAX_LEVEL,
                    NUM_NODES
                >,
            >()
        );
        return bd;
    }

    #[test]
    fn test() {
        let mut flash = Flash::<START_ADDR, PAGE_SIZE, END_ADDR>::new();
        let mut flash_allocator = init_stm32f303e(&mut flash);
        let initial_state = format!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Allocation 1
        let alloc1 = flash_allocator.allocate(BLOCK_SIZE as u32, BlockType::COMPONENT).unwrap();
        println!("Allocated at: {:#010x}", alloc1.get_base_address());
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        let old_state = format!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Destroy allocator
        drop(flash_allocator);
        // Recreate from flash
        let mut flash_allocator_rec = init_stm32f303e(&mut flash);
        let new_state = format!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        assert!(new_state.eq(&old_state));
        // Deallocate 1
        flash_allocator_rec
            .deallocate(alloc1.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Allocate 2
        let alloc2 = flash_allocator_rec.allocate(3 * BLOCK_SIZE as u32, BlockType::COMPONENT).unwrap();
        println!("Allocated at: {:#010x}", alloc2.get_base_address());
        // Allocate 3
        let alloc3 = flash_allocator_rec.allocate(4 * BLOCK_SIZE as u32, BlockType::COMPONENT).unwrap();
        println!("Allocated at: {:#010x}", alloc3.get_base_address());
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 2
        flash_allocator_rec
            .deallocate(alloc2.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 3
        flash_allocator_rec
            .deallocate(alloc3.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        let final_state = format!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        assert!(final_state.eq(&initial_state));
    }
}

fn main() {}

mod fake_flash;
mod flash_structure;

#[cfg(test)]
mod tests {
    use crate::{fake_flash::Flash, flash_structure::FLASH_PAGES};
    use flash_allocator::flash::{
        FlashAllocator, FlashAllocatorImpl, FlashMethods,
    };
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
    const START_SCAN_ADDR: u32 = 0x0800_1000; // ALLOCATOR_START_SCAN_ADDR - ALLOCATOR_START_ADDR MUST BE A POWER OF 2
    const END_ADDR: u32 = 0x0807_FFFF;
    const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x80000 -> 2^19 -> 524288
    const BLOCK_SIZE: usize = 4096;
    const FLAG_SIZE: usize = 2;

    fn init_stm32f303e<'a>(
        flash: &'a mut dyn FlashMethods<'a>
    ) -> impl FlashAllocator<'a, FLAG_SIZE> {
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 128
        const NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1
        let bd= FlashAllocatorImpl::<
            START_ADDR,
            END_ADDR,
            START_SCAN_ADDR,
            BLOCK_SIZE,
            NUM_BLOCKS,
            NUM_SLOTS,
            FLAG_SIZE,
        >::from_flash(flash);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                FlashAllocatorImpl::<
                    START_ADDR,
                    END_ADDR,
                    START_SCAN_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    NUM_SLOTS,
                    FLAG_SIZE,
                >,
            >()
        );
        return bd;
    }

    #[test]
    fn test() {
        let mut flash = Flash::new(SIZE, START_ADDR, &FLASH_PAGES);
        let mut flash_allocator = init_stm32f303e(&mut flash);
        // Allocation 1
        let alloc1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc1.get_base_address());
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Destroy allocator
        drop(flash_allocator);
        // Recreate from flash
        let mut flash_allocator_rec = init_stm32f303e(&mut flash);
        // Deallocate 1
        flash_allocator_rec
            .deallocate(alloc1.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Allocate 2
        let alloc2 = flash_allocator_rec.allocate(3 * BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc2.get_base_address());
        // Allocate 3
        let alloc3 = flash_allocator_rec.allocate(4 * BLOCK_SIZE as u32).unwrap();
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
    }
}

fn main() {}

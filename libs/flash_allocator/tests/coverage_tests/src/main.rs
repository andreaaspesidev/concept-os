#![feature(generic_const_exprs)]
/**
 * The following tests takes as scenarios the examples presented graphically
 * in docs/FlashMemory.md (section Non-Uniform Flash Page Sizes/Swapping/Examples).
 */
mod fake_flash;
mod flash_allocator;

#[cfg(test)]
mod tests {
    use crate::fake_flash::Flash;
    use crate::flash_allocator::flash::{
        FlashAllocator, FlashAllocatorImpl, FlashMethods, FlashPage,
    };
    use std::fmt;

    /*
        Auxiliary object to supply a formatter in order to print messages
        from the no_std lib.
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

    /*
        Auxiliary functions to check whether data is retained for each allocated block
        during any operation.
    */

    /// Fills the flash memory with a known byte (usually the page number)
    fn fill_block(flash: &mut [u8], start_addr: usize, block_size: usize, fill_with: u8) {
        for i in start_addr..(start_addr + block_size - 12) {
            flash[i - ALLOCATOR_START_ADDR as usize] = fill_with;
        }
    }

    /// Checks whether the whole page is filled with the known byte
    fn check_block(flash: &mut [u8], start_addr: usize, block_size: usize, filled_with: u8) {
        for i in start_addr..(start_addr + block_size - 12) {
            if flash[i - ALLOCATOR_START_ADDR as usize] != filled_with {
                panic!("Broken block");
            }
        }
    }

    /// Force the mark of a block as deallocated, to check recovery procedures
    fn mark_deallocated(flash: &mut [u8], start_addr: usize) {
        let header_start: usize = start_addr - ALLOCATOR_START_ADDR as usize - 12;
        flash[header_start + 2] = 0x00;
        flash[header_start + 3] = 0x00;
    }

    /*
        Flash memory generation
    */

    // Flash: 0x0800 0000 - 0x0807 FFFF
    // Size: 512Kb
    // "Flash Layout": "../../../../../docs/images/vfp_example_0.svg"
    const ALLOCATOR_START_ADDR: u32 = 0x0800_0000;
    const ALLOCATOR_END_ADDR: u32 = 0x0800_7FFF;
    const ALLOCATOR_SIZE: usize = (ALLOCATOR_END_ADDR - ALLOCATOR_START_ADDR + 1) as usize; // 0x8000 -> 32768

    const FLASH_START_ADDR: u32 = 0x0800_0000;
    const FLASH_END_ADDR: u32 = 0x0807_FFFF;
    const FLASH_SIZE: usize = (FLASH_END_ADDR - FLASH_START_ADDR + 1) as usize; // 0x8000 -> 32768

    const BLOCK_SIZE: usize = 2048;
    const FLAG_SIZE: usize = 2;
    const NUM_BLOCKS: usize = ALLOCATOR_SIZE / BLOCK_SIZE as usize; // 16
    const NUM_SLOTS: usize = 4 + 1; // clog2(NUM_BLOCKS) + 1

    const SWAP_PAGE_NUM: u16 = 5;

    static FLASH_PAGES: [FlashPage; 5] = [
        FlashPage::new(1, 0x08000000, 4096),  // 1
        FlashPage::new(2, 0x08001000, 8192),  // 2
        FlashPage::new(3, 0x08003000, 12288), // 3
        FlashPage::new(4, 0x08006000, 8192),  // 4
        FlashPage::new(5, 0x08008000, 12288), // swap page
    ];

    fn init_allocator<'a>(
        flash: &'a mut dyn FlashMethods<'a>,
        from_flash: bool,
    ) -> impl FlashAllocator<'a, FLAG_SIZE> {
        assert!(ALLOCATOR_SIZE % BLOCK_SIZE as usize == 0);
        let bd;
        if !from_flash {
            bd = FlashAllocatorImpl::<
                ALLOCATOR_START_ADDR,
                ALLOCATOR_END_ADDR,
                BLOCK_SIZE,
                NUM_BLOCKS,
                NUM_SLOTS,
                FLAG_SIZE,
            >::new(flash);
        } else {
            bd = FlashAllocatorImpl::<
                ALLOCATOR_START_ADDR,
                ALLOCATOR_END_ADDR,
                BLOCK_SIZE,
                NUM_BLOCKS,
                NUM_SLOTS,
                FLAG_SIZE,
            >::from_flash(flash);
        }
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                FlashAllocatorImpl::<
                    ALLOCATOR_START_ADDR,
                    ALLOCATOR_END_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    NUM_SLOTS,
                    FLAG_SIZE,
                >,
            >()
        );
        return bd;
    }

    /// Perform a simple allocation/deallocation to check the basic functionality works
    #[test]
    fn test_basic_functionality() {
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );
        let mut flash_allocator = init_allocator(&mut flash, false);
        // Allocation 1
        let alloc1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc1);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Destroy allocator
        drop(flash_allocator);
        // Recreate from flash
        let mut flash_allocator_rec = init_allocator(&mut flash, true);
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 1
        flash_allocator_rec.deallocate(alloc1);
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Allocate 2
        let alloc2 = flash_allocator_rec.allocate(3 * BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc2);
        // Allocate 3
        let alloc3 = flash_allocator_rec.allocate(4 * BLOCK_SIZE as u32).unwrap();
        println!("Allocated at: {:#010x}", alloc3);
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 2
        flash_allocator_rec.deallocate(alloc3);
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 3
        flash_allocator_rec.deallocate(alloc2);
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
    }

    /// Deallocate block 1, keeping the other intact (see vfp_example_1.svg)
    #[test]
    fn test_deallocate_block1() {
        println!("\n====================== Example 1 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );

        let mut flash_allocator = init_allocator(&mut flash, false);
        // Construct initial layout
        let block1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        let alloc2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        check_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        let block2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        let alloc4 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        let block3 = flash_allocator.allocate(4 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        let block4 = flash_allocator.allocate(8 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc2);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc4);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 1
        flash_allocator.deallocate(block1);
        // Check the other blocks are still intact
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        // Check that the block 1 is now free
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0xFF);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Deallocate block 2, keeping the other intact (see vfp_example_2.svg)
    #[test]
    fn test_deallocate_block2() {
        println!("\n====================== Example 2 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );

        let mut flash_allocator = init_allocator(&mut flash, false);
        // Construct initial layout
        let block1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        let alloc2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        check_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        let block2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        let alloc4 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        let block3 = flash_allocator.allocate(4 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        let block4 = flash_allocator.allocate(8 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc2);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc4);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 2
        flash_allocator.deallocate(block2);
        // Check the other blocks are still intact
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        // Check that the block 2 is now free
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0xFF);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Deallocate block 3, keeping the other intact (see vfp_example_3.svg)
    #[test]
    fn test_deallocate_block3() {
        println!("\n====================== Example 3 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );

        let mut flash_allocator = init_allocator(&mut flash, false);
        // Construct initial layout
        let block1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);

        let alloc2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        check_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);

        let block2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);

        let alloc4 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);

        let block3 = flash_allocator.allocate(4 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);

        let block4 = flash_allocator.allocate(8 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc2);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc4);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 3
        flash_allocator.deallocate(block3);
        // Check the others are still intact
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        // Check that the block 3 is now free
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0xFF);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Deallocate block 4, keeping the other intact (see vfp_example_4.svg)
    #[test]
    fn test_deallocate_block4() {
        println!("\n====================== Example 4 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );

        let mut flash_allocator = init_allocator(&mut flash, false);
        // Construct initial layout
        let block1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        let alloc2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        check_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        let block2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        let alloc4 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        let block3 = flash_allocator.allocate(4 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        let block4 = flash_allocator.allocate(8 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc2);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc4);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 4
        flash_allocator.deallocate(block4);
        // Check the others are still intact
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        // Check that the block 4 is now free
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0xFF);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    #[test]
    fn test_deallocate_all() {
        println!("\n====================== Example All ======================\n");
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );

        let mut flash_allocator = init_allocator(&mut flash, false);
        // Construct the initial layout
        let block1 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        let alloc2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        check_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        let block2 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        let alloc4 = flash_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        let block3 = flash_allocator.allocate(4 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        let block4 = flash_allocator.allocate(8 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc2);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);

        flash_allocator.deallocate(alloc4);
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 1,2,3,4
        flash_allocator.deallocate(block4);
        flash_allocator.deallocate(block2);
        flash_allocator.deallocate(block1);
        flash_allocator.deallocate(block3);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Tests whether a block still not allocated is safely removed upon first scan
    #[test]
    fn test_recovery_remove_unallocated_on_start() {
        const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS - 1) as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash =
            Flash::<BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, FLAG_SIZE, SWAP_PAGE_NUM>::new(
                FLASH_START_ADDR,
                &FLASH_PAGES,
                &mut flash_content,
            );

        // Create the initial layout
        let mut initial_allocator = init_allocator(&mut flash, false);
        // Construct the initial layout
        let block1 = initial_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        let alloc2 = initial_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc2 as usize, BLOCK_SIZE, 0x02);
        let block2 = initial_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        let alloc4 = initial_allocator.allocate(BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, alloc4 as usize, BLOCK_SIZE, 0x04);
        let block3 = initial_allocator.allocate(4 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0x05);
        let block4 = initial_allocator.allocate(8 * BLOCK_SIZE as u32).unwrap();
        fill_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
        initial_allocator.deallocate(alloc2);
        initial_allocator.deallocate(alloc4);
        println!("{:?}", &Fmt(|f| initial_allocator.dump(f)));

        // Modify the memory and mark the third block as deallocated
        drop(initial_allocator);
        mark_deallocated(&mut shadow_copy, block3 as usize);

        // Create the allocator from flash
        let flash_allocator = init_allocator(&mut flash, true);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Check whether the block has been freed
        check_block(&mut shadow_copy, block1 as usize, BLOCK_SIZE, 0x01);
        check_block(&mut shadow_copy, block2 as usize, BLOCK_SIZE, 0x03);
        check_block(&mut shadow_copy, block3 as usize, 4 * BLOCK_SIZE, 0xFF);
        check_block(&mut shadow_copy, block4 as usize, 8 * BLOCK_SIZE, 0x06);
    }
}

fn main() {}

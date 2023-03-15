// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(generic_const_exprs)]

/**
 * The following tests takes as scenarios the examples presented graphically
 * in docs/FlashMemory.md (section Non-Uniform Flash Page Sizes/Swapping/Examples).
 */
mod fake_flash;
mod flash_allocator;

// For inspecting memory, address swap: 0x7FFFF7BEF4C0 (could change on a different pc)

#[cfg(test)]
mod tests {
    use abi::flash::BlockType;

    use crate::fake_flash::Flash;
    use crate::flash_allocator::flash::header::{self, BlockHeader};
    use crate::flash_allocator::flash::page::FlashPage;
    use crate::flash_allocator::flash::walker::{FlashWalker, FlashWalkerImpl};
    use crate::flash_allocator::flash::{
        utils, FlashAllocator, FlashAllocatorImpl, FlashBlock, FlashMethods, self,
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
    fn fill_block_region(flash: &mut [u8], start_addr: usize, block_size: usize, fill_with: u8) {
        for i in start_addr..(start_addr + block_size - flash::HEADER_SIZE) {
            flash[i - ALLOCATOR_START_ADDR as usize] = fill_with;
        }
    }

    fn check_block_region(flash: &mut [u8], start_addr: usize, block_size: usize, filled_with: u8) {
        for i in start_addr..(start_addr + block_size - flash::HEADER_SIZE) {
            if flash[i - ALLOCATOR_START_ADDR as usize] != filled_with {
                panic!("Broken block");
            }
        }
    }

    /// Checks whether the whole page is filled with the known byte
    fn check_block(flash: &mut [u8], block: &FlashBlock, filled_with: u8) {
        for i in block.get_base_address()..(block.get_base_address() + block.get_size()) {
            if flash[(i - ALLOCATOR_START_ADDR) as usize] != filled_with {
                panic!("Broken block");
            }
        }
    }

    /// Force the mark of a block as deallocated, to check recovery procedures
    fn mark_deallocated(flash: &mut [u8], start_addr: usize) {
        let header_start: usize = start_addr - ALLOCATOR_START_ADDR as usize - flash::HEADER_SIZE;
        for i in 0..FLAG_BYTES {
            flash[header_start + 2 + i] = 0x00;
        }
    }

    /*
        Flash memory generation
    */

    // Flash: 0x0800 0000 - 0x0807 FFFF
    // Size: 512Kb

    // Allocator Generic Constraints
    const ALLOCATOR_START_ADDR: u32 = 0x0800_0000;
    const ALLOCATOR_END_ADDR: u32 = 0x0800_7FFF;
    const ALLOCATOR_SIZE: usize = (ALLOCATOR_END_ADDR - ALLOCATOR_START_ADDR + 1) as usize; // 0x8000 -> 32768
                                                                                            // Allocator scan constraints (blocks before are considered allocated space)
    const ALLOCATOR_START_SCAN_ADDR: u32 = 0x0800_0000; // ALLOCATOR_START_SCAN_ADDR - ALLOCATOR_START_ADDR MUST BE A POWER OF 2

    const FLASH_START_ADDR: u32 = 0x0800_0000;
    const FLASH_END_ADDR: u32 = 0x0807_FFFF;
    const FLASH_SIZE: usize = (FLASH_END_ADDR - FLASH_START_ADDR + 1) as usize; // 0x8000 -> 32768

    const BLOCK_SIZE: usize = 2048;
    const NUM_BLOCKS: usize = ALLOCATOR_SIZE / BLOCK_SIZE as usize; // 16
    const TREE_MAX_LEVEL: usize = 4; // log2(num_blocks) = log2(memory_area / block_size)
    const NUM_NODES: usize = 2 * NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

    const FLAG_BYTES: usize = 8; // 64 bits

    const SWAP_PAGE_NUM: u16 = 5;

    static FLASH_PAGES: [FlashPage; 5] = [
        FlashPage::new(1, 0x08000000, 4096),  // 1
        FlashPage::new(2, 0x08001000, 8192),  // 2
        FlashPage::new(3, 0x08003000, 12288), // 3
        FlashPage::new(4, 0x08006000, 8192),  // 4
        FlashPage::new(5, 0x08008000, 12288), // swap page
    ];

    fn init_allocator<'a>(flash: &'a mut dyn FlashMethods<'a>) -> impl FlashAllocator<'a> {
        assert!(ALLOCATOR_SIZE % BLOCK_SIZE as usize == 0);
        let bd = FlashAllocatorImpl::<
            ALLOCATOR_START_ADDR,
            ALLOCATOR_END_ADDR,
            ALLOCATOR_START_SCAN_ADDR,
            BLOCK_SIZE,
            NUM_BLOCKS,
            TREE_MAX_LEVEL,
            NUM_NODES
        >::from_flash(flash, false, false);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                FlashAllocatorImpl::<
                    ALLOCATOR_START_ADDR,
                    ALLOCATOR_END_ADDR,
                    ALLOCATOR_START_SCAN_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    TREE_MAX_LEVEL,
                    NUM_NODES
                >,
            >()
        );
        return bd;
    }

    /// Perform a simple allocation/deallocation to check the basic functionality works
    #[test]
    fn test_basic_functionality() {
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );
        let mut flash_allocator = init_allocator(&mut flash);
        // Allocation 1
        let alloc1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc1.get_size(), BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        println!(
            "Allocated at: {:#010x}, actual usable size: {}",
            alloc1.get_base_address(),
            alloc1.get_size()
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Destroy allocator
        drop(flash_allocator);
        // Recreate from flash
        let mut flash_allocator_rec = init_allocator(&mut flash);
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 1
        flash_allocator_rec
            .deallocate(alloc1.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Allocate 2
        let alloc2 = flash_allocator_rec
            .allocate(3 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc2.get_size(), 4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        println!(
            "Allocated at: {:#010x}, actual usable size: {}",
            alloc2.get_base_address(),
            alloc2.get_size()
        );
        // Allocate 3
        let alloc3 = flash_allocator_rec
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc3.get_size(), 4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        println!(
            "Allocated at: {:#010x}, actual usable size: {}",
            alloc3.get_base_address(),
            alloc3.get_size()
        );
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Try to deallocate a wrong address
        flash_allocator_rec
            .deallocate(alloc3.get_base_address() + 33)
            .unwrap_err();
        // Deallocate 3
        flash_allocator_rec
            .deallocate(alloc3.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Deallocate 2
        flash_allocator_rec
            .deallocate(alloc2.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator_rec.dump(f)));
        // Try to deallocate a wrong address
        flash_allocator_rec
            .deallocate(alloc2.get_base_address() + 33)
            .unwrap_err();
        // Try to deallocate a free block
        flash_allocator_rec
            .deallocate(ALLOCATOR_START_ADDR)
            .unwrap_err();
        flash_allocator_rec
            .deallocate(alloc3.get_base_address())
            .unwrap_err();
    }

    #[test]
    fn test_basic_functionality_start_end() {
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );
        const SCAN_START: u32 = 0x0800_1000; // 0x0800_1000 - 0x0800_0000 = 4096 (4Kb)
        let mut flash_allocator = FlashAllocatorImpl::<
            ALLOCATOR_START_ADDR,
            ALLOCATOR_END_ADDR,
            SCAN_START,
            BLOCK_SIZE,
            NUM_BLOCKS,
            TREE_MAX_LEVEL,
            NUM_NODES
        >::from_flash(&mut flash, false, false);
        // Safely color the prev. space
        fill_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Allocation 1
        let alloc1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc1.get_size(), BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        println!(
            "Allocated at: {:#010x}, actual usable size: {}",
            alloc1.get_base_address(),
            alloc1.get_size()
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Check
        check_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        // Deallocate 1
        flash_allocator
            .deallocate(alloc1.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Check
        check_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        // Allocate 2
        let alloc2 = flash_allocator
            .allocate(3 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc2.get_size(), 4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        println!(
            "Allocated at: {:#010x}, actual usable size: {}",
            alloc2.get_base_address(),
            alloc2.get_size()
        );
        // Check
        check_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        // Allocate 3
        let alloc3 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc3.get_size(), 4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        println!(
            "Allocated at: {:#010x}, actual usable size: {}",
            alloc3.get_base_address(),
            alloc3.get_size()
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Check
        check_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        // Try to deallocate a wrong address
        flash_allocator
            .deallocate(alloc3.get_base_address() + 33)
            .unwrap_err();
        // Deallocate 3
        flash_allocator
            .deallocate(alloc3.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Check
        check_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        // Deallocate 2
        flash_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        // Check
        check_block_region(
            &mut shadow_copy,
            ALLOCATOR_START_ADDR as usize,
            (SCAN_START - ALLOCATOR_START_ADDR + 1) as usize,
            0x11,
        );
        // Try to deallocate a wrong address
        flash_allocator
            .deallocate(alloc2.get_base_address() + 33)
            .unwrap_err();
        flash_allocator
            .deallocate(ALLOCATOR_START_ADDR)
            .unwrap_err();
        // Try to deallocate a free block
        flash_allocator.deallocate(SCAN_START).unwrap_err();
        flash_allocator
            .deallocate(alloc3.get_base_address())
            .unwrap_err();
    }

    #[test]
    fn test_block_attributes() {
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );
        let mut flash_allocator = init_allocator(&mut flash);
        // Allocation 1
        let alloc1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::COMPONENT)
            .unwrap();
        assert_eq!(alloc1.get_size(), BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32 - 8);
        assert_eq!(alloc1.get_type(), BlockType::COMPONENT);
        assert!(!alloc1.is_finalized());
        // Allocation 2
        let alloc1 = flash_allocator
            .allocate(3 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(alloc1.get_size(), 4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        assert_eq!(alloc1.get_type(), BlockType::NONE);
        assert!(!alloc1.is_finalized());
    }

    #[test]
    fn test_block_refresh() {
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );
        let mut flash_allocator = init_allocator(&mut flash);
        // Allocation 1
        let mut block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        assert_eq!(block1.get_size(), BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32);
        assert_eq!(block1.get_type(), BlockType::NONE);
        assert!(!block1.is_finalized());
        // Finalize component
        drop(flash_allocator);
        utils::finalize_block::<FLASH_START_ADDR, TREE_MAX_LEVEL>(&mut flash, block1).unwrap();
        let flash_allocator1 = init_allocator(&mut flash);
        flash_allocator1.refresh(&mut block1);
        assert_eq!(block1.get_type(), BlockType::NONE);
        assert!(block1.is_finalized());
    }

    #[test]
    fn test_block_iterator() {
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );
        let mut flash_allocator = init_allocator(&mut flash);
        // Allocation 1
        let block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        // Allocation 2
        let block2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        // Allocation 3
        let block3 = flash_allocator
            .allocate(3 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        // Allocation 4
        let block4 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        // Deallocate block 2
        flash_allocator
            .deallocate(block3.get_base_address())
            .unwrap();
        // Create a new iterator
        drop(flash_allocator); // Needed to release the flash interface
        let mut iterator = FlashWalkerImpl::<
            ALLOCATOR_START_ADDR,
            ALLOCATOR_END_ADDR,
            ALLOCATOR_START_SCAN_ADDR,
            TREE_MAX_LEVEL
        >::new(&mut flash);
        let b1 = iterator.next();
        assert!(b1.is_some());
        assert_eq!(b1.unwrap(), block1);
        let b2 = iterator.next();
        assert!(b2.is_some());
        assert_eq!(b2.unwrap(), block2);
        let b4 = iterator.next();
        assert!(b4.is_some());
        assert_eq!(b4.unwrap(), block4);
        let b5 = iterator.next();
        assert!(b5.is_none());
        // Iterate from the beginning
        let bb1 = iterator.nth(0);
        assert!(bb1.is_some());
        assert_eq!(bb1.unwrap(), block1);

        // Try using the wrapper class
        let iterator_ref: &mut dyn FlashWalker = &mut iterator;
        assert_eq!(iterator_ref.nth(0).unwrap(), block1);
        assert_eq!(iterator_ref.nth(1).unwrap(), block2);
        assert_eq!(iterator_ref.next().unwrap(), block4);
    }

    /// Deallocate block 1, keeping the other intact (see vfp_example_1.svg)
    #[test]
    fn test_deallocate_block1() {
        println!("\n====================== Example 1 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );

        let mut flash_allocator = init_allocator(&mut flash);
        // Construct initial layout
        let block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0x01,
        );
        check_block(&mut shadow_copy, &block1, 0x01);
        let alloc2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc2.get_base_address() as usize,
            BLOCK_SIZE,
            0x02,
        );
        check_block(&mut shadow_copy, &alloc2, 0x02);
        let block2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0x03,
        );
        check_block(&mut shadow_copy, &block2, 0x03);
        let alloc4 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc4.get_base_address() as usize,
            BLOCK_SIZE,
            0x04,
        );
        check_block(&mut shadow_copy, &alloc4, 0x04);
        let block3 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0x05,
        );
        check_block(&mut shadow_copy, &block3, 0x05);
        let block4 = flash_allocator
            .allocate(8 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0x06,
        );
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &alloc4, 0x04);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc4.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 1
        flash_allocator
            .deallocate(block1.get_base_address())
            .unwrap();
        // Check the other blocks are still intact
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        // Check that the block 1 is now free
        check_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0xFF,
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Deallocate block 2, keeping the other intact (see vfp_example_2.svg)
    #[test]
    fn test_deallocate_block2() {
        println!("\n====================== Example 2 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );

        let mut flash_allocator = init_allocator(&mut flash);
        // Construct initial layout
        let block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0x01,
        );
        check_block(&mut shadow_copy, &block1, 0x01);
        let alloc2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc2.get_base_address() as usize,
            BLOCK_SIZE,
            0x02,
        );
        check_block(&mut shadow_copy, &alloc2, 0x02);
        let block2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0x03,
        );
        check_block(&mut shadow_copy, &block2, 0x03);
        let alloc4 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc4.get_base_address() as usize,
            BLOCK_SIZE,
            0x04,
        );
        check_block(&mut shadow_copy, &alloc4, 0x04);
        let block3 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0x05,
        );
        check_block(&mut shadow_copy, &block3, 0x05);
        let block4 = flash_allocator
            .allocate(8 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0x06,
        );
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &alloc4, 0x04);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc4.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 2
        flash_allocator
            .deallocate(block2.get_base_address())
            .unwrap();
        // Check the other blocks are still intact
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        // Check that the block 2 is now free
        check_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0xFF,
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Deallocate block 3, keeping the other intact (see vfp_example_3.svg)
    #[test]
    fn test_deallocate_block3() {
        println!("\n====================== Example 3 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );

        let mut flash_allocator = init_allocator(&mut flash);
        // Construct initial layout
        let block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0x01,
        );
        check_block(&mut shadow_copy, &block1, 0x01);
        let alloc2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc2.get_base_address() as usize,
            BLOCK_SIZE,
            0x02,
        );
        check_block(&mut shadow_copy, &alloc2, 0x02);
        let block2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0x03,
        );
        check_block(&mut shadow_copy, &block2, 0x03);
        let alloc4 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc4.get_base_address() as usize,
            BLOCK_SIZE,
            0x04,
        );
        check_block(&mut shadow_copy, &alloc4, 0x04);
        let block3 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0x05,
        );
        check_block(&mut shadow_copy, &block3, 0x05);
        let block4 = flash_allocator
            .allocate(8 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0x06,
        );
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &alloc4, 0x04);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc4.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 3
        flash_allocator
            .deallocate(block3.get_base_address())
            .unwrap();
        // Check the others are still intact
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block4, 0x06);
        // Check that the block 3 is now free
        check_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0xFF,
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Deallocate block 4, keeping the other intact (see vfp_example_4.svg)
    #[test]
    fn test_deallocate_block4() {
        println!("\n====================== Example 4 ======================\n");
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );

        let mut flash_allocator = init_allocator(&mut flash);
        // Construct initial layout
        let block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0x01,
        );
        check_block(&mut shadow_copy, &block1, 0x01);
        let alloc2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc2.get_base_address() as usize,
            BLOCK_SIZE,
            0x02,
        );
        check_block(&mut shadow_copy, &alloc2, 0x02);
        let block2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0x03,
        );
        check_block(&mut shadow_copy, &block2, 0x03);
        let alloc4 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc4.get_base_address() as usize,
            BLOCK_SIZE,
            0x04,
        );
        check_block(&mut shadow_copy, &alloc4, 0x04);
        let block3 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0x05,
        );
        check_block(&mut shadow_copy, &block3, 0x05);
        let block4 = flash_allocator
            .allocate(8 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0x06,
        );
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &alloc4, 0x04);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc4.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 4
        flash_allocator
            .deallocate(block4.get_base_address())
            .unwrap();
        // Check the others are still intact
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        // Check that the block 4 is now free
        check_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0xFF,
        );
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    #[test]
    fn test_deallocate_all() {
        println!("\n====================== Example All ======================\n");
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );

        let mut flash_allocator = init_allocator(&mut flash);
        // Construct initial layout
        let block1 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0x01,
        );
        check_block(&mut shadow_copy, &block1, 0x01);
        let alloc2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc2.get_base_address() as usize,
            BLOCK_SIZE,
            0x02,
        );
        check_block(&mut shadow_copy, &alloc2, 0x02);
        let block2 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0x03,
        );
        check_block(&mut shadow_copy, &block2, 0x03);
        let alloc4 = flash_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc4.get_base_address() as usize,
            BLOCK_SIZE,
            0x04,
        );
        check_block(&mut shadow_copy, &alloc4, 0x04);
        let block3 = flash_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0x05,
        );
        check_block(&mut shadow_copy, &block3, 0x05);
        let block4 = flash_allocator
            .allocate(8 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0x06,
        );
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &alloc4, 0x04);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);

        flash_allocator
            .deallocate(alloc4.get_base_address())
            .unwrap();
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block(&mut shadow_copy, &block3, 0x05);
        check_block(&mut shadow_copy, &block4, 0x06);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Dismiss block 1,2,3,4
        flash_allocator
            .deallocate(block4.get_base_address())
            .unwrap();
        flash_allocator
            .deallocate(block2.get_base_address())
            .unwrap();
        flash_allocator
            .deallocate(block1.get_base_address())
            .unwrap();
        flash_allocator
            .deallocate(block3.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
    }

    /// Tests whether a block still not allocated is safely removed upon first scan
    #[test]
    fn test_recovery_remove_unallocated_on_start() {
        const BLOCK_MAX_LEVEL: u16 = TREE_MAX_LEVEL as u16;
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut shadow_copy: &mut [u8];
        unsafe {
            let ptr = flash_content.as_mut_ptr();
            shadow_copy = core::slice::from_raw_parts_mut(ptr, FLASH_SIZE);
        }
        let mut flash = Flash::<FLASH_START_ADDR, FLASH_END_ADDR, BLOCK_SIZE, BLOCK_MAX_LEVEL, ALLOCATOR_SIZE, SWAP_PAGE_NUM, FLAG_BYTES>::new(
            &FLASH_PAGES,
            &mut flash_content,
        );

        // Create the initial layout
        let mut initial_allocator = init_allocator(&mut flash);
        let block1 = initial_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block1.get_base_address() as usize,
            BLOCK_SIZE,
            0x01,
        );
        let alloc2 = initial_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc2.get_base_address() as usize,
            BLOCK_SIZE,
            0x02,
        );
        let block2 = initial_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block2.get_base_address() as usize,
            BLOCK_SIZE,
            0x03,
        );
        let alloc4 = initial_allocator
            .allocate(BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            alloc4.get_base_address() as usize,
            BLOCK_SIZE,
            0x04,
        );
        let block3 = initial_allocator
            .allocate(4 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0x05,
        );
        let block4 = initial_allocator
            .allocate(8 * BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32, BlockType::NONE)
            .unwrap();
        fill_block_region(
            &mut shadow_copy,
            block4.get_base_address() as usize,
            8 * BLOCK_SIZE,
            0x06,
        );
        initial_allocator
            .deallocate(alloc2.get_base_address())
            .unwrap();
        initial_allocator
            .deallocate(alloc4.get_base_address())
            .unwrap();
        println!("{:?}", &Fmt(|f| initial_allocator.dump(f)));

        // Modify the memory and mark the third block as deallocated
        drop(initial_allocator);
        mark_deallocated(&mut shadow_copy, block3.get_base_address() as usize);

        // Create the allocator from flash
        let flash_allocator = init_allocator(&mut flash);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));

        // Check whether the block has been freed
        check_block(&mut shadow_copy, &block1, 0x01);
        check_block(&mut shadow_copy, &block2, 0x03);
        check_block_region(
            &mut shadow_copy,
            block3.get_base_address() as usize,
            4 * BLOCK_SIZE,
            0xFF,
        );
        check_block(&mut shadow_copy, &block4, 0x06);
    }
}

fn main() {}

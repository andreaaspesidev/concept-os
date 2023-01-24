fn main() {}

/// Very little testing. The objective is to allow manual debugging
/// of the internal functions, rather than supporting regression testing.
#[cfg(test)]
mod tests {
    use core::fmt;

    use buddy_allocator::{BinaryBuddyImpl, BuddyAllocator, ListBuddyImpl};

    // Workaround to get a formatter
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
    fn init_stm32f303e_sram() -> impl BuddyAllocator {
        // SRAM: 0x2000 0000 - 0x2000 FFFF
        // Size: 64Kb
        const START_ADDR: u32 = 0x2000_0000;
        const END_ADDR: u32 = 0x2000_FFFF;
        const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x10000 -> 2^16 -> 65536
        const BLOCK_SIZE: usize = 1024;
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 64
        const NUM_SLOTS: usize = 6 + 1; // clog2(NUM_BLOCKS) + 1
        let bd =
            ListBuddyImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>::new(false);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                ListBuddyImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>,
            >()
        );
        return bd;
    }

    fn init_stm32f303e_flash() -> impl BuddyAllocator {
        // Flash: 0x0800 0000 - 0x0807 FFFF
        // Size: 512Kb
        const START_ADDR: u32 = 0x0800_0000;
        const END_ADDR: u32 = 0x0807_FFFF;
        const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x80000 -> 2^19 -> 524288
        const BLOCK_SIZE: usize = 4096;
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 128
        const NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1
        let bd =
            ListBuddyImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>::new(false);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                ListBuddyImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>,
            >()
        );
        return bd;
    }

    fn init_stm32f303e_binary_sram() -> impl BuddyAllocator {
        // SRAM: 0x2000 0000 - 0x2000 FFFF
        // Size: 64Kb
        const START_ADDR: u32 = 0x2000_0000;
        const END_ADDR: u32 = 0x2000_FFFF;
        const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x10000 -> 2^16 -> 65536
        const BLOCK_SIZE: usize = 1024;
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 64

        const TREE_MAX_LEVEL: usize = 6; // log2(num_blocks) = log2(memory_area / block_size)
        const NUM_NODES: usize = 2 * NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

        let bd = BinaryBuddyImpl::<
            START_ADDR,
            END_ADDR,
            BLOCK_SIZE,
            NUM_BLOCKS,
            TREE_MAX_LEVEL,
            NUM_NODES,
        >::new(false);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                BinaryBuddyImpl::<
                    START_ADDR,
                    END_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    TREE_MAX_LEVEL,
                    NUM_NODES,
                >,
            >() // 1104
        );
        return bd;
    }

    fn init_stm32f303e_binary_flash() -> impl BuddyAllocator {
        // Flash: 0x0800 0000 - 0x0807 FFFF
        // Size: 512Kb
        const START_ADDR: u32 = 0x0800_0000;
        const END_ADDR: u32 = 0x0807_FFFF;
        const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize; // 0x80000 -> 2^19 -> 524288
        const BLOCK_SIZE: usize = 2048; // Page size
        assert!(SIZE % BLOCK_SIZE as usize == 0);
        const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 256

        const TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
        const NUM_NODES: usize = 2 * NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

        let bd = BinaryBuddyImpl::<
            START_ADDR,
            END_ADDR,
            BLOCK_SIZE,
            NUM_BLOCKS,
            TREE_MAX_LEVEL,
            NUM_NODES,
        >::new(false);
        println!(
            "Required memory bytes: {}",
            core::mem::size_of::<
                BinaryBuddyImpl::<
                    START_ADDR,
                    END_ADDR,
                    BLOCK_SIZE,
                    NUM_BLOCKS,
                    TREE_MAX_LEVEL,
                    NUM_NODES,
                >,
            >() // 520
        );
        return bd;
    }

    #[test]
    fn test_1() {
        let mut bd = init_stm32f303e_sram();
        assert!(bd.is_all_available());
        // Create some allocations
        println!("Starting allocations at: 0x{:02X}", bd.start_addr());
        let size1: usize = 1024 as usize;
        let alloc1 = bd.alloc(size1).unwrap();
        println!("Allocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size2: usize = 3 * 1024 as usize;
        let alloc2 = bd.alloc(size2).unwrap();
        println!("Allocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size3: usize = 1024 as usize;
        let alloc3 = bd.alloc(size3).unwrap();
        println!("Allocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc1, size1) };
        println!("Deallocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc2, size2) };
        println!("Deallocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc3, size3) };
        println!("Deallocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        assert!(bd.is_all_available());
    }

    #[test]
    fn test_2() {
        let mut bd = init_stm32f303e_binary_sram();
        assert!(bd.is_all_available());
        // Create some allocations
        println!("Starting allocations at: 0x{:02X}", bd.start_addr());
        let size1: usize = 1024 as usize;
        let alloc1 = bd.alloc(size1).unwrap();
        println!("Allocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size2: usize = 3 * 1024 as usize;
        let alloc2 = bd.alloc(size2).unwrap();
        println!("Allocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size3: usize = 1024 as usize;
        let alloc3 = bd.alloc(size3).unwrap();
        println!("Allocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc1, size1) };
        println!("Deallocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc2, size2) };
        println!("Deallocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc3, size3) };
        println!("Deallocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        assert!(bd.is_all_available());
    }
    #[test]
    fn test_3() {
        let mut bd = init_stm32f303e_flash();
        assert!(bd.is_all_available());
        // Create some allocations
        println!("Starting allocations at: 0x{:02X}", bd.start_addr());
        let size1: usize = 1024 as usize;
        let alloc1 = bd.alloc(size1).unwrap();
        println!("Allocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size2: usize = 3 * 1024 as usize;
        let alloc2 = bd.alloc(size2).unwrap();
        println!("Allocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size3: usize = 1024 as usize;
        let alloc3 = bd.alloc(size3).unwrap();
        println!("Allocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc1, size1) };
        println!("Deallocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc2, size2) };
        println!("Deallocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc3, size3) };
        println!("Deallocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        assert!(bd.is_all_available());
    }

    #[test]
    fn test_4() {
        let mut bd = init_stm32f303e_binary_flash();
        assert!(bd.is_all_available());
        // Create some allocations
        println!("Starting allocations at: 0x{:02X}", bd.start_addr());
        let size1: usize = 1024 as usize;
        let alloc1 = bd.alloc(size1).unwrap();
        println!("Allocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size2: usize = 3 * 1024 as usize;
        let alloc2 = bd.alloc(size2).unwrap();
        println!("Allocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        let size3: usize = 1024 as usize;
        let alloc3 = bd.alloc(size3).unwrap();
        println!("Allocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc1, size1) };
        println!("Deallocated {} at 0x{:02X}", size1, alloc1);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc2, size2) };
        println!("Deallocated {} at 0x{:02X}", size2, alloc2);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        unsafe { bd.dealloc(alloc3, size3) };
        println!("Deallocated {} at 0x{:02X}", size3, alloc3);
        println!("{:?}", &Fmt(|f| bd.dump(f)));
        assert!(bd.is_all_available());
    }
}

mod fake_flash;
mod flash_structure;

#[cfg(test)]
mod tests {
    use crate::{fake_flash::Flash, flash_structure::FLASH_PAGES};
    use flash_allocator::flash::{FlashAllocatorImpl, BlockType, self};
    use ram_allocator::{RAMAllocator, RAMAllocatorImpl};
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
    const FLASH_ALLOCATOR_SIZE: usize =
        (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1) as usize;
    const FLASH_ALLOCATOR_START_SCAN_ADDR: u32 = 0x0800_1000; // ALLOCATOR_START_SCAN_ADDR - ALLOCATOR_START_ADDR MUST BE A POWER OF 2

    const FLASH_BLOCK_SIZE: usize = 2048; // Single page size
    const FLASH_NUM_BLOCKS: usize = FLASH_ALLOCATOR_SIZE / FLASH_BLOCK_SIZE as usize; // 256
    const FLASH_TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
    const FLASH_NUM_NODES: usize = 2 * FLASH_NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1
    // const FLASH_FLAG_BYTES: usize = 2;

    const SRAM_START_ADDR: u32 = 0x2000_0000;
    const SRAM_END_ADDR: u32 = 0x2000_FFFF;
    const SRAM_SIZE: usize = (SRAM_END_ADDR - SRAM_START_ADDR + 1) as usize; // 64Kb

    const SRAM_RESERVED: u32 = 4096;
    const SRAM_BLOCK_SIZE: usize = 256;
    const SRAM_NUM_BLOCKS: usize = SRAM_SIZE / SRAM_BLOCK_SIZE as usize; // 256
    const SRAM_TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
    const SRAM_NUM_NODES: usize = 2 * SRAM_NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

    #[test]
    fn test() {
        let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
        let mut flash = Flash::<FLASH_BLOCK_SIZE, FLASH_ALLOCATOR_SIZE>::new(
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
            FLASH_TREE_MAX_LEVEL,
            FLASH_NUM_NODES
        >::from_flash(&mut flash, false, false);
        println!("{:?}", &Fmt(|f| flash_allocator.dump(f)));
        println!("Flash header size: {}", flash::HEADER_SIZE);
        // Allocation 1
        let flash_alloc1 = flash_allocator
            .allocate(
                FLASH_BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32,
                BlockType::COMPONENT,
            )
            .unwrap();
        println!("Allocated at: {:#010x}", flash_alloc1.get_base_address());
        // Allocate 2
        let flash_alloc2 = flash_allocator
            .allocate(
                3 * FLASH_BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32,
                BlockType::COMPONENT,
            )
            .unwrap();
        println!("Allocated at: {:#010x}", flash_alloc2.get_base_address());
        // Allocate 3
        let flash_alloc3 = flash_allocator
            .allocate(
                4 * FLASH_BLOCK_SIZE as u32 - flash::HEADER_SIZE as u32,
                BlockType::COMPONENT,
            )
            .unwrap();
        println!("Allocated at: {:#010x}", flash_alloc3.get_base_address());

        drop(flash_allocator);

        let mut sram_allocator = RAMAllocatorImpl::<
            SRAM_START_ADDR,
            SRAM_END_ADDR,
            SRAM_BLOCK_SIZE,
            SRAM_NUM_BLOCKS,
            SRAM_TREE_MAX_LEVEL,
            SRAM_NUM_NODES,
            SRAM_RESERVED,
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_TREE_MAX_LEVEL
        >::from_flash(&mut flash);
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));
        // Allocate 1
        let ram_alloc1 = sram_allocator
            .allocate(flash_alloc1.get_base_address(), 256)
            .unwrap();
        println!("Allocated at: {:#010x}", ram_alloc1.get_base_address());
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));
        // Allocate 2
        let ram_alloc2 = sram_allocator
            .allocate(flash_alloc2.get_base_address(), 512)
            .unwrap();
        println!("Allocated at: {:#010x}", ram_alloc2.get_base_address());
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));
        // Allocate 3
        let ram_alloc3 = sram_allocator
            .allocate(flash_alloc3.get_base_address(), 4096)
            .unwrap();
        println!("Allocated at: {:#010x}", ram_alloc3.get_base_address());
        println!("{:?}", &Fmt(|f| sram_allocator.dump(f)));

        drop(sram_allocator);

        // Recreate
        let sram_allocator_rec = RAMAllocatorImpl::<
            SRAM_START_ADDR,
            SRAM_END_ADDR,
            SRAM_BLOCK_SIZE,
            SRAM_NUM_BLOCKS,
            SRAM_TREE_MAX_LEVEL,
            SRAM_NUM_NODES,
            SRAM_RESERVED,
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_TREE_MAX_LEVEL
        >::from_flash(&mut flash);
        println!("{:?}", &Fmt(|f| sram_allocator_rec.dump(f)));
    }
}

fn main() {}

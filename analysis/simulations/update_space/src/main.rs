use fake_flash::Flash;
use flash_allocator::flash::{FlashAllocator, FlashAllocatorImpl, FlashBlock, FlashMethods};
use stm32l476rg::{
    FLASH_ALLOCATOR_END_ADDR, FLASH_ALLOCATOR_START_ADDR, FLASH_ALLOCATOR_START_SCAN_ADDR,
    FLASH_BLOCK_SIZE, FLASH_NUM_BLOCKS, FLASH_NUM_NODES, FLASH_TREE_MAX_LEVEL,
};

use crate::visualize_stats::{AllocStats, visualize_flash};

mod fake_flash;
mod visualize_stats;

fn init_stm32l476rg<'a>(flash: &'a mut dyn FlashMethods<'a>) -> impl FlashAllocator<'a> {
    let bd = FlashAllocatorImpl::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_BLOCK_SIZE,
        FLASH_NUM_BLOCKS,
        FLASH_TREE_MAX_LEVEL,
        FLASH_NUM_NODES,
    >::from_flash(flash, false, false);
    println!(
        "Required SRAM bytes: {}",
        core::mem::size_of::<
            FlashAllocatorImpl::<
                FLASH_ALLOCATOR_START_ADDR,
                FLASH_ALLOCATOR_END_ADDR,
                FLASH_ALLOCATOR_START_SCAN_ADDR,
                FLASH_BLOCK_SIZE,
                FLASH_NUM_BLOCKS,
                FLASH_TREE_MAX_LEVEL,
                FLASH_NUM_NODES,
            >,
        >()
    );
    return bd;
}


fn main() {
    const FLASH_SIZE: usize = (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1) as usize;
    let mut flash_content: [u8; FLASH_SIZE] = [0xFF; FLASH_SIZE];
    let mut flash = Flash::<
        FLASH_ALLOCATOR_START_ADDR,
        { FLASH_BLOCK_SIZE as u32 },
        FLASH_ALLOCATOR_END_ADDR,
    >::new(&mut flash_content);
    let mut flash_allocator = init_stm32l476rg(&mut flash);
    let mut allocs: Vec<FlashBlock> = Vec::new();
    // 11
    allocs.push(flash_allocator
        .allocate(1908, flash_allocator::flash::BlockType::COMPONENT)
        .unwrap(),
    );
    // 8
    allocs.push(flash_allocator
        .allocate(184, flash_allocator::flash::BlockType::COMPONENT)
        .unwrap());
    // 2
    allocs.push(flash_allocator
        .allocate(768, flash_allocator::flash::BlockType::COMPONENT)
        .unwrap(),
    );
    // 10
    allocs.push(flash_allocator
            .allocate(5588, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 4
    allocs.push(flash_allocator
            .allocate(4644, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 3
    allocs.push(flash_allocator
            .allocate(5516, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 5
    allocs.push(flash_allocator
            .allocate(6960, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    let mut out1 = std::env::current_dir().unwrap();
    out1.push("Report1.html");
    visualize_flash(&AllocStats{
        entries: allocs.clone(),
        flash_start: FLASH_ALLOCATOR_START_ADDR,
        flash_size: (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1),
    }, &out1);
    // new component
    allocs.push(flash_allocator
            .allocate(6000, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    let mut out2 = std::env::current_dir().unwrap();
    out2.push("Report2.html");
    visualize_flash(&AllocStats{
        entries: allocs.clone(),
        flash_start: FLASH_ALLOCATOR_START_ADDR,
        flash_size: (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1),
    }, &out2);
}

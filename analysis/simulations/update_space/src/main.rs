use fake_flash::Flash;
use flash_allocator::flash::{FlashAllocator, FlashAllocatorImpl, FlashBlock, FlashMethods};
use stm32l476rg::{
    FLASH_ALLOCATOR_END_ADDR, FLASH_ALLOCATOR_START_ADDR, FLASH_ALLOCATOR_START_SCAN_ADDR,
    FLASH_BLOCK_SIZE, FLASH_NUM_BLOCKS, FLASH_NUM_NODES, FLASH_TREE_MAX_LEVEL,
};

mod fake_flash;

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

fn process_alloc(highest_address: &mut u32, alloc: FlashBlock) {
    println!(
        "Allocated at {:x} for {} bytes",
        alloc.get_nominal_base_address(),
        alloc.get_nominal_size()
    );
    let current_end_address = alloc.get_nominal_base_address() + alloc.get_nominal_size();
    if current_end_address > *highest_address {
        *highest_address = current_end_address;
    }
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
    let mut highest_address: u32 = 0;
    // 8
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(184, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 11
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(2096, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 10
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(5588, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 2
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(768, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 4
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(4644, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 3
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(5516, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // 3
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(6960, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    // new component
    process_alloc(
        &mut highest_address,
        flash_allocator
            .allocate(6960, flash_allocator::flash::BlockType::COMPONENT)
            .unwrap(),
    );
    println!("Highest address: {:x}", highest_address);
}

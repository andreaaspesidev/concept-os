// Flash: 0x0800 0000 - 0x0807 FFFF
// Size: 512Kb
pub const ALLOCATOR_START_ADDR: u32 = 0x0800_0000;
pub const ALLOCATOR_END_ADDR: u32 = 0x0800_7FFF;
pub const ALLOCATOR_SIZE: usize = (ALLOCATOR_END_ADDR - ALLOCATOR_START_ADDR + 1) as usize;  // 0x8000 -> 32768

pub const FLASH_START_ADDR: u32 = 0x0800_0000;
pub const FLASH_END_ADDR: u32 = 0x0807_FFFF;
pub const FLASH_SIZE: usize = (FLASH_END_ADDR - FLASH_START_ADDR + 1) as usize;  // 0x8000 -> 32768

pub const BLOCK_SIZE: usize = 4096;
pub const FLAG_SIZE: usize = 2;

pub const NUM_BLOCKS: usize = ALLOCATOR_SIZE / BLOCK_SIZE as usize; // 128
pub const NUM_SLOTS: usize = 7 + 1;   // clog2(NUM_BLOCKS) + 1
pub const BLOCK_MAX_LEVEL: u16 = (NUM_SLOTS-1) as u16;

pub const FLASH_PAGE_SIZE: u32 = 2048;
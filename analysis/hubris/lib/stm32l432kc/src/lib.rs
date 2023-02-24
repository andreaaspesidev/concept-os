#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;
pub mod device;

/**
 * STM32 F303RE
 * - Flash Constant
 *
 * Flash: 0x0800 0000 - 0x0807 FFFF
 * Size: 512Kb
 *
 * Notes:
 * - In order for the system to work correcly, the first part of the flash
 *   (starting from address 0x0800 0000) must be reserved to the kernel.
 *   The allocator have two requirements:
 *   - Must have a base address (FLASH_ALLOCATOR_START_ADDR) aligned with its size (FLASH_ALLOCATOR_SIZE).
 *     To alleviate this huge limitation, it's possible to reserve a whole subspace
 *     at the beginning by selecting a  FLASH_ALLOCATOR_START_SCAN_ADDR > FLASH_ALLOCATOR_START_ADDR
 *   - The page containing FLASH_ALLOCATOR_START_SCAN_ADDR must not contain important data (like kernel code).
 *     The allocator will need to erase this page in order to deallocate the first blocks.
 *     For this reason, let's impose that FLASH_ALLOCATOR_START_SCAN_ADDR points to the
 *     beginning of the first free page after the one containing the last kernel code.
 */
pub const FLASH_ALLOCATOR_START_ADDR: u32 = 0x0800_0000; // Page 0
pub const FLASH_ALLOCATOR_END_ADDR: u32 = 0x0803_FFFF; // Page 127
pub const FLASH_ALLOCATOR_SIZE: usize =
    (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1) as usize; // 0x40000 -> 262144

// This value needs to be fixed (automatically during build),
// and (FLASH_ALLOCATOR_START_SCAN_ADDR - FLASH_ALLOCATOR_START_ADDR) must be a valid multiple of FLASH_PAGE_SIZE
// (even 0 is fine)
pub const FLASH_ALLOCATOR_START_SCAN_ADDR: u32 = 0x0800_A000; // 38912 bytes for the kernel

pub const FLASH_START_ADDR: u32 = 0x0800_0000;
pub const FLASH_END_ADDR: u32 = 0x0803_FFFF;
// pub const FLASH_SIZE: usize = (FLASH_END_ADDR - FLASH_START_ADDR + 1) as usize; // 0x8000 -> 32768

pub const FLASH_BLOCK_SIZE: usize = 2048; // Single page size
pub const FLASH_NUM_BLOCKS: usize = FLASH_ALLOCATOR_SIZE / FLASH_BLOCK_SIZE as usize; // 128
pub const FLASH_TREE_MAX_LEVEL: usize = 7; // log2(num_blocks) = log2(memory_area / block_size)
pub const FLASH_NUM_NODES: usize = 2 * FLASH_NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1
pub const FLASH_FLAG_BYTES: usize = 8; // 64-bits

pub const FLASH_PAGE_SIZE: u32 = 2048;

// Flash operation timings: worst case scenario
pub const FLASH_ERASE_MS: u32 = 25;
pub const FLASH_WRITES_PER_MS: u32 = 1000 / 91 * 8;

// Compile time checks
static_assertions::const_assert_eq!((2 << (FLASH_TREE_MAX_LEVEL + 1 - 1)) - 1, FLASH_NUM_NODES);

/**
 * STM32 F303RE
 * - RAM Constant
 *
 * RAM: 0x2000 0000 - 0x2000 FFFF
 * Size: 64Kb
 */
pub const SRAM_START_ADDR: u32 = 0x2000_0000;
pub const SRAM_END_ADDR: u32 = 0x2000_FFFF;
pub const SRAM_SIZE: usize = (SRAM_END_ADDR - SRAM_START_ADDR + 1) as usize; // 64Kb

pub const SRAM_RESERVED: u32 = 6656; // Kernel memory
pub const SRAM_BLOCK_SIZE: usize = 256;
pub const SRAM_NUM_BLOCKS: usize = SRAM_SIZE / SRAM_BLOCK_SIZE as usize; // 256
pub const SRAM_TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
pub const SRAM_NUM_NODES: usize = 2 * SRAM_NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

// Compile time checks
static_assertions::const_assert_eq!((2 << (SRAM_TREE_MAX_LEVEL + 1 - 1)) - 1, SRAM_NUM_NODES);

/**
 * STM32 L432KC
 * - Flash Interface
 *
 * Provides methods to read, but expecially write and erase
 * the flash memory.
 */
pub const FLASH_KEY1: u32 = 0x4567_0123;
pub const FLASH_KEY2: u32 = 0xCDEF_89AB;


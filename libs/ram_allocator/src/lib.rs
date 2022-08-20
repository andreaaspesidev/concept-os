#![no_std]
#![feature(generic_const_exprs)]

use abi::flash::BlockType;
use buddy_allocator::{BuddyAllocator, BuddyAllocatorImpl};
use core::fmt::Formatter;
use flash_allocator::flash::{self, walker::FlashWalkerImpl, FlashBlock, FlashMethods};

pub mod utils {
    use crate::u32_from_le;
    use abi::flash::BlockType;
    use flash_allocator::flash::{self, FlashMethods};

    use super::RAMBlock;

    pub fn get_ram_block<
        'a,
        const FLASH_START_ADDR: u32,
        const FLASH_END_ADDR: u32,
        const FLASH_START_SCAN_ADDR: u32,
        const FLASH_NUM_SLOTS: usize,
        const FLASH_BLOCK_SIZE: usize,
        const FLAG_BYTES: usize,
    >(
        flash: &'a dyn FlashMethods<'a>,
        block_base_address: u32,
    ) -> Option<RAMBlock> {
        // Check this is actually a component, by reading the header
        let block_search = flash::utils::get_flash_block::<
            FLASH_START_ADDR,
            FLASH_END_ADDR,
            FLASH_START_SCAN_ADDR,
            FLASH_NUM_SLOTS,
            FLASH_BLOCK_SIZE,
            FLAG_BYTES,
        >(flash, block_base_address, false);
        if block_search.is_none() {
            return None;
        }
        let block = block_search.unwrap();
        if block.get_type() != BlockType::COMPONENT {
            return None;
        }
        // Read fields
        let data = flash.read(block.get_base_address(), 8).unwrap();
        let sram_base = u32_from_le(&data[0..4]);
        let sram_size = u32_from_le(&data[4..8]);
        return Some(RAMBlock {
            block_base_address: sram_base,
            block_size: sram_size,
            flash_position: block,
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RAMBlock {
    block_base_address: u32,
    block_size: u32,
    flash_position: FlashBlock,
}

impl<'a> RAMBlock {
    pub fn get_base_address(&self) -> u32 {
        self.block_base_address
    }
    pub fn get_size(&self) -> u32 {
        self.block_size
    }
    pub fn get_flash_position(&self) -> FlashBlock {
        self.flash_position
    }
    /// Gets data of the block as a slice, without copying into memory.
    /// Unsafe as it attempts to read the data directly, without passing through
    /// the interface. If the component does not have read access, this call will make
    /// the component crash, instead of giving error
    pub unsafe fn get_data(&self) -> &'a [u8] {
        core::slice::from_raw_parts(
            self.block_base_address as *const u8,
            self.block_size as usize,
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AllocatorError {
    OutOfRAM = 1,
    InvalidBlock = 2,
}

pub trait RAMAllocator<const FLAG_BYTES: usize> {
    fn allocate(&mut self, block_base_address: u32, size: u32) -> Result<RAMBlock, AllocatorError>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized;
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error>;
}

pub struct RAMAllocatorImpl<
    'a,
    const START_ADDR: u32,   // Allocator start address
    const END_ADDR: u32,     // Allocator end address
    const BLOCK_SIZE: usize, // Minimum granularity of the allocator
    const NUM_BLOCKS: usize,
    const NUM_SLOTS: usize,
    const KERNEL_RESERVED: u32, // Initial RAM block reserved to the kernel
    const FLASH_ALLOCATOR_START_ADDR: u32,
    const FLASH_ALLOCATOR_END_ADDR: u32,
    const FLASH_ALLOCATOR_START_SCAN_ADDR: u32,
    const FLASH_NUM_SLOTS: usize,
    const FLASH_BLOCK_SIZE: usize,
    const FLAG_BYTES: usize, // Number of bytes to reserve for each flag
> {
    buddy_allocator: BuddyAllocatorImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>,
    flash_methods: &'a mut dyn FlashMethods<'a>,
}

impl<
        'a,
        const START_ADDR: u32,   // Allocator start address
        const END_ADDR: u32,     // Allocator end address
        const BLOCK_SIZE: usize, // Minimum granularity of the allocator
        const NUM_BLOCKS: usize,
        const NUM_SLOTS: usize,
        const KERNEL_RESERVED: u32, // Initial RAM block reserved to the kernel
        const FLASH_ALLOCATOR_START_ADDR: u32,
        const FLASH_ALLOCATOR_END_ADDR: u32,
        const FLASH_ALLOCATOR_START_SCAN_ADDR: u32,
        const FLASH_NUM_SLOTS: usize,
        const FLASH_BLOCK_SIZE: usize,
        const FLAG_BYTES: usize, // Number of bytes to reserve for each flag
    >
    RAMAllocatorImpl<
        'a,
        START_ADDR,
        END_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        NUM_SLOTS,
        KERNEL_RESERVED,
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_NUM_SLOTS,
        FLASH_BLOCK_SIZE,
        FLAG_BYTES,
    >
{
    const ALLOCATOR_SIZE: usize = (END_ADDR - START_ADDR + 1) as usize;

    pub fn from_flash(flash_methods: &'a mut dyn FlashMethods<'a>) -> Self {
        // Some asserts
        assert!(START_ADDR < END_ADDR);
        // Create a new allocator
        let mut allocator =
            BuddyAllocatorImpl::<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS>::new(
                true,
            );
        // Scan flash to populate the buddy allocator
        let mut curr_addr: u32 = START_ADDR + KERNEL_RESERVED; // Skip the segment reserved
        while curr_addr <= END_ADDR {
            // Check if it's part of an active component
            let mut flash_walker = FlashWalkerImpl::<
                FLASH_ALLOCATOR_START_ADDR,
                FLASH_ALLOCATOR_END_ADDR,
                FLASH_ALLOCATOR_START_SCAN_ADDR,
                FLASH_NUM_SLOTS,
                FLASH_BLOCK_SIZE,
                FLAG_BYTES,
            >::new(flash_methods);
            // Scan the whole flash again
            let mut occupied = false;
            while let Some(block) = flash_walker.next() {
                // Check if this block is assigned to a component
                if block.get_type() == BlockType::COMPONENT {
                    // In this case, the first 4 bytes of the block are the allocated address
                    let address_bytes = flash_walker.read(block.get_base_address(), 4).unwrap();
                    let ram_size = flash_walker.read(block.get_base_address() + 4, 4).unwrap();
                    let address = u32_from_le(address_bytes);
                    let size = u32_from_le(ram_size);
                    if address == 0xFFFF_FFFF || size == 0xFFFF_FFFF {
                        // Malformed block, just skip for now.
                        // Will be erased at next reboot
                        assert!(!block.is_finalized());
                        continue;
                    }
                    let end_addr_excl = address + size;
                    assert!(address >= START_ADDR && address <= END_ADDR);
                    if curr_addr >= address && curr_addr < end_addr_excl {
                        // Skip this whole block (or whatever remains)
                        curr_addr = end_addr_excl;
                        occupied = true;
                        break;
                    }
                }
            }
            if !occupied {
                // Add as free
                let block_number = (curr_addr - START_ADDR) / (BLOCK_SIZE as u32);
                allocator.add_free_block(block_number as u8);
                // Move one step
                curr_addr += BLOCK_SIZE as u32;
            }
        }

        Self {
            buddy_allocator: allocator,
            flash_methods: flash_methods,
        }
    }

    fn allocate(&mut self, block_base_address: u32, size: u32) -> Result<RAMBlock, AllocatorError>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        // Get flash block
        let flash_block_res = flash::utils::get_flash_block::<
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_NUM_SLOTS,
            FLASH_BLOCK_SIZE,
            FLAG_BYTES,
        >(self.flash_methods, block_base_address, false);
        // Check if this block is valid
        if flash_block_res.is_none() {
            return Err(AllocatorError::InvalidBlock);
        }
        let flash_block = flash_block_res.unwrap();
        if flash_block.get_type() != BlockType::NONE {
            return Err(AllocatorError::InvalidBlock);
        }
        // Get a new block of RAM
        let addr_result = self.buddy_allocator.alloc(size as usize);
        if addr_result.is_none() {
            return Err(AllocatorError::OutOfRAM);
        }
        let addr = addr_result.unwrap();
        let level: u16 = self.buddy_allocator.size_to_level(size as usize).unwrap() as u16;
        let actual_size = Self::ALLOCATOR_SIZE as u32 >> level;

        // Write configuration in flash
        let addr_bytes = addr.to_le_bytes();
        let size_bytes = actual_size.to_le_bytes();
        for i in 0u32..4u32 {
            // Write a byte of address
            self.flash_methods
                .write(block_base_address + i, addr_bytes[i as usize])
                .unwrap();
        }
        for i in 0u32..4u32 {
            // Write a byte of size
            self.flash_methods
                .write(block_base_address + 4 + i, size_bytes[i as usize])
                .unwrap();
        }
        // Flush write buffer
        self.flash_methods.flush_write_buffer().unwrap();

        // Mark the block as a component
        flash_allocator::flash::utils::mark_block::<
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_NUM_SLOTS,
            FLAG_BYTES,
        >(self.flash_methods, flash_block, BlockType::COMPONENT)
        .unwrap();

        // Return the allocation
        return Ok(RAMBlock {
            block_base_address: addr,
            block_size: actual_size,
            flash_position: flash_block,
        });
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        f.write_str("\n------- Allocator free_list -------\n")?;
        self.buddy_allocator.dump(f)?;
        Ok(())
    }
}

impl<
        'a,
        const START_ADDR: u32,   // Allocator start address
        const END_ADDR: u32,     // Allocator end address
        const BLOCK_SIZE: usize, // Minimum granularity of the allocator
        const NUM_BLOCKS: usize,
        const NUM_SLOTS: usize,
        const KERNEL_RESERVED: u32, // Initial RAM block reserved to the kernel
        const FLASH_ALLOCATOR_START_ADDR: u32,
        const FLASH_ALLOCATOR_END_ADDR: u32,
        const FLASH_ALLOCATOR_START_SCAN_ADDR: u32,
        const FLASH_NUM_SLOTS: usize,
        const FLASH_BLOCK_SIZE: usize,
        const FLAG_BYTES: usize, // Number of bytes to reserve for each flag
    > RAMAllocator<FLAG_BYTES>
    for RAMAllocatorImpl<
        'a,
        START_ADDR,
        END_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        NUM_SLOTS,
        KERNEL_RESERVED,
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_NUM_SLOTS,
        FLASH_BLOCK_SIZE,
        FLAG_BYTES,
    >
{
    fn allocate(&mut self, block_base_address: u32, size: u32) -> Result<RAMBlock, AllocatorError>
    where
        [(); FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        self.allocate(block_base_address, size)
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }
}

pub fn u32_from_le(arr: &[u8]) -> u32 {
    ((arr[0] as u32) << 0)
        + ((arr[1] as u32) << 8)
        + ((arr[2] as u32) << 16)
        + ((arr[3] as u32) << 24)
}

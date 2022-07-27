#![no_std]

use abi::flash::BlockType;
use buddy_allocator::{BuddyAllocator, BuddyAllocatorImpl};
use core::fmt::Formatter;
use flash_allocator::flash::{self, walker::FlashWalkerImpl, FlashBlock, FlashMethods};

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
    fn allocate(&mut self, block_base_address: u32, size: u32) -> Result<RAMBlock, AllocatorError>;
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
                    let address_bytes = flash_walker.read(block.get_base_address(), 4);
                    let ram_size = flash_walker.read(block.get_base_address() + 4, 4);
                    let address = u32_from_le(address_bytes);
                    let size = u32_from_le(ram_size);
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

    fn allocate(&mut self, block_base_address: u32, size: u32) -> Result<RAMBlock, AllocatorError> {
        // Get flash block
        let flash_block = flash::utils::get_flash_block::<
            FLASH_ALLOCATOR_START_ADDR,
            FLASH_ALLOCATOR_END_ADDR,
            FLASH_ALLOCATOR_START_SCAN_ADDR,
            FLASH_NUM_SLOTS,
            FLASH_BLOCK_SIZE,
            FLAG_BYTES,
        >(self.flash_methods, block_base_address, false);
        if flash_block.is_none() {
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
                .write(block_base_address + i, addr_bytes[i as usize]);
            // Write a byte of size
            self.flash_methods
                .write(block_base_address + 4 + i, size_bytes[i as usize]);
        }
        // Return the allocation
        return Ok(RAMBlock {
            block_base_address: addr,
            block_size: actual_size,
            flash_position: flash_block.unwrap(),
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
    fn allocate(&mut self, block_base_address: u32, size: u32) -> Result<RAMBlock, AllocatorError> {
        self.allocate(block_base_address, size)
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }
}

fn u32_from_le(arr: &[u8]) -> u32 {
    ((arr[0] as u32) << 0)
        + ((arr[1] as u32) << 8)
        + ((arr[2] as u32) << 16)
        + ((arr[3] as u32) << 24)
}
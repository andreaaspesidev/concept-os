#![no_std]

use heapless::Vec;
use core::fmt::{Debug, Write};
use core::fmt::Formatter;

pub trait BuddyAllocator {
    /// Returns the first memory address managed by this allocator
    fn start_addr(&self) -> u32;
    /// Maximum size managed by this allocator
    fn max_size(&self) -> usize;
    /// Whether a given physical address belongs to this allocator
    fn contains(&self, addr: u32) -> bool;
    /// Allocates memory for the requested size, returning the address of the base of this allocation
    fn alloc(&mut self, size: usize) -> Option<u32>;
    /// Deallocates memory previously allocated
    fn dealloc(&mut self, addr: u32, size: usize);
    /// Dump a structure representation to the specified formatter
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error>;
}

/// Creates a BuddyAllocator
/// NUM_BLOCKS: number of blocks (each of block_size bytes).
/// NUM_SLOTS: number of levels +1. Given the number of blocks, NUM_SLOTS = ceil(log2(NUM_BLOCKS)) + 1
/// 
/// TODO: replace level 0 with a flag (save up to NUM_BLOCKS bytes)
/// TODO: automatically infer NUM_SLOTS when the compiler will support operations on constants
pub struct BuddyAllocatorImpl
    <
        const START_ADDR: u32,
        const END_ADDR: u32,
        const BLOCK_SIZE: u16,
        const NUM_BLOCKS: usize, 
        const NUM_SLOTS: usize,
    > {
    num_levels: u8,
    free_lists: Vec<Vec<u8,NUM_BLOCKS>,NUM_SLOTS>
}

impl <const START_ADDR: u32, 
      const END_ADDR: u32,
      const BLOCK_SIZE: u16,
      const NUM_BLOCKS: usize, 
      const NUM_SLOTS: usize> BuddyAllocatorImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS> {
    
    /// Creates a new instance of the allocator
    pub fn new() -> BuddyAllocatorImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, NUM_SLOTS> {
        let mut free_lists: Vec<_,NUM_SLOTS> = Vec::<_,NUM_SLOTS>::new();
        // Check for hard limits
        if NUM_BLOCKS > 256 {
            panic!("Too many blocks");
        }
        // Create the sublists
        for _ in 0..NUM_SLOTS {
            free_lists.push(Vec::<u8, NUM_BLOCKS>::new()).unwrap();
        }
        // Populate the first one
        free_lists[0].push(0).unwrap();
        // Return the instance
        BuddyAllocatorImpl::<START_ADDR,END_ADDR,BLOCK_SIZE,NUM_BLOCKS,NUM_SLOTS> {
            num_levels: (NUM_SLOTS - 1) as u8,
            free_lists: free_lists
        }
    }
    
    fn max_size(&self) -> usize {
        (BLOCK_SIZE as usize) << (self.num_levels as usize)
    }
    
    fn contains(&self, addr: u32) -> bool {
        addr >= START_ADDR && addr <= END_ADDR
    }

    fn req_size_to_level(&self, size: usize) -> Option<usize> {
        // Find the level of this allocator than can accommodate the required memory size.
        let max_size = self.max_size();
        if size > max_size {
            // can't allocate more than the maximum size for this allocator!
            None
        } else {
            // find the largest block level that can support this size
            let mut next_level = 1; // As level 0 stores the whole memory
            while (max_size >> next_level) >= size {  
                next_level += 1;
            }
            // ...but not larger than the max level!
            let req_level = core::cmp::min(next_level - 1, self.num_levels as usize);
            Some(req_level)
        }
    }

    fn get_free_block(&mut self, level: usize) -> Option<u8> {
        // Get a block from the free list at this level or split a block above and
        // return one of the splitted blocks.
        self.free_lists[level]
            .pop()
            .or_else(|| self.split_level(level))
    }

    fn split_level(&mut self, level: usize) -> Option<u8> {
        // We reached the maximum level, we can't split anymore! We can't support this allocation.
        if level == 0 {
            None
        } else {
            self.get_free_block(level - 1).map(|block| {
                // Get a block from 1 level above us and split it.
                // We push the second of the splitted blocks to the current free list
                // and we return the other one as we now have a block for this allocation!
                self.free_lists[level].push(block * 2 + 1).unwrap();
                block * 2
            })
        }
    }

    fn alloc(&mut self, size: usize) -> Option<u32> {
        // We should always be aligned due to how the buddy allocator works
        // (everything will be aligned to block_size bytes).
        // find which level of this allocator can accommodate this amount of memory (if any)
        self.req_size_to_level(size).and_then(|req_level| {
            // We can accommodate it! Now to check if we actually have / can make a free block
            // or we're too full.
            self.get_free_block(req_level).map(|block| {
                // We got a free block!
                // get_free_block gives us the index of the block in the given level
                // so we need to find the size of each block in that level and multiply by the index
                // to get the offset of the memory that was allocated.
                let offset = block as u32 * (self.max_size() >> req_level as usize) as u32;
                // Add the base address of this buddy allocator's block and return
                START_ADDR + offset
            })
        })
    }

    fn merge_buddies(&mut self, level: usize, block_num: u8) {
        // toggle last bit to get buddy block
        let buddy_block = block_num ^ 1;
        // if buddy block in free list
        if let Some(buddy_idx) = self.free_lists[level]
            .iter()
            .position(|blk| *blk == buddy_block) {
            // remove current block (in last place)
            self.free_lists[level].pop();
            // remove buddy block
            self.free_lists[level].swap_remove(buddy_idx);  //TODO: check that an unordered list is not a problem for allocations
            // add free block to free list 1 level above
            self.free_lists[level - 1].push(block_num / 2).unwrap();
            // repeat the process!
            self.merge_buddies(level - 1, block_num / 2)
        }
    }

    fn dealloc(&mut self, addr: u32, size: usize) {
        // As above, find which size was used for this allocation so that we can find the level
        // that gave us this memory block.
        // find which level of this allocator was used for this memory request
        if let Some(req_level) = self.req_size_to_level(size) {
            // find size of each block at this level
            let level_block_size = self.max_size() >> req_level;
            // calculate which # block was just freed by using the start address and block size
            let block_num = ((addr - START_ADDR) as usize / level_block_size) as u8;
            // push freed block to the free list so we can reuse it
            self.free_lists[req_level].push(block_num).unwrap();
            // try merging buddy blocks now that we might have some to merge
            self.merge_buddies(req_level, block_num);
        }
    }

    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        for s in 0..NUM_SLOTS {
            f.write_fmt(format_args!("[{}] ", s))?;
            let slot_list = &self.free_lists[s];
            for b in 0..slot_list.len() {
                f.write_fmt(format_args!("{} ", slot_list[b]))?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }

}

impl <const START_ADDR: u32, 
      const END_ADDR: u32,
      const BLOCK_SIZE: u16,
      const NUM_BLOCKS: usize, 
      const NUM_SLOTS: usize> Debug for BuddyAllocatorImpl<START_ADDR,END_ADDR,BLOCK_SIZE,NUM_BLOCKS,NUM_SLOTS> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }
}

impl<const START_ADDR: u32, 
     const END_ADDR: u32,
     const BLOCK_SIZE: u16,
     const NUM_BLOCKS: usize, 
     const NUM_SLOTS: usize> BuddyAllocator for BuddyAllocatorImpl<START_ADDR,END_ADDR,BLOCK_SIZE,NUM_BLOCKS,NUM_SLOTS> {
        
    fn start_addr(&self) -> u32 {
        START_ADDR
    }

    fn max_size(&self) -> usize {
        self.max_size()
    }

    fn contains(&self, addr: u32) -> bool {
        self.contains(addr)
    }

    fn alloc(&mut self, size: usize) -> Option<u32> {
        self.alloc(size)
    }

    fn dealloc(&mut self, addr: u32, size: usize) {
        self.dealloc(addr, size)
    }

    fn dump(&self, formatter: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(formatter)
    }
}
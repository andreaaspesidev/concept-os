
use core::fmt::Formatter;
use core::fmt::Write;
use crate::BuddyAllocator;

/// A structure representing a node of the binary tree (chunk of memory).
/// Internally, it occupies a single byte (8 bits), threated as a series of flags.
/// 
/// x x x x  x x x 0
///                |-> available (free and not splitted) 
#[derive(Debug, Clone, Copy)]
struct TreeNode {
    flags: u8,
}

impl Default for TreeNode {
    fn default() -> Self {
        Self { flags: 0 }  // Not available
    }
}

impl TreeNode {
    pub fn is_available(&self) -> bool {
        self.flags > 0
    }
    pub fn take(&mut self) {
        assert!(self.is_available());
        self.flags = 0;
    }
    pub fn free(&mut self) {
        assert!(!self.is_available());
        self.flags = 1;
    }
}

///
/// Buddy allocator implemented using a binary tree, to minimize memory footprint.
/// 
/// The binary tree is implemented using an array. Each element of the array corresponds to a
/// node of the binary tree, in the following fashion:
/// 
/// array:
/// N[0] N[1] N[2] N[3] N[4] N[5] N[6] ...........
/// 
/// tree:
///                  N[0]           - level 0 (1): all memory available
///                 /    \
///             N[1]      N[2]      - level 1 (1/2): N[1] bottom-half of the memory 
///            /   \      /   \                N[2] top-half of the memory
///           N[3] N[4] N[5]  N[6]  - level 2 (1/4)
///          .....................  
/// 
/// We have 2^level - 1 nodes. Each node contains two bits:
/// - free bit: if asserted, that memory block cannot be allocated
/// - split bit: if asserted, it means at some point this block was split in two parts
///              to provide smaller contiguous areas
/// 
/// The first node N[0] denotes the whole memory area. It's the only one initialized
/// as free, but still not split. All the other nodes are initialized as not free and not split.
/// 
/// All sizes are considered units to be multiplied to the elementary block size to get the
/// actual size in bytes.
/// 
/// The memory footprint depends on the number of blocks (NUM_BLOCKS). As each block
/// can be allocated independently, this means we should have NUM_BLOCKS leaves in the tree.
/// At each level, we have 2^level nodes. So level = ceil(log2(NUM_BLOCKS)).
/// In a tree, we have 2^(max_level +1) -1 nodes.  
/// 
pub struct BinaryBuddyImpl<
    const START_ADDR: u32,
    const END_ADDR: u32,
    const BLOCK_SIZE: usize,
    const NUM_BLOCKS: usize,        // memory_area / block_size
    const TREE_MAX_LEVEL: usize,    // log2(num_blocks) = log2(memory_area / block_size)
    const NUM_NODES: usize          // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1
> {
    binary_tree: [TreeNode; NUM_NODES],
    highest_splitted_level: usize
}

impl<
        const START_ADDR: u32,
        const END_ADDR: u32,
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const TREE_MAX_LEVEL: usize,
        const NUM_NODES: usize
    > BinaryBuddyImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, TREE_MAX_LEVEL, NUM_NODES>
{
    /// Creates a new instance of the allocator
    /// @initialize_empty: if set, no free block exists in this allocator (everything is considered
    ///                    allocated).
    pub fn new(initialize_empty: bool) -> Self {
        // Inizialize the binary tree with no split nodes
        let mut binary_tree: [TreeNode; NUM_NODES] = [TreeNode::default(); NUM_NODES];
        if !initialize_empty {
            // Set the first node as free
            binary_tree[0].free();
            // Put the smallest_available_level to the first element (the root node)
            Self { binary_tree: binary_tree, highest_splitted_level: 0 }
        } else {
            // Consider everything allocated, i.e. search for blocks directly at the last level
            Self { binary_tree: binary_tree, highest_splitted_level: TREE_MAX_LEVEL }
        }
    }

    /// Returns the maximum number of size units a block can contain.
    /// This corresponds to the number of leaves.
    fn max_size_units(&self) -> usize {
        NUM_BLOCKS
    }

    fn get_level_from_size(&self, size: usize) -> Option<usize> {
        // The requested size is approximated to the next power of 2, 
        // then it is converted to size units ( / BLOCK_SIZE)
        let size_units = core::cmp::max(size.next_power_of_two() / BLOCK_SIZE, 1);
        // Then we have to check the minimum level we can use for this request
        // ideally, level = floor(TREE_MAX_LEVEL - log2(size_units))
        let max_size_units = self.max_size_units();
        if size_units > max_size_units {
            // Can't allocate more than the maximum size for this allocator!
            None
        } else {
            // Find the largest block level that can support this size
            let mut next_level = 1; // As level 0 stores the whole memory
            while (max_size_units >> next_level) >= size_units {
                next_level += 1;
            }
            // ...but not larger than the max level!
            let req_level = core::cmp::min(next_level - 1, TREE_MAX_LEVEL);
            Some(req_level)
        }
    }

    /// Splits a level of the tree, returning one of the two buddies
    fn split_level(&mut self, level: usize) -> Option<usize> {
        // We reached the maximum level, we can't split anymore! We can't support this allocation.
        if level == 0 {
            None
        } else {
            // Get a block from the level above and split
            self.get_free_block(level - 1).and_then(|node_index| {
                // Mark we reached this level of split
                self.highest_splitted_level = core::cmp::max(self.highest_splitted_level, level);
                // First, the parent is already taken
                if let (Some(left_child_index), Some(right_child_index)) = (self.node_left_child_index(node_index), self.node_right_child_index(node_index)) {
                    // Then the right child is left unused, mark as available
                    self.binary_tree[right_child_index].free();
                    // The other child is already marked as taken, just return
                    return Some(left_child_index);
                }
                return None;
            })
        }
    }

    /// Finds a free block at the requested level
    fn get_free_block(&mut self, requested_level: usize) -> Option<usize> {
        // If we have already split at this level, then change is we have an available block
        if requested_level <= self.highest_splitted_level {
            // We have to scan all the nodes at this level to find one that is available
            let (level_index_start, level_index_end) = self.nodes_from_level(requested_level);
            for node_index in level_index_start..level_index_end {
                let node = &mut self.binary_tree[node_index];
                if node.is_available() {
                    // Got an available node, first set as not free
                    node.take();
                    // ... then return the index
                    return Some(node_index);
                }
            }
        }
        // ... we got no available node. We have to split a smaller level
        self.split_level(requested_level)
    }

    /// Request a block to be allocated.
    /// @requested_size refers to the size in bytes.
    pub fn allocate(&mut self, requested_size: usize) -> Option<u32> {
        // We have to check the minimum level we can use for this request
        self.get_level_from_size(requested_size).and_then(|req_level| {
            // We can accommodate it! Now to check if we actually have / can make a free block
            // or we're too full.
            self.get_free_block(req_level).map(|node_index| {
                // Now we just have to get the memory area offset based on this block index
                let node_number = self.node_number_on_level(node_index, req_level);
                let level_size = ((self.max_size_units() >> req_level) * BLOCK_SIZE) as u32;
                let offset = node_number as u32 * level_size;
                // Add the base address of this buddy allocator's block and return
                START_ADDR + offset
            })
        })
    }

    fn merge_buddies(&mut self, level: usize, node_index: usize) {
        // Check if we reached the first level
        if level == 0 {
            return; // Break the recursion
        }
        // Get the buddy of this block
        let buddy_node_index = self.node_sibiling_index(node_index);
        // Check if it's free
        if self.binary_tree[buddy_node_index].is_available() {
            // Mark both as not available
            self.binary_tree[buddy_node_index].take();
            self.binary_tree[node_index].take();
            // Mark the parent as free
            let parent_index = self.node_parent_index(node_index).unwrap();
            self.binary_tree[parent_index].free();
            // Repeat the process at the upper level
            self.merge_buddies(level - 1, parent_index);
        }
    }

    fn contains(&self, addr: u32) -> bool {
        addr >= START_ADDR && addr <= END_ADDR
    }

    /// Deallocate a block of memory.
    pub fn deallocate(&mut self, addr: u32, size: usize) -> Option<()> {
        // Check we manage this address
        if !self.contains(addr) {
            return None;
        }
        // Convert to level
        self.get_level_from_size(size).and_then(|req_level| {
            // Find size of each block at this level
            let level_block_size_units = self.max_size_units() >> req_level;
            // Calculate which # node was just freed by using the start address and block size
            let node_number = (addr - START_ADDR) as usize / BLOCK_SIZE / level_block_size_units;
            // Launch the removal
            self.deallocate_by_node_number(node_number, req_level);
            return Some(());
        })
    }

    fn deallocate_by_node_number(&mut self, node_number: usize, level: usize) {
        // Get back the block index
        let node_index = self.node_index_from_number_and_level(node_number, level);
        // Mark the node as free
        self.binary_tree[node_index].free();
        // Try merge nodes now, starting from this.
        self.merge_buddies(level, node_index);
    }

    /*
        Auxiliary functions
    */
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        for level in 0..TREE_MAX_LEVEL+1 {
            f.write_fmt(format_args!("[{}] ", level))?;
            let (node_start, node_end) = self.nodes_from_level(level);
            for index in node_start..node_end {
                let node = &self.binary_tree[index];
                if node.is_available() {
                    let node_num = self.node_number_on_level(index, level);
                    f.write_fmt(format_args!("{} ", node_num))?;
                }
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }

    /*
        Binary Tree auxiliary functions
    */
    fn node_left_child_index(&self, node_index: usize) -> Option<usize> {
        let value = 2*node_index + 1;
        if value >= self.binary_tree.len() {
            None
        } else {
            Some(value)
        }
    }
    fn node_right_child_index(&self, node_index: usize) -> Option<usize> {
        let value = 2*node_index + 2;
        if value >= self.binary_tree.len() {
            None
        } else {
            Some(value)
        }
    }
    fn node_parent_index(&self, node_index: usize) -> Option<usize> {
        if node_index == 0 {
            return None;
        }
        return Some((node_index-1) / 2);
    }
    fn node_sibiling_index(&self, node_index: usize) -> usize {
        // ((5 – 1) ^ 1) + 1 = (4 ^ 1) + 1 = (100b ^ 1b) + 1b = 101b + 1b = 110b = 6
        // ((6 – 1) ^ 1) + 1 = (5 ^ 1) + 1 = (101b ^ 1b) + 1b = 100b + 1b = 101b = 5
        ((node_index - 1) ^ 1) + 1
    }
    fn node_number_on_level(&self, node_index: usize, node_level: usize) -> usize {
        // Get the number of nodes at the prev. level = 2^(node_level) -1
        // Ex. N[3] (level 2): 3 - ((1 << 2) - 1) = 3 - (3) = 0
        //     N[4] (level 2): 4 - (4 - 1) = 4 - 3 = 1
        node_index - ((1 << node_level) - 1)
    }
    fn node_index_from_number_and_level(&self, node_number: usize, node_level: usize) -> usize {
        // Get the number of nodes at the prev. level = 2^(node_level) -1
        // Ex. N[3] (level 2): 3 - ((1 << 2) - 1) = 3 - (3) = 0
        //     N[4] (level 2): 4 - (4 - 1) = 4 - 3 = 1
        node_number + ((1 << node_level) - 1)
    }
    /// Returns two indices
    /// - the first indicates the index first node at that level
    /// - the second instead indicates the index of the last node (escluded) at that level
    fn nodes_from_level(&self, level: usize) -> (usize, usize) {
        if level == 0 {
            return (0,1);
        }
        // To get the index of the first element, we have to sum
        // all the lengths of the previous levels, considering that
        // the elements are saved sequential per level.

        // Let's remember that the sum of power of 2 is one less than
        // the next power of 2 (https://jarednielsen.com/sum-consecutive-powers-2/)

        // level 1:
        //   start: (1 << 1) - 1 = 2 - 1 = 1
        //   size: 1 << 1 = 2
        //   [0] [1] [2] [3] [4] [5] [6] [7] [8] [9] [10] [11] [12] [13] [14]
        //       -------
        // level 2:
        //   start: (1 << 2) - 1 = 4 - 1 = 3
        //   size: 1 << 2 = 4
        //   [0] [1] [2] [3] [4] [5] [6] [7] [8] [9] [10] [11] [12] [13] [14]
        //               ---------------
        // level 3:
        //   start: (1 << 3) - 1 = 8 - 1 = 7
        //   size: 1 << 3 = 8
        //   [0] [1] [2] [3] [4] [5] [6] [7] [8] [9] [10] [11] [12] [13] [14]
        //                               ------------------------------------
        let start: usize = (1 << level) -1;
        let length: usize = 1 << level;
        return (start, start + length);
    }

    pub const fn smallest_block_level() -> usize {
        TREE_MAX_LEVEL
    }
}

impl<
        const START_ADDR: u32,
        const END_ADDR: u32,
        const BLOCK_SIZE: usize,
        const NUM_BLOCKS: usize,
        const TREE_MAX_LEVEL: usize,
        const NUM_NODES: usize
    > BuddyAllocator for BinaryBuddyImpl<START_ADDR, END_ADDR, BLOCK_SIZE, NUM_BLOCKS, TREE_MAX_LEVEL, NUM_NODES>
{
    fn alloc(&mut self, size: usize) -> Option<u32> {
        self.allocate(size)
    }
    fn contains(&self, addr: u32) -> bool {
        self.contains(addr)
    }
    unsafe fn dealloc(&mut self, addr: u32, size: usize) {
        self.deallocate(addr, size).unwrap()
    }
    fn add_free_block(&mut self, block_num: usize) -> Option<()> {
        self.deallocate_by_node_number(block_num, TREE_MAX_LEVEL);
        Some(())
    }
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.dump(f)
    }
    fn max_size(&self) -> usize {
        self.max_size_units() * BLOCK_SIZE
    }
    fn size_to_level(&self, size: usize) -> Option<usize> {
        self.get_level_from_size(size)
    }
    fn start_addr(&self) -> u32 {
        START_ADDR
    }
    fn is_all_available(&self) -> bool {
        self.binary_tree[0].is_available()
    }
}
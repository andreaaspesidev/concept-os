# RAM Memory
Each component -also the kernel- requires a stack in order to execute.
This stack is placed in volatile ram (SRAM), along with the `.data` section (as contains mutable data, cannot be placed in flash).

The addressing space on STM32 boards is unified, and a specific portion is mapped to SRAM. More SRAMs could be present, as some are accessible via DMA: in our case we consider the bigger one.

Some STM32 boards (like the STM32F303) can have more specific SRAMs, that can be used for example for the kernel stack, or DMA with some specific peripheral.

In any case, only one main SRAM is considered here, as this is the common case.

## Challenges of SRAM allocation
Allocating SRAM requires:
1. Satisfying the alignment constraint of the MPU: start address of each allocation must be naturally aligned (*i.e. multiple of*) with its size.
2. Allocations for a component must be permanent, as some references could have been fixed in component flash (this should not be the case though). This can help also debugging, and avoid reallocation at every start-up but only when the system is changing. 
3. The kernel still needs SRAM to execute, but this must not be deallocated in any case (and be allocated to a component).

## Buddy Allocator
>This two sections below are based on the clear [explanation/implementation](https://nfil.dev/kernel/rust/coding/rust-buddy-allocator/) of *Nikos Filippakis* for Rust OS. The algorithm was then redesigned to exploit a binary-tree.

We consider this simple allocator, as:

- it allows compaction of memory with little overhead, at the same time showing a little [external fragmentation](https://en.wikipedia.org/wiki/Fragmentation_(computer)#External_fragmentation).
Problems of internal fragmentation are inevitable. 

- if the start address for the allocations is naturally aligned to the size, all splits will be placed to addresses naturally aligned with that split size (see [here, section 1.1](https://cs.au.dk/~gerth/papers/actainformatica05.pdf)). This is great as satisfies as side effect the MPU requirements (point 1).

The working principle is quite intuitive:

**Allocation:**
1. every time we get a request for a memory block, we round up the request size to the next power of two.
2. if we do not have a free block for this size, we take the next available bigger block, and we halve it until we reach the requested size. The other half is called `buddy` and gives the name to the algorithm.
    ```
    1.  128 bytes of memory and you get a request for 16 bytes
    |                   128                    | (initial memory state)
    |------------------------------------------|
    |         64          |         64         | (divide 128-block)
    |------------------------------------------|
    |    32    |    32    |         64         | (divide first 64-block)
    |------------------------------------------|
    | 16 | 16  |    32    |         64         | (divide first 32-block)
    |------------------------------------------|
    | xx | 16  |    32    |         64         | (serve first 16-block and mark as used)
    
    2. another request for 32 bytes
    | xx | 16  |    32    |         64         | (initial memory state)
    |------------------------------------------|
    | xx | 16  |    xx    |         64         | (serve first 32-block and mark as used)
    ```

**Deallocation:**
1. Upon a memory deallocation, we search for the buddy of this block (`block_address XOR 1` gives the buddy), and if the buddy is free, we merge them back recursively.
    ```
    3. previous 16 bytes are freed
    | xx | 16  |    xx    |         64         | (initial memory state)
    |------------------------------------------|
    | 16 | 16  |    xx    |         64         | (the previous block was freed)
    |------------------------------------------|
    |    32    |    xx    |         64         | (the two buddies have been merged)
    ```

### Buddy terminology
The standard buddy implementations relies on a memory structure made of a list of lists:

- A `block` is a contiguous block of memory. It is identified by its level in the buddy allocator and its index in that level. It can be split into two blocks of half the size (1 level below) or be united with its buddy block to make a block of double the size (1 level above).
- A `buddy_block` is relative to another block, and it’s the block the other block could be united with. The buddy of block i is the block i xor 1, so the blocks 0 and 1 in a level are buddies, and so are 10 and 11 (but not 9 and 10!).
- A `level` is a list of blocks of the same size. A buddy allocator starts off with a single level 0 block which can then be split into two level 1 blocks, and so on. On each level, the first block has index 0, so the blocks 0 and 1 at level 4 if united would generate the block level 0 at level 3 (and vice-versa if split).
- `num_levels` is the number of non-leaf levels for a buddy allocator. If we support levels 0, 1 and 2 (ie. our L0 block can be split into 2 L1 blocks or 4 L2 blocks), our level number would be 2.
- The `block_size` shall be the size of each block on the leaf level, so the minimum size of a memory block that we can return. One level above that, the blocks will be of size block_size * 2 and so on.

We keep an outer list of num_levels elements, called `free_list`, each corresponding to the level list of that index.

### Storing metadata
In order to satisfy the second point (permanent assignments), we need to store the base address of the allocation for each component (the size is not needed as it will always be the size required by the component in the `HBF`).

This information can be efficiently stored inside the block allocated in flash for storing the `HBF` of this component, that can be organized in this way:

```
        | 12 bytes |  4 bytes  |  4 bytes  |... | ...        |
    ... |  Header  | SRAM Base | SRAM Size |HBF | Free Space | ... 
```

This keeps the HBF start aligned to 4 bytes (requirement), and avoid using a complex filesystem in order to allocate new components.

#### Reconstructing structures from metadata
In order to work correctly, buddy algorithm requires the `free_list` to be correctly populated. It's important to be able to reconstruct this list from the information stored in flash.

1. It's always possible to obtain a list of (SRAM base, size) for each component, by reading the data from the flash (*this information could be provided by the kernel itself, as it loads this data during the start-up procedure*).
2. This list must then be ordered by base ascending.
3. The list is then scanned from the first to the last element. Each time, `(addr_j - (addr_i + size_i)) / block_size` blocks are added to the last level of the `free_list`, where `j` is the index of the element in the list, `i` is the index of the prev. element.
4. Each time a block is added to the `free_list`, buddy merging procedure is tried on this block.


## Kernel SRAM Conflicts
Two approaches are possible here, depending on the resources available:
- the most naive solution, in case of high resources or another SRAM available for the kernel (as the `CCM SRAM` in `STM32F303`), is to allocate only a subregion of SRAM to the buddy allocator. **The main problem with this approach is that the base of this subregion must be naturally aligned with the size (or all the reasoning about MPU requirements will be void).** This dramatically reduces the SRAM that will be available for the components if no additional SRAM can be used for the kernel (as for `CCM SRAM`).
- the other approach is to assign to the allocator all the available SRAM, but then during initialization of the allocator, as first step, allocate the SRAM needed by the kernel (hard-coded, or asked via a syscall). **We used this approach.**
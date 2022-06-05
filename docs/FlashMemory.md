# Flash Memory
On STM32 boards, programs are stored in Flash Memory. They can be also directly executed from Flash: SRAMs are usually too small to fit also the code, and are reserved only for stacks/runtime data.

## Flash Programming
Flash memory differs from standard memory for what concerns writes. 

In order to reprogram flash memory, an `in-application programming (IAP)` is needed: IAP allows the user to re-program the Flash memory while the application is running.

Flash memory is divided into erase sectors, which can be in some cases also of different sizes and rather large (es. STMF4, STMF7, ...).

- An erase sector is the minimum granularity for an erase operation. *Erasing a sector sets all bits in the sector to 1*.

- Writes must be performed with the granularity of the specific Flash memory (in some cases can be modified using an apposite register for parallelism). Usually goes from half-byte (*16 bits*) to a full word (*32 bits*).

- When setting bits from 1 to 0 no erase is required. But it's not possible to perform the opposite operation without erasing the whole sector first.

All these operations are possible by exploiting the memory-mapped registers of the IAP (`FLASH_SR`, `FLASH_CR`, ). But first the flash must be unlocked using the ad-hoc key for `FLASH_KEYR`.
By setting the `PG` flag (programming flag) in the register `FLASH_CR`, after the unlock procedure, it's possible to issue a write to the corresponding flash location, with the right granularity.

## Flash Layout
The minimum erase granularity complicates the operations needed in order for the system to be updated.

The only hard constraint for the positioning of the code in flash (for Cortex-M4), is the following initial layout:

<img src="images/cortex-m4-vectors.png">


For simplicity, we can start placing all the Kernel code at the beginning of the Flash, along with all entrypoints needed by the hardware (*HardFault, DefaultHandler, ...*)

```
+--------------+------+------....
|  0x08000000  | .... |
+--------------+------+------....
|    IVT + Kernel     |
+--------------+------+------....
```

The Kenel can now start, but in order to have a functioning systems also Components must be loaded. Components are also placed in Flash as `HBF` files (see `toolchain/HubrisBinaryFormat.md`).

From a design perspective:
- the kernel must be able to find these HBF binaries in flash. As the system can be updated, this information is not known a priori, and must be stored itself in flash.
- as Memory Protection Unit (MPU) is adopted to increase reliability, HBF must be placed in a way to satisfy MPU strict requirements on *Base Address* and *Size*.
- HBF must be placed in memory in a way to control fragmentation, or the system can rapidly become impossible to be updated.

### MPU Requirements
The MPU shipped on board of STM32 Cortex-M4 has quite strict requirements.
Up to max 8 regions can be created. Each region can be then split into 8 subregions (each can be enabled/disabled).
Each region:
- has a start address that must be multiple of the MPU size (natural alignment).
- has a size that must be 2^n x 32 bytes.

The main problem for space allocation is the fact that as the size needed by a component increases, we start having less and less suitable base addresses.

## Buddy Allocator
We consider a modified version of this simple allocator, as:

- it allows compaction of memory with little overhead, at the same time showing a little [external fragmentation](https://en.wikipedia.org/wiki/Fragmentation_(computer)#External_fragmentation).
Problems of internal fragmentation are inevitable. 

- if the start address for the allocations is naturally aligned to the size, all splits will be placed to addresses naturally aligned with that split size (see [here, section 1.1](https://cs.au.dk/~gerth/papers/actainformatica05.pdf)). This is great as satisfies as side effect the MPU requirements.

See `RAMMemory.md` for details.

### Storing metadata
One of the critical aspects of the allocator is the ability to reconstruct flash state after an hard reboot. Two different strategies can be used:
- store the metadata about flash layout in a dedicated flash area, with a journaling fashion.
- store the metadata at the beginning of a block itself. As metadata changes always refer to the block we are working on, this avoids the need to have a complex filesystem. The drawback is consuming space of the block.

This second option is considered, due to the constraint resources of the STM32 devices. Each block (apart from the one dedicated to the kernel, more later) starts with an header. This header must be aligned to 4 in size, to avoid problems later.

Offset| Size (bytes) | Field Name  |            Content           |
------|--------------|-------------|------------------------------|
0x00  |      2       | Block State | Flags associated with this block
0x02  |      2       | Block Level | Level of this block, used to derive the size

In particular:
- `Block State`: Represents information to understand the content of the block. Bits are set low, (0 = on, 1 = off).
    |    15     |...| 7 | 6 | 5 | 4 | 3 |     2     |     1     |     0     |
    |-----------|---|---|---|---|---|---|-----------|-----------|-----------|
    | COMPONENT | R | R | R | R | R | R | FINALIZED | DISMISSED | ALLOCATED |
    
    where:
    - `ALLOCATED` means this block was once allocated. `Block Level` can be used to determine the allocated size (in use).
    - `DISMISSED` means this block was then deallocated. `Block Level` can be used to retrieve the old size of the block. 
    - `FINALIZED` means all the information has been written successfully in this block, and so it's ready to be used. **On start-up, all allocated blocks that are not finalized are marked `DISMISSED`.**

    - `COMPONENT` means this block contains the code of a component, so an HBF is expected after the header.

### Reconstructing State from Metadata
It's always possible to retrieve the `free_list` from the flash memory upon start-up, using the following procedure:

<img src="images/allocator_reconstruction.png">

### Changes to the algorithm
Let's define:
- A `free_block` is a block that was erased, and that can be written in any location. It's not assigned yet. (`ALLOCATED = 0`, `DISMISSED = 0`)
- An `allocated_block` is a block that was allocated, so we must presume it was written. It's locked and cannot be modified. (`ALLOCATED = 1`, `DISMISSED = 0`)
- A `freed_block` is an `allocated_block` after deallocation for it is requested. We must presume it was written, so cannot be used before being erased again. (`ALLOCATED = 1`, `DISMISSED = 1`)

The key principle are the following:
1. we assume the memory was initially completely erased.
2. upon any block deallocation, we have two possible behaviors:
    1. the page containing this `freed_block` contains only `freed_blocks` or `free_blocks`. In this case, we erase the page, and add all the corresponding blocks back into the `free_list`, as  now they are `free_blocks`
    2. the page containing this `freed_block` contains at least one `allocated_block`. In this case, we act lazy and do nothing, apart marking `DISMISSED = 1` on the block
3. upon any block allocation, we search for a candidate `free_block` in the `free_list`. If no block of the requested size is found, try to split another block from an upper `level` of the `free_list`. It's the same behavior of the standard case. If no such block is found, we have two cases:
    - we actually have no more memory, the request fails.
    - we have some memory, but this cannot be used before the recollection process. Launch the process, then try again the request.

#### Memory Scan and Recollection
This process is executed lazily at the first time the memory allocation fails. It proceeds as following:

**Scan phase:**
1. Read the first block header at the beginning of the memory area managed by this allocator. Scans the whole flash sector, searching for a block with `DISMISSED = 1`. If such block is found, remember whether this sector contains a `freed_block` is in a temp SRAM list (one flag per sector)
2. If we find a `freed_block`, jump directly to the next sector, and continue. Otherwise jump by one block at a time, skipping `allocated_blocks`.

**Recollection phase:**
1. Erase the swap block.
2. Start from the first page again. If the page was marked, then copy all blocks to swap if they have are `allocated_blocks`. After the page is copied, erase the page and copy back the `allocated_blocks`. Add the `freed_blocks` (now `free_blocks`) to the `free_list`.
3. Continue for each flash sector after this.

**Notes:**
When performing a swap operation, the component contained (or overlapping with) the sector under process must be put on hold. Actually the flash write operation itself should stall the CPU, but alone is not enough: **swap itself must be managed by the kernel, to enable swapping the block of this component**.

#### A simpler solution
If the Flash memory has erase sectors of constant size (possibly small), this becomes easier. For example, considering `STM32F303E` (that can be found on the `NUCLEO-F303RE`), these blocks are 2Kb in size (see [here](https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf)):

| Address Range             | Size (bytes)  |  Name     |
|---------------------------|---------------|-----------|
| 0x0800 0000 - 0x0800 07FF |  2K           | Page 0    |
| 0x0800 0800 - 0x0800 0FFF |  2K           | Page 1    |
| 0x0800 1000 - 0x0800 17FF |  2K           | Page 2    |
| 0x0800 1800 - 0x0800 1FFF |  2K           | Page 3    |
|   .............           |  2K           | ......    |
| 0x0807 F800 - 0x0807 FFFF |  2K           | Page 255  |

The problem of swapping can be easily avoided by having a minimum granularity in the allocation of 2Kb, so each page deallocated can be directly erased without swapping first.

### Kernel Flash Conflicts
Also the Kernel need space (at the beginning of the Flash).
Two approaches are possible here, depending on the resources available:
- the most naive solution, in case of a big Flash Memory, is to allocate only a subregion of Flash to the buddy allocator. **The main problem with this approach is that the base of this subregion must be naturally aligned with the size (or all the reasoning about MPU requirements will be void).** This dramatically reduces the Flash that will be available for the components (half the size of the Flash).
- the other approach is to assign to the allocator all the available Flash, but then during initialization of the allocator, as first step, allocate the Flash needed by the kernel (hard-coded, or asked via a syscall).

# PIC Code Generation
Components could be loaded in a different position in flash and ram with respect to the one they were linked with. This surely breaks code that is statically compiled, but during the years the concept of `PIC` (Position Independent Code) was introduced.

In particular, *ARM* introduced a few relocation models for ARM targets:
- `ROPI` (Read-only position independence): Read-Only data is placed in FLASH and accessed only PC-relative
- `RWPI` (Read-write position independence): Read-Write data is accessed relative to a base address stored in the `base_register` R9 (fixed in the compiler/linker)
- `ROPI/RWPI`: a combination of the two above

## Ops...
The perfect relocation model for our purpose is `ROPI/RWPI`. 
**The problem is that this relocation model is yet not completely correct for Rust**. In our investigation, we found that PC-relative and R9-relative addressing of data works correctly (only in *release*), but `.rodata` and `.data` still contains in some cases (*global mutable variables*, *traits with dynamic dispatching*, ...) absolute addresses fixed during linking.

Here is proposed an algorithm to correct this problem during the placement of the component in flash, when resources for the component are allocated.

### Origin of the problem
This problem is known since ARM proposed the relocation model in LLVM in 2015 (for C and C++). Changes related to this problem were never merged, as require changes also to the `rustc` compiler. 
ARM proposed to solve the problem (for C and C++) populating these locations at start-up, instead of doing so at the linking step.
The application had a special entry point routine that fixed the addresses before jumping to the main.

# Our solution
If we compile the code in a static way, we can make the few changes needed to make it relocatable.
The LLVM compiler can emit relocations alongside the ELF with the `-q` parameter. [Here](https://github.com/ARM-software/abi-aa/releases/download/2022Q3/aaelf32.pdf) are a list of possible relocations.

We found in Rust the only relocations emitted during code generation are:
- `R_ARM_THM_CALL (10)`: relative jump
- `R_ARM_THM_MOVW_ABS_NC (47)`: absolute move (low half-word)
    <image src="../images/reloc-1.png">
- `R_ARM_THM_MOVT_ABS (48)`: absolute move (high half-word)
    <image src="../images/reloc-2.png">

Inspired by the [LLVM blog post](https://blog.llvm.org/posts/2021-10-01-generating-relocatable-code-for-arm-processors/) we noticed that:
- The “ARM Relocation LLVM” method only allows a single fix to the code. It’s not possible to move that code again (“losing” the PIC characterization).
- The compiler (es. rustc) might emit MOVW and MOVT not necessarily sequential, even interleave movs related to different destination registers and add other instructions in-between.

<image src="../images/reloc-3.png">

- The compiler can interleave movs related to different destination registers and add other instructions in-between.

<image src="../images/reloc-4.png">

Using the option `-q` the LLVM linker emits the final relocation section, with adjusted offsets. From this section, it’s possible to understand the relationships shown below. 
**For each of these relocations, a relocation point is constructed.**

To correct a fixed address, it’s enough to use the following formula:
```
NEW_ADDRESS = value - LINKED_BASE_ADDRESS + NEW_BASE
```
To fix MOV relocations, it’s necessary to reconstruct such an address from both the paired MOVW and MOVT. This means it’s necessary to encode this information into the relocation point, in form of offset.

<image src="../images/reloc-5.png">

## Relocation Points

Currently, relocations are referred to the beginning of the CBF. 
- At least one bit should be reserved to check for MOV/ABS FIELD. It’s possible to avoid this bit and check whether the pointed element is a valid MOV, still misinterpretations might occur.
- Some relocations might be paired. This is the case for MOV, where we have the MOVW and MOVT. To relocate one of these, the other one should be accessed. Some bits are reserved to encode the offset of the pair, if any.

<image src="../images/reloc-6.png">

## Fixing a Relocation Point
Due to the structure of the flash, the relocation method is functional only if it can be applied “on-the-fly” while storing the component on a new flash position. Here it’s described the proposed implementation,
where the relocation points are assumed known (metadata previously transmitted). Such points are consumed in-order.

<image src="../images/reloc-7.png">

The working buffer is filled in FIFO mode with as much data coming from the network buffer as possible.
Then it’s flushed in FIFO until a relocation point is found. Then both paired relocation points are fixed, and the buffer it’s flushed up to that point.

<image src="../images/reloc-8.png">

The next relocation point is loaded, and the buffer filled again, then flushed again until that point is reached.
*Of course this “filling” is avoided in practice if there is no chance of reaching that point with the data coming from the network buffer.*

<image src="../images/reloc-9.png">

Compiled with size minimization, the algorithm could occupy about 2676 bytes (code + constants).
It’s feasible to be used in an embedded environment.

## In the toolchain
We have a python script `elf_relocations.py` that exports the ELF relocations in a format similar to the following:
```toml
[relocations]
text = [ [ 0, 0, 716], [ 0, 0, 720], [ 0, 0, 1332] ]
rodata = [ [ 0, 0, 0], [ 0, 0, 12]]
data = []
```
- The first element identifies the type:
  - `0`: `ABSOLUTE ADDRESS`
  - `1`: `MOVW` instruction
  - `2`: `MOVT` instruction
- The second element is the paired instruction (0 = no paired). Will be encoded in 5-bit 2's complement.
- The third is the offset from the start of the section. All these offsets will be relative to the start of the CBF.

All these elements are then saved in the CBF apposite sections (see `ConceptOSBinaryFormat.md`) as 32bit numbers.

Then these points are parsed and given to the relocating algorithm presented above and shipped with the `UPDATE` component.
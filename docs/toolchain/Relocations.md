# Relocations
Components could be loaded in a different position in flash and ram with respect to the one they were linked with. This surely breaks code that is statically compiled, but during the years the concept of `PIC` (Position Independent Code) was introduced.

In particular, *ARM* introduced a few relocation model for ARM targets:
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

### The solution (?)
As its far from trivial implementing the changes of ARM for Rust, here is proposed a different approach.

The idea behind is that if we can understand with precision where these locations are, we could mark them and later fix them with a procedure that must be very simple.

This was achieved using the linker-generated mapping file, that specifies for each section of the output file which section of the original object files was considered (following the linker script rules provided).
Mostly important, the original object files had relocation information, so it's known whether a certain section of the object file has undergone assolute addressing fixing by the linker.

On these premises, the script works as following:
1. The linker map file is parsed
2. For each object section of the interested output sections (`.rodata` and `.data`), the original object files are opened.
    i.  For each interested object sections, relocation info is searched in the object
    ii. Section-relative offset are returned
3. The script then tries to reconstruct the output section layouts using retrieved data. In doing so, converts relocations points to *object section-relative* to *output section-relative*.

An example output is the following:
```toml
[relocations]
rodata = [ 124, 136, 140, 152,]
data = [ 16,]
```

Relocation points are kept *output section-relative*, in order to be then later processed during the construction of the `HBF`. **Each point specifies then the offset in bytes from the start of the that section of an absolute address of 32bits.**
*Is rather simple converting these points `ELF`-start based if needed.*

### Limitations
For the algorithm to work, the linker must use only data of the object files. Sometimes the linker introduces some `<internal>` tags inside sections.
It's not possible to check for absolute addresses in these bytes, so if any absolute address happens to hide here, the overall relocation will fail.
**Heuristics could be implemented for these bytes.**

*It could be further investigated if this can really happen, or it's just a non-problem. It's ignored at the moment.*
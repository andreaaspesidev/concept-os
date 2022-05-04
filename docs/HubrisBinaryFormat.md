# Hubris Binary Format
Each Hubris component must be stored in flash in the Hubris Binary Format (HBF).  
A HBF includes a header portion, which encodes meta-data about the process, followed by a binary blob which is executed directly.
```
Hubris Component Binary:

Start of HBF  ->    +-------------------+
                    | HBF Header        |
                    +-------------------+
                    |                   |
                    | Compiled comp.    |
                    | binary            |
                    |                   |
  End of HBF  ->    +-------------------+
```

The component header stores all the information needed to load and execute the component itself. This comprises:
- the component priority, needed for scheduling
- the component flags, needed to customize system behaviour with respect to this component
- how much SRAM the component need (stack size)
- every region of the address space the component needs in order to work correctly
- every interrupt number the component will manage

## Placement
In order to work correctly, the start address in flash of every HBF file must be word-aligned (multiple of 4). This is due to the ARM requirements of alignment of instructions (see later).

## HBF Header
The HBF header is composed as follows:

```
start of HBF ->  +---------------------+ 0x00
                 |                     |
                 |     Header Base     |
                 |                     |
                 +---------------------+ 0x24 
                 |     Header Main     |
                 +---------------------+ 0x34
                 |   Header Region #1  |
                 +---------------------+ 0x34 + 0x0C
                 |        ....         |
                 +---------------------+
                 |   Header Region #N  |
                 +---------------------+ 0x34 + 0x0C*N
                 | Header Interrupt #1 |
                 +---------------------+ 0x34 + 0x0C*N + 0x08
                 |        ....         |
                 +---------------------+
                 | Header Interrupt #I |
                 +---------------------+ 0x34 + 0x0C*N + 0x08*I
                 |  Header Reloc. #1   |
                 +---------------------+ 0x34 + 0x0C*N + 0x08*I + 0x04
                 |        ....         |
                 +---------------------+
                 |  Header Reloc. #R   |
end of header -> +---------------------+ 0x34 + 0x0C*N + 0x08*I + 0x04*R
```
*Total size: 36 (base) + 16 (main) + 12*N (region) + 8*I (interrupts) + 4*R (relocs) = 52 + 12*N + 8*I + 4*R [bytes]*
**(must be multple of 4 for alignment problems)**

### HBF Header Base

Offset    | Size (bytes)  |  Field Name        |    Content    |
----------|---------------|--------------------|---------------|
0x00      |       4       | Magic Number       | `0x7F` followed by HBF (`0x48 0x42 0x46`) in ASCII
0x04      |       2       | HBF Version        | Integer 1-65535 indicating the version of the HBF
0x06      |       4       | Total size         | Total size of the HBF in bytes
0x0A      |       2       | Component ID       | Integer 1-65535 indicating the ID of the component. 0 is reserved to the kernel
0x0C      |       2       | Component Version  | Integer 0-65535 indicating the component major version, for compatibility checks
0x0E      |       2       | Main Offset        | Offset in bytes (from the start of the HBF) of the Header Main structure
0x10      |       2       | Region Offset      | Offset in bytes (from the start of the HBF) of the first Header Region structure
0x12      |       1       | Region Count       | Number of entries of the prev. structure
0x13      |       2       | Interrupt Offset   | Offset in bytes (from the start of the HBF) of the first Header Interrupt structure
0x15      |       1       | Interrupt Count    | Number of entries of the prev. structure
0x16      |       4       | Relocation Base    | Last Flash Address of when the relocation was performed
0x1A      |       2       | Relocation Offset  | Offset in bytes (from the start of the HBF) of the first Header Relocation structure
0x1C      |       4       | Relocation Count   | Number of entries of the prev. structure
0x20      |       4       | Checksum           | CRC-32b of the whole HBF (except this field)

*Total size: 36 bytes*

### HBF Header Main
This structure encodes all the information needed to start and schedule the component

Offset    | Size (bytes)  |  Field Name         |    Content    |
----------|---------------|---------------------|---------------|
0x00      |       1       | Component Priority  | Component scheduled priority 0-255
0x01      |       1       | Flags               | Flags associated to the component. Currently bit 0 set indicates load at startup 
0x02      |       2       | Component RAM       | Minumum SRAM needed by the component (stack)
0x04      |       4       | Entry Point Offset  | Offset of the main entry point from the start of the HBF
0x08      |       4       | Data Section Offset | Offset of the data section (`.data`) to be moved into RAM at startup
0x0C      |       4       | Data Section Size   | Size in bytes of the data section (`.data` + `.bss`)

*Total size: 16 bytes*

### HBF Header Region
This structure contains data regarding each memory region the component requires in order to work

Offset    | Size (bytes)  |  Field Name         |    Content    |
----------|---------------|---------------------|---------------|
0x00      |       4       | Region Base Address | Address of start of region
0x04      |       4       | Region Size         | Size of region, in bytes (on ARMv7-M, it must be a power of two greater than 16)
0x08      |       4       | Region Attributes   | Flags describing what can be done with this region (see next)

*Total size: 12 bytes* (**must be multiple of 4 to avoid alignment problems**)

### HBF Header Interrupt
This structure contains data regarding each interrupt that this component manages, and the corresponding behaviour

Offset    | Size (bytes)  |  Field Name        |    Content    |
----------|---------------|--------------------|---------------|
0x00      |       4       | IRQ Number         | Interrupt number
0x04      |       4       | Notification Mask  | This value will be OR-ed with the notification bit mask of the component upon interrupt reception

*Total size: 8 bytes* (**must be multiple of 4 to avoid alignment problems**)

### HBF Header Relocation
This structure contains relocation offsets for the component, pointing to 4 byte fields that need to be fixed before the application is moved in/around flash.  
**Workaround for ROPI/RWPI + Rust**

Offset    | Size (bytes)  |  Field Name        |    Content    |
----------|---------------|--------------------|---------------|
0x00      |       4       | Address Offset     | Offset of the address field to fix from the start of the HBF

*Total size: 4 bytes* (**must be multiple of 4 to avoid alignment problems**)

## HBF Compiled Component Binary
This section contains `.data`, (`.bss`,) `.text`, `.rodata`  sections.
In particular, the last two sections `.text` and `.rodata` are consecutive and cannot be separated.

```
end of header -> +---------------------+
                 |                     |
                 |        .text        |
                 |                     |
                 +---------------------+
                 |       .rodata       |
                 +---------------------+
                 |        .data        |
   end of HBF -> +---------------------+
```

### MPU Note
As it is quite challenging to include only `.text` and `.rodata` in the MPU region, due to the base address alignment problem (and also the minor problem with the size), the entire region will be given in read-only to the component via MPU.

### Alignment Problems
Instructions in `.text` sections must be word-aligned (4 bytes), as requested in the ARM specification. Also addresses in `.rodata` works better with a 16 bytes alignment (to exploit cache better).

We must ensure that the start of `.text` is word-aligned. The internal alignment is ensured by the linker. Also the alignment of `.rodata` is then assured by the compiler (they are placed in the same segment in the ELF).

The fastest way to ensure this is to have a header size multiple of 4, or to add padding between the header and this section. **The first option is implemented**.
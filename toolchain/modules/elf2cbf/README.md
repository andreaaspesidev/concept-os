# ELF2CBF
The tool `elf2cbf` is used to generate the binary image of a component, that can be loaded by the kernel.

It follows the specification in `docs/ConceptOSBinaryFormat.md`.
Takes inspiration from [`elf2tab`](https://github.com/tock/elf2tab) of TockOS.

## Usage
```
elf2cbf 0.0.1
A tool to generate CBF binaries for ConceptOS Components starting from ELF and some configuration files

USAGE:
    elf2cbf --component-config-file <COMPONENT_CONFIG_FILE> --component-elf-file <COMPONENT_ELF_FILE> --component-relocations-file <COMPONENT_RELOCATIONS_FILE> --cbf-output-path <CBF_OUTPUT_PATH>

OPTIONS:
    -c, --component-config-file <COMPONENT_CONFIG_FILE>              
    -e, --component-elf-file <COMPONENT_ELF_FILE>                    
    -h, --help                                                       Print help information
    -o, --cbf-output-path <CBF_OUTPUT_PATH>                          
    -r, --component-relocations-file <COMPONENT_RELOCATIONS_FILE>    
    -V, --version                                                    Print version information
```
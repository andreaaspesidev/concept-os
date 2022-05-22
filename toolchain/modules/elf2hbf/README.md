# ELF2HBF
The tool `elf2hbf` is used to generate the binary image of a component, that can be loaded by the Hubris `AppLoader` component.

It follows the specification in `docs/HubrisBinaryFormat.md`.
Takes inspiration from [`elf2tab`](https://github.com/tock/elf2tab) of TockOS.

## Usage
```
elf2hbf 0.0.1
A tool to generate HBF binaries for Hubris Components starting from ELF and some configuration files

USAGE:
    elf2hbf --component-config-file <COMPONENT_CONFIG_FILE> --component-elf-file <COMPONENT_ELF_FILE> --component-relocations-file <COMPONENT_RELOCATIONS_FILE> --hbf-output-path <HBF_OUTPUT_PATH>

OPTIONS:
    -c, --component-config-file <COMPONENT_CONFIG_FILE>              
    -e, --component-elf-file <COMPONENT_ELF_FILE>                    
    -h, --help                                                       Print help information
    -o, --hbf-output-path <HBF_OUTPUT_PATH>                          
    -r, --component-relocations-file <COMPONENT_RELOCATIONS_FILE>    
    -V, --version                                                    Print version information
```
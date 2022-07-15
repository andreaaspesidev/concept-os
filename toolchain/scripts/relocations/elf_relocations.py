import argparse
import sys
import lief
from obj_relocations import scan_relocations
from serde import serde
from serde.toml import to_toml
from typing import List

@serde
class Relocations:
    rodata: List[int]
    data: List[int]

@serde
class TomlConfig:
    relocations: 'Relocations'



# This script extract the absolute addresses fixes for  section .rodata and .data
# given an ARM ELF32, the linker map, and the directory of the build used for the ELF.

def save_toml(file_name: str, content: Relocations):
    with open(file_name, 'w') as file:
        file.write(to_toml(content))

def get_section_relocations(section, map_file: str, verbose: bool):
    tot_size, relocations = scan_relocations(map_file, section.name[1:], verbose)
    if tot_size != section.size:
        print("\ERROR: linked section size different from the supposed one.\nMost likely relocation will fail")
        exit(-1)
    return relocations

def elf_scan_relocations(src_elf: str, dest_file: str, map_file: str, verbose: bool):
    # Open the file
    binary = lief.parse(src_elf)
    # Find original linked app start
    # app_text_section = binary.get_section(".text")
    # app_flash_original = app_text_section.virtual_address
    # Get sections
    app_rodata_section = binary.get_section(".rodata")
    app_data_section = binary.get_section(".data")
    # Prepare for result
    result = TomlConfig
    result.relocations = Relocations
    result.relocations.rodata = get_section_relocations(section=app_rodata_section, map_file=map_file, verbose=verbose)
    result.relocations.data = get_section_relocations(section=app_data_section, map_file=map_file, verbose=verbose)
    save_toml(dest_file, result)

#elf_reloc('image.elf', 'out.json', 'out.map')

if __name__ == "__main__":
    # parse the command-line arguments
    argparser = argparse.ArgumentParser(
            usage='usage: %(prog)s <src-elf-file> <map-file> <dst-info-file> <verbose>',
            description="Generate section-relative relocations for .data and .rodata sections",
            add_help=False,
            prog='elf_relocations.py')   
    argparser.add_argument('src_file',
            nargs='?', default=None,
            help='Source ELF file')
    argparser.add_argument('map_file',
            nargs='?', default=None,
            help='Linker map file')
    argparser.add_argument('dst_file',
            nargs='?', default=None,
            help='Destination info file')
    argparser.add_argument('verbose',
            nargs='?', default=0,
            help='Verbose mode')

    args = argparser.parse_args()
    if not args.src_file or not args.dst_file or not args.map_file:
        argparser.print_help()
        sys.exit(0)
    
    elf_scan_relocations(args.src_file, args.dst_file, args.map_file, args.verbose)

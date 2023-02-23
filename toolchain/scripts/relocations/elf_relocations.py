import sys
from typing import List, Optional
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection
from serde import serde
from serde.toml import to_toml
import lief
import argparse


@serde
class Relocations:
    text: List[List[int]]  # type, offset_pair, offset_base
    rodata: List[List[int]]
    data: List[List[int]]


@serde
class TomlConfig:
    relocations: 'Relocations'


def save_toml(file_name: str, content: TomlConfig):
    with open(file_name, 'w') as file:
        file.write(to_toml(content))


def read_relocations(elf_file, section_name):
    # Read the obj file
    relocs = []
    with open(elf_file, 'rb') as file:
        # Parse the file
        elf_file = ELFFile(file)
        # Search for relocations
        for section in elf_file.iter_sections():
            # Skip sections that are not relocation
            if not isinstance(section, RelocationSection):
                continue
            if section.name == '.rel' + section_name:
                for rel in section.iter_relocations():
                    if rel['r_info_type'] == 2:  # R_ARM_ABS32
                        relocs.append({
                            'r_type': 0,  # 'R_ARM_ABS32',
                            'r_sym': rel['r_info_sym'],
                            'r_offset': rel['r_offset'],
                            'paired': False
                        })
                    elif rel['r_info_type'] == 47:  # R_ARM_THM_MOVW_ABS_NC
                        relocs.append({
                            'r_type': 1,  # 'R_ARM_THM_MOVW_ABS_NC',
                            'r_sym': rel['r_info_sym'],
                            'r_offset': rel['r_offset'],
                            'paired': False
                        })
                    elif rel['r_info_type'] == 48:  # R_ARM_THM_MOVT_ABS
                        relocs.append({
                            'r_type': 2,  # 'R_ARM_THM_MOVT_ABS',
                            'r_sym': rel['r_info_sym'],
                            'r_offset': rel['r_offset'],
                            'paired': False
                        })
    return relocs


def search_paired_relocation(relocs, sym, skip_index):
    # Iterate over all relocs. We could optimize this,
    # but relocations are a few.
    for i in range(0, len(relocs)):
        if i != skip_index and relocs[i]['paired'] == False and relocs[i]['r_sym'] == sym:
            # Mark as paired
            relocs[i]['paired'] = True
            # Return the index
            return i
    # No paired relocation found
    return -1


def process_relocations(relocs, section_base_address):
    for i in range(0, len(relocs)):
        current_reloc = relocs[i]
        # Prepare a null offset
        offset = 0
        if current_reloc['r_type'] == 1 or current_reloc['r_type'] == 2:

            # Search for the linked one
            paired_index = search_paired_relocation(
                relocs, current_reloc['r_sym'], i)
            if paired_index >= 0:
                offset = paired_index - i
        # Add to the current object
        current_reloc['paired_offset'] = offset
        # Store the offsets removing the absolute address
        current_reloc['r_offset'] = current_reloc['r_offset'] - \
            section_base_address


def get_section_relocations(elf_file: str, section: str, section_base_address: int):
    relocs = read_relocations(elf_file, section)
    process_relocations(relocs, section_base_address)
    return [[rel['r_type'], rel['paired_offset'], rel['r_offset']] for rel in relocs]


def extract_relocation_points(elf_file: str, output_file: Optional[str] = None):
    # Open the file
    binary = lief.parse(elf_file)
    # Get sections
    app_text_section = binary.get_section(".text")
    app_rodata_section = binary.get_section(".rodata")
    app_data_section = binary.get_section(".data")
    # Prepare for result
    result = TomlConfig
    result.relocations = Relocations
    # -> relocation points
    result.relocations.text = get_section_relocations(
        elf_file=elf_file, section=".text", section_base_address=app_text_section.virtual_address)
    result.relocations.rodata = get_section_relocations(
        elf_file=elf_file, section=".rodata", section_base_address=app_rodata_section.virtual_address)
    result.relocations.data = get_section_relocations(
        elf_file=elf_file, section=".data", section_base_address=app_data_section.virtual_address)
    if output_file is not None:
        save_toml(output_file, result)
    return result


# extract_relocation_points('image.elf', 'out.toml')

if __name__ == "__main__":
    # parse the command-line arguments
    argparser = argparse.ArgumentParser(
        usage='usage: %(prog)s <src-elf> <dst-file>',
        description="Generate section-relative relocations for .text,.rodata and .data sections",
        add_help=False,
        prog='elf_relocations.py')
    argparser.add_argument('src_elf',
                           nargs='?', default=None,
                           help='Source ELF file')
    argparser.add_argument('dst_file',
                           nargs='?', default=None,
                           help='Destination info file')

    args = argparser.parse_args()
    if not args.src_elf or not args.dst_file:
        argparser.print_help()
        sys.exit(0)

    extract_relocation_points(args.src_elf, args.dst_file)

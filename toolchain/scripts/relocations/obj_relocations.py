import math
from elftools.elf.elffile import ELFFile
from elftools.elf.relocation import RelocationSection
import json
import re
from pathlib import Path
import argparse
import sys
from typing import Optional

def print_mask(tot_size, relocs):
    out = "\t"
    i = 0
    while(i < tot_size):
        if i in relocs:
            out += "11111111" # each byte is two symbols
            i += 4
        else:
            out += "00"
            i += 1
        if i % 16 == 0:
            out += "\n\t"
        elif i % 4 == 0:
            out += " "     
    print("\t" + out.strip('\n\t'))

def save_json(file_name: str, content):
    with open(file_name, 'w') as file:
        file.write(json.dumps(content, indent=1))

SECTION_START_PATTERN = r'[ \t]*([a-f0-9]+)\s+([a-f0-9]+)\s+([a-f0-9]+)\s+([0-9]+)\s+\.([A-Za-z0-9\.]+)\n'
OBJ_PATTERN_1 = r'[ \t]*([a-f0-9]+)\s+([a-f0-9]+)\s+([a-f0-9]+)\s+([0-9]+)\s+([A-Za-z0-9\/\-\_\.]+\.o):\(([A-Za-z0-9\/\_\.]+)\)'
OBJ_PATTERN_2 = r'[ \t]*([a-f0-9]+)\s+([a-f0-9]+)\s+([a-f0-9]+)\s+([0-9]+)\s+([A-Za-z0-9\/\-\_\.]+\.rlib).+:\(([A-Za-z0-9\/\_\.]+)\)'
OBJ_PATTERN_3 = r'[ \t]*([a-f0-9]+)\s+([a-f0-9]+)\s+([a-f0-9]+)\s+([0-9]+)\s+<internal>:\(([A-Za-z0-9\.\-\_]+)\)'
ALIGN_PATTERN = r'[ \t]*([a-f0-9]+)\s+([a-f0-9]+)\s+([a-f0-9]+)\s+([0-9]+)\s+.\s+=\s+ALIGN'
def read_map(map_file_path: str, verbose: bool):
    # Open the txt file
    sections = {}
    object_files = set()
    c_section_name = None
    c_section_vma = None
    c_section_lma = None
    c_section_size = None
    c_section_align = None
    c_section_objects = []
    with open(map_file_path, 'r') as file:
        for line in file.readlines():
            section_header = re.search(SECTION_START_PATTERN, line)
            if section_header is not None:
                # Decode section
                start_vma = section_header.group(1)
                start_lma = section_header.group(2)
                size = int(section_header.group(3),16)
                align = int(section_header.group(4),10)
                section_name = section_header.group(5)
                # Check what to do
                if section_name != c_section_name and c_section_name is not None:
                    # Push last section data
                    sections[c_section_name] = {
                        'vma': c_section_vma,
                        'lma': c_section_lma,
                        'size': c_section_size,
                        'align': c_section_align,
                        'objects': c_section_objects
                    }
                # Save new section
                c_section_name = section_name
                c_section_vma = start_vma
                c_section_lma = start_lma
                c_section_size = size
                c_section_align = align
                c_section_objects = []
                continue
            if c_section_name is not None:
                obj_line_1 = re.search(OBJ_PATTERN_1, line)
                if obj_line_1 is not None:
                    # Decode line
                    obj_vma = obj_line_1.group(1)
                    obj_lma = obj_line_1.group(2)
                    obj_size = int(obj_line_1.group(3),16)
                    obj_align = int(obj_line_1.group(4),10)
                    # Convert object path
                    obj_file = obj_line_1.group(5)
                    file_name = Path(obj_file).stem
                    crate_name = file_name[0:file_name.index('-')]
                    obj_file_name = file_name[0:file_name.index(crate_name,file_name.index('-')+1)-1]
                    obj_file = str(Path(obj_file).parent) + "/" + obj_file_name + ".o"
                    
                    obj_element = obj_line_1.group(6)
                obj_line_2 = re.search(OBJ_PATTERN_2, line)
                if obj_line_2 is not None:
                    # Decode line
                    obj_vma = obj_line_2.group(1)
                    obj_lma = obj_line_2.group(2)
                    obj_size = int(obj_line_2.group(3),16)
                    obj_align = int(obj_line_2.group(4),10)
                    obj_file = obj_line_2.group(5)
                    # As the file is exploded (zip with the obj file)
                    parent_dir = str(Path(obj_file).parent)
                    obj_file = parent_dir + "/" + str(Path(obj_file).stem + '.o')[3:]
                    obj_element = obj_line_2.group(6)
                obj_line_3 = re.search(OBJ_PATTERN_3, line)
                if obj_line_3 is not None:
                    # Decode line
                    obj_vma = obj_line_3.group(1)
                    obj_lma = obj_line_3.group(2)
                    obj_size = int(obj_line_3.group(3),16)
                    obj_align = int(obj_line_3.group(4),10)
                    sec_name = obj_line_3.group(5)
                    obj_file = None     # Compiler internal repositioning, does not depend on rustc
                    obj_element = None
                    if verbose:
                        print(f"\tFound compiler generated data inside section {sec_name}. Cannot check for relocations for this data")
                align = re.search(ALIGN_PATTERN, line)
                if align is not None: 
                    # Get only size and align. This must be appended to the section
                    obj_vma = align.group(1)
                    obj_lma = align.group(2)
                    obj_size = int(align.group(3),16)
                    obj_align = int(align.group(4),10)
                    obj_file = None     # Align
                    obj_element = None
                    
                # Save object
                if obj_line_1 or obj_line_2 or obj_line_3 or align:
                    if obj_file not in object_files:
                        object_files.add(obj_file)
                    c_section_objects.append({
                        'vma': obj_vma,
                        'lma': obj_lma,
                        'size': obj_size,
                        'align': obj_align,
                        'obj_file': obj_file,
                        'name': obj_element
                    })
        # Push last section data
        sections[c_section_name] = {
            'vma': c_section_vma,
            'lma': c_section_lma,
            'size': c_section_size,
            'align': c_section_align,
            'objects': c_section_objects
        }
    save_json(map_file_path + ".json", sections)
    return object_files, sections

def read_relocations(object_file, section_name):
    # Read the obj file
    relocs = []
    with open(object_file, 'rb') as file:
        # Parse the file
        elf_file = ELFFile(file)
        # Search for relocations
        for section in elf_file.iter_sections():
            # Skip sections that are not relocation
            if not isinstance(section, RelocationSection):
                continue
            if section.name == '.rel' + section_name:
                 for rel in section.iter_relocations():
                    if rel['r_info_type'] == 2: # R_ARM_ABS32
                        relocs.append({
                            'offset': rel['r_offset']
                        })
    return relocs


def index_align(index: int, base: int):
    if base == 1 or index == 0:
        return index
    return int(math.ceil(index / base)*base)

def search_for_relocations(section_data):
    obj_elements = section_data['objects']
    index = 0
    result = []
    for obj_elem in obj_elements:
        index = index_align(index, obj_elem['align'])
        if obj_elem['obj_file'] is not None:
            relocs = read_relocations(obj_elem['obj_file'], obj_elem['name'])
            if len(relocs) > 0:
                for rel in relocs:
                    result.append(index + rel['offset'])
        index += obj_elem['size']
    return result, index

def scan_relocations(map_file:str, section: str, verbose: bool, out_file: Optional[str] = None):
    # Scan linker map
    _, sections = read_map(map_file, verbose)
    # Search for absolute relocations in each object
    if section not in sections:
        sys.exit(1)
    
    section_data = sections[section]
    relocations, tot_size = search_for_relocations(section_data)
    #tot_size = size_align(tot_size, section_data['align'])
    if verbose:
        print(f"\n\t---- .{section} -----")
        print_mask(tot_size, relocations)
        print(f"\t---------------------\n")
    if out_file:
        save_json(out_file, {'size': tot_size, 'relocs': relocations})
    return tot_size, relocations

#scan_relocations('out.map', 'rodata', 'out.json')

if __name__ == "__main__":
    # parse the command-line arguments
    argparser = argparse.ArgumentParser(
            usage='usage: %(prog)s <linker-map-file> <section> <out-json-file> <verbose>',
            description="Returns absolute relocations for the specified section",
            add_help=False,
            prog='obj_relocations.py')   
    argparser.add_argument('map_file',
            nargs='?', default=None,
            help='Linker MAP file')
    argparser.add_argument('section',
            nargs='?', default=None,
            help='Section to be searched (data/rodata/...)')
    argparser.add_argument('out_file',
            nargs='?', default=None,
            help='Output file (json array)')
    argparser.add_argument('verbose',
            nargs='?', default=False,
            help='Verbose')

    args = argparser.parse_args()
    if not args.map_file or not args.section:
        argparser.print_help()
        sys.exit(0)
    
    scan_relocations(args.map_file, args.section, args.verbose, args.out_file)
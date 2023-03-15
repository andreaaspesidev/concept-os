// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{path::PathBuf};

pub struct ElfSection {
    pub start_addr_relative: u32,
    pub data: Vec<u8>
}

pub struct HubrisELF {
    pub total_size: u32,
    pub sections: Vec<ElfSection>
}

pub fn parse_elf(path: &PathBuf) -> HubrisELF {
    let mut hs_vec = Vec::<ElfSection>::new();
    let mut total_size: u32 = 0;
    // Read file bytes
    let file_bytes = std::fs::read(path).expect("Cannot read ELF file");
    // Parse as elf
    let elf = goblin::elf::Elf::parse(&file_bytes).expect("Invalid ELF");
    // For each section header, populate structure
    for sh in &elf.section_headers {
        if sh.sh_type == 1 { // PROGBITS
            let data = Vec::from(&file_bytes[sh.file_range().unwrap()]);
            total_size += data.len() as u32;
            hs_vec.push(ElfSection {
                start_addr_relative: sh.sh_addr as u32 - 0x08000000,
                data: data
            })
        }
    }
    // Return the object
    return HubrisELF { total_size: total_size, sections: hs_vec };
}
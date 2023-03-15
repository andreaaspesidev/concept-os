// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{error::Error, path::Path, fs};

use elf_rs::{self, Elf32, ElfFile, ElfEndian, ElfMachine};

use crate::structures::HbfError;

pub struct ElfSection {
    pub address: u32,
    pub size: u32,
    pub content: Vec<u8>
}

pub struct ParseResult {
    pub rel_entrypoint: u32,
    pub text_section: ElfSection,
    pub rodata_section: Option<ElfSection>,
    pub data_section: Option<ElfSection>,
    pub bss_size: u32
}

pub fn parse(elf_path: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Read elf data
    let bytes = elf_bytes(&elf_path)?;
    // Parse elf
    let elf = elf_parse(&bytes)?;
    // Check elf
    elf_check(&elf)?;
    // Construct result
    return Ok(ParseResult{
        rel_entrypoint: offset_entrypoint(&elf)?,
        text_section: elf_read_section(&elf, b".text")?,
        rodata_section: match elf_section_exits(&elf, b".rodata") {
            true => Some(elf_read_section(&elf, b".rodata").unwrap()),
            false => None
        },
        data_section: match elf_section_exits(&elf, b".data") {
            true => Some(elf_read_section(&elf, b".data").unwrap()),
            false => None
        },
        bss_size: match elf_section_exits(&elf, b".bss") {
            true => elf_read_bss(&elf).unwrap(),
            false => 0
        }
    });
}


#[allow(dead_code)]
pub fn entrypoint_bytes(elf_path: &str, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    // Read elf data
    let bytes = elf_bytes(&elf_path)?;
    // Parse elf
    let elf = elf_parse(&bytes)?;
    // Check elf
    elf_check(&elf)?;
    // Read bytes
    let offset = offset_entrypoint(&elf)? as usize;
    let text = elf_read_section(&elf, b".text")?;
    Ok(Vec::from(&text.content[offset..offset+length]))
}
#[allow(dead_code)]
pub fn bss_size(elf_path: &str) -> Result<u32, Box<dyn Error>> {
    // Read elf data
    let bytes = elf_bytes(&elf_path)?;
    // Parse elf
    let elf = elf_parse(&bytes)?;
    // Check elf
    elf_check(&elf)?;
    Ok(elf_read_bss(&elf)?)
}

fn elf_bytes(elf_path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let elf_path = Path::new(elf_path);
    let elf_buffer = fs::read(elf_path)?;
    return Ok(elf_buffer);
}
fn elf_parse(elf_buffer: &Vec<u8>) -> Result<Elf32, Box<dyn Error>> {
    let elf_result = elf_rs::Elf32::from_bytes(elf_buffer.as_slice());
    if elf_result.is_err() {
        return Err(Box::new(HbfError{msg: String::from("Cannot read ELF")}));
    }
    let elf = elf_result.unwrap();
    return Ok(elf);
}
fn elf_check(component_elf: &Elf32) -> Result<(), HbfError> {
    if component_elf.elf_header().endianness() != ElfEndian::LittleEndian {
        return Err(HbfError{msg: String::from("Wrong endianness: little endian expected")});
    }
    if component_elf.elf_header().machine() != ElfMachine::ARM {
        return Err(HbfError{msg: String::from("Wrong machine: ARM supported only")});
    }
    Ok(())
}
fn elf_section_exits(component_elf: &Elf32, section_name: &[u8]) -> bool {
    let section_result = component_elf.lookup_section(section_name);
    if section_result.is_none() {
        return false;
    }
    let section = section_result.unwrap();
    if section.size() == 0 {    // Discard empty sections
        return false;
    }
    return true;
}
fn elf_read_section(component_elf: &Elf32, section_name: &[u8]) -> Result<ElfSection, HbfError> {
    let section_result = component_elf.lookup_section(section_name);
    if section_result.is_none() {
        return Err(HbfError{msg: format!("Cannot find {:?} section in ELF", section_name)});
    }
    let section = section_result.unwrap();
    return Ok(ElfSection{
        address: section.addr() as u32,
        size: section.size() as u32,
        content: Vec::from(section.content())
    });
}
fn elf_read_bss(component_elf: &Elf32) -> Result<u32, HbfError> {
    let section_result = component_elf.lookup_section(b".bss");
    if section_result.is_none() {
        return Err(HbfError{msg: format!("Cannot find .bss section in ELF")});
    }
    let section = section_result.unwrap();
    return Ok(section.size() as u32);
}   
fn offset_entrypoint(component_elf: &Elf32) -> Result<u32, HbfError> {
    // This is the memory address of the entry point from where the process starts executing. 
    // This field is either 32 or 64 bits long, depending on the format defined earlier (byte 0x04). 
    // If the file doesn't have an associated entry point, then this holds zero.
    let entry_address_elf = component_elf.elf_header().entry_point() as u32;
    let text_section = elf_read_section(component_elf, b".text");
    let text_start_addr = text_section?.address;
    let offset_text = entry_address_elf - text_start_addr -1; // The OR 1 is to indicate Thumb mode.
    return Ok(offset_text);
}   

/*
    Tests
*/
#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use crate::parse_elf::offset_entrypoint;

    use super::{elf_bytes, elf_check, elf_parse, elf_read_section, parse};

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    fn read_test_file_binary(name: &str) -> Vec<u8> {
        let file_path = get_test_file_path(name);
        return std::fs::read(file_path).unwrap();
    }

    #[test]
    fn open_elf() {
        let elf_file = get_test_file_path("component1/output/image.elf");
        let bytes = elf_bytes(&elf_file).unwrap();
        let elf = elf_parse(&bytes).unwrap();
        assert!(elf_check(&elf).is_ok());
    }
    #[test]
    fn entry_offset() {
        let elf_file = get_test_file_path("component1/output/image.elf");
        let bytes = elf_bytes(&elf_file).unwrap();
        let elf = elf_parse(&bytes).unwrap();
        let entry_offset = offset_entrypoint(&elf).unwrap();
        assert_eq!(entry_offset, 0);
    }
    #[test]
    fn read_text() {
        let elf_file = get_test_file_path("component1/output/image.elf");
        let bytes = elf_bytes(&elf_file).unwrap();
        let elf = elf_parse(&bytes).unwrap();
        let text_section = elf_read_section(&elf, b".text").unwrap();
        assert_eq!(text_section.address, 0x08000000);
        assert_eq!(text_section.size, 0x00014);
        assert_eq!(text_section.content.as_slice(), read_test_file_binary("component1/output/image.text").as_slice());
    }

    #[test]
    fn read_example1() {
        let elf_file = get_test_file_path("component1/output/image.elf");
        let result = parse(&elf_file).unwrap();
        // Entry point
        assert_eq!(result.rel_entrypoint, 0);
        // .text section
        assert_eq!(result.text_section.address, 0x08000000);
        assert_eq!(result.text_section.size, 0x00014);
        assert_eq!(result.text_section.content.as_slice(), read_test_file_binary("component1/output/image.text").as_slice());
        // .rodata section
        assert!(result.rodata_section.is_none());
        // .data section
        assert!(result.data_section.is_none());
        // .bss section
        assert_eq!(result.bss_size, 0);
    }
    #[test]
    fn read_example2() {
        let elf_file = get_test_file_path("component2/output/image.elf");
        let result = parse(&elf_file).unwrap();
        // Entry point
        assert_eq!(result.rel_entrypoint, 0);
        // .text section
        assert_eq!(result.text_section.address, 0x08000000);
        assert_eq!(result.text_section.size, 0x0109c);
        assert_eq!(result.text_section.content.as_slice(), read_test_file_binary("component2/output/image.text").as_slice());
        // .rodata section
        let rodata_section = result.rodata_section.unwrap();
        assert_eq!(rodata_section.size, 0x00168);
        assert_eq!(rodata_section.content.as_slice(), read_test_file_binary("component2/output/image.rodata").as_slice());
        // .data section
        assert!(result.data_section.is_none());
        // .bss section
        assert_eq!(result.bss_size, 8);
    }
    #[test]
    fn read_example3() {
        let elf_file = get_test_file_path("component3/output/image.elf");
        let result = parse(&elf_file).unwrap();
        // Entry point
        assert_eq!(result.rel_entrypoint, 0);
        // .text section
        assert_eq!(result.text_section.address, 0x08000000);
        assert_eq!(result.text_section.size, 0x00144);
        assert_eq!(result.text_section.content.as_slice(), read_test_file_binary("component3/output/image.text").as_slice());
        // .rodata section
        let rodata_section = result.rodata_section.unwrap();
        assert_eq!(rodata_section.size, 0x00060);
        assert_eq!(rodata_section.content.as_slice(), read_test_file_binary("component3/output/image.rodata").as_slice());
        // .data section
        let data_section = result.data_section.unwrap();
        assert_eq!(data_section.size, 0x00028);
        assert_eq!(data_section.content.as_slice(), read_test_file_binary("component3/output/image.data").as_slice());
        // .bss section
        assert_eq!(result.bss_size, 0);
    }
    #[test]
    fn read_example4() {
        let elf_file = get_test_file_path("component4/output/image.elf");
        let result = parse(&elf_file).unwrap();
        // Entry point
        assert_eq!(result.rel_entrypoint, 0);
        // .text section
        assert_eq!(result.text_section.address, 0x08000000);
        assert_eq!(result.text_section.size, 0x000d8);
        assert_eq!(result.text_section.content.as_slice(), read_test_file_binary("component4/output/image.text").as_slice());
        // .rodata section
        let rodata_section = result.rodata_section.unwrap();
        assert_eq!(rodata_section.size, 0x0001c);
        assert_eq!(rodata_section.content.as_slice(), read_test_file_binary("component4/output/image.rodata").as_slice());
        // .data section
        assert!(result.data_section.is_none());
        // .bss section
        assert_eq!(result.bss_size, 0x0000c);
    }
}


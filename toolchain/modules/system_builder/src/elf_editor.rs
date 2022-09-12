use flash_allocator::flash::{FlashAllocatorImpl, FlashMethods};
use goblin::{container::Container, elf64::program_header::PT_LOAD};
use hbf_rs::HbfFile;
use ram_allocator::{RAMAllocator, RAMAllocatorImpl};
use std::process::Command;
use std::{collections::BTreeMap, path::PathBuf};

pub struct ElfEditor<'a> {
    dest_path: &'a PathBuf,
    output_sections: BTreeMap<u32, Vec<u8>>,
    current_flash_size: usize,
    board_name: &'a String,
    kentry: u32,
    flash_buffer: BufferFlash,
}

impl<'a> ElfEditor<'a> {
    pub fn new(dest_path: &'a PathBuf, board_name: &'a String) -> Self {
        Self {
            dest_path: dest_path,
            output_sections: BTreeMap::new(),
            current_flash_size: 0,
            kentry: 0,
            board_name: board_name,
            flash_buffer: BufferFlash {
                base_addr: 0,
                buffer: vec![0xFF; 1048576],
            },
        }
    }
    pub fn add_kernel(&mut self, elf_path: &PathBuf) {
        // Read bytes
        let elf_bytes = std::fs::read(elf_path).expect("Cannot read kernel ELF");
        // Parse elf
        let elf = goblin::elf::Elf::parse(&elf_bytes).expect("Cannot parse kernel ELF");
        // Check elf
        if elf.header.container().unwrap() != Container::Little {
            panic!("Big endian elf?");
        }
        if elf.header.e_machine != goblin::elf::header::EM_ARM {
            panic!("The ELF must be ARM");
        }
        for phdr in &elf.program_headers {
            if phdr.p_type != PT_LOAD {
                continue; // Ignore
            }
            let offset = phdr.p_offset as usize;
            let size = phdr.p_filesz as usize;
            let addr = phdr.p_paddr as u32;
            // Update stats
            self.current_flash_size += size;
            // Add to our structure
            self.output_sections
                .insert(addr, elf_bytes[offset..offset + size].to_vec());
        }
        self.kentry = elf.header.e_entry as u32;
    }

    pub fn add_component(&mut self, hbf_path: &PathBuf) {
        // Read bytes
        let mut hbf_bytes =
            std::fs::read(hbf_path).expect(&format!("Cannot read HBF at: {}", hbf_path.display()));
        // Parse HBF
        let hbf = hbf_rs::parse_hbf(&hbf_bytes).expect("Cannot parse HBF");
        // First, check it's a valid HBF
        if !hbf.validate() {
            panic!("HBF not valid at: {}", hbf_path.display());
        }
        // Get the allocation for this HBF
        let needed_flash = hbf.header_base().total_size();
        let needed_ram = hbf.header_main().component_min_ram();
        // Create a big enough buffer
        let alloc_result = perform_allocation(
            self.board_name.clone(),
            &mut self.flash_buffer,
            needed_flash,
            needed_ram,
        );
        let block_base_addr: u32 = alloc_result.flash_address;
        // Generate bytes
        let mut component_bytes: Vec<u8> = Vec::new();
        // Add block header bytes
        component_bytes.extend_from_slice(&alloc_result.data);
        // Edit hbf and append
        let relocs = extract_hbf_relocations(&hbf);
        let dest_base_address = block_base_addr + 8 + hbf.read_only_section().offset() + 12;
        drop(hbf);
        relocate_hbf(&mut hbf_bytes, dest_base_address, &relocs);
        fix_checksum_hbf(&mut hbf_bytes);
        component_bytes.extend_from_slice(&hbf_bytes);
        // Add section
        self.output_sections
            .insert(block_base_addr, component_bytes);
    }

    fn write_srec(&mut self) -> String {
        // Generate SREC
        let mut srec_out = vec![srec::Record::S0("hubris".to_string())];
        for (&base, sec) in &self.output_sections {
            // SREC record size limit is 255 (0xFF). 32-bit addressed records
            // additionally contain a four-byte address and one-byte checksum, for a
            // payload limit of 255 - 5.
            let mut addr = base;
            for chunk in sec.chunks(255 - 5) {
                srec_out.push(srec::Record::S3(srec::Data {
                    address: srec::Address32(addr),
                    data: chunk.to_vec(),
                }));
                addr += chunk.len() as u32;
            }
        }
        let out_sec_count = srec_out.len() - 1; // header
        if out_sec_count < 0x1_00_00 {
            srec_out.push(srec::Record::S5(srec::Count16(out_sec_count as u16)));
        } else if out_sec_count < 0x1_00_00_00 {
            srec_out.push(srec::Record::S6(srec::Count24(out_sec_count as u32)));
        } else {
            panic!("SREC limit of 2^24 output sections exceeded");
        }

        srec_out.push(srec::Record::S7(srec::Address32(self.kentry)));

        let srec_image = srec::writer::generate_srec_file(&srec_out);
        let mut srec_file_path = String::from(self.dest_path.to_str().unwrap());
        srec_file_path += ".srec";
        std::fs::write(&srec_file_path, srec_image).expect("Cannot write SREC");
        srec_file_path
    }

    pub fn finish(mut self) {
        // Generate SREC
        let srec_path = PathBuf::from(self.write_srec());
        let mut elf_file_path = String::from(self.dest_path.to_str().unwrap());
        elf_file_path += ".elf";
        let elf_path_buff = PathBuf::from(elf_file_path);
        // Convert to ELF
        objcopy_translate_format("srec", &srec_path, "elf32-littlearm", &elf_path_buff);
        // Convert to BIN
        let mut bin_path = String::from(self.dest_path.to_str().unwrap());
        bin_path += ".bin";
        let bin_path_buff = PathBuf::from(bin_path);
        objcopy_translate_to_binary("srec", &srec_path, &bin_path_buff);
        // Convert to iHEX
        let mut ihex_path = String::from(self.dest_path.to_str().unwrap());
        ihex_path += ".ihex";
        let ihex_path_buff = PathBuf::from(ihex_path);
        objcopy_translate_format("srec", &srec_path, "ihex", &ihex_path_buff);
    }
}

fn objcopy_translate_format(in_format: &str, src: &PathBuf, out_format: &str, dest: &PathBuf) {
    let mut cmd = Command::new("arm-none-eabi-objcopy");
    cmd.arg("-I")
        .arg(in_format)
        .arg("-O")
        .arg(out_format)
        .arg(src)
        .arg(dest);
    let status = cmd.status().expect("failed to objcopy");
    if !status.success() {
        panic!("objcopy failed, see output for details");
    }
}

fn objcopy_translate_to_binary(in_format: &str, src: &PathBuf, dest: &PathBuf) {
    let mut cmd = Command::new("arm-none-eabi-objcopy");
    cmd.arg("-I")
        .arg(in_format)
        .arg("-O")
        .arg("binary")
        .arg("--gap-fill")
        .arg("0xFF")
        .arg(src)
        .arg(dest);
    let status = cmd.status().expect("failed to objcopy");
    if !status.success() {
        panic!("objcopy failed, see output for details");
    }
}

fn extract_hbf_relocations(hbf: &dyn HbfFile) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::new();
    for reloc in hbf.relocation_iter() {
        let pointed_addr = reloc.pointed_addr();
        let offset = reloc.offset();
        result.push((pointed_addr, offset));
    }
    result
}

pub const ORIGINAL_FLASH_ADDR: u32 = 0x0800_0000;

fn relocate_hbf(hbf_bytes: &mut [u8], dest_base_address: u32, relocs: &Vec<(u32, u32)>) {
    // Read hbf relocations
    for (pointed_addr, offset) in relocs {
        // Calculate relocation
        let new_addr = *pointed_addr - ORIGINAL_FLASH_ADDR + dest_base_address;
        let new_addr_bytes = new_addr.to_le_bytes();
        // Apply bytes
        for i in 0..4 {
            hbf_bytes[(*offset as usize) + i] = new_addr_bytes[i];
        }
    }
}

fn fix_checksum_hbf(hbf_bytes: &mut [u8]) {
    let mut index: usize = 0;
    let mut checksum: u32 = 0;
    loop {
        let mut word: u32 = 0;
        let mut available: usize = 4;
        // Check if enough bytes are available
        if hbf_bytes.len() <= index + 4 {
            available = hbf_bytes.len() - index;
            if available == 0 {
                break;
            }
        }
        if index == hbf_rs::HBF_CHECKSUM_OFFSET {
            // Consider the checksum field as zeros
            word = 0;
        } else {
            // Convert the 4 bytes into a word
            let mut i = 0;
            for c in &hbf_bytes[index..index + available] {
                word |= u32::from(*c) << (8 * i);
                i += 1;
            }
        }
        checksum ^= word;
        index += available;
    }
    // Write new checksum
    let checksum_bytes = checksum.to_le_bytes();
    for i in 0..4 {
        hbf_bytes[hbf_rs::HBF_CHECKSUM_OFFSET + i] = checksum_bytes[i];
    }
}

#[allow(dead_code)]
struct AllocatedComponent {
    pub flash_address: u32,
    pub flash_size: u32,
    pub sram_address: u32,
    pub sram_size: u32,
    pub data: Vec<u8>,
}

fn perform_allocation(
    board_name: String,
    flash_buffer: &mut BufferFlash,
    needed_flash: u32,
    needed_ram: u32,
) -> AllocatedComponent {
    match board_name.as_str() {
        "stm32f303re" => {
            const FLASH_START_ADDR: u32 = stm32f303re::FLASH_ALLOCATOR_START_ADDR;
            const FLASH_END_ADDR: u32 = stm32f303re::FLASH_ALLOCATOR_END_ADDR;
            const FLASH_ALLOCATOR_SCAN: u32 = stm32f303re::FLASH_ALLOCATOR_START_SCAN_ADDR;
            const FLASH_BLOCK_SIZE: usize = stm32f303re::FLASH_BLOCK_SIZE;
            const FLASH_NUM_BLOCKS: usize = stm32f303re::FLASH_NUM_BLOCKS;
            const FLASH_NUM_SLOTS: usize = stm32f303re::FLASH_NUM_SLOTS;
            assert_eq!(stm32f303re::FLASH_FLAG_SIZE, 2);
            // Create fake flash memory
            flash_buffer.change_base_address(FLASH_START_ADDR);
            // Create the standard allocator
            let mut flash_alloc = FlashAllocatorImpl::<
                FLASH_START_ADDR,
                FLASH_END_ADDR,
                FLASH_ALLOCATOR_SCAN,
                FLASH_BLOCK_SIZE,
                FLASH_NUM_BLOCKS,
                FLASH_NUM_SLOTS,
                2,
            >::from_flash(flash_buffer);
            // Perform the allocation
            let flash_block = flash_alloc
                .allocate(needed_flash)
                .expect("Failed to allocate space for HBF");

            drop(flash_alloc);

            // Construct the RAM allocator
            const SRAM_START_ADDR: u32 = stm32f303re::SRAM_START_ADDR;
            const SRAM_END_ADDR: u32 = stm32f303re::SRAM_END_ADDR;
            const SRAM_BLOCK_SIZE: usize = stm32f303re::SRAM_BLOCK_SIZE;
            const SRAM_NUM_BLOCKS: usize = stm32f303re::SRAM_NUM_BLOCKS;
            const SRAM_NUM_SLOTS: usize = stm32f303re::SRAM_NUM_SLOTS;
            const SRAM_RESERVED: u32 = stm32f303re::SRAM_RESERVED;

            let mut ram_alloc = RAMAllocatorImpl::<
                SRAM_START_ADDR,
                SRAM_END_ADDR,
                SRAM_BLOCK_SIZE,
                SRAM_NUM_BLOCKS,
                SRAM_NUM_SLOTS,
                SRAM_RESERVED,
                FLASH_START_ADDR,
                FLASH_END_ADDR,
                FLASH_ALLOCATOR_SCAN,
                FLASH_NUM_SLOTS,
                FLASH_BLOCK_SIZE,
                2,
            >::from_flash(flash_buffer);

            let ram_block = ram_alloc
                .allocate(flash_block.get_base_address(), needed_ram)
                .expect("Cannot allocate memory for HBF");

            drop(ram_alloc);

            // Now finalize the block header
            flash_allocator::flash::utils::finalize_block::<FLASH_START_ADDR, FLASH_NUM_SLOTS, 2>(
                flash_buffer,
                flash_block,
            )
            .unwrap();

            let actual_base = flash_block.get_base_address() - 12;
            let actual_size = flash_block.get_size() + 12;
            let header_bytes = flash_buffer.read(actual_base, 12 + 8).unwrap();

            return AllocatedComponent {
                flash_address: actual_base,
                flash_size: actual_size,
                sram_address: ram_block.get_base_address(),
                sram_size: ram_block.get_size(),
                data: Vec::from(header_bytes),
            };
        }
        _ => {}
    };
    panic!("Unsupported board: {}", board_name);
}

struct BufferFlash {
    base_addr: u32,
    buffer: Vec<u8>,
}
impl BufferFlash {
    pub fn change_base_address(&mut self, new_base_addr: u32) {
        self.base_addr = new_base_addr;
    }
}

impl<'a> FlashMethods<'a> for BufferFlash {
    fn read(&self, address: u32, len: usize) -> Result<&'a [u8], ()> {
        if address < self.base_addr || address > self.base_addr + self.buffer.len() as u32 {
            return Err(());
        }
        let offset = (address - self.base_addr) as usize;
        Ok(unsafe {
            core::slice::from_raw_parts(self.buffer.as_ptr().add(offset) as *const u8, len)
        })
    }

    fn write(&mut self, address: u32, value: u8) -> Result<(), ()> {
        if address < self.base_addr || address > self.base_addr + self.buffer.len() as u32 {
            return Err(());
        }
        let offset = (address - self.base_addr) as usize;
        self.buffer[offset] = value;
        Ok(())
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn page_from_address(&self, _address: u32) -> Option<flash_allocator::flash::page::FlashPage> {
        panic!("Not used");
    }

    fn page_from_number(&self, _page_num: u16) -> Option<flash_allocator::flash::page::FlashPage> {
        panic!("Not used");
    }

    fn prev_page(&self, _page_num: u16) -> Option<flash_allocator::flash::page::FlashPage> {
        panic!("Not used");
    }

    fn erase(&mut self, _page_num: u16) -> Result<(), ()> {
        panic!("Not used");
    }
}
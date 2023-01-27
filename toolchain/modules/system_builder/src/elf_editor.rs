use flash_allocator::flash::{FlashAllocatorImpl, FlashMethods, BlockType};
use goblin::{container::Container, elf64::program_header::PT_LOAD};
use hbf_rs::HbfFile;
use ram_allocator::{RAMAllocator, RAMAllocatorImpl};
use std::process::Command;
use std::{collections::BTreeMap, path::PathBuf};

/// Maximum size of the kernel during an update operation.
/// This value was calculated reading memory with the debugger,
/// after initially 0xff clearing it
const DEFAULT_KERNEL_STACK_SIZE: u32 = 2900;

#[derive(Debug, Clone)]
pub struct AllocStatEntry {
    pub name: String,
    pub component_id: u16,
    pub flash_address: u32,
    pub flash_size: u32,
    pub flash_needed_size: u32,
    pub ram_address: u32,
    pub ram_size: u32,
    pub ram_needed_size: u32,
}

pub struct AllocStats {
    pub flash_start: u32,
    pub flash_size: u32,
    pub ram_start: u32,
    pub ram_size: u32,
    pub kernel_reserved_ram: u32,
    pub kernel_reserved_flash: u32,
    pub entries: Vec<AllocStatEntry>,
}

pub struct ElfEditor<'a> {
    dest_path: &'a PathBuf,
    output_sections: BTreeMap<u32, Vec<u8>>,
    current_flash_size: usize,
    board_name: &'a String,
    kentry: u32,
    flash_buffer: BufferFlash,
    allocation_stats: AllocStats,
}

impl<'a> ElfEditor<'a> {
    pub fn new(dest_path: &'a PathBuf, board_name: &'a String) -> Self {
        let (flash_start, flash_size, ram_start, ram_size, kernel_ram, kernel_flash) = match board_name.as_str() {
            "stm32f303re" => (
                stm32f303re::FLASH_START_ADDR,
                stm32f303re::FLASH_ALLOCATOR_SIZE as u32,
                stm32f303re::SRAM_START_ADDR,
                stm32f303re::SRAM_END_ADDR - stm32f303re::SRAM_START_ADDR + 1,
                stm32f303re::SRAM_RESERVED,
                stm32f303re::FLASH_ALLOCATOR_START_SCAN_ADDR - stm32f303re::FLASH_ALLOCATOR_START_ADDR
            ),
            "stm32l432kc" => (
                stm32l432kc::FLASH_START_ADDR,
                stm32l432kc::FLASH_ALLOCATOR_SIZE as u32,
                stm32l432kc::SRAM_START_ADDR,
                stm32l432kc::SRAM_END_ADDR - stm32l432kc::SRAM_START_ADDR + 1,
                stm32l432kc::SRAM_RESERVED,
                stm32l432kc::FLASH_ALLOCATOR_START_SCAN_ADDR - stm32l432kc::FLASH_ALLOCATOR_START_ADDR
            ),
            _ => panic!("Unsupported board"),
        };
        println!("Flash Start: {:#010x}", flash_start);
        println!("Flash Size: {}", flash_size);
        println!("RAM Start: {:#010x}", ram_start);
        println!("RAM Size: {}", ram_size);
        Self {
            dest_path: dest_path,
            output_sections: BTreeMap::new(),
            current_flash_size: 0,
            kentry: 0,
            board_name: board_name,
            flash_buffer: BufferFlash {
                base_addr: 0,
                buffer: vec![0xFF; flash_size as usize],
            },
            allocation_stats: AllocStats {
                flash_start: flash_start,
                flash_size: flash_size,
                ram_start: ram_start,
                ram_size: ram_size,
                kernel_reserved_ram: kernel_ram,
                kernel_reserved_flash: kernel_flash,
                entries: Vec::new(),
            },
        }
    }
    fn is_in_flash(&self, addr: u32) -> bool {
        addr >= self.allocation_stats.flash_start
            && addr < self.allocation_stats.flash_start + self.allocation_stats.flash_size
    }
    fn is_in_sram(&self, addr: u32) -> bool {
        addr >= self.allocation_stats.ram_start
            && addr < self.allocation_stats.ram_start + self.allocation_stats.ram_size
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
        let mut flash_used: u32 = 0;
        let mut sram_used: u32 = 0;
        for phdr in &elf.program_headers {
            if phdr.p_type != PT_LOAD {
                continue; // Ignore
            }
            let offset = phdr.p_offset as usize;
            let size = phdr.p_filesz as usize;
            let addr = phdr.p_paddr as u32;
            let dest_addr = phdr.p_vaddr as u32;
            let mem_size = phdr.p_memsz as usize;
            if self.is_in_flash(addr) {
                flash_used += size as u32;
            }
            if self.is_in_sram(dest_addr) || self.is_in_sram(addr) {
                sram_used += mem_size as u32;
            }
            // Update stats
            self.current_flash_size += size;
            // Add to our structure
            self.output_sections
                .insert(addr, elf_bytes[offset..offset + size].to_vec());
        }
        self.kentry = elf.header.e_entry as u32;
        // Add stat
        self.allocation_stats.entries.push(AllocStatEntry {
            name: String::from("Kernel"),
            component_id: 0,
            flash_address: self.allocation_stats.flash_start,
            flash_size: self.allocation_stats.kernel_reserved_flash,
            flash_needed_size: flash_used,
            ram_address: self.allocation_stats.ram_start,
            ram_size: self.allocation_stats.kernel_reserved_ram,
            ram_needed_size: sram_used + DEFAULT_KERNEL_STACK_SIZE,
        });
        /*if self.allocation_stats.kernel_reserved_ram < sram_used + DEFAULT_KERNEL_STACK_SIZE {
            panic!(
                "Not enough memory for kernel: {} bytes available, but {} bytes needed",
                self.allocation_stats.kernel_reserved_ram,
                sram_used + DEFAULT_KERNEL_STACK_SIZE
            );
        }*/
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
        println!(
            "Allocated component {} at flash: {:#010x} [size: {}], ram: {:#010x} [size: {}]",
            hbf.header_base().component_id(),
            alloc_result.flash_address,
            alloc_result.flash_size,
            alloc_result.sram_address,
            alloc_result.sram_size
        );

        // Add stat
        self.allocation_stats.entries.push(AllocStatEntry {
            name: format!(
                "Component: {} [v {}]",
                hbf.header_base().component_id(),
                hbf.header_base().component_version()
            ),
            component_id: hbf.header_base().component_id(),
            flash_address: alloc_result.flash_address,
            flash_size: alloc_result.flash_size,
            flash_needed_size: needed_flash,
            ram_address: alloc_result.sram_address,
            ram_size: alloc_result.sram_size,
            ram_needed_size: needed_ram,
        });

        let block_base_addr: u32 = alloc_result.flash_address;
        // Generate bytes
        let mut component_bytes: Vec<u8> = Vec::new();
        // Add block header bytes
        component_bytes.extend_from_slice(&alloc_result.data);
        // Edit hbf and append
        let relocs = extract_hbf_relocations(&hbf);
        let dest_base_address = block_base_addr + 8 + hbf.read_only_section().offset() + flash_allocator::flash::HEADER_SIZE as u32;
        let checksum_offset = hbf.checksum_offset() as usize;
        drop(hbf);
        relocate_hbf(&mut hbf_bytes, dest_base_address, &relocs);
        fix_checksum_hbf(&mut hbf_bytes, checksum_offset);
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

    pub fn finish(mut self) -> AllocStats {
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
        // Return stats
        self.allocation_stats
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

fn fix_checksum_hbf(hbf_bytes: &mut [u8], checksum_offset: usize) {
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
        if index == checksum_offset {
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
        hbf_bytes[checksum_offset + i] = checksum_bytes[i];
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
            const FLASH_TREE_MAX_LEVEL: usize = stm32f303re::FLASH_TREE_MAX_LEVEL;
            const FLASH_NUM_NODES: usize = stm32f303re::FLASH_NUM_NODES;
            // Create fake flash memory
            flash_buffer.change_base_address(FLASH_START_ADDR);
            // Create the standard allocator
            let mut flash_alloc = FlashAllocatorImpl::<
                FLASH_START_ADDR,
                FLASH_END_ADDR,
                FLASH_ALLOCATOR_SCAN,
                FLASH_BLOCK_SIZE,
                FLASH_NUM_BLOCKS,
                FLASH_TREE_MAX_LEVEL,
                FLASH_NUM_NODES
            >::from_flash(flash_buffer, false, false);
            // Perform the allocation
            let flash_block = flash_alloc
                .allocate(needed_flash + 8, BlockType::COMPONENT)
                .expect("Failed to allocate space for HBF");

            drop(flash_alloc);

            // Construct the RAM allocator
            const SRAM_START_ADDR: u32 = stm32f303re::SRAM_START_ADDR;
            const SRAM_END_ADDR: u32 = stm32f303re::SRAM_END_ADDR;
            const SRAM_BLOCK_SIZE: usize = stm32f303re::SRAM_BLOCK_SIZE;
            const SRAM_NUM_BLOCKS: usize = stm32f303re::SRAM_NUM_BLOCKS;
            const SRAM_TREE_MAX_LEVEL: usize = stm32f303re::SRAM_TREE_MAX_LEVEL;
            const SRAM_NUM_NODES: usize = stm32f303re::SRAM_NUM_NODES;
            
            const SRAM_RESERVED: u32 = stm32f303re::SRAM_RESERVED;

            let mut ram_alloc = RAMAllocatorImpl::<
                SRAM_START_ADDR,
                SRAM_END_ADDR,
                SRAM_BLOCK_SIZE,
                SRAM_NUM_BLOCKS,
                SRAM_TREE_MAX_LEVEL,
                SRAM_NUM_NODES,
                SRAM_RESERVED,
                FLASH_START_ADDR,
                FLASH_END_ADDR,
                FLASH_ALLOCATOR_SCAN,
                FLASH_TREE_MAX_LEVEL
            >::from_flash(flash_buffer);

            let ram_block = ram_alloc
                .allocate(flash_block.get_base_address(), needed_ram)
                .expect("Cannot allocate memory for HBF");

            drop(ram_alloc);

            // Now finalize the block header
            flash_allocator::flash::utils::finalize_block::<FLASH_START_ADDR, FLASH_TREE_MAX_LEVEL>(
                flash_buffer,
                flash_block,
            )
            .unwrap();
            const BLOCK_HEADER_SIZE: usize = flash_allocator::flash::HEADER_SIZE;
            let actual_base = flash_block.get_base_address() - BLOCK_HEADER_SIZE as u32;
            let actual_size = flash_block.get_size() + BLOCK_HEADER_SIZE as u32 + 8;
            let mut header_bytes: [u8; BLOCK_HEADER_SIZE + 8] = [0x00; BLOCK_HEADER_SIZE + 8];
            flash_buffer.read(actual_base, &mut header_bytes).unwrap();

            return AllocatedComponent {
                flash_address: actual_base,
                flash_size: actual_size,
                sram_address: ram_block.get_base_address(),
                sram_size: ram_block.get_size(),
                data: Vec::from(header_bytes),
            };
        },
        "stm32l432kc" => {
            const FLASH_START_ADDR: u32 = stm32l432kc::FLASH_ALLOCATOR_START_ADDR;
            const FLASH_END_ADDR: u32 = stm32l432kc::FLASH_ALLOCATOR_END_ADDR;
            const FLASH_ALLOCATOR_SCAN: u32 = stm32l432kc::FLASH_ALLOCATOR_START_SCAN_ADDR;
            const FLASH_BLOCK_SIZE: usize = stm32l432kc::FLASH_BLOCK_SIZE;
            const FLASH_NUM_BLOCKS: usize = stm32l432kc::FLASH_NUM_BLOCKS;
            const FLASH_TREE_MAX_LEVEL: usize = stm32l432kc::FLASH_TREE_MAX_LEVEL;
            const FLASH_NUM_NODES: usize = stm32l432kc::FLASH_NUM_NODES;
            // Create fake flash memory
            flash_buffer.change_base_address(FLASH_START_ADDR);
            // Create the standard allocator
            let mut flash_alloc = FlashAllocatorImpl::<
                FLASH_START_ADDR,
                FLASH_END_ADDR,
                FLASH_ALLOCATOR_SCAN,
                FLASH_BLOCK_SIZE,
                FLASH_NUM_BLOCKS,
                FLASH_TREE_MAX_LEVEL,
                FLASH_NUM_NODES
            >::from_flash(flash_buffer, false, false);
            // Perform the allocation
            let flash_block = flash_alloc
                .allocate(needed_flash + 8, BlockType::COMPONENT)
                .expect("Failed to allocate space for HBF");

            drop(flash_alloc);

            // Construct the RAM allocator
            const SRAM_START_ADDR: u32 = stm32l432kc::SRAM_START_ADDR;
            const SRAM_END_ADDR: u32 = stm32l432kc::SRAM_END_ADDR;
            const SRAM_BLOCK_SIZE: usize = stm32l432kc::SRAM_BLOCK_SIZE;
            const SRAM_NUM_BLOCKS: usize = stm32l432kc::SRAM_NUM_BLOCKS;
            const SRAM_TREE_MAX_LEVEL: usize = stm32l432kc::SRAM_TREE_MAX_LEVEL;
            const SRAM_NUM_NODES: usize = stm32l432kc::SRAM_NUM_NODES;
            
            const SRAM_RESERVED: u32 = stm32l432kc::SRAM_RESERVED;

            let mut ram_alloc = RAMAllocatorImpl::<
                SRAM_START_ADDR,
                SRAM_END_ADDR,
                SRAM_BLOCK_SIZE,
                SRAM_NUM_BLOCKS,
                SRAM_TREE_MAX_LEVEL,
                SRAM_NUM_NODES,
                SRAM_RESERVED,
                FLASH_START_ADDR,
                FLASH_END_ADDR,
                FLASH_ALLOCATOR_SCAN,
                FLASH_TREE_MAX_LEVEL
            >::from_flash(flash_buffer);

            let ram_block = ram_alloc
                .allocate(flash_block.get_base_address(), needed_ram)
                .expect("Cannot allocate memory for HBF");

            drop(ram_alloc);

            // Now finalize the block header
            flash_allocator::flash::utils::finalize_block::<FLASH_START_ADDR, FLASH_TREE_MAX_LEVEL>(
                flash_buffer,
                flash_block,
            )
            .unwrap();
            const BLOCK_HEADER_SIZE: usize = flash_allocator::flash::HEADER_SIZE;
            let actual_base = flash_block.get_base_address() - BLOCK_HEADER_SIZE as u32;
            let actual_size = flash_block.get_size() + BLOCK_HEADER_SIZE as u32 + 8;
            let mut header_bytes: [u8; BLOCK_HEADER_SIZE + 8] = [0x00; BLOCK_HEADER_SIZE + 8];
            flash_buffer.read(actual_base, &mut header_bytes).unwrap();

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
    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), ()> {
        if address < self.base_addr || address > self.base_addr + self.buffer.len() as u32 {
            return Err(());
        }
        let offset = (address - self.base_addr) as usize;
        self.buffer[offset] = value;
        Ok(())
    }
}

impl<'a> FlashMethods<'a> for BufferFlash {
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
        if address < self.base_addr || address > self.base_addr + self.buffer.len() as u32 {
            return Err(());
        }
        let offset = (address - self.base_addr) as usize;
        for i in 0..buffer.len() {
            buffer[i] = self.buffer[offset + i];
        }
        Ok(())
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
        for i in 0..data.len() {
            self.write_u8(address + i as u32, data[i])?;
        }
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

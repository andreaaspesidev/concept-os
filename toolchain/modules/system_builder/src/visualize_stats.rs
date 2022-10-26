use std::{fs::File, io::Write, path::PathBuf};

use crate::elf_editor::{AllocStatEntry, AllocStats};

const TOTAL_WIDTH: usize = 1000;

pub fn visualize(stats: AllocStats, root: &PathBuf) {
    visualize_flash(&stats, root);
    visualize_ram(&stats, root);
}

struct Block {
    //start_addr: u32,
    size: u32,
    entry: Option<AllocStatEntry>,
}

fn visualize_flash(stats: &AllocStats, root: &PathBuf) {
    // Create a new writer
    let mut flash_report_path = PathBuf::from(root);
    flash_report_path.push("FlashReport.html");
    let file = File::create(flash_report_path).unwrap();
    // First sort stats by address
    let mut ordered_entries = stats.entries.to_vec();
    ordered_entries.sort_by(|a, b| a.flash_address.cmp(&b.flash_address));
    // Now iterate for each address to find blocks
    let mut curr_address: u32 = stats.flash_start;
    let mut curr_ptr: usize = 0;
    let mut blocks: Vec<Block> = Vec::new();
    let mut used_flash: u32 = 0;
    // Iterate over all flash
    let flash_end = stats.flash_start + stats.flash_size;
    while curr_address < flash_end {
        // Check whether we have an allocation
        if curr_ptr < ordered_entries.len() {
            // Get the current allocation
            let curr_entry = &ordered_entries[curr_ptr];
            // We have three cases
            if curr_entry.flash_address > curr_address {
                // This is free flash
                blocks.push(Block {
                    //start_addr: curr_address,
                    size: curr_entry.flash_address - curr_address,
                    entry: None,
                });
                curr_address = curr_entry.flash_address;
            } else if curr_entry.flash_address == curr_address {
                // We got to our block, add it then skip it
                blocks.push(Block {
                    //start_addr: curr_address,
                    size: curr_entry.flash_size,
                    entry: Some(curr_entry.clone()),
                });
                used_flash += curr_entry.flash_size;
                curr_ptr += 1;
                curr_address = curr_entry.flash_address + curr_entry.flash_size;
            } else {
                // We might have problems with entries ordering
                panic!("Something went wrong");
            }
        } else {
            // Otherwise we finished all the allocations: all free space
            blocks.push(Block {
                //start_addr: curr_address,
                size: flash_end - curr_address,
                entry: None,
            });
            break;
        }
    }
    // Print flash
    let mut writer = StatWriter::new(file, stats.flash_size, used_flash, TOTAL_WIDTH, flash_info);
    for b in blocks {
        writer.add_block(b);
    }
    writer.generate();
}

fn visualize_ram(stats: &AllocStats, root: &PathBuf) {
    // Create a new writer
    let mut ram_report_path = PathBuf::from(root);
    ram_report_path.push("RAMReport.html");
    let file = File::create(ram_report_path).unwrap();
    // First sort stats by address
    let mut ordered_entries = stats.entries.to_vec();
    ordered_entries.sort_by(|a, b| a.ram_address.cmp(&b.ram_address));
    // Now iterate for each address to find blocks
    let mut curr_address: u32 = stats.ram_start;
    let mut curr_ptr: usize = 0;
    let mut blocks: Vec<Block> = Vec::new();
    let mut used_ram: u32 = 0;
    // Iterate over all ram
    let ram_end = stats.ram_start + stats.ram_size;
    while curr_address < ram_end {
        // Check whether we have an allocation
        if curr_ptr < ordered_entries.len() {
            // Get the current allocation
            let curr_entry = &ordered_entries[curr_ptr];
            // We have three cases
            if curr_entry.ram_address > curr_address {
                // This is free ram
                blocks.push(Block {
                    //start_addr: curr_address,
                    size: curr_entry.ram_address - curr_address,
                    entry: None,
                });
                curr_address = curr_entry.ram_address;
            } else if curr_entry.ram_address == curr_address {
                // We got to our block, add it then skip it
                blocks.push(Block {
                    //start_addr: curr_address,
                    size: curr_entry.ram_size,
                    entry: Some(curr_entry.clone()),
                });
                used_ram += curr_entry.ram_size;
                curr_ptr += 1;
                curr_address = curr_entry.ram_address + curr_entry.ram_size;
            } else {
                // We might have problems with entries ordering
                panic!("Something went wrong");
            }
        } else {
            // Otherwise we finished all the allocations: all free space
            blocks.push(Block {
                //start_addr: curr_address,
                size: ram_end - curr_address,
                entry: None,
            });
            break;
        }
    }
    // Print ram
    let mut writer = StatWriter::new(file, stats.ram_size, used_ram, TOTAL_WIDTH, ram_info);
    for b in blocks {
        writer.add_block(b);
    }
    writer.generate();
}

fn flash_info(e: &AllocStatEntry) -> String {
    let mut entry = String::new();
    entry.push_str(&format!("{}<br>", e.name));
    entry.push_str(&format!(
        "&emsp;Start Address: {:#010x}<br>",
        e.flash_address
    ));
    entry.push_str(&format!("&emsp;Size: {}<br>", e.flash_size));
    entry.push_str(&format!("&emsp;Needed Size: {}<br>", e.flash_needed_size));
    entry.push_str(&format!(
        "&emsp;Wasted Space: {}%<br><br>",
        (e.flash_size - e.flash_needed_size) as f32 / e.flash_size as f32 * 100.0
    ));
    entry
}

fn ram_info(e: &AllocStatEntry) -> String {
    let mut entry = String::new();
    entry.push_str(&format!("{}<br>", e.name));
    entry.push_str(&format!("&emsp;Start Address: {:#010x}<br>", e.ram_address));
    entry.push_str(&format!("&emsp;Size: {}<br>", e.ram_size));
    entry.push_str(&format!("&emsp;Needed Size: {}<br>", e.ram_needed_size));
    entry.push_str(&format!(
        "&emsp;Wasted Space: {}%<br><br>",
        (e.ram_size - e.ram_needed_size) as f32 / e.ram_size as f32 * 100.0
    ));
    entry
}

struct StatWriter {
    file: File,
    width: usize,
    total_size: u32,
    used_size: u32,
    blocks: Vec<Block>,
    info_gen: fn(&AllocStatEntry) -> String,
}

impl StatWriter {
    pub fn new(
        file: File,
        total_size: u32,
        used_size: u32,
        width: usize,
        info_gen: fn(&AllocStatEntry) -> String,
    ) -> Self {
        Self {
            file: file,
            width: width,
            total_size: total_size,
            used_size: used_size,
            info_gen: info_gen,
            blocks: Vec::new(),
        }
    }
    fn size_to_scale(&self, size: u32) -> usize {
        (size as f32 / self.total_size as f32 * self.width as f32).ceil() as usize
    }
    pub fn add_block(&mut self, b: Block) {
        self.blocks.push(b);
    }
    pub fn generate(mut self) {
        let mut out = String::new();
        // Generate scaled bar
        out.push_str(&format!(
            "<table style='table-layout: fixed; width: {}px;border: 1px solid;'>",
            self.width
        ));
        out.push_str("<tr>");
        for b in &self.blocks {
            let width = self.size_to_scale(b.size);
            let cell_text = match &b.entry {
                Some(entry) => format!("{}", entry.component_id),
                None => String::new(),
            };
            out.push_str(&format!(
                "<td style='overflow:hidden; width: {}px;border: 1px solid;'>{}</td>",
                width, cell_text
            ));
        }
        out.push_str("</tr>");
        out.push_str("</table><br>");
        // Put total size
        out.push_str(&format!(
            "Used: {} / {} [{}%]<br><hr><br>",
            self.used_size,
            self.total_size,
            self.used_size as f32 / self.total_size as f32 * 100.0
        ));
        // Put infos
        for b in &self.blocks {
            if let Some(entry) = &b.entry {
                let info = (self.info_gen)(entry);
                out.push_str(&info);
            }
        }
        // Write output
        self.file.write(out.as_bytes()).unwrap();
    }
}

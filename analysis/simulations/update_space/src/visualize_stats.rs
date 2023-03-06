use std::{fs::File, io::Write, path::PathBuf};

use flash_allocator::flash::FlashBlock;

use crate::fake_flash::Flash;

const TOTAL_WIDTH: usize = 1000;

pub struct AllocStats {
    pub entries: Vec<FlashBlock>,
    pub flash_start: u32,
    pub flash_size: u32
}

struct Block {
    // start_addr: u32,
    size: u32,
    entry: Option<FlashBlock>,
}

pub fn visualize_flash(stats: &AllocStats, flash_report_path: &PathBuf) {
    // Create a new writer
    let file = File::create(flash_report_path).unwrap();
    // First sort stats by address
    let mut ordered_entries = stats.entries.to_vec();
    ordered_entries.sort_by(|a, b| a.get_nominal_base_address().cmp(&b.get_nominal_base_address()));
    // Now iterate for each address to find blocks
    let mut curr_address: u32 = stats.flash_start;
    let mut curr_ptr: usize = 0;
    let mut blocks: Vec<Block> = Vec::new();
    let mut used_flash: u32 = 0;
    let mut actual_used_flash: u32 = 0;
    // Iterate over all flash
    let flash_end = stats.flash_start + stats.flash_size;
    while curr_address < flash_end {
        // Check whether we have an allocation
        if curr_ptr < ordered_entries.len() {
            // Get the current allocation
            let curr_entry = &ordered_entries[curr_ptr];
            // We have three cases
            if curr_entry.get_nominal_base_address() > curr_address {
                // This is free flash
                blocks.push(Block {
                    //start_addr: curr_address,
                    size: curr_entry.get_nominal_base_address() - curr_address,
                    entry: None,
                });
                curr_address = curr_entry.get_nominal_base_address();
                actual_used_flash = curr_address - stats.flash_start;
            } else if curr_entry.get_nominal_base_address() == curr_address {
                // We got to our block, add it then skip it
                blocks.push(Block {
                    //start_addr: curr_address,
                    size: curr_entry.get_nominal_size(),
                    entry: Some(curr_entry.clone()),
                });
                used_flash += curr_entry.get_nominal_size();
                curr_ptr += 1;
                curr_address = curr_entry.get_nominal_base_address() + curr_entry.get_nominal_size();
                actual_used_flash = curr_address - stats.flash_start;
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
    let mut writer = StatWriter::new(file, stats.flash_size, used_flash, actual_used_flash, TOTAL_WIDTH, flash_info);
    for b in blocks {
        writer.add_block(b);
    }
    writer.generate();
}

fn flash_info(e: &FlashBlock, index: usize) -> String {
    let mut entry = String::new();
    entry.push_str(&format!("{}<br>", index));
    entry.push_str(&format!(
        "&emsp;Start Address: {:#010x}<br>",
        e.get_nominal_base_address()
    ));
    entry.push_str(&format!("&emsp;Size: {}<br>", e.get_nominal_size()));
    entry
}
struct StatWriter {
    file: File,
    width: usize,
    total_size: u32,
    used_size: u32,
    actual_used_size: u32,
    blocks: Vec<Block>,
    info_gen: fn(&FlashBlock, usize) -> String,
}

impl StatWriter {
    pub fn new(
        file: File,
        total_size: u32,
        used_size: u32,
        actual_used_size: u32,
        width: usize,
        info_gen: fn(&FlashBlock, usize) -> String,
    ) -> Self {
        Self {
            file: file,
            width: width,
            total_size: total_size,
            used_size: used_size,
            actual_used_size: actual_used_size,
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
        for (index, b) in self.blocks.iter().enumerate() {
            let width = self.size_to_scale(b.size);
            let cell_text = match &b.entry {
                Some(_entry) => format!("{}", index),
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
            "Used: {} / {} [{}%]<br>Actual Used: {} / {} [{}%]<br><hr><br>",
            self.used_size,
            self.total_size,
            self.used_size as f32 / self.total_size as f32 * 100.0,
            self.actual_used_size,
            self.total_size,
            self.actual_used_size as f32 / self.total_size as f32 * 100.0,
        ));
        // Put infos
        for (index, b) in self.blocks.iter().enumerate() {
            if let Some(entry) = &b.entry {
                let info = (self.info_gen)(entry,index);
                out.push_str(&info);
            }
        }
        // Write output
        self.file.write(out.as_bytes()).unwrap();
    }
}

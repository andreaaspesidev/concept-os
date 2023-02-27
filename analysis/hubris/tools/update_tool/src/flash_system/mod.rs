use crossbeam_channel::{Receiver, Sender};
use pbr::ProgressBar;

use crate::{common_messages::{SerializableMessage, RawPacket, FragmentedMessage}, utils::*};
use std::{path::PathBuf, io::Stdout};

use self::{
    elf::{parse_elf, ElfSection},
    messages::{CommandStartMessage, SectionHeaderMessage, UpdateMessages},
};

mod elf;
mod messages;

pub fn flash_system(
    channel_in_consumer: Receiver<u8>,
    channel_out_producer: Sender<Vec<u8>>,
    elf_file: String,
    verbose: bool,
) {
    if verbose {
        println!("---> Flashing System");
    }
    // Validate paths
    let elf_path = PathBuf::from(elf_file);
    if !elf_path.exists() {
        panic!("Cannot find image at '{}'", elf_path.display());
    }
    // Parse elf
    let elf = parse_elf(&elf_path);
    // Setup progress
    let mut progress = ProgressBar::new(elf.total_size as u64);
    progress.show_speed = false;
    progress.show_counter = false;
    progress.show_time_left = false;
    progress.set_width(Some(80));
    // Start by sending command
    progress.message("Connection Setup   ");
    let hello_msg = CommandStartMessage::new(elf.total_size);
    channel_flush_read(&channel_in_consumer);
    channel_write(&channel_out_producer, &hello_msg.get_raw());
    // Send each section
    for section in &elf.sections {
        // Wait section header request
        wait_section_header(&channel_in_consumer, &channel_out_producer);
        // Produce and send packet
        let section_header = SectionHeaderMessage::new(
            section.start_addr_relative,
            section.data.len() as u32,
        );
        channel_write(&channel_out_producer, &section_header.get_raw());
        // Wait for next fragment until all the section is processed
        send_section(&channel_in_consumer, &channel_out_producer, section, &mut progress);
    }
    // Success
    progress.message("Success");
    progress.finish();
}

fn wait_section_header(
    channel_in_consumer: &Receiver<u8>,
    _channel_out_producer: &Sender<Vec<u8>>,
) {
    let mut sh_buff: [u8; 1] = [0x00; 1];
    channel_read(channel_in_consumer, &mut sh_buff);
    if sh_buff[0] != UpdateMessages::SendSectionHeader as u8 {
        panic!(
            "Unexpected response from device at section header: {:?}",
            sh_buff[0]
        );
    }
}

fn wait_next_fragment(
    channel_in_consumer: &Receiver<u8>,
    _channel_out_producer: &Sender<Vec<u8>>,
) {
    let mut sh_buff: [u8; 1] = [0x00; 1];
    channel_read(channel_in_consumer, &mut sh_buff);
    if sh_buff[0] != UpdateMessages::SendNextFragment as u8 {
        panic!(
            "Unexpected response from device at section transmission: {:?}",
            sh_buff[0]
        );
    }
}

fn send_section(
    channel_in_consumer: &Receiver<u8>,
    channel_out_producer: &Sender<Vec<u8>>,
    section: &ElfSection,
    progress: &mut ProgressBar<Stdout>
) {
    let mut section_packet = RawPacket::new(&section.data);
    // Wait for fragment request
    wait_next_fragment(channel_in_consumer, channel_out_producer);
    loop {
        // Update progress
        progress.message(&format!(
            "Fragment {}/{}   ",
            section_packet.get_next_fragment_number().unwrap(),
            section_packet.get_total_fragments()
        ));
        // Send data
        let fragment_data = section_packet.get_next_fragment().unwrap();
        channel_write(channel_out_producer, &fragment_data);
        // Update progress
        progress.add((fragment_data.len() - 1) as u64);
        if section_packet.get_next_fragment_number().is_some() {
            // Wait for fragment request
            wait_next_fragment(channel_in_consumer, channel_out_producer);
        } else {
            break;
        }
    }
}

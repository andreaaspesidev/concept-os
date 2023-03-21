// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod messages;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use cbf_rs::CbfFile;
use pbr::ProgressBar;
use std::{io::Stdout, path::PathBuf};

use self::messages::*;
use crate::common_messages::*;
use crate::utils::*;

pub fn flash_component(
    channel_in_consumer: Receiver<u8>,
    channel_out_producer: Sender<Vec<u8>>,
    cbf_file: String,
    verbose: bool,
) {
    if verbose {
        println!("---> Flashing Component");
    }
    // First, check the cbf path
    let cbf_path = PathBuf::from(cbf_file.clone());
    if !cbf_path.exists() {
        panic!("Cannot find the CBF at '{}'", cbf_file);
    }
    // Read the whole cbf in memory (surely small for a PC)
    let cbf_bytes =
        std::fs::read(cbf_path).expect(&format!("Cannot open the CBF at '{}'", cbf_file));
    // Parse the cbf
    let cbf_result = cbf_rs::parse_cbf(&cbf_bytes);
    if cbf_result.is_err() {
        match cbf_result.unwrap_err() {
            cbf_rs::Error::BufferTooShort | cbf_rs::Error::InvalidMagic => {
                panic!("CBF file not valid!")
            }
            cbf_rs::Error::UnsupportedVersion => {
                panic!("CBF version still not supported by the tool")
            }
        }
    }
    let cbf = cbf_result.unwrap();
    // Validate cbf
    if !cbf.validate() {
        panic!("CBF file integrity test failed!");
    }
    // If verbose, print some info
    if verbose {
        println!("\n\tComponent ID: {}", cbf.header_base().component_id());
        println!(
            "\tComponent Version: {}",
            cbf.header_base().component_version()
        );
        println!("\tRequired Flash Size: {}", cbf.header_base().total_size());
        println!(
            "\tRequired SRAM Size: {}",
            cbf.header_main().component_min_ram()
        );
    }
    // Send hello
    println!("");
    let mut progress = ProgressBar::new((cbf.header_base().total_size() + 4) as u64);
    progress.show_speed = false;
    progress.show_counter = false;
    progress.show_time_left = false;
    progress.set_width(Some(80));
    begin_communication(
        &channel_in_consumer,
        &channel_out_producer,
        &cbf,
        &mut progress,
        verbose,
    );
}

fn begin_communication(
    channel_in_consumer: &Receiver<u8>,
    channel_out_producer: &Sender<Vec<u8>>,
    cbf: &dyn CbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    // Send hello message
    progress.message("Connection Setup   ");
    let hello_msg = HelloMessage::new(OperationType::ComponentUpdate);
    channel_flush_read(channel_in_consumer);
    channel_write(channel_out_producer, &hello_msg.get_raw());
    // Read hello response
    let mut buff: [u8; HelloResponseMessage::get_size()] = [0x00; HelloResponseMessage::get_size()];
    channel_read(channel_in_consumer, &mut buff);
    // Validate hello response
    let hello_response = HelloResponseMessage::from(&buff);
    if hello_response.is_err() {
        eprintln!("Wrong response from device at HELLO");
        return;
    }
    if verbose {
        println!("Got HELLO!");
    }
    progress.inc();
    // Wait for header request
    let mut buff: [u8; 1] = [0x00; 1];
    //flush_read(serial);
    channel_read(channel_in_consumer, &mut buff);
    if buff[0] != ComponentUpdateCommand::SendComponentFixedHeader as u8 {
        eprintln!(
            "Unexpected response from device at first step (Fixed Header): {:?}",
            MessageError::from(buff[0])
        );
        return;
    }
    send_fixed_header(channel_in_consumer, channel_out_producer, cbf, progress, verbose);
}

fn send_fixed_header(
    channel_in_consumer: &Receiver<u8>,
    channel_out_producer: &Sender<Vec<u8>>,
    cbf: &dyn CbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Fixed Header");
    }
    progress.inc();
    // Send fixed header
    let base_header_raw = cbf.header_base().get_raw();
    let main_header_raw = cbf.header_main().get_raw();
    // Combine in a single packet
    let mut out_buff = Vec::<u8>::new();
    out_buff.extend_from_slice(base_header_raw);
    out_buff.extend_from_slice(main_header_raw);
    // Construct packet and send
    progress.message("Header   ");
    let fixed_header_msg = FixedHeaderMessage::new(&out_buff);
    channel_write(channel_out_producer, &fixed_header_msg.get_raw());
    // Update progress
    progress.add((out_buff.len() - 1) as u64);
    // Wait for variable header request
    let mut buff: [u8; 1] = [0x00; 1];
    channel_read(channel_in_consumer, &mut buff);
    if buff[0] != ComponentUpdateCommand::SendComponentVariableHeader as u8 {
        eprintln!(
            "Unexpected response from device at second step (Variable Header): {:?}",
            MessageError::from(buff[0])
        );
        return;
    }
    send_variable_header(channel_in_consumer, channel_out_producer, cbf, progress, verbose);
}

fn send_variable_header(
    channel_in_consumer: &Receiver<u8>,
    channel_out_producer: &Sender<Vec<u8>>,
    cbf: &dyn CbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Variable Header");
    }
    progress.inc();
    // Generate bytes
    let vhb = extract_variable_header(cbf);
    // Start sending
    let mut pkt = RawPacket::new(&vhb);
    loop {
        let mut buff: [u8; 1] = [0x00; 1];
        // Wait for next request
        channel_read(channel_in_consumer, &mut buff);

        if buff[0] == ComponentUpdateCommand::SendComponentPayload as u8 {
            // Check we actually finished sending the variable header
            if pkt.get_next_fragment().is_some() {
                eprintln!("Still some header to be send!");
                return;
            }
            break; // Everything okay
        } else if buff[0] != ComponentUpdateCommand::SendNextFragment as u8 {
            eprintln!(
                "Unexpected response from device at third step (Variable Header): {:?}",
                MessageError::from(buff[0])
            );
            return;
        }
        // Send fragment
        progress.message(&format!(
            "Header Fragment {}/{}   ",
            pkt.get_next_fragment_number().unwrap(),
            pkt.get_total_fragments()
        ));
        //println!("\tSending Fragment {}/{}", pkt.get_next_fragment_number().unwrap(), pkt.get_total_fragments());
        let fragment_data = pkt.get_next_fragment().unwrap();
        channel_write(channel_out_producer, &fragment_data);
        // Update progress
        progress.add((fragment_data.len() - 1) as u64);
    }
    send_payload(channel_in_consumer, channel_out_producer, cbf, progress, verbose);
}

fn send_payload(
    channel_in_consumer: &Receiver<u8>,
    channel_out_producer: &Sender<Vec<u8>>,
    cbf: &dyn CbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Payload");
    }
    progress.inc();
    // -------> Sending Payload
    // Get bytes
    let mut payload_bytes = Vec::<u8>::new();
    payload_bytes.extend_from_slice(cbf.read_only_section().content());
    if cbf.data_section().is_some() {
        payload_bytes.extend_from_slice(cbf.data_section().unwrap().content());
    }
    // Generate packet
    let mut pkt = RawPacket::new(&payload_bytes);
    loop {
        // Wait for next request
        let mut buff: [u8; 1] = [0x00; 1];
        channel_read(channel_in_consumer, &mut buff);

        if buff[0] == ComponentUpdateCommand::SendComponentTrailer as u8 {
            // Check we actually finished sending the variable header
            if pkt.get_next_fragment().is_some() {
                eprintln!("Still some payload to be send!");
                return;
            }
            break;
        } else if buff[0] != ComponentUpdateCommand::SendNextFragment as u8 {
            eprintln!(
                "Unexpected response from device at fourth step (Payload) {:?}",
                MessageError::from(buff[0])
            );
            return;
        }
        // Send fragment
        //println!("\tSending Fragment {}/{}", pkt.get_next_fragment_number().unwrap(), pkt.get_total_fragments());
        progress.message(&format!(
            "Payload Fragment {}/{}   ",
            pkt.get_next_fragment_number().unwrap(),
            pkt.get_total_fragments()
        ));
        let fragment_data = pkt.get_next_fragment().unwrap();
        channel_write(channel_out_producer, &fragment_data);
        // Update progress
        progress.add((fragment_data.len() - 1) as u64);
    }
    send_trailer(channel_in_consumer, channel_out_producer, cbf, progress, verbose);
}

fn send_trailer(
    channel_in_consumer: &Receiver<u8>,
    channel_out_producer: &Sender<Vec<u8>>,
    cbf: &dyn CbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Trailer");
    }
    progress.inc();
    // -------> Sending Payload
    // Get bytes
    let mut checksum_bytes = Vec::<u8>::new();
    checksum_bytes.extend_from_slice(&cbf.trailer().checksum().to_le_bytes());
    // Send data
    progress.message("Checksum   ");
    channel_write(channel_out_producer, &checksum_bytes);

    // Wait for confirmation
    let mut buff: [u8; 1] = [0x00; 1];
    channel_read(channel_in_consumer, &mut buff);
    if buff[0] != ComponentUpdateResponse::Success as u8 {
        eprintln!(
            "Unexpected response from device at final step: {:?}",
            MessageError::from(buff[0])
        );
        return;
    }
    progress.finish();

    println!("\nSuccess!");
}

fn extract_variable_header(cbf: &dyn CbfFile) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    // Start with regions
    for r in cbf.region_iter() {
        let raw_data = r.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Next append interrupts
    for i in cbf.interrupt_iter() {
        let raw_data = i.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Next append relocations
    for r in cbf.relocation_iter() {
        let raw_data = r.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Next append dependencies
    for d in cbf.dependency_iter() {
        let raw_data = d.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Then append padding bytes
    for _ in 0..cbf.header_base().padding_bytes() {
        buffer.extend([0xFF]);
    }
    buffer
}

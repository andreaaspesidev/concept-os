mod messages;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use hbf_rs::HbfFile;
use pbr::ProgressBar;
use std::{io::Stdout, path::PathBuf, time::Duration};

use self::messages::*;
use crate::common_messages::*;
use crate::utils::*;

pub fn flash_component(
    mqtt_in_consumer: Receiver<u8>,
    mqtt_out_producer: Sender<Vec<u8>>,
    hbf_file: String,
    verbose: bool,
) {
    if verbose {
        println!("---> Flashing Component");
    }
    // First, check the hbf path
    let hbf_path = PathBuf::from(hbf_file.clone());
    if !hbf_path.exists() {
        eprintln!("Cannot find the HBF at '{}'", hbf_file);
        return;
    }
    // Read the whole hbf in memory (surely small for a PC)
    let read_result = std::fs::read(hbf_path);
    if read_result.is_err() {
        eprintln!("Cannot open the HBF at '{}'", hbf_file);
        return;
    }
    let hbf_bytes = read_result.unwrap();
    // Parse the hbf
    let hbf_result = hbf_rs::parse_hbf(&hbf_bytes);
    if hbf_result.is_err() {
        match hbf_result.unwrap_err() {
            hbf_rs::Error::BufferTooShort | hbf_rs::Error::InvalidMagic => {
                eprintln!("HBF file not valid!")
            }
            hbf_rs::Error::UnsupportedVersion => {
                eprintln!("HBF version still not supported by the tool")
            }
        }
        return;
    }
    let hbf = hbf_result.unwrap();
    // Validate hbf
    if !hbf.validate() {
        eprintln!("HBF file integrity test failed!");
        return;
    }
    // If verbose, print some info
    if verbose {
        println!("\n\tComponent ID: {}", hbf.header_base().component_id());
        println!(
            "\tComponent Version: {}",
            hbf.header_base().component_version()
        );
        println!("\tRequired Flash Size: {}", hbf.header_base().total_size());
        println!(
            "\tRequired SRAM Size: {}",
            hbf.header_main().component_min_ram()
        );
    }
    // Send hello
    println!("");
    let mut progress = ProgressBar::new((hbf.header_base().total_size() + 4) as u64);
    progress.show_speed = false;
    progress.show_counter = false;
    progress.show_time_left = false;
    progress.set_width(Some(80));
    begin_communication(&mqtt_in_consumer, &mqtt_out_producer, &hbf, &mut progress, verbose);
}

fn begin_communication(
    mqtt_in_consumer: &Receiver<u8>,
    mqtt_out_producer: &Sender<Vec<u8>>,
    hbf: &dyn HbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    // Send hello message
    progress.message("Connection Setup   ");
    let hello_msg = HelloMessage::new(OperationType::ComponentUpdate, [0x00; 16]);
    flush_read(mqtt_in_consumer);
    serial_write(mqtt_out_producer, &hello_msg.get_raw());
    // Read hello response
    let mut buff: [u8; HelloResponseMessage::get_size()] = [0x00; HelloResponseMessage::get_size()];
    serial_read(mqtt_in_consumer, &mut buff);
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
    serial_read(mqtt_in_consumer, &mut buff);
    if buff[0] != ComponentUpdateCommand::SendComponentFixedHeader as u8 {
        eprintln!(
            "Unexpected response from device at first step (Fixed Header): {:?}",
            buff[0]
        );
        return;
    }
    send_fixed_header(mqtt_in_consumer, mqtt_out_producer, hbf, progress, verbose);
}

fn send_fixed_header(
    mqtt_in_consumer: &Receiver<u8>,
    mqtt_out_producer: &Sender<Vec<u8>>,
    hbf: &dyn HbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Fixed Header");
    }
    progress.inc();
    // Send fixed header
    let base_header_raw = hbf.header_base().get_raw();
    let main_header_raw = hbf.header_main().get_raw();
    // Combine in a single packet
    let mut out_buff = Vec::<u8>::new();
    out_buff.extend_from_slice(base_header_raw);
    out_buff.extend_from_slice(main_header_raw);
    // Construct packet and send
    progress.message("Header   ");
    let fixed_header_msg = FixedHeaderMessage::new(&out_buff);
    serial_write(mqtt_out_producer, &fixed_header_msg.get_raw());
    // Update progress
    progress.add((out_buff.len() - 1) as u64);
    // Wait for variable header request
    let mut buff: [u8; 1] = [0x00; 1];
    serial_read(mqtt_in_consumer, &mut buff);
    if buff[0] != ComponentUpdateCommand::SendComponentVariableHeader as u8 {
        eprintln!(
            "Unexpected response from device at second step (Variable Header): {:?}",
            buff[0]
        );
        return;
    }
    send_variable_header(mqtt_in_consumer, mqtt_out_producer, hbf, progress, verbose);
}

fn send_variable_header(
    mqtt_in_consumer: &Receiver<u8>,
    mqtt_out_producer: &Sender<Vec<u8>>,
    hbf: &dyn HbfFile,
    progress: &mut ProgressBar<Stdout>,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Variable Header");
    }
    progress.inc();
    // Generate bytes
    let vhb = extract_variable_header(hbf);
    // Start sending
    let mut pkt = RawPacket::new(&vhb);
    loop {
        let mut buff: [u8; 1] = [0x00; 1];
        // Wait for next request
        serial_read(mqtt_in_consumer, &mut buff);

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
                buff[0]
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
        serial_write(mqtt_out_producer, &fragment_data);
        // Update progress
        progress.add((fragment_data.len() - 1) as u64);
    }
    send_payload(mqtt_in_consumer, mqtt_out_producer, hbf, progress, verbose);
}

fn send_payload(
    mqtt_in_consumer: &Receiver<u8>,
    mqtt_out_producer: &Sender<Vec<u8>>,
    hbf: &dyn HbfFile,
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
    payload_bytes.extend_from_slice(hbf.read_only_section().content());
    if hbf.data_section().is_some() {
        payload_bytes.extend_from_slice(hbf.data_section().unwrap().content());
    }
    // Generate packet
    let mut pkt = RawPacket::new(&payload_bytes);
    loop {
        // Wait for next request
        let mut buff: [u8; 1] = [0x00; 1];
        serial_read(mqtt_in_consumer, &mut buff);

        if buff[0] == ComponentUpdateResponse::Success as u8 {
            // Check we actually finished sending the variable header
            if pkt.get_next_fragment().is_some() {
                eprintln!("Still some payload to be send!");
                return;
            }
            break;
        } else if buff[0] != ComponentUpdateCommand::SendNextFragment as u8 {
            eprintln!(
                "Unexpected response from device at fourth step (Payload) {:?}",
                buff[0]
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
        serial_write(mqtt_out_producer, &fragment_data);
        // Update progress
        progress.add((fragment_data.len() - 1) as u64);
    }
    progress.finish();

    println!("\nSuccess!");
}

fn extract_variable_header(hbf: &dyn HbfFile) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();
    // Start with regions
    for r in hbf.region_iter() {
        let raw_data = r.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Next append interrupts
    for i in hbf.interrupt_iter() {
        let raw_data = i.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Next append relocations
    for r in hbf.relocation_iter() {
        let raw_data = r.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    // Lastly append dependencies
    for d in hbf.dependency_iter() {
        let raw_data = d.get_raw();
        buffer.extend_from_slice(raw_data);
    }
    buffer
}

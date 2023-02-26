use std::time::Duration;

use serialport::{SerialPort, TTYPort};

use crate::erase_component::messages::{ComponentEraseCommand, ComponentIDMessage, ComponentEraseResponse};
use crate::{common_messages::*};
use crate::utils::*;

mod messages;


pub fn erase_component(serial_port: String, component_id: u16, component_version: u32, verbose: bool) {
    // Open Serial Port
    let serial_result = serialport::new(&serial_port.clone(), SERIAL_BAUDRATE).open_native();
    if serial_result.is_err() {
        eprintln!(
            "The port '{}' cannot be opened. Check permissions!",
            serial_port
        );
    }
    let mut serial_port = serial_result.unwrap();
    serial_port.set_timeout(Duration::from_secs(5)).unwrap();
    // Send hello
    begin_communication(&mut serial_port, component_id, component_version, verbose);
}

fn begin_communication(serial: &mut TTYPort, component_id: u16, component_version: u32, verbose: bool) {
    // Send hello message
    let hello_msg = HelloMessage::new(OperationType::ComponentErase);
    flush_read(serial);
    serial_write(serial, &hello_msg.get_raw());
    // Read hello response
    let mut buff: [u8; HelloResponseMessage::get_size()] = [0x00; HelloResponseMessage::get_size()];
    serial_read(serial, &mut buff);
    // Validate hello response
    let hello_response = HelloResponseMessage::from(&buff);
    if hello_response.is_err() {
        eprintln!("Wrong response from device at HELLO");
        return;
    }
    if verbose {
        println!("Got HELLO!");
    }
    // Wait for header request
    let mut buff: [u8; 1] = [0x00; 1];
    //flush_read(serial);
    serial_read(serial, &mut buff);
    if buff[0] != ComponentEraseCommand::SendComponentID as u8 {
        eprintln!("Unexpected response from device at first step (SendComponentID): {:?}", buff[0]);
        return;
    }
    send_component_id(serial, component_id, component_version, verbose);
}

fn send_component_id(serial: &mut TTYPort, component_id: u16, component_version: u32, verbose: bool) {
    if verbose {
        println!("--> Send Component ID");
    }
    // Construct packet and send
    let component_id_mgs = ComponentIDMessage::new(component_id, component_version);
    serial_write(serial,&component_id_mgs.get_raw());
    // Wait for result
    let mut buff: [u8; 1] = [0x00; 1];
    serial_read(serial, &mut buff);
    if buff[0] != ComponentEraseResponse::Success as u8 {
        eprintln!("Unexpected response from device: {:?}", buff[0]);
        return;
    }
    println!("Success!");
}
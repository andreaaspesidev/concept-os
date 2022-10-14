mod messages;

use std::time::Duration;

use serialport::{SerialPort, TTYPort};

use crate::common_messages::*;
use crate::utils::*;

use self::messages::ComponentInfoMessage;
use self::messages::ComponentInfoResult;

pub fn info(serial_port: String, verbose: bool) {
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
    begin_communication(&mut serial_port, verbose);
}

fn begin_communication(serial: &mut TTYPort, verbose: bool) {
    // Send hello message
    let hello_msg = HelloMessage::new(OperationType::SystemInfo);
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
    read_system_info(serial);
}

fn read_system_info(serial: &mut TTYPort) {
    println!("----------- System Status -----------");
    let mut at_least_one: bool = false;
    loop {
        // Start two read the first bytes
        let mut buff: [u8; ComponentInfoMessage::max_size()] = [0x00; ComponentInfoMessage::max_size()];
        serial_read(serial, &mut buff[..ComponentInfoMessage::min_size()]);
        let parse_res = ComponentInfoMessage::from(&buff[..ComponentInfoMessage::min_size()]);
        if parse_res.is_err() {
            match parse_res.unwrap_err() {
                ComponentInfoResult::NoMoreComponents => break, // Finished
                ComponentInfoResult::InvalidMessage | ComponentInfoResult::InvalidCRC => panic!("Got invalid message"),
                ComponentInfoResult::NeedMoreBytes => {
                    // Read missing bytes
                    serial_read(serial, &mut buff[ComponentInfoMessage::min_size()..]);
                    // Try to parse now
                    let msg_result = ComponentInfoMessage::from(&buff);
                    if msg_result.is_err() {
                        panic!("Cannot read component status message");
                    }
                    let msg = msg_result.unwrap();
                    println!("{:?}", msg);
                    at_least_one = true;
                },
            }
        }
    }
    if !at_least_one {
        println!("\n\tNo components found!");
    }
    println!("\n----------- ------------- -----------");
}

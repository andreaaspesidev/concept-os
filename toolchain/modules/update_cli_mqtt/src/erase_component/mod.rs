use std::time::Duration;

use crossbeam_channel::{Sender, Receiver};
use crate::common_messages::*;
use crate::erase_component::messages::{
    ComponentEraseCommand, ComponentEraseResponse, ComponentIDMessage,
};
use crate::utils::*;

mod messages;

pub fn erase_component(
    mqtt_in_consumer: Receiver<u8>,
    mqtt_out_producer: Sender<Vec<u8>>,
    component_id: u16,
    component_version: u32,
    verbose: bool,
) {
    // Send hello message
    let hello_msg = HelloMessage::new(OperationType::ComponentErase, [0x00; 16]);
    flush_read(&mqtt_in_consumer);
    serial_write(&mqtt_out_producer, &hello_msg.get_raw());
    // Read hello response
    let mut buff: [u8; HelloResponseMessage::get_size()] = [0x00; HelloResponseMessage::get_size()];
    serial_read(&mqtt_in_consumer, &mut buff);
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
    serial_read(&mqtt_in_consumer, &mut buff);
    if buff[0] != ComponentEraseCommand::SendComponentID as u8 {
        eprintln!(
            "Unexpected response from device at first step (SendComponentID): {:?}",
            buff[0]
        );
        return;
    }
    send_component_id(
        &mqtt_in_consumer,
        &mqtt_out_producer,
        component_id,
        component_version,
        verbose,
    );
}

fn send_component_id(
    mqtt_in_consumer: &Receiver<u8>,
    mqtt_out_producer: &Sender<Vec<u8>>,
    component_id: u16,
    component_version: u32,
    verbose: bool,
) {
    if verbose {
        println!("--> Send Component ID");
    }
    // Construct packet and send
    let component_id_mgs = ComponentIDMessage::new(component_id, component_version);
    serial_write(mqtt_out_producer, &component_id_mgs.get_raw());
    // Wait for result
    let mut buff: [u8; 1] = [0x00; 1];
    serial_read(mqtt_in_consumer, &mut buff);
    if buff[0] != ComponentEraseResponse::Success as u8 {
        eprintln!("Unexpected response from device: {:?}", buff[0]);
        return;
    }
    println!("Success!");
}

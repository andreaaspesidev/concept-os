#![no_std]
#![no_main]

mod crc;
mod messages;
mod utils;
mod update;
mod erase;
mod info;
mod consts;

use userlib::*;
use uart_channel_api::*;

use messages::*;
use update::component_add_update;
use erase::component_erase;
use info::system_info;

#[export_name = "main"]
fn main() -> ! {
    // Listen for the initial packet on serial
    let mut usart = UartChannel::new();

    // Main loop
    loop {
        // Create a buffer where to store the message
        let mut hello_buffer: [u8; HelloMessage::get_size()] =
            [0x00; HelloMessage::get_size()];
        // Read message
        if usart.read_block(&mut hello_buffer).is_ok() {
            // Validate message
            let parsed = HelloMessage::from(&hello_buffer);
            if parsed.is_ok() {
                let msg = parsed.unwrap();
                // Respond to the message
                let response = HelloResponseMessage::new(msg.get_operation());
                hl::sleep_for(500);
                if usart.write_block(&response.get_raw()).is_ok() {
                    // Process this message
                    hello_arrived(&msg, &mut usart).unwrap(); // Ignore errors
                } else {
                    panic!("Cannot write!");
                }
            }
        }
    }
}

fn hello_arrived(
    msg: &HelloMessage,
    usart: &mut UartChannel,
) -> Result<(), MessageError> {
    match msg.get_operation() {
        OperationType::ComponentUpdate => component_add_update(usart),
        OperationType::SystemInfo => system_info(usart),
        OperationType::ComponentErase => component_erase(usart),
    }
}

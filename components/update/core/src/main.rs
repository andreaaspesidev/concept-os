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
use utils::channel_write_single;

#[export_name = "main"]
fn main() -> ! {
    // Wait for state to give time to the old version to terminate cleanly
    let mut state_buff: [u8; 4] = [0; 4];
    hl::get_state(&mut state_buff, (), |s, m: &u32| {
        sys_log!("Got state!");
    });
    // Then activate
    kipc::activate_task();
    // Immediately set the handler
    kipc::set_update_handler(update_handler);
    // Listen for the initial packet on serial
    let mut usart = UartChannel::new();

    // Main loop
    sys_log!("[UPDATE] Hello");
    loop {
        // Create a buffer where to store the message
        let mut hello_buffer: [u8; HelloMessage::get_size()] =
            [0x00; HelloMessage::get_size()];
        // Read message
        if usart.read_block(&mut hello_buffer).is_ok() {
            // Validate message
            sys_log!("[UPDATE] Got hello message");
            let parsed = HelloMessage::from(&hello_buffer);
            if parsed.is_ok() {
                let msg = parsed.unwrap();
                // Respond to the message
                let response = HelloResponseMessage::new(msg.get_operation());
                if usart.write_block(&response.get_raw()).is_ok() {
                    sys_log!("[UPDATE] Wrote hello response");
                    // Process this message
                    hello_arrived(&msg, &mut usart); // Ignore errors
                } else {
                    panic!("Cannot write!");
                }
            }
        } else {
            sys_log!("[UPDATE] Read error");
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

fn update_handler() {
    // If we are here, then we are updating ourselves.
    // Just signal we completed the update
    let mut usart = UartChannel::new();
    channel_write_single(&mut usart, ComponentUpdateResponse::Success as u8);
    // Now transfer some state just to signal we are working correctly
    let mock_state: u32 = 1;
    hl::transfer_state(mock_state);
}
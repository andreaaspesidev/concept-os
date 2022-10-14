#![no_std]
#![no_main]

mod crc;
mod messages;
mod update;
mod utils;
// mod erase;
mod consts;
mod info;

use uart_channel_api::*;
use userlib::*;

use messages::*;
use update::component_add_update;
// use erase::component_erase;
use info::system_info;
use utils::channel_write_single;

#[export_name = "main"]
fn main() -> ! {
    // Wait for state to give time to the old version to terminate cleanly
    let mut state_buff: [u8; 4] = [0; 4];
    if hl::get_state(&mut state_buff, (), |_s, _m: &u32| {}).is_ok() {
        sys_log!("Got state!");
    }
    // Then activate
    kipc::activate_task();
    // Immediately set the handler
    kipc::set_update_support(true);
    // Listen for the initial packet on serial
    let mut usart = UartChannel::new();

    // Main loop
    sys_log!("[UPDATE] Hello");
    loop {
        // Create a buffer where to store the message
        let mut hello_buffer: [u8; HelloMessage::get_size()] = [0x00; HelloMessage::get_size()];
        // Read message
        if usart.read_block(&mut hello_buffer).is_ok() {
            // Validate message
            sys_log!("[UPDATE] Got hello message");
            let parsed = HelloMessage::from(&hello_buffer);
            if parsed.is_ok() {
                let msg = parsed.unwrap_lite();
                // Respond to the message
                let response = HelloResponseMessage::new(msg.get_operation());
                if usart.write_block(&response.get_raw()).is_ok() {
                    sys_log!("[UPDATE] Wrote hello response");
                    // Process this message
                    let result = hello_arrived(&msg, &mut usart);
                    if result.is_err() {
                        error_response(result.unwrap_err(), &mut usart);
                    }
                } else {
                    panic!("Cannot write!");
                }
            }
        } else {
            sys_log!("[UPDATE] Read error");
        }
        // Check for update request
        if kipc::is_state_transfer_requested() {
            update_handler();
        }
    }
}

fn hello_arrived(msg: &HelloMessage, usart: &mut UartChannel) -> Result<(), MessageError> {
    match msg.get_operation() {
        OperationType::ComponentUpdate => component_add_update(usart),
        OperationType::SystemInfo => system_info(usart),
        OperationType::ComponentErase => Err(MessageError::InvalidOperation), //component_erase(usart),
    }
}

fn error_response(error: MessageError, usart: &mut UartChannel) {
    match error {
        MessageError::ChannelError => (), // Ignore
        other => {
            // Write back error on the channel
            channel_write_single(usart, other as u8).ok(); // Ignore error in errors
        }
    }
}

fn update_handler() -> ! {
    // Now transfer some state just to signal we are working correctly
    let mock_state: u32 = 1;
    hl::transfer_state(mock_state);
}

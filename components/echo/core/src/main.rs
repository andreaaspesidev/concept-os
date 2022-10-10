#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    kipc::activate_task();
    let mut serial = uart_channel_api::UartChannel::new();
    let mut buff: [u8; 1] = [0x00; 1];
    sys_log!("[ECHO] Online!");
    loop {
        // Wait for bytes
        serial.read_block(&mut buff).unwrap();
        sys_log!("[ECHO] Got Data!");
        serial.write_block(&buff).unwrap();
    }
}
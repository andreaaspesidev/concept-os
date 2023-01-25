#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    // Main loop
    #[cfg(feature = "board_stm32f303re")]
    let mut buffer = [0; 4];
    loop {
        // Wait for a command
        let rec_result = sys_recv_closed(&mut buffer, 0x0000_0000, TaskId(2));
        if rec_result.is_ok() {
            let message = rec_result.unwrap();
            if message.operation == 0x0001 {
                // Parse size
                let _requested_size = u32::from_be_bytes(buffer);
                sys_reply(TaskId(2), 0x0, &[]);
            } else if message.operation == 0x0002 {
                // Parse allocation address
                let _address = u32::from_be_bytes(buffer);
                sys_reply(TaskId(2), 0x1, &[]);
            }
        }
    }
}

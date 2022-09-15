#![no_std]
#![no_main]

use test_b_api::*;
use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    // Read state
    let mut buffer: [u8; 4] = [0; 4]; // Create enough space for the state. Will be checked!
    let result = hl::get_state(&mut buffer, (), |_state_ref, data: &u32| {
        if *data == 21 {
            sys_log!("[TEST_B] Got state correcly!");
        }
    });
    kipc::activate_task();
    // Register callback for state transfer
    kipc::set_update_handler(update_handler);

    let recv_handler = |op: Operation, msg: hl::Message| -> Result<(), BError> {
        match op {
            Operation::Mock1 => {
                let (msg, caller) = msg.fixed::<Mock1Request, ()>().ok_or(BError::BadArgument)?;
                sys_log!("[TEST_B] Got request");
                // Sleep a bit
                hl::sleep_for(1000);
                sys_log!("[TEST_B] Replyed");
                caller.reply(());
                Ok(())
            }
        }
    };

    let mut buff: [u8; 8] = [0; 8];
    loop {
        hl::recv_without_notification(&mut buff, recv_handler);
    }
}

fn update_handler() {
    sys_log!("[TEST_B] State transfering...");
    let state: u32 = 21;
    hl::transfer_state(state);
}

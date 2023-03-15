// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

use test_b_api::*;
use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    // Read state
    let mut buffer: [u8; 4] = [0; 4]; // Create enough space for the state. Will be checked!
    let _result = hl::get_state(&mut buffer, (), |_state_ref, data: &u32| {
        if *data == 21 {
            sys_log!("[TEST_B] Got state correcly!");
        }
    });
    kipc::activate_task();
    // Register callback for state transfer
    kipc::set_update_support(true);

    let recv_handler = |_state, op: Operation, msg: hl::Message| -> Result<(), BError> {
        match op {
            Operation::Mock1 => {
                let (_msg, caller) = msg.fixed::<Mock1Request, ()>().ok_or(BError::BadArgument)?;
                sys_log!("[TEST_B] Got request");
                // Sleep a bit
                hl::sleep_for(1000);
                sys_log!("[TEST_B] Replied");
                caller.reply(());
                Ok(())
            }
        }
    };

    sys_log!("[TEST_B] Online!");

    let mut buff: [u8; 8] = [0; 8];
    let state: u32 = 21;
    loop {
        hl::recv(
            &mut buff,
            STATE_TRANSFER_REQUESTED_MASK,
            &state,
            |state_ref, _bits| {
                // State transfer requested
                update_handler(*state_ref);
            },
            recv_handler,
        );
    }
}

fn update_handler(state: u32) {
    sys_log!("[TEST_B] State transfering...");
    hl::transfer_state(state);
}

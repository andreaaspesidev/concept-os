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
            Operation::SimpleSend => {
                let (msg, caller) = msg.fixed::<SimpleSendRequest, u32>().ok_or(BError::BadArgument)?;
                sys_log!("[TEST_B] Got request");
                // Sleep a bit
                hl::sleep_for(5);

                sys_log!("[TEST_B] Replied");
                caller.reply(msg.a + msg.b);
                Ok(())
            }
            Operation::SendWithLease => {
                let (msg, caller) = msg.fixed_with_leases::<SendWithLeaseRequest, u32>(2).ok_or(BError::BadArgument)?;
                sys_log!("[TEST_B] Got (lease) request");
                // Sleep a bit
                hl::sleep_for(5);
                // Validate leases
                let incoming = caller.borrow(0);
                let outgoing = caller.borrow(1);
                let incoming_info = incoming.info().ok_or_else(|| BError::BadArgument)?;
                let outgoing_info = outgoing.info().ok_or_else(|| BError::BadArgument)?;
                if !incoming_info.attributes.contains(LeaseAttributes::READ) {
                    return Err(BError::BadArgument);
                }
                if !outgoing_info.attributes.contains(LeaseAttributes::WRITE) {
                    return Err(BError::BadArgument);
                }
                if outgoing_info.len != incoming_info.len {
                    return Err(BError::BadArgument);
                }
                // Perform computation
                for i in 0..incoming_info.len {
                    outgoing.write_at(i, incoming.read_at::<u8>(i).unwrap() + 1).unwrap();
                }
                sys_log!("[TEST_B] Replied (lease)");
                caller.reply(msg.a + msg.b);
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

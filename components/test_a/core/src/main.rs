#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    sys_log!("[TEST_A] Waiting for state");
    // Read state
    let mut buffer: [u8; 4] = [0; 4]; // Create enough space for the state. Will be checked!
    let _result = hl::get_state(&mut buffer, (), |_state_ref, data: &u32| {
        if *data == 20 {
            sys_log!("[TEST_A] Got state correcly!");
        }
    });
    sys_log!("[TEST_A] Activating");
    kipc::activate_task();
    sys_log!("[TEST_A] Setting handler");
    // Register support for state transfer
    kipc::set_update_support(true);
    let mut test_b = test_b_api::TestB::new();

    let state: u32 = 20;
    loop {
        sys_log!("[TEST_A] v1 online");
        hl::sleep_for(100);
        // Ask something to component b
        let response = test_b.simple_send(2, 3);
        if response.is_err() {
            sys_log!("[TEST_A] Request to B failed!");
        } else {
            let result = response.unwrap();
            if result == 5 {
                sys_log!("[TEST_A] Response okay");
            } else {
                sys_log!("[TEST_A] Response wrong");
            }
        }
        // Ask something to component b (lease)
        let out_data: [u8; 2] = [123, 124];
        let mut in_data: [u8; 2] = [0; 2];

        let response = test_b.send_with_lease(2, 3, &out_data, &mut in_data);
        if response.is_err() {
            sys_log!("[TEST_A] Request (lease) to B failed!");
        } else {
            // Check data
            if in_data[0] == out_data[0] + 1 && in_data[1] == out_data[1] + 1 && response.unwrap() == 5 {
                sys_log!("[TEST_A] Response (lease) okay");
            } else {
                sys_log!("[TEST_A] Response (lease) wrong");
            }
        }
        // Check if state transfer is requested
        if kipc::is_state_transfer_requested() {
            update_handler(state);
        }
    }
}

fn update_handler(state: u32) {
    sys_log!("[TEST_A] State transfering...");
    hl::transfer_state(state);
}

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
        hl::sleep_for(5000);
        // Ask something to component b
        if test_b.mock1(2, 3).is_err() {
            sys_log!("Request to B failed!");
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

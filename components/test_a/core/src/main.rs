#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    sys_log!("[TEST_A] Waiting for state");
    // Read state
    let mut buffer: [u8; 4] = [0; 4]; // Create enough space for the state. Will be checked!
    let result = hl::get_state(&mut buffer, (), |_state_ref, data: &u32| {
        if *data == 20 {
            sys_log!("[TEST_A] Got state correcly!");
        }
    });
    sys_log!("[TEST_A] Activating");
    kipc::activate_task();
    sys_log!("[TEST_A] Setting handler");
    // Register callback for state transfer
    kipc::set_update_handler(update_handler);
    let mut test_b = test_b_api::TestB::new();

    loop {
        sys_log!("[TEST_A] v1 online");
        hl::sleep_for(5000);
        // Ask something to component b
        test_b.mock1(2, 3).unwrap();
    }
}

fn update_handler() {
    sys_log!("[TEST_A] State transfering...");
    let state: u32 = 20;
    hl::transfer_state(state);
}

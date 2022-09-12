#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    // Read state
    let mut buffer: [u8; 4] = [0; 4]; // Create enough space for the state. Will be checked!
    let result = hl::get_state(&mut buffer, (), |_state_ref, data: &u32| {
        if *data == 20 {
            sys_log!("Got state correcly!");
        }
    });
    if result.is_err() {
        match result.unwrap_err() {
            hl::StateError::NotAvailable => sys_log!("State not available"),
            hl::StateError::Timeout => sys_log!("State timeout"),
            hl::StateError::BufferTooSmall => sys_log!("State invalid buffer size"),
            hl::StateError::RecvError => sys_log!("State recv error"),
        }
    }
    // Activate
    sys_log!("Activating");
    kipc::activate_task();
    sys_log!("Activated!");
    // Register callback for state transfer
    kipc::set_update_handler(update_handler);
    loop {
        sys_log!("Test v2 online");
        hl::sleep_for(1000);
    }
}

fn update_handler() {
    sys_log!("Hello from update handler!");
    let state: u32 = 21;
    hl::transfer_state(state);
}

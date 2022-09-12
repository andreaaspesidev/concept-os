#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    // Register callback for state transfer
    kipc::set_update_handler(update_handler);
    loop {
        sys_log!("Test v1 online");
        hl::sleep_for(5000);
    }
}

fn update_handler() {
    sys_log!("Hello from update handler!");
    let state: u32 = 20;
    hl::transfer_state(state);
}

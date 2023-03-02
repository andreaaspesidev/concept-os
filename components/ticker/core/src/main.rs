#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    kipc::activate_task();
    sys_log!("[TICKER] Online!");
    loop {
        hl::sleep_for(1);
    }
}
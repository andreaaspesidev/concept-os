#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    kipc::activate_task();
    loop {
        // Wait For Interrupt to pause the processor until an ISR arrives,
        // which could wake some higher-priority task.
        cortex_m::asm::wfi();
    }
}
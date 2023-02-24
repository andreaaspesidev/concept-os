// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

// Make sure we actually link in userlib, despite not using any of it explicitly
// - we need it for our _start routine.

use userlib::task_slot;
extern crate userlib;

task_slot!(CHANNEL, channel);

#[export_name = "main"]
fn main() -> ! {
    // Get instance
    let mut channel = channel_api::UartChannel::new(CHANNEL.get_task_id());
    loop {
        // Wait for a byte
        let mut data: [u8; 1] = [0x00; 1];
        channel.read_block(&mut data).unwrap();
        // Write back
        channel.write_block(&mut data).unwrap();
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

// Make sure we actually link in userlib, despite not using any of it explicitly
// - we need it for our _start routine.

use userlib::{task_slot, sys_log};
extern crate userlib;

task_slot!(CHANNEL, channel);

#[export_name = "main"]
fn main() -> ! {
    // Get instance
    let mut channel = channel_api::UartChannel::new(CHANNEL.get_task_id());

    #[cfg(feature = "channel1")]
    let channel_id: u16 = 1;
    #[cfg(feature = "channel2")]
    let channel_id: u16 = 2;

    loop {
        // Wait for 3 bytes
        let mut data: [u8; 3] = [0x00; 3];

        channel.read_block(channel_id, &mut data).unwrap();
        sys_log!("Got data, now reply!");
        // Write back
        channel.write_block(channel_id, &mut data).unwrap();
    }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    loop {
        // Request scheduling every 1ms
        hl::sleep_for(1);
    }
}

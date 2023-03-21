// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

static mut F1_STR : &'static str = "F1_STR1\n";

use crate::print_utils::print;

#[no_mangle]
#[inline(never)]
pub fn function1() {
    print(unsafe{F1_STR});
    unsafe{F1_STR = "F1_STR2\n"};
    print(unsafe{F1_STR});
}
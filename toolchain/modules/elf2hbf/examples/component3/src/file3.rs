// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

static mut F3_STR1 : &'static str = "F3_STR1\n";
static mut F3_STR2 : &'static str = "F3_STR2\n";

use crate::print_utils::print;

#[no_mangle]
#[inline(never)]
pub fn function3() {
    print(unsafe{F3_STR1});
    print(unsafe{F3_STR2});
    unsafe{
        let tmp = F3_STR2;
        F3_STR2 = F3_STR1;
        F3_STR1 = tmp;
    }
    print(unsafe{F3_STR1});
}
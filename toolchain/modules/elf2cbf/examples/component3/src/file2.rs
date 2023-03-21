// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

static mut F2_STR1 : &'static str = "F2_STR1\n";
static mut F2_STR2 : &'static str = "F2_STR2\n";

use crate::print_utils::print;

#[no_mangle]
#[inline(never)]
pub fn function2() {
    print(unsafe{F2_STR1});
    print(unsafe{F2_STR2});
    unsafe{
        let tmp = F2_STR2;
        F2_STR2 = F2_STR1;
        F2_STR1 = tmp;
    }
    print(unsafe{F2_STR1});
}

static mut F1_STR : &'static str = "F1_STR1\n";

use crate::print_utils::print;

#[no_mangle]
#[inline(never)]
pub fn function1() {
    print(unsafe{F1_STR});
    unsafe{F1_STR = "F1_STR2\n"};
    print(unsafe{F1_STR});
}
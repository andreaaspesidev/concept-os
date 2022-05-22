#![no_std]
#![no_main]

extern crate userlib;

mod file1;
mod file2;
mod file3;
mod print_utils;

use crate::print_utils::print;

#[export_name = "main"]
fn main() -> ! {
    loop {
        example7();
    }
}

static mut TEXT_FIELD : &'static str = "This is a static text 1\n";

#[no_mangle]
#[inline(never)]
fn example7() {
    print(unsafe{TEXT_FIELD});
    unsafe{TEXT_FIELD = "This is a static text 2\n"};
    print(unsafe{TEXT_FIELD});
    file1::function1();
    file2::function2();
    file3::function3();
}
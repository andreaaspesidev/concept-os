#![no_std]
#![no_main]

extern crate userlib;

mod print_utils;
mod macros;

use crate::print_utils::print;

#[export_name = "main"]
fn main() -> ! {
    loop {
        example8();
    }
}

enum MultiTypeEnum {
    V1(u8, u8),
    V2(u32),
    V3(&'static str)
}

static mut MUT_ENUM: MultiTypeEnum = MultiTypeEnum::V1(0,1);

#[no_mangle]
#[inline(never)]
fn example8() {
    unsafe {
        MUT_ENUM = MultiTypeEnum::V2(23);
        MUT_ENUM = MultiTypeEnum::V3("Test String 1");
        print_enum();
        MUT_ENUM = MultiTypeEnum::V3("Test String 2");
        print_enum();
    }
}

#[no_mangle]
#[inline(never)]
fn print_enum() {
    print(unsafe{cast!(MUT_ENUM, MultiTypeEnum::V3)});
}
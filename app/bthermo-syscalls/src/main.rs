// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]
#![feature(asm_const)]

// We have to do this if we don't otherwise use it to ensure its vector table
// gets linked in.
extern crate stm32l476rg;
extern crate panic_itm;

mod clocks;
mod profiling;

use core::arch::asm;

use cortex_m_rt::pre_init;
use cortex_m_rt::entry;
use fugit::RateExtU32;

#[pre_init]
unsafe fn clear_ram() {
    asm!("
        ldr r0,=__sheap
        ldr r1,=_stack_start
        movw r2,#65535
        movt r2,#65535
     0: cmp r1, r0
        beq 1f
        stm r0!, {{r2}}
        b 0b
     1: 
    ");
}

#[entry]
fn main() -> ! {
    const CYCLES_PER_MS: u32 = 80_000;

    // Boost clock frequency
    let rcc = crate::clocks::CFGR::new();

    let _clocks = rcc
        .msi(clocks::MsiFreq::RANGE4M)
        .pll_source(clocks::PllSource::MSI)
        .sysclk(80_u32.MHz())
        .freeze();

    // Configure profiling
    profiling::configure_profiling();
    
    unsafe { kern::startup::start_kernel(CYCLES_PER_MS) }
}

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

mod clocks;
mod profiling;

// We have to do this if we don't otherwise use it to ensure its vector table
// gets linked in.
extern crate stm32l4;

use cortex_m_rt::entry;
use fugit::RateExtU32;

#[entry]
fn main() -> ! {
    const CYCLES_PER_MS: u32 = 80_000;

    // Set the VTOR
    cortex_m::interrupt::free(|_| {
        let p = unsafe{&*cortex_m::peripheral::SCB::PTR};
        unsafe{p.vtor.write(0x08000000)};
    });

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

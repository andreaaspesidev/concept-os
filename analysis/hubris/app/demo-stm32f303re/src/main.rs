// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

// We have to do this if we don't otherwise use it to ensure its vector table
// gets linked in.
extern crate stm32f3;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Default boot speed, until we bother raising it:
    const CYCLES_PER_MS: u32 = 72_000; //8_000;

    // Turn up clock speed to maximum allowed
    let rcc = unsafe { &*stm32f3::stm32f303::RCC::ptr() };
    let flash = unsafe { &*stm32f3::stm32f303::FLASH::ptr() };

    // --> Configure oscillators
    rcc.cr.modify(|_, w| w.hsion().set_bit()); // Turn on internal oscillator (should be already on)
    rcc.cfgr2.modify(|_, w| w.prediv().div1());
    rcc.cfgr.modify(|_, w| w.pllmul().mul9());
    rcc.cfgr.modify(|_, w| w.pllsrc().hsi_div_prediv());
    rcc.cr.modify(|_, w| w.pllon().set_bit());
    // Wait PLL to turn on
    loop {
        if rcc.cr.read().pllrdy().bit_is_set() {
            break;
        }
    }
    // --> Configure clocks
    // Modify flash latency because higher speed
    flash.acr.modify(|_, w| w.latency().ws2());

    // Not that reordering is likely here, since we polled, but: we
    // really do need the Flash to be programmed with more wait states
    // before switching the clock.
    cortex_m::asm::dmb();

    // Apply sys-clock configuration
    rcc.cfgr.modify(|_, w| w.sw().pll());
    // Wait to the modified value to take place
    loop {
        if rcc.cfgr.read().sw().is_pll() {
            break;
        }
    }
    // PCLK1 configuration
    rcc.cfgr.modify(|_, w| w.ppre1().div2());
    // PCLK2 configuration
    rcc.cfgr.modify(|_, w| w.ppre2().div1()); // in HAL << 3

    // TBS, calculate actual clock frequency
    /*const aPLLMULFactorTable: [u8; 16] =
        [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 16];
    const aPredivFactorTable: [u8; 16] =
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    const AHBPrescTable: [u8; 16] =
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];
    let cfgr = rcc.cfgr.read();
    let ppl_mul = aPLLMULFactorTable[cfgr.pllmul().bits() as usize] as u64;
    let ppl_prediv =
        aPredivFactorTable[rcc.cfgr2.read().prediv().bits() as usize] as u64;
    let pll_clk = 8000000u64 / ppl_prediv * ppl_mul;
    let curr_freq =
        pll_clk >> AHBPrescTable[rcc.cfgr.read().hpre().bits() as usize];

    unsafe {
        let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[1];
        cortex_m::iprintln!(stim, "Clock frequency: {}", curr_freq);
    }*/

    unsafe { kern::startup::start_kernel(CYCLES_PER_MS) }
}

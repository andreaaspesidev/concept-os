// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]
#![feature(asm_const)]

// We have to do this if we don't otherwise use it to ensure its vector table
// gets linked in.
extern crate stm32l432kc;
extern crate panic_itm;

mod clocks;

use core::arch::asm;

use cortex_m_rt::pre_init;
use cortex_m_rt::entry;
use fugit::RateExtU32;
use stm32l432kc::device;

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

fn get_sys_clock_freq() -> u32 {
    let rcc = unsafe { &*stm32l432kc::device::RCC::ptr() };
    
    let sysclk_source = rcc.cfgr.read().sws().bits();

    //cortex_m_semihosting::hprintln!("SYSCLK_SRC: {}", sysclk_source);

    let pll_oscsource = rcc.pllcfgr.read().pllsrc().bits();

    //cortex_m_semihosting::hprintln!("PLL_SRC: {}", pll_oscsource);

    let mut msirange: u32 = 0;
    let mut sysclockfreq: u32 = 0;

    const RCC_CFGR_SWS_MSI: u8 = 0;
    const RCC_CFGR_SWS_HSI: u8 = 4;
    const RCC_CFGR_SWS_HSE: u8 = 8;
    const RCC_CFGR_SWS_PLL: u8 = 3; //0xC;
    const RCC_PLLSOURCE_MSI: u8 = 1;
    const RCC_PLLSOURCE_HSI: u8 = 2;
    const RCC_PLLSOURCE_HSE: u8 = 3;

    if (sysclk_source == RCC_CFGR_SWS_MSI) || (sysclk_source == RCC_CFGR_SWS_PLL && pll_oscsource == RCC_PLLSOURCE_MSI) { // MSI
        if rcc.cr.read().bits() & (1 << 3) == 0 {
            msirange = rcc.csr.read().msisrange().bits() as u32;
        } else {
            msirange = rcc.cr.read().msirange().bits() as u32;
        }
        const MSIRangeTable: [u32;12] = [100000,   200000,   400000,   800000,  1000000,  2000000, 
            4000000, 8000000, 16000000, 24000000, 32000000, 48000000];
        msirange = MSIRangeTable[msirange as usize];
        //cortex_m_semihosting::hprintln!("MSI_RANGE: {}", msirange);
        if sysclk_source == 0 { // MSI
            sysclockfreq = msirange;
        }
    } else if sysclk_source == RCC_CFGR_SWS_HSI {
        sysclockfreq = 16000000;
        //cortex_m_semihosting::hprintln!("HSI");
    } else if sysclk_source == RCC_CFGR_SWS_HSE {
        //cortex_m_semihosting::hprintln!("HSE");
        sysclockfreq = 8000000;
    }

    if sysclk_source == RCC_CFGR_SWS_PLL {  // PLL
        let pllsource = rcc.pllcfgr.read().pllsrc().bits();
        let mut pllvco: u32 = 0;
        
        if pllsource == RCC_PLLSOURCE_HSI {
            pllvco = 16000000;
        } else if pllsource == RCC_PLLSOURCE_HSE {
            pllvco = 8000000;
        } else {
            pllvco = msirange;
        }
        let pllm = rcc.pllcfgr.read().pllm().bits() + 1;
        pllvco = pllvco * rcc.pllcfgr.read().plln().bits() as u32 / pllm as u32;
        let pllr = (rcc.pllcfgr.read().pllr().bits() + 1) * 2;
        sysclockfreq = pllvco / pllr as u32;
    }
    return sysclockfreq;
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
    
    //let freq = get_sys_clock_freq();
    //cortex_m_semihosting::hprintln!("Clock Frequency: {}", freq);

    unsafe { kern::startup::start_kernel(CYCLES_PER_MS) }
}

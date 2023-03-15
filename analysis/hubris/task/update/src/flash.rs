// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use stm32l4::stm32l4x6 as device;
use userlib::{sys_irq_control, sys_log, sys_recv_closed, TaskId, UnwrapLite};

const BANK2_BASE: u32 = 0x0808_0000; // Page 256
const FLASH_KEY1: u32 = 0x4567_0123;
const FLASH_KEY2: u32 = 0xCDEF_89AB;
const FLASH_OPT_KEY1: u32 = 0x0819_2A3B;
const FLASH_OPT_KEY2: u32 = 0x4C5D_6E7F;

pub struct Flash {
    flash: &'static device::flash::RegisterBlock,
}

impl Flash {
    pub fn new() -> Self {
        let flash = unsafe { &*device::FLASH::ptr() };
        sys_log!("BFB2: {}", flash.optr.read().bfb2().bit());
        Self::init_flash(flash);
        Self { flash }
    }
    fn init_flash(flash: &'static device::flash::RegisterBlock) {
        // Unlock flash
        flash.keyr.write(|w| unsafe { w.bits(FLASH_KEY1) });
        flash.keyr.write(|w| unsafe { w.bits(FLASH_KEY2) });
        // Assert we unlocked the bank
        assert!(!flash.cr.read().lock().bit());
        // Unlock option bytes
        flash.optkeyr.write(|w| unsafe { w.bits(FLASH_OPT_KEY1) });
        flash.optkeyr.write(|w| unsafe { w.bits(FLASH_OPT_KEY2) });
        assert!(!flash.cr.read().optlock().bit());
    }
    pub fn erase_update_bank(&mut self) {
        sys_irq_control(notifications::FLASH_IRQ_MASK, true);
        // Setup interrupts
        self.flash.cr.modify(|_, w| {
            w.eopie().set_bit().errie().set_bit().rderrie().set_bit()
        });
        // Configure for bank mass erase
        // IMPORTANT: banks here do not switch, so we have to alternate
        sys_log!("OPTR: {}", self.flash.optr.read().bits());
        if self.flash.optr.read().bfb2().bit() {
            sys_log!("Erasing bank 1");
            self.flash.cr.modify(|_, w| w.mer1().set_bit());
        } else {
            sys_log!("Erasing bank 2");
            self.flash.cr.modify(|_, w| w.mer2().set_bit());
        }

        self.flash.cr.modify(|_, w| w.start().set_bit());
        // Wait for EOP notification via interrupt.
        loop {
            sys_recv_closed(
                &mut [],
                notifications::FLASH_IRQ_MASK,
                TaskId::KERNEL,
            )
            .unwrap_lite();
            if self.flash.sr.read().eop().bit() {
                break;
            } else {
                // Spurious wakeup, sleep again
                sys_irq_control(notifications::FLASH_IRQ_MASK, true);
            }
        }
        // Clear values
        self.flash.cr.modify(|_, w| {
            w.mer2().clear_bit().mer1().clear_bit().start().clear_bit()
        });
    }

    fn wait_flash_operation(&mut self) -> Result<(), ()> {
        let mut errors: bool = false;
        // Wait until the operation is completed (reset SR.BSY)
        // (theoretically the program itself can stall)
        loop {
            if self.flash.sr.read().bsy().bit_is_clear() {
                break;
            }
        }
        // Read and clear flags of SR
        self.flash.sr.modify(|r, w| {
            // Reset by writing 1
            if r.eop().bit() {
                w.eop().set_bit(); // Clear bit
            }
            if r.pgserr().bit() {
                errors = true;
                sys_log!("Programming sequence error");
                w.pgserr().set_bit(); // Clear bit
            }
            if r.pgaerr().bit() {
                errors = true;
                sys_log!("Programming alignment error");
                w.pgaerr().set_bit(); // Clear bit
            }
            if r.wrperr().bit() {
                errors = true;
                sys_log!("Write protection error");
                w.wrperr().set_bit(); // Clear bit
            }
            if r.sizerr().bit() {
                errors = true;
                sys_log!("Size error");
                w.sizerr().set_bit(); // Clear bit
            }
            w
        });
        if errors {
            return Err(());
        }
        return Ok(());
    }

    pub fn write_to_update_bank(
        &mut self,
        rel_addr: u32,
        word: u64,
    ) -> Result<(), ()> {
        // Sum the base address
        let base = BANK2_BASE;
        let address = base + rel_addr;
        let mut tries: usize = 0;
        let mut result = Ok(());
        while tries < 10 {
            // Clear old errors
            let _ = self.wait_flash_operation();
            // Set the PG bit
            self.flash.cr.modify(|_, w| w.pg().set_bit());
            unsafe {
                core::ptr::write_volatile(address as *mut u64, word);
            }
            // Wait result of the operation and clear flags
            result = self.wait_flash_operation();
            // Reset PROGRAM flag
            self.flash.cr.modify(|_, w| w.pg().clear_bit());
            if result.is_err() {
                userlib::hl::sleep_for(10);
                tries += 1;
            } else {
                break;
            }
        }

        // Return the result
        result
    }
    
    #[allow(unused)]
    pub fn force_bank1(&mut self) -> Result<(), ()> {
        if self.flash.optr.read().bfb2().bit_is_set() {
            self.flash.optr.modify(|_, w| w.bfb2().clear_bit());
            // In any case, perform the write
            self.flash.cr.modify(|_, w| w.optstrt().set_bit());
            // Wait for completion
            self.wait_flash_operation()?;
            // Clear the bit
            self.flash.cr.modify(|_, w| w.optstrt().clear_bit());
            // Set OBL_LAUNCH
            self.flash.cr.modify(|_,w| w.obl_launch().set_bit());
        }
        Ok(())
    }

    pub fn swap_banks(&mut self) -> Result<(), ()> {
        // Inform the bootloader of the swap
        if self.flash.optr.read().bfb2().bit_is_set() {
            self.flash.optr.modify(|_, w| w.bfb2().clear_bit());
        } else {
            self.flash.optr.modify(|_, w| w.bfb2().set_bit());
        }
        // In any case, perform the write
        self.flash.cr.modify(|_, w| w.optstrt().set_bit());
        // Wait for completion
        self.wait_flash_operation()?;
        // Clear the bit
        self.flash.cr.modify(|_, w| w.optstrt().clear_bit());
        // Return
        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/notifications.rs"));

#![no_std]
#![no_main]

use rcc_api::*;

// STM32F3
#[cfg(feature = "stm32f303re")]
use stm32f303re::device as device;

use userlib::*;
use zerocopy::AsBytes;

// None of the registers we interact with have the same types, and they share no
// useful traits, so we can't extract the bit-setting routine into a function --
// we have no choice but to use macros.
macro_rules! set_bits {
    ($reg:expr, $mask:expr) => {
        $reg.modify(|r, w| unsafe { w.bits(r.bits() | $mask) })
    };
}

// None of the registers we interact with have the same types, and they share no
// useful traits, so we can't extract the bit-clearing routine into a function
// -- we have no choice but to use macros.
macro_rules! clear_bits {
    ($reg:expr, $mask:expr) => {
        $reg.modify(|r, w| unsafe { w.bits(r.bits() & !$mask) })
    };
}

#[export_name = "main"]
fn main() -> ! {
    // Activate task
    kipc::activate_task();
    // From thin air, pluck a pointer to the RCC register block.
    //
    // Safety: this is needlessly unsafe in the API. The RCC is essentially a
    // static, and we access it through a & reference so aliasing is not a
    // concern. Were it literally a static, we could just reference it.
    let rcc = unsafe { &*device::RCC::ptr() };

    // Message handler
    let recv_handler = |op: Operation, msg: hl::Message| -> Result<(), RCCError> {
        match op {
            Operation::EnableClock => {
                // Parse message (the same for all)
                let (msg, caller) =
                    msg.fixed::<EnableClockRequest, ()>().ok_or(RCCError::BadArgument)?;

                let pmask: u32 = 1 << msg.bit;
                let bus = Bus::from_u32(msg.bus).ok_or(RCCError::BadArgument)?;
                // Apply
                match bus {
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB1 => set_bits!(rcc.ahbenr, pmask),
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB2 | Bus::AHB3 => {
                        panic!(); // The function of mapping should be correct
                    },
                    Bus::APB1 => set_bits!(rcc.apb1enr, pmask),
                    Bus::APB2 => set_bits!(rcc.apb2enr, pmask),
                };
                // Respond
                caller.reply(());
                Ok(())
            },
            Operation::DisableClock => {
                // Parse message (the same for all)
                let (msg, caller) =
                    msg.fixed::<DisableClockRequest, ()>().ok_or(RCCError::BadArgument)?;

                let pmask: u32 = 1 << msg.bit;
                let bus = Bus::from_u32(msg.bus).ok_or(RCCError::BadArgument)?;

                // Apply
                match bus {
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB1 => clear_bits!(rcc.ahbenr, pmask),
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB2 | Bus::AHB3 => {
                        panic!(); // The function of mapping should be correct
                    },
                    Bus::APB1 => clear_bits!(rcc.apb1enr, pmask),
                    Bus::APB2 => clear_bits!(rcc.apb2enr, pmask),
                };
                // Respond
                caller.reply(());
                Ok(())
            },
            Operation::EnterReset => {
                // Parse message (the same for all)
                let (msg, caller) =
                    msg.fixed::<EnterResetRequest, ()>().ok_or(RCCError::BadArgument)?;

                let pmask: u32 = 1 << msg.bit;
                let bus = Bus::from_u32(msg.bus).ok_or(RCCError::BadArgument)?;

                // Apply
                match bus {
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB1 => set_bits!(rcc.ahbrstr, pmask),
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB2 | Bus::AHB3 => {
                        panic!(); // The function of mapping should be correct
                    },
                    Bus::APB1 => set_bits!(rcc.apb1rstr, pmask),
                    Bus::APB2 => set_bits!(rcc.apb2rstr, pmask),
                };
                // Respond
                caller.reply(());
                Ok(())
            },
            Operation::LeaveReset => {
                // Parse message (the same for all)
                let (msg, caller) =
                    msg.fixed::<LeaveResetRequest, ()>().ok_or(RCCError::BadArgument)?;

                let pmask: u32 = 1 << msg.bit;
                let bus = Bus::from_u32(msg.bus).ok_or(RCCError::BadArgument)?;

                // Apply
                match bus {
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB1 => clear_bits!(rcc.ahbrstr, pmask),
                    #[cfg(feature = "stm32f303re")]
                    Bus::AHB2 | Bus::AHB3 => {
                        panic!(); // The function of mapping should be correct
                    },
                    Bus::APB1 => clear_bits!(rcc.apb1rstr, pmask),
                    Bus::APB2 => clear_bits!(rcc.apb2rstr, pmask),
                };
                // Respond
                caller.reply(());
                Ok(())
            }
        }
    };

    // Incoming message buffer
    // Ensure our buffer is aligned properly for two u32 by declaring it as one.
    let mut buffer = [0u32; 2];

    // Main loop
    loop {
        // Wait for a command
        // TODO: implement notification as it could be a message from the OS
        hl::recv_without_notification(buffer.as_bytes_mut(), recv_handler);
    }
}
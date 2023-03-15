// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use bstringify;
pub use paste;
use crate::{
    structures::{KHash, TaskIndexes},
    task::Task,
};
use abi::{InterruptOwner, HUBRIS_MAX_IRQS, HUBRIS_MAX_SUPPORTED_TASKS};

#[cfg(feature = "log-enabled")]
use unwrap_lite::UnwrapLite;

cfg_if::cfg_if! {
    if #[cfg(feature = "log-itm")] {
        macro_rules! sys_log {
            ($s:expr) => {
                unsafe {
                    let stim = &mut (*cortex_m::peripheral::ITM::PTR).stim[0];
                    cortex_m::iprintln!(stim, $s);
                }
            };
            ($s:expr, $($tt:tt)*) => {
                unsafe {
                    let stim = &mut (*cortex_m::peripheral::ITM::PTR).stim[0];
                    cortex_m::iprintln!(stim, $s, $($tt)*);
                }
            };
        }
    } else if #[cfg(feature = "log-semihosting")] {
        macro_rules! sys_log {
            ($s:expr) => {
                { let _ = cortex_m_semihosting::hprintln!($s); }
            };
            ($s:expr, $($tt:tt)*) => {
                { let _ = cortex_m_semihosting::hprintln!($s, $($tt)*); }
            };
        }
    } else {
        macro_rules! sys_log {
            ($s:expr) => {};
            ($s:expr, $($x:expr),*$(,)?) => {
                {
                    $(
                        let _ = &$x;
                    )*
                }
            };
        }
    }
}

#[cfg(feature = "log-enabled")]
pub fn log_task(task: &Task) {
    sys_log!(
        "- Component with ID: {} [ident: {}] [orig: {}]",
        task.id(),
        task.current_identifier().0,
        task.descriptor().component_id()
    );
    // Print component regions
    sys_log!("  Regions:");
    for r in task.region_table().into_iter() {
        sys_log!("  -Addr: {:#010x}", r.base);
        sys_log!("   Size: {}", r.size);
        sys_log!("   Attr: {:?}", r.attributes);
    }
    // Print component irqs
    sys_log!("  Interrupts:");
    for interrupt_num in 0..task.descriptor().num_interrupts() {
        let interrupt = task.descriptor().interrupt_nth(interrupt_num);
        sys_log!("  -IRQ: {}", interrupt.irq_num);
        sys_log!("   Mask: {:#010x}", interrupt.notification);
    }
    // Print entrypoint
    sys_log!("  Entrypoint at: {:#010x}", task.descriptor().entry_point());
}

#[cfg(not(feature = "log-enabled"))]
pub fn log_task(_task: &Task) { }

#[cfg(feature = "log-enabled")]
pub fn log_structures(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    irq_map: &mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>,
) {
    // Print components
    for cindex in task_map.indexes().into_iter() {
        log_task(&task_list[*cindex]);
    }
    // Print interrupts
    for irq in irq_map.keys().into_iter() {
        let owner = irq_map.get(*irq).unwrap_lite();
        sys_log!(
            "- IRQ {} mapped to cid {} on bits {:#010x}",
            irq,
            owner.task_id,
            owner.notification
        );
    }
}

#[cfg(not(feature = "log-enabled"))]
pub fn log_structures(
    _task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    _task_map: &mut TaskIndexes,
    _irq_map: &mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>,
) { }

pub(crate) use sys_log;

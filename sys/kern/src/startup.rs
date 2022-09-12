// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Kernel startup.

use heapless::FnvIndexMap;
use unwrap_lite::UnwrapLite;

use crate::{atomic::AtomicExt, utils::log_task};
use crate::structures::populate_kernel_structures;
use crate::sys_log;
use crate::task::Task;
use core::sync::atomic::{AtomicBool, Ordering};

use abi::{InterruptOwner, HUBRIS_MAX_IRQS, HUBRIS_MAX_SUPPORTED_TASKS};

/// Tracks when a mutable reference to the task table is floating around in
/// kernel code, to prevent production of a second one. This forms a sort of
/// ad-hoc Mutex around the task table.
///
/// Notice that this begins life initialized to `true`. This prevents use of
/// `with_task_table` et al before the kernel is properly started. We set it to
/// `false` late in `start_kernel`.
static TASK_TABLE_IN_USE: AtomicBool = AtomicBool::new(true);

pub const HUBRIS_FAULT_NOTIFICATION: u32 = 1;

// These new structures becomes defined in the kernel, instead of being
// generated during build process.

/// Structure of Task structures, sorted by Task ID
static mut TASK_MAP: FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS> =
    FnvIndexMap::new();

/// Structure of IRQs, in order to get to the tasks
pub static mut IRQ_TO_TASK: FnvIndexMap<
    u32,            // IRQ
    InterruptOwner, // Task
    HUBRIS_MAX_IRQS,
> = FnvIndexMap::new();

/// The main kernel entry point.
///
/// We currently expect an application to provide its own `main`-equivalent
/// function, which does basic hardware setup and then calls this function.
///
/// Parameters:
///
/// - `tick_divisor`: a platform-specific way of converting "machine ticks" into
///   "kernel ticks." On ARM M-profile, this is CPU cycles per tick, where a
///   tick is typically a millisecond.
///
/// # Safety
///
/// This function has architecture-specific requirements for safe use -- on ARM,
/// for instance, it must be called from the main (interrupt) stack in
/// privileged mode.
///
/// This function may not be called reentrantly or from multiple cores.
pub unsafe fn start_kernel(tick_divisor: u32) -> ! {
    // Set our clock frequency so debuggers can find it as needed
    //
    // Safety: TODO it is not clear that this operation needs to be unsafe.
    unsafe {
        crate::arch::set_clock_freq(tick_divisor);
    }

    // Load structures from flash
    populate_kernel_structures(unsafe { &mut TASK_MAP }, unsafe {
        &mut IRQ_TO_TASK
    });

    // Get a safe reference
    let task_map = unsafe { &mut TASK_MAP };

    // Debug!
    sys_log!("--------- Kernel Start ----------");
    // Print components
    for (_cid, task) in task_map.iter() {
        log_task(task);
    }
    // Print interrupts
    let interr_map = unsafe { &mut IRQ_TO_TASK };
    for (irq, owner) in interr_map.iter() {
        sys_log!(
            "- IRQ {} mapped to cid {} on bits {:#010x}",
            irq,
            owner.task_id,
            owner.notification
        );
    }

    // With that done, set up initial register state etc.
    for (_, task) in task_map.iter_mut() {
        crate::arch::reinitialize(task);
    }

    // Great! Pick our first task. We'll act like we're scheduling after the
    // last task, which will cause a scan from 0 on.
    let first_task_id = crate::task::select(
        *task_map.keys().last().expect("No Component Loaded"),
        &task_map,
    );

    let first_task = task_map.get_mut(&first_task_id).unwrap_lite();

    // Setup memory protection for this task
    crate::arch::apply_memory_protection(first_task);

    // Mark the table as not in use, so we can now issue `with_task_table`
    TASK_TABLE_IN_USE.store(false, Ordering::Release);

    crate::arch::start_first_task(tick_divisor, first_task)
}

/// Runs `body` with a reference to the task table.
///
/// To preserve uniqueness of the `&mut` reference passed into `body`, this
/// function will detect any attempts to call it recursively and panic.
pub(crate) fn with_task_table<R>(
    body: impl FnOnce(&mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>) -> R,
) -> R {
    if TASK_TABLE_IN_USE.swap_polyfill(true, Ordering::Acquire) {
        panic!(); // recursive use of with_task_table
    }
    // Safety: we have observed `TASK_TABLE_IN_USE` being false, which means the
    // task table is initialized (note that at reset it starts out true) and
    // that we're not already within a call to with_task_table. Thus, we can
    // produce a reference to the task table without aliasing, and we can be
    // confident that the memory it's pointing to is initialized.

    let task_map_ptr = unsafe { &mut TASK_MAP };

    let r = body(task_map_ptr);

    // Mark we are no more using the structure
    TASK_TABLE_IN_USE.store(false, Ordering::Release);

    r
}

pub(crate) fn with_irq_table<R>(
    body: impl FnOnce(&mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>) -> R,
) -> R {
    let irq_map_ptr = unsafe { &mut IRQ_TO_TASK };
    let r = body(irq_map_ptr);
    r
}

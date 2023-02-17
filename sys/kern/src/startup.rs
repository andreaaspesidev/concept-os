// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Kernel startup.

use crate::atomic::AtomicExt;
use crate::log::sys_log;
use crate::structures::{populate_kernel_structures, KHash, TaskIndexes};
use crate::task::Task;
use crate::utils::log_structures;
use core::mem::MaybeUninit;
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
pub const HUBRIS_STORAGE_ANALYZE_NOTIFICATION: u32 = 1;

// These new structures becomes defined in the kernel, instead of being
// generated during build process.

/// Task descriptors
static mut TASK_TABLE: MaybeUninit<[Task; HUBRIS_MAX_SUPPORTED_TASKS]> =
    MaybeUninit::uninit();

/// Map ID -> task index
static mut TASK_MAP: MaybeUninit<TaskIndexes> = MaybeUninit::uninit();

/// Map ID -> interrupt descriptor (mask & owner)
pub static mut IRQ_TO_TASK: MaybeUninit<
    KHash<
        InterruptOwner, // Task
        HUBRIS_MAX_IRQS,
    >,
> = MaybeUninit::uninit();

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
        crate::arch::initialize_native();
    }

    // Initialize structures
    // ---- > Task Table
    // Safety: MaybeUninit<[T]> -> [MaybeUninit<T>] is defined as safe.
    let task_table: &mut [MaybeUninit<Task>; HUBRIS_MAX_SUPPORTED_TASKS] =
        unsafe { &mut *(&mut TASK_TABLE as *mut _ as *mut _) };
    for task in task_table.iter_mut() {
        *task = MaybeUninit::new(Task::default());
    }
    // Safety: we have fully initialized this and can shed the uninit part.
    let task_table: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS] = unsafe { &mut *(task_table as *mut _ as *mut _) };
    
    // ---- > Task Map
    let task_map: &mut MaybeUninit<TaskIndexes> = unsafe {&mut TASK_MAP};
    *task_map = MaybeUninit::new(TaskIndexes::new());
    let task_map: &mut TaskIndexes = unsafe {&mut *(task_map as *mut _ as *mut _)};

    // ---- > IRQs
    let irq_map: &mut MaybeUninit<KHash<InterruptOwner,HUBRIS_MAX_IRQS>> = unsafe {&mut IRQ_TO_TASK};
    *irq_map = MaybeUninit::new(KHash::new());
    let irq_map: &mut KHash<InterruptOwner,HUBRIS_MAX_IRQS> = unsafe {&mut *(irq_map as *mut _ as *mut _)};

    sys_log!("Populating structures...");

    // Load structures from flash
    populate_kernel_structures(
        task_table,
        task_map,
        irq_map,
    );

    // Debug!
    sys_log!("--------- Kernel Start ----------");
    log_structures(task_table, task_map, irq_map);

    // With that done, set up initial register state etc.
    let mask = task_map.indexes_mask();
    for (index, task) in task_table.iter_mut().enumerate() {
        if !mask[index] {
            continue; // Ignore not valid positions
        }
        crate::arch::reinitialize(task);
    }

    // Great! Pick our first task. We'll act like we're scheduling after the
    // last task, which will cause a scan from 0 on.
    let first_task_index = crate::task::select(
        task_map.first_index().expect("No Component Loaded"),
        task_table,
        task_map
    );

    let first_task = &mut task_table[first_task_index];

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
    body: impl FnOnce(
        &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
        &mut TaskIndexes,
    ) -> R,
) -> R {
    if TASK_TABLE_IN_USE.swap_polyfill(true, Ordering::Acquire) {
        panic!(); // recursive use of with_task_table
    }
    // Safety: we have observed `TASK_TABLE_IN_USE` being false, which means the
    // task table is initialized (note that at reset it starts out true) and
    // that we're not already within a call to with_task_table. Thus, we can
    // produce a reference to the task table without aliasing, and we can be
    // confident that the memory it's pointing to is initialized.

    let task_table_ptr = unsafe { TASK_TABLE.assume_init_mut() };
    let task_indexes_ptr = unsafe { TASK_MAP.assume_init_mut() };

    let r = body(task_table_ptr, task_indexes_ptr);

    // Mark we are no more using the structure
    TASK_TABLE_IN_USE.store(false, Ordering::Release);

    r
}

pub(crate) fn with_irq_table<R>(
    body: impl FnOnce(&mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>) -> R,
) -> R {
    let irq_map_ptr = unsafe { IRQ_TO_TASK.assume_init_mut() };
    let r = body(irq_map_ptr);
    r
}

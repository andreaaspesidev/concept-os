use crate::task::Task;

use abi::{HUBRIS_MAX_IRQS, HUBRIS_MAX_SUPPORTED_TASKS, InterruptOwner};
use heapless::FnvIndexMap;
use unwrap_lite::UnwrapLite;

/// Gets a task from the map.
/// Panics if the task id is not present
pub fn get_task_mut(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
) -> &mut Task {
    tasks.get_mut(&caller_id).unwrap_lite()
}

pub fn get_task(
    tasks: &FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
) -> &Task {
    tasks.get(&caller_id).unwrap_lite()
}

#[macro_export]
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

pub fn log_task(task: &Task) {
    sys_log!(
        "- Component with ID: {} [ident: {}] [orig: {}]",
        task.id(),
        task.current_identifier().0,
        task.descriptor().component_id()
    );
    // Print component regions
    sys_log!("  Regions:");
    for r in task.region_table() {
        sys_log!("  -Addr: {:#010x}", r.base);
        sys_log!("   Size: {}", r.size);
        sys_log!("   Attr: {:?}", r.attributes);
    }
}

pub fn log_structures(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
) {
    // Print components
    for (_cid, task) in task_map.iter() {
        log_task(task);
    }
    // Print interrupts
    for (irq, owner) in irq_map.iter() {
        sys_log!(
            "- IRQ {} mapped to cid {} on bits {:#010x}",
            irq,
            owner.task_id,
            owner.notification
        );
    }
}

use crate::task::Task;

use heapless::FnvIndexMap;
use abi::HUBRIS_MAX_SUPPORTED_TASKS;
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
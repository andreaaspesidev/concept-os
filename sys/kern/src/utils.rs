use crate::task::Task;

use heapless::FnvIndexMap;
use abi::HUBRIS_MAX_SUPPORTED_TASKS;
use unwrap_lite::UnwrapLite;

/// Gets a task from the map
/// panics if the task id is not present
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

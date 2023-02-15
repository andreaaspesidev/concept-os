// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implementation of IPC operations on the virtual kernel task.

use abi::{
    FaultInfo, FaultSource, SchedState, UsageError,
    HUBRIS_MAX_SUPPORTED_TASKS,
};
use flash_allocator::flash::FlashMethods;
use unwrap_lite::UnwrapLite;

use crate::err::UserError;
use crate::log::sys_log;
use crate::startup::{with_irq_table};
use crate::structures::{load_component_at, TaskIndexes, self};
use crate::task;
use crate::task::{ArchState, NextTask, Task};
use crate::umem::USlice;

/// Message dispatcher.
pub fn handle_kernel_message(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
) -> Result<NextTask, UserError> {
    // Copy out arguments.
    let caller_index = task_map.get_task_index(caller_id).unwrap_lite();
    let args = task_list[caller_index].save().as_send_args();

    match args.operation {
        1 => read_task_status(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
            args.response?,
        ),
        2 => restart_task(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
        ),
        3 => fault_task(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
        ),
        10 => set_update_capability(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
        ),
        11 => {
            get_state_availability(task_list, task_map, caller_id, caller_index)
        }
        12 => state_transfer_requested(
            task_list,
            task_map,
            caller_id,
            caller_index,
        ),
        20 => activate_task(task_list, task_map, caller_id, caller_index),
        21 => load_component(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
        ),
        31 => flash_write(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
            args.response?,
        ),
        32 => flash_flush_buffer(task_list, task_map, caller_id, caller_index),
        36 => flash_erase(
            task_list,
            task_map,
            caller_id,
            caller_index,
            args.message?,
        ),
        _ => {
            // Task has sent an unknown message to the kernel. That's bad.
            Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
                UsageError::BadKernelMessage,
            )))
        }
    }
}

fn deserialize_message<T>(
    task: &Task,
    message: USlice<u8>,
) -> Result<T, UserError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let (msg, _) = ssmarshal::deserialize(task.try_read(&message)?)
        .map_err(|_| UsageError::BadKernelMessage)?;
    Ok(msg)
}

fn serialize_response<T>(
    task: &mut Task,
    mut buf: USlice<u8>,
    val: &T,
) -> Result<usize, UserError>
where
    T: serde::Serialize,
{
    match ssmarshal::serialize(task.try_write(&mut buf)?, val) {
        Ok(size) => Ok(size),
        Err(ssmarshal::Error::EndOfStream) => {
            // The client provided a response buffer that is too small. We
            // actually tolerate this, and report back the size of a buffer that
            // *would have* worked. It's up to the caller to notice.
            Ok(core::mem::size_of::<T>())
        }
        Err(_) => Err(UsageError::BadKernelMessage.into()),
    }
}

fn read_task_status(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    _caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
    response: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read arguments
    let id: u32 = deserialize_message(&task_list[caller_index], message)?;
    let id = id as u16;

    let other_index = task_map.get_task_index(id);

    if other_index.is_none() {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::TaskNotFound,
        )));
    }
    let other_index = other_index.unwrap_lite();
    // cache other state before taking out a mutable borrow on tasks
    let other_state = *task_list[other_index].state();

    let response_len = serialize_response(
        &mut task_list[caller_index],
        response,
        &other_state,
    )?;
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(0, response_len);
    Ok(NextTask::Same)
}

fn restart_task(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read the message
    let (target_id, start): (u32, bool) =
        deserialize_message(&task_list[caller_index], message)?;
    // Force conversion
    let target_id = target_id as u16;
    // Search the task
    let other_task_index = task_map.get_task_index(target_id);
    if other_task_index.is_none() {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::TaskNotFound,
        )));
    }
    let other_task_index = other_task_index.unwrap_lite();
    let old_identifier = task_list[other_task_index].current_identifier();
    // Reinitialize task
    task_list[other_task_index].reinitialize();
    if start {
        task_list[other_task_index].set_healthy_state(SchedState::Runnable);
    }

    // Restart pending tasks
    task::restart_pending_tasks(task_list, task_map, target_id, old_identifier);

    if target_id == caller_id {
        // Welp, they've restarted themselves. Best not return anything then.
        if !start {
            // And they have asked not to be started, so we can't even fast-path
            // return to their task!
            return Ok(NextTask::Other);
        }
    } else {
        task_list[caller_index]
            .save_mut()
            .set_send_response_and_length(0, 0);
    }
    Ok(NextTask::Same)
}

///
/// Inject a fault into a specified task.  The injected fault will be of a
/// distinct type (`FaultInfo::Injected`) and will contain as a payload the
/// task that injected the fault.  As with restarting, we allow any task to
/// inject a fault into any other task but -- unlike restarting -- we
/// (1) explicitly forbid any fault injection into the supervisor and
/// (2) explicitly forbid any fault injection into the current task (for
/// which the caller should be instead explicitly panicking).
///
fn fault_task(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Parse arguments
    let id: u32 = deserialize_message(&task_list[caller_index], message)?;
    let id = id as u16;
    // Mask out some critical identifiers
    if id == abi::SUPERVISOR_ID || id == caller_id {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }
    // Check if this id actually exists
    let target_task_index = task_map.get_task_index(id);
    if target_task_index.is_none() {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::TaskNotFound,
        )));
    }
    // Inject the fault
    let identifier = task_list[caller_index].current_identifier();
    let _ = crate::task::force_fault(
        task_list,
        task_map,
        id,
        FaultInfo::Injected(identifier),
    );
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(0, 0);

    Ok(NextTask::Same)
}

fn set_update_capability(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    _task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read the handler address
    let state_transfer_support: bool =
        deserialize_message(&task_list[caller_index], message)?;
    sys_log!(
        "Set update support for {}: {}",
        caller_id,
        state_transfer_support
    );
    // Set the new handler
    task_list[caller_index].set_state_transfer_support(state_transfer_support);
    // Respond to the task
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(0, 0);
    // Return to that task
    Ok(NextTask::Same)
}

fn get_state_availability(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize,
) -> Result<NextTask, UserError> {
    // A mature component will never have state to receive
    if caller_id != abi::UPDATE_TEMP_ID {
        task_list[caller_index]
            .save_mut()
            .set_send_response_and_length(0, 0);
        return Ok(NextTask::Same);
    }
    // Otherwise, it depends on whether the old component is active and
    // is willing to transfer state
    let mut state_available: Option<u16> = None; // We will return the current identifier
    let nominal_id = task_list[caller_index].descriptor().component_id();
    // Search for the nominal component ID
    let nominal_index = task_map.get_task_index(nominal_id);
    if nominal_index.is_some() {
        let nominal_task = &task_list[nominal_index.unwrap_lite()];
        if nominal_task.support_state_transfer() {
            state_available = Some(nominal_task.current_identifier().0);
        }
    }
    // Respond to the task
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(
            match state_available {
                Some(id) => id as u32,
                None => 0,
            },
            0,
        );
    return Ok(NextTask::Same);
}

fn state_transfer_requested(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    _task_map: &mut TaskIndexes,
    _caller_id: u16,
    caller_index: usize,
) -> Result<NextTask, UserError> {
    let requested = task_list[caller_index].is_transfer_requested();
    // Respond to the task
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(requested as u32, 0);
    return Ok(NextTask::Same);
}

fn activate_task(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize
) -> Result<NextTask, UserError> {
    // If the task is mature, just ignore the call.
    // In this way, components that does not need the state can
    // just issue this call at the beginning.
    if caller_id != abi::UPDATE_TEMP_ID {
        task_list[caller_index]
            .save_mut()
            .set_send_response_and_length(0, 0);
        return Ok(NextTask::Same);
    }
    sys_log!("Activating update task!");
    // To be sure, schedule another task after this.
    // In fact, the CURRENT_TASK_PTR is now pointing to a wrong memory area
    let mut next_hint = NextTask::Other;
    // Read the nominal id of the task
    let nominal_id = task_list[caller_index].descriptor().component_id();
    // Launch the activation procedure
    let storage_woken = structures::activate_component(task_list, task_map, caller_index, nominal_id);
    if storage_woken {
        next_hint = NextTask::Specific(task_map.get_task_index(abi::STORAGE_ID).unwrap_lite());
    }
    // Alert the task
    let task = &mut task_list[caller_index];
    task.save_mut().set_send_response_and_length(0, 0);
    Ok(next_hint)
}

fn load_component(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    _caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read the block address
    let block_base_address: u32 =
        deserialize_message(&task_list[caller_index], message)?;
    // Try to load the component
    let load_result = with_irq_table(|irq_map| {
        load_component_at(task_list, task_map, irq_map, block_base_address)
    });
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(load_result.is_err() as u32, 0);
    Ok(NextTask::Same)
}

fn flash_write(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
    address: USlice<u8>,
) -> Result<NextTask, UserError> {
    // With an abuse, the message is actually the source buffer, while in the outgoing
    // buffer we can find the destination address. Maybe this choice will be revisited
    // in the future.

    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }
    // Then extract the address
    let address: u32 =
        deserialize_message(&task_list[caller_index], address)?;

    // Check the task can actually reference this location.
    // We threat this as a fault if something is not right
    let task = &task_list[caller_index];
    if !task.can_read(&message) {
        return Err(UserError::Unrecoverable(FaultInfo::MemoryAccess {
            address: Some(message.base_addr() as u32),
            source: FaultSource::Kernel,
        }));
    }

    // Perform the operation
    let source_buffer = unsafe { message.assume_readable() };
    let flash_methods = crate::arch::get_flash_interface();
    let response_code: u32;
    let mut next_task = NextTask::Same;
    match flash_methods.write_timed(task_list, task_map, address, source_buffer) {
        Ok(switch) => {
            response_code = 0;
            next_task = switch;
        }
        Err(_) => response_code = 1,
    }
    // Write the operation response
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    Ok(next_task)
}

fn flash_flush_buffer(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    _task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize
) -> Result<NextTask, UserError> {
    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }

    // Perform the operation
    let flash_methods = crate::arch::get_flash_interface();
    let response_code: u32;
    match flash_methods.flush_write_buffer() {
        Ok(_) => response_code = 0,
        Err(_) => response_code = 1,
    }
    // Write the operation response
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    Ok(NextTask::Same)
}

fn flash_erase(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_id: u16,
    caller_index: usize,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }
    // Then extract the page number
    let page_num: u16 =
        deserialize_message(&task_list[caller_index], message)?;

    // Now use the flash methods to perform this operation
    let flash_methods = crate::arch::get_flash_interface();
    let response_code: u32;
    let mut next_task = NextTask::Same;
    match flash_methods.erase_timed(task_list, task_map, page_num) {
        Ok(switch) => {
            response_code = 0;
            next_task = switch;
        }
        Err(_) => response_code = 1,
    }
    // Write the operation response
    task_list[caller_index]
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    // It's important to check again timing as these operation
    // may have halt the CPU for more than expected.
    Ok(next_task)
}

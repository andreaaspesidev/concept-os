// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implementation of IPC operations on the virtual kernel task.

use abi::{
    FaultInfo, FaultSource, SchedState, TaskId, UsageError,
    HUBRIS_MAX_SUPPORTED_TASKS,
};
use flash_allocator::flash::FlashMethods;
use unwrap_lite::UnwrapLite;

use crate::err::UserError;
use crate::startup::{with_irq_table, HUBRIS_STORAGE_ANALYZE_NOTIFICATION};
use crate::structures::load_component_at;
use crate::task::{ArchState, NextTask, NotificationSet, Task};
use crate::umem::USlice;
use crate::utils::{get_task, get_task_mut};
use crate::task;
use crate::log::sys_log;
use heapless::FnvIndexMap;

/// Message dispatcher.
pub fn handle_kernel_message(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
) -> Result<NextTask, UserError> {
    // Copy out arguments.
    let args = get_task(tasks, caller_id).save().as_send_args();

    match args.operation {
        1 => read_task_status(tasks, caller_id, args.message?, args.response?),
        2 => restart_task(tasks, caller_id, args.message?),
        3 => fault_task(tasks, caller_id, args.message?),
        10 => set_update_capability(tasks, caller_id, args.message?),
        11 => get_state_availability(tasks, caller_id),
        12 => state_transfer_requested(tasks, caller_id),
        20 => activate_task(tasks, caller_id),
        21 => load_component(tasks, caller_id, args.message?),
        30 => flash_read(tasks, caller_id, args.message?, args.response?),
        31 => flash_write(tasks, caller_id, args.message?, args.response?),
        32 => flash_flush_buffer(tasks, caller_id),
        33 => flash_page_from_address(
            tasks,
            caller_id,
            args.message?,
            args.response?,
        ),
        34 => flash_page_from_number(
            tasks,
            caller_id,
            args.message?,
            args.response?,
        ),
        35 => flash_prev_page(tasks, caller_id, args.message?, args.response?),
        36 => flash_erase(tasks, caller_id, args.message?),
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
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
    response: USlice<u8>,
) -> Result<NextTask, UserError> {
    let id: u32 = deserialize_message(get_task(tasks, caller_id), message)?;
    let id = id as u16;

    let other_task_search = tasks.get_mut(&id);

    if other_task_search.is_none() {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::TaskNotFound,
        )));
    }
    // cache other state before taking out a mutable borrow on tasks
    let other_state = *other_task_search.unwrap_lite().state();

    let response_len = serialize_response(
        get_task_mut(tasks, caller_id),
        response,
        &other_state,
    )?;
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(0, response_len);
    Ok(NextTask::Same)
}

fn restart_task(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    let (target_id, start): (u32, bool) =
        deserialize_message(get_task(tasks, caller_id), message)?;
    // Force conversion
    let target_id = target_id as u16;
    // Search the task
    let other_task_search = tasks.get_mut(&target_id);
    if other_task_search.is_none() {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::TaskNotFound,
        )));
    }
    let other_task = other_task_search.unwrap_lite();
    let old_identifier = other_task.current_identifier();
    other_task.reinitialize();
    if start {
        other_task.set_healthy_state(SchedState::Runnable);
    }

    // Restart pending tasks
    task::restart_pending_tasks(tasks, target_id, old_identifier);

    if target_id == caller_id {
        // Welp, they've restarted themselves. Best not return anything then.
        if !start {
            // And they have asked not to be started, so we can't even fast-path
            // return to their task!
            return Ok(NextTask::Other);
        }
    } else {
        get_task_mut(tasks, caller_id)
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
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    let id: u32 = deserialize_message(get_task(tasks, caller_id), message)?;
    let id = id as u16;

    if id == abi::SUPERVISOR_ID || id == caller_id {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }

    let target_task_search = tasks.get(&id);
    if target_task_search.is_none() {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::TaskNotFound,
        )));
    }

    let identifier = get_task(tasks, caller_id).current_identifier();
    let _ =
        crate::task::force_fault(tasks, id, FaultInfo::Injected(identifier));
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(0, 0);

    Ok(NextTask::Same)
}

fn set_update_capability(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read the handler address
    let state_transfer_support: bool =
        deserialize_message(get_task(tasks, caller_id), message)?;
    sys_log!(
        "Set update support for {}: {}",
        caller_id,
        state_transfer_support
    );
    // Set the new handler
    get_task_mut(tasks, caller_id)
        .set_state_transfer_support(state_transfer_support);
    // Respond to the task
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(0, 0);
    // Return to that task
    Ok(NextTask::Same)
}

fn get_state_availability(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
) -> Result<NextTask, UserError> {
    // A mature component will never have state to receive
    if caller_id != abi::UPDATE_TEMP_ID {
        get_task_mut(tasks, caller_id)
            .save_mut()
            .set_send_response_and_length(0, 0);
        return Ok(NextTask::Same);
    }
    // Otherwise, it depends on whether the old component is active and
    // is willing to transfer state
    let mut state_available: Option<u16> = None;
    let nominal_id = get_task(tasks, caller_id).descriptor().component_id();
    // Search for the nominal component ID
    let task = tasks.get(&nominal_id);
    if task.is_some() {
        let task_data = task.unwrap_lite();
        if task_data.support_state_transfer() {
            state_available = Some(task_data.current_identifier().0);
        }
    }
    // Respond to the task
    get_task_mut(tasks, caller_id)
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
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
) -> Result<NextTask, UserError> {
    let requested = get_task(tasks, caller_id).is_transfer_requested();
    // Respond to the task
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(requested as u32, 0);
    return Ok(NextTask::Same);
}

fn activate_task(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
) -> Result<NextTask, UserError> {
    // If the task is mature, just ignore the call.
    // In this way, components that does not need the state can
    // just issue this call at the beginning.
    if caller_id != abi::UPDATE_TEMP_ID {
        get_task_mut(tasks, caller_id)
            .save_mut()
            .set_send_response_and_length(0, 0);
        return Ok(NextTask::Same);
    }
    sys_log!("Activating update task!");
    let mut next_hint = NextTask::Other;
    // Read the nominal id of the task
    let nominal_id = get_task(tasks, caller_id).descriptor().component_id();

    let old_task = tasks.get(&nominal_id);
    let mut old_identifier: Option<TaskId> = None;
    if old_task.is_some() {
        // Before removing, save the generation
        old_identifier = Some(old_task.unwrap_lite().current_identifier());
        // Here we just removed IRQs from the old task, just delete it and use the ID
        // for the new task
        let old_task = tasks.remove(&nominal_id).unwrap_lite();
        // Mark the corresponding block for removal
        unsafe {
            crate::arch::dismiss_block(
                old_task.descriptor().get_descriptor_block(),
            )
            .unwrap_lite();
        }
        // Do not schedule the block for removal here, as we might not have the task
        // yet, if we are updating the storage component
    }

    let mut task = tasks.remove(&abi::UPDATE_TEMP_ID).unwrap_lite();
    // Switch the mode on the task
    task.end_update(old_identifier.map(|id| id.generation().next()));
    // Add again the task
    tasks.insert(nominal_id, task).unwrap_lite();
    // Redirect all IRQs
    with_irq_table(|irq_map| {
        let task = get_task(tasks, nominal_id);
        let tot_irqs = task.descriptor().num_interrupts();
        for interrupt_num in 0..tot_irqs {
            let interrupt = task.descriptor().interrupt_nth(interrupt_num);
            let entry = irq_map.get_mut(&interrupt.irq_num).unwrap_lite();
            entry.task_id = nominal_id;
        }
    });
    if let Some(old_id) = old_identifier {
        // Restart pending tasks
        task::restart_pending_tasks(tasks, nominal_id, old_id);
        // Now is safe to schedule the old block for removal
        let storage_awoken = get_task_mut(tasks, abi::STORAGE_ID)
            .post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
        if storage_awoken {
            next_hint = NextTask::Specific(abi::STORAGE_ID);
        }
    }
    // Alert the task
    let task = get_task_mut(tasks, nominal_id);
    task.save_mut().set_send_response_and_length(0, 0);
    // Finalize block
    crate::arch::finalize_block(task.descriptor().get_descriptor_block())
        .unwrap_lite();
    // To be sure, schedule another task after this.
    // In fact, the CURRENT_TASK_PTR is now pointing to a wrong memory area
    Ok(next_hint)
}

fn load_component(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read the block address
    let block_base_address: u32 =
        deserialize_message(get_task(tasks, caller_id), message)?;
    // Try to load the component
    let load_result = with_irq_table(|irq_map| {
        load_component_at(tasks, irq_map, block_base_address)
    });
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(load_result.is_err() as u32, 0);
    Ok(NextTask::Same)
}

fn flash_read(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
    mut response: USlice<u8>,
) -> Result<NextTask, UserError> {
    // The wanted address is inside the message, while the task expects the response
    // to be written to the buffer pointed by response slice.

    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }
    // Then extract the address
    let address: u32 =
        deserialize_message(get_task(tasks, caller_id), message)?;

    // Check the task can actually write to this location.
    // We threat this as a fault if something is not right
    let task = get_task(tasks, caller_id);
    if !task.can_write(&response) {
        return Err(UserError::Unrecoverable(FaultInfo::MemoryAccess {
            address: Some(response.base_addr() as u32),
            source: FaultSource::Kernel,
        }));
    }

    // Now use the flash methods to perform this operation
    let dest_buffer = unsafe { response.assume_writable() };
    let flash_methods = crate::arch::get_flash_interface();
    let response_code: u32;
    match flash_methods.read(address, dest_buffer) {
        Ok(_) => response_code = 0,
        Err(_) => response_code = 1,
    }
    // Write the operation response
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    Ok(NextTask::Same)
}

fn flash_write(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
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
        deserialize_message(get_task(tasks, caller_id), address)?;

    // Check the task can actually reference this location.
    // We threat this as a fault if something is not right
    let task = get_task(tasks, caller_id);
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
    match flash_methods.write_timed(tasks, address, source_buffer) {
        Ok(switch) => {
            response_code = 0;
            next_task = switch;
        }
        Err(_) => response_code = 1,
    }
    // Write the operation response
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    Ok(next_task)
}

fn flash_flush_buffer(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
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
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    Ok(NextTask::Same)
}

fn flash_page_from_address(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
    response: USlice<u8>,
) -> Result<NextTask, UserError> {
    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }

    // Then extract the address
    let address: u32 =
        deserialize_message(get_task(tasks, caller_id), message)?;

    // Perform the operation
    let flash_methods = crate::arch::get_flash_interface();
    match flash_methods.page_from_address(address) {
        Some(page) => {
            let response_len = serialize_response(
                get_task_mut(tasks, caller_id),
                response,
                &page,
            )?;
            get_task_mut(tasks, caller_id)
                .save_mut()
                .set_send_response_and_length(0, response_len);
        }
        None => get_task_mut(tasks, caller_id)
            .save_mut()
            .set_send_response_and_length(1, 0),
    }
    Ok(NextTask::Same)
}

fn flash_page_from_number(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
    response: USlice<u8>,
) -> Result<NextTask, UserError> {
    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }

    // Then extract the address
    let number: u16 = deserialize_message(get_task(tasks, caller_id), message)?;

    // Perform the operation
    let flash_methods = crate::arch::get_flash_interface();
    match flash_methods.page_from_number(number) {
        Some(page) => {
            let response_len = serialize_response(
                get_task_mut(tasks, caller_id),
                response,
                &page,
            )?;
            get_task_mut(tasks, caller_id)
                .save_mut()
                .set_send_response_and_length(0, response_len);
        }
        None => get_task_mut(tasks, caller_id)
            .save_mut()
            .set_send_response_and_length(1, 0),
    }
    Ok(NextTask::Same)
}

fn flash_prev_page(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
    response: USlice<u8>,
) -> Result<NextTask, UserError> {
    // First validate this task. Only the storage task is allowed to perform these calls
    if caller_id != abi::STORAGE_ID {
        return Err(UserError::Unrecoverable(FaultInfo::SyscallUsage(
            UsageError::IllegalTask,
        )));
    }

    // Then extract the address
    let number: u16 = deserialize_message(get_task(tasks, caller_id), message)?;

    // Perform the operation
    let flash_methods = crate::arch::get_flash_interface();
    match flash_methods.prev_page(number) {
        Some(page) => {
            let response_len = serialize_response(
                get_task_mut(tasks, caller_id),
                response,
                &page,
            )?;
            get_task_mut(tasks, caller_id)
                .save_mut()
                .set_send_response_and_length(0, response_len);
        }
        None => get_task_mut(tasks, caller_id)
            .save_mut()
            .set_send_response_and_length(1, 0),
    }
    Ok(NextTask::Same)
}

fn flash_erase(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
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
        deserialize_message(get_task(tasks, caller_id), message)?;

    // Now use the flash methods to perform this operation
    let flash_methods = crate::arch::get_flash_interface();
    let response_code: u32;
    let mut next_task = NextTask::Same;
    match flash_methods.erase_timed(tasks, page_num) {
        Ok(switch) => {
            response_code = 0;
            next_task = switch;
        }
        Err(_) => response_code = 1,
    }
    // Write the operation response
    get_task_mut(tasks, caller_id)
        .save_mut()
        .set_send_response_and_length(response_code, 0);
    // It's important to check again timing as these operation
    // may have halt the CPU for more than expected.
    Ok(next_task)
}

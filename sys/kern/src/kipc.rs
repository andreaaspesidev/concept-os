// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implementation of IPC operations on the virtual kernel task.

use abi::{
    FaultInfo, Generation, SchedState, TaskState, UsageError,
    HUBRIS_MAX_SUPPORTED_TASKS,
};
use unwrap_lite::UnwrapLite;

use crate::err::UserError;
use crate::startup::{with_irq_table, HUBRIS_STORAGE_ANALYZE_NOTIFICATION};
use crate::structures::load_component_at;
use crate::sys_log;
use crate::task::{ArchState, NextTask, NotificationSet, Task};
use crate::umem::USlice;
use crate::utils::{get_task, get_task_mut};
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
        4 => set_update_handler(tasks, caller_id, args.message?),
        5 => get_state_availability(tasks, caller_id),
        6 => activate_task(tasks, caller_id),
        7 => load_component(tasks, caller_id, args.message?),
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

    // Restarting a task can have implications for other tasks. We don't want to
    // leave tasks sitting around waiting for a reply that will never come, for
    // example. So, make a pass over the task table and unblock anyone who was
    // expecting useful work from the now-defunct task.
    for (id, task) in tasks.iter_mut() {
        // Just to make this a little easier to think about, don't check either
        // of the tasks involved in the restart operation. Neither should be
        // affected anyway.
        if *id == caller_id || *id == target_id {
            continue;
        }

        // We'll skip processing faulted tasks, because we don't want to lose
        // information in their fault records by changing their states.
        if let TaskState::Healthy(sched) = task.state() {
            match sched {
                SchedState::InRecv(Some(peer))
                | SchedState::InSend(peer)
                | SchedState::InReply(peer)
                    if peer == &old_identifier =>
                {
                    // Please accept our sincere condolences on behalf of the
                    // kernel.
                    let code = abi::dead_response_code(peer.generation());

                    task.save_mut().set_error_response(code);
                    task.set_healthy_state(SchedState::Runnable);
                }
                _ => (),
            }
        }
    }

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

fn set_update_handler(
    tasks: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    caller_id: u16,
    message: USlice<u8>,
) -> Result<NextTask, UserError> {
    // Read the handler address
    let handler_address: u32 =
        deserialize_message(get_task(tasks, caller_id), message)?;
    sys_log!("Set update handler for {}", caller_id);
    // Set the new handler
    get_task_mut(tasks, caller_id).set_update_handler(handler_address);
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
    let mut state_available: Option<u16> = None;
    let nominal_id = get_task(tasks, caller_id).descriptor().component_id();
    // Search for the nominal component ID
    let task = tasks.get(&nominal_id);
    if task.is_some() {
        let task_data = task.unwrap_lite();
        if task_data.get_update_handler().is_some() {
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
    let mut next_hint = NextTask::Other;
    // Read the nominal id of the task
    let nominal_id = get_task(tasks, caller_id).descriptor().component_id();

    let old_task = tasks.get(&nominal_id);
    let mut old_generation: Option<Generation> = None;
    if old_task.is_some() {
        // Before removing, save the generation
        old_generation = Some(old_task.unwrap_lite().generation().next());
        // Here we just removed IRQs from the old task, just delete it and use the ID
        // for the new task
        let old_task = tasks.remove(&nominal_id).unwrap_lite();
        // Mark the corresponding block for removal
        unsafe {
            crate::arch::dismiss_block(
                old_task.descriptor().get_descriptor_block(),
            ).unwrap_lite();
        }
        // Immediately schedule the block for removal
        let storage_awoken = get_task_mut(tasks, abi::STORAGE_ID)
            .post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
        if storage_awoken {
            next_hint = NextTask::Specific(abi::STORAGE_ID);
        }
    }

    let mut task = tasks.remove(&abi::UPDATE_TEMP_ID).unwrap_lite();
    // Switch the mode on the task
    task.end_update(old_generation);
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

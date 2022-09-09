// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Implementation of IPC operations on the virtual kernel task.

use abi::{FaultInfo, SchedState, TaskState, UsageError, HUBRIS_MAX_SUPPORTED_TASKS};
use unwrap_lite::UnwrapLite;

use crate::err::UserError;
use crate::task::{ArchState, NextTask, Task};
use crate::umem::USlice;
use crate::utils::{get_task_mut,get_task};
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

    let response_len =
        serialize_response(get_task_mut(tasks, caller_id), response, &other_state)?;
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
        get_task_mut(tasks, caller_id).save_mut().set_send_response_and_length(0, 0);
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

    if id == 0 || id == caller_id {
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
    let _ = crate::task::force_fault(tasks, id, FaultInfo::Injected(identifier));
    get_task_mut(tasks, caller_id).save_mut().set_send_response_and_length(0, 0);

    Ok(NextTask::Same)
}
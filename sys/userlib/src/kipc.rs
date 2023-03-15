// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
// Originally forked from: https://github.com/oxidecomputer/hubris

//! Operations implemented by IPC with the kernel task.
use unwrap_lite::UnwrapLite;
use zerocopy::AsBytes;

use crate::*;

pub fn read_task_status(task_id: u16) -> abi::TaskState {
    // Coerce `task` to a known size (Rust doesn't assume that usize == u32)
    let task_id = task_id as u32;
    let mut response = [0; core::mem::size_of::<abi::TaskState>()];
    let (rc, len) = sys_send(TaskId::KERNEL, 1, task_id.as_bytes(), &mut response, &[]);
    if rc != 0 {
        panic!();
    }
    ssmarshal::deserialize(&response[..len]).unwrap_lite().0
}

pub fn restart_task(task_id: u16, start: bool) {
    // Coerce `task` to a known size (Rust doesn't assume that usize == u32)
    let msg = (task_id as u32, start);
    let mut buf = [0; core::mem::size_of::<(u32, bool)>()];
    ssmarshal::serialize(&mut buf, &msg).unwrap_lite();
    let (rc, _len) = sys_send(TaskId::KERNEL, 2, &mut buf, &mut [], &[]);
    if rc != 0 {
        panic!();
    }
}

pub fn fault_task(task_id: u16) {
    // Coerce `task` to a known size (Rust doesn't assume that usize == u32)
    let task_id = task_id as u32;
    let (rc, _len) = sys_send(TaskId::KERNEL, 3, task_id.as_bytes(), &mut [], &[]);
    if rc != 0 {
        panic!();
    }
}

pub fn set_update_support(is_supported: bool) {
    let (rc, _len) = sys_send(TaskId::KERNEL, 10, is_supported.as_bytes(), &mut [], &[]);
    if rc != 0 {
        panic!();
    }
}

pub fn get_state_availability() -> u16 {
    let (rc, _len) = sys_send(TaskId::KERNEL, 11, &[], &mut [], &[]);
    return rc as u16;
}

pub fn is_state_transfer_requested() -> bool {
    let (rc, _len) = sys_send(TaskId::KERNEL, 12, &[], &mut [], &[]);
    return rc != 0;
}

pub fn activate_task() {
    let (rc, _len) = sys_send(TaskId::KERNEL, 20, &[], &mut [], &[]);
    if rc != 0 {
        panic!();
    }
}

pub fn load_component(block_base_address: u32) -> bool {
    let (rc, _len) = sys_send(
        TaskId::KERNEL,
        21,
        block_base_address.as_bytes(),
        &mut [],
        &[],
    );
    return rc == 0;
}

pub fn write_flash(address: u32, buffer: &[u8]) -> Result<(), ()> {
    let mut addr_buff = address.to_le_bytes();
    let (rc, _len) = sys_send(TaskId::KERNEL, 31, buffer, &mut addr_buff, &[]);
    if rc == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn flash_flush_buffer() -> Result<(), ()> {
    let (rc, _len) = sys_send(TaskId::KERNEL, 32, &[], &mut [], &[]);
    if rc == 0 {
        Ok(())
    } else {
        Err(())
    }
}

pub fn flash_erase(page_num: u16) -> Result<(), ()> {
    let (rc, _len) = sys_send(TaskId::KERNEL, 36, page_num.as_bytes(), &mut [], &[]);
    if rc == 0 {
        Ok(())
    } else {
        Err(())
    }
}
#![no_std]

use core::cell::Cell;

use userlib::{hl, TaskId, FromPrimitive, Lease};
use zerocopy::{AsBytes,FromBytes};

/**
 * Constants
 */
const TEST_B_TASK_ID: TaskId = TaskId(16);

/**
 * Error Type
 */
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum BError {
    BadArgument = 1,
    ComponentUnavailable = 2
}
impl From<u32> for BError {
    fn from(x: u32) -> Self {
        match x {
            1 => BError::BadArgument,
            _ => BError::ComponentUnavailable
        }
    }
}
impl From<BError> for u32 {
    fn from(x: BError) -> Self {
        x as u32
    }
}

/**
 * Operations
 */
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Operation {
    SimpleSend = 1,
    SendWithLease = 2
}

/// Simple Send
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct SimpleSendRequest {
    pub a: u32,
    pub b: u32
}
impl hl::Call for SimpleSendRequest {
    const OP: u16 = Operation::SimpleSend as u16;
    type Response = u32;
    type Err = BError;
}

/// Send With Lease
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct SendWithLeaseRequest {
    pub a: u32,
    pub b: u32
}
impl hl::Call for SendWithLeaseRequest {
    const OP: u16 = Operation::SendWithLease as u16;
    type Response = u32;
    type Err = BError;
}

// API Class
pub struct TestB(Cell<TaskId>);

impl TestB {
    pub fn new() -> Self {
        Self {
            0: Cell::new(TEST_B_TASK_ID)
        }
    }
    pub fn simple_send(&mut self, a: u32, b: u32) -> Result<u32,BError> {
        hl::send_with_retry(&self.0, &SimpleSendRequest{
            a: a,
            b: b
        }, &[])
    }
    pub fn send_with_lease(&mut self, a: u32, b: u32, outgoing: &[u8], incoming: &mut [u8]) -> Result<u32,BError> {
        hl::send_with_retry(&self.0, &SendWithLeaseRequest{
            a: a,
            b: b
        }, &[
            Lease::read_only(outgoing),
            Lease::read_write(incoming)
        ])
    }
}
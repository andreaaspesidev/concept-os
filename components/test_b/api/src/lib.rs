#![no_std]

use core::cell::Cell;

use userlib::{hl, TaskId, FromPrimitive};
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
    Mock1 = 1,
}

/// Enable clock
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct Mock1Request {
    pub a: u32,
    pub b: u32
}
impl hl::Call for Mock1Request {
    const OP: u16 = Operation::Mock1 as u16;
    type Response = ();
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
    pub fn mock1(&mut self, a: u32, b: u32) -> Result<(),BError> {
        hl::send_with_retry(&self.0, &Mock1Request{
            a: a,
            b: b
        }, &[])
    }
}
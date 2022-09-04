#![no_std]

use userlib::{sys_send, Lease, TaskId, FromPrimitive};
use zerocopy::{FromBytes,AsBytes};

/**
 * Constants
 */
const UART_CHANNEL_ID: TaskId = TaskId(3);

/**
 * Error Type
 */
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum ChannelError {
    ChannelBusy = 1,
    BadArgument = 2,
    ReadTimeOut = 3,
    ComponentUnavailable = 4,
}
impl From<u32> for ChannelError {
    fn from(x: u32) -> Self {
        match x {
            1 => ChannelError::ChannelBusy,
            2 => ChannelError::BadArgument,
            3 => ChannelError::ReadTimeOut,
            _ => ChannelError::ComponentUnavailable,
        }
    }
}
impl From<ChannelError> for u32 {
    fn from(x: ChannelError) -> Self {
        x as u32
    }
}

/**
 * Operations
 */
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Operation {
    WriteBlock = 1,
    ReadBlock = 2,
    ReadBlockTimed = 3,
}

// ReadBlockTimed
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadBlockTimedRequest {
    pub timeout_ticks: u32
}

/**
 * Single transmitter - Receiver interface
 * for USART2
 */
pub struct UartChannel();

impl UartChannel {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn write_block(&mut self, data: &[u8]) -> Result<(), ChannelError> {
        let (code,_) = sys_send(
            UART_CHANNEL_ID,
            Operation::WriteBlock as u16,
            &[],
            &mut [],
            &[Lease::read_only(data)],
        );
        if code == 0 {
            Ok(())
        } else {
            return Err(ChannelError::from(code));
        }
    }
    pub fn read_block(&mut self, data: &mut [u8]) -> Result<(), ChannelError> {
        let (code,_) = sys_send(
            UART_CHANNEL_ID,
            Operation::ReadBlock as u16,
            &[],
            &mut [],
            &[Lease::write_only(data)],
        );
        if code == 0 {
            Ok(())
        } else {
            return Err(ChannelError::from(code));
        }
    }
    pub fn read_block_timed(&mut self, data: &mut [u8], timeout_ticks: u32) -> Result<(), ChannelError> {
        let message = &ReadBlockTimedRequest{
            timeout_ticks: timeout_ticks
        };
        let (code,_) = sys_send(
            UART_CHANNEL_ID,
            Operation::ReadBlockTimed as u16,
            message.as_bytes(),
            &mut [],
            &[Lease::write_only(data)],
        );
        if code == 0 {
            Ok(())
        } else {
            return Err(ChannelError::from(code));
        }
    }
}

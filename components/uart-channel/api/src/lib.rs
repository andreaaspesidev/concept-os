#![no_std]

use userlib::{sys_send, FromPrimitive, Lease, TaskId};
use zerocopy::{AsBytes, FromBytes};

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
    TransmitTimed = 4,
}

// ReadBlockTimed
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadBlockTimedRequest {
    pub timeout_ticks: u32,
}

// TransmitTimed
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct TransmitTimedRequest {
    pub timeout_ticks: u32,
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
        let (code, _) = sys_send(
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
        let (code, _) = sys_send(
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
    pub fn read_block_timed(
        &mut self,
        data: &mut [u8],
        timeout_ticks: u32,
    ) -> Result<(), ChannelError> {
        let message = &ReadBlockTimedRequest {
            timeout_ticks: timeout_ticks,
        };
        let (code, _) = sys_send(
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
    /// New method that allow transmitting data while first setting up the system for reception.
    /// This is especially useful when dealing with quick responses, that could be missed for
    /// unlucky context switches that delay the setup of the standard reception buffer
    pub fn transmit_timed(
        &mut self,
        data_out: &[u8],
        data_in: &mut [u8],
        timeout_ticks: u32,
    ) -> Result<(), ChannelError> {
        let message = &TransmitTimedRequest {
            timeout_ticks: timeout_ticks,
        };
        let (code, _) = sys_send(
            UART_CHANNEL_ID,
            Operation::TransmitTimed as u16,
            message.as_bytes(),
            &mut [],
            &[Lease::read_only(data_out), Lease::write_only(data_in)],
        );
        if code == 0 {
            Ok(())
        } else {
            return Err(ChannelError::from(code));
        }
    }
}

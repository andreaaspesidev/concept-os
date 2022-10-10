#![no_std]

use core::cell::Cell;

use userlib::{hl, FromPrimitive, Lease, TaskId};
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

#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct WriteBlockRequest {}

impl hl::Call for WriteBlockRequest {
    const OP: u16 = Operation::WriteBlock as u16;
    type Response = ();
    type Err = ChannelError;
}

#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadBlockRequest {}

impl hl::Call for ReadBlockRequest {
    const OP: u16 = Operation::ReadBlock as u16;
    type Response = ();
    type Err = ChannelError;
}

// ReadBlockTimed
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadBlockTimedRequest {
    pub timeout_ticks: u32,
}
impl hl::Call for ReadBlockTimedRequest {
    const OP: u16 = Operation::ReadBlockTimed as u16;
    type Response = ();
    type Err = ChannelError;
}

// TransmitTimed
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct TransmitTimedRequest {
    pub timeout_ticks: u32,
}
impl hl::Call for TransmitTimedRequest {
    const OP: u16 = Operation::TransmitTimed as u16;
    type Response = ();
    type Err = ChannelError;
}

/**
 * Multiple transmitters - Receivers interface
 * for USART
 */
pub struct UartChannel(Cell<TaskId>);

impl UartChannel {
    pub fn new() -> Self {
        Self {
            0: Cell::new(UART_CHANNEL_ID),
        }
    }

    pub fn write_block(&mut self, data: &[u8]) -> Result<(), ChannelError> {
        hl::send_with_retry(&self.0, &WriteBlockRequest {}, &[Lease::read_only(data)])
    }
    pub fn read_block(&mut self, data: &mut [u8]) -> Result<(), ChannelError> {
        hl::send_with_retry(&self.0, &ReadBlockRequest {}, &[Lease::write_only(data)])
    }
    pub fn read_block_timed(
        &mut self,
        data: &mut [u8],
        timeout_ticks: u32,
    ) -> Result<(), ChannelError> {
        hl::send_with_retry(
            &self.0,
            &ReadBlockTimedRequest {
                timeout_ticks: timeout_ticks,
            },
            &[Lease::write_only(data)],
        )
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
        hl::send_with_retry(
            &self.0,
            &TransmitTimedRequest {
                timeout_ticks: timeout_ticks,
            },
            &[Lease::read_only(data_out), Lease::write_only(data_in)],
        )
    }
}

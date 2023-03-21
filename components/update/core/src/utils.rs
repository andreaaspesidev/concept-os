// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::consts::*;
use crate::messages::MessageError;
use cbf_lite::BufferReader;
use storage_api::*;
use uart_channel_api::*;

/**
 * Flash Reader
 */
pub struct FlashReader {
    base_addr: u32,
    size: u32,
}

impl FlashReader {
    pub fn from(base_addr: u32, size: u32) -> Self {
        Self {
            base_addr: base_addr,
            size: size,
        }
    }
}

impl<'a> BufferReader<'a> for FlashReader {
    fn read(&self, offset: u32, dest: &mut [u8]) -> Result<(), cbf_lite::CbfError> {
        // Check offset
        if offset >= self.size {
            return Err(cbf_lite::CbfError::ReadError);
        }
        // Ask to read to storage
        let storage = Storage::new();
        if storage.read_stream(self.base_addr, offset, dest).is_err() {
            return Err(cbf_lite::CbfError::ReadError);
        }
        Ok(())
    }
}

/**
 * Channel functions
 */
#[cfg(feature = "multi-support")]
pub fn channel_write_single(channel: &mut UartChannel, value: u8) -> Result<(), MessageError> {
    let buffer: [u8; 1] = [value; 1];
    Ok(channel
        .write_block(CHANNEL_ID, &buffer)
        .map_err(|_| MessageError::ChannelError)?)
}
#[cfg(not(feature = "multi-support"))]
pub fn channel_write_single(channel: &mut UartChannel, value: u8) -> Result<(), MessageError> {
    let buffer: [u8; 1] = [value; 1];
    Ok(channel
        .write_block(&buffer)
        .map_err(|_| MessageError::ChannelError)?)
}
#[cfg(feature = "multi-support")]
pub fn channel_write(channel: &mut UartChannel, buff: &[u8]) -> Result<(), MessageError> {
    Ok(channel
        .write_block(CHANNEL_ID, &buff)
        .map_err(|_| MessageError::ChannelError)?)
}
#[cfg(not(feature = "multi-support"))]
pub fn channel_write(channel: &mut UartChannel, buff: &[u8]) -> Result<(), MessageError> {
    Ok(channel
        .write_block(&buff)
        .map_err(|_| MessageError::ChannelError)?)
}
#[cfg(feature = "multi-support")]
pub fn channel_ask(
    channel: &mut UartChannel,
    cmd: u8,
    buffer: &mut [u8],
) -> Result<(), MessageError> {
    let buffer_out: [u8; 1] = [cmd; 1];
    Ok(channel
        .transmit_timed(CHANNEL_ID, &buffer_out, buffer, READ_TIMEOUT_TICKS)
        .map_err(|e| {
            if e == ChannelError::ReadTimeOut {
                return MessageError::TimeoutError;
            }
            return MessageError::ChannelError;
        })?)
}
#[cfg(not(feature = "multi-support"))]
pub fn channel_ask(
    channel: &mut UartChannel,
    cmd: u8,
    buffer: &mut [u8],
) -> Result<(), MessageError> {
    let buffer_out: [u8; 1] = [cmd; 1];
    Ok(channel
        .transmit_timed(&buffer_out, buffer, READ_TIMEOUT_TICKS)
        .map_err(|e| {
            if e == ChannelError::ReadTimeOut {
                return MessageError::TimeoutError;
            }
            return MessageError::ChannelError;
        })?)
}

pub fn u32_from_le_bytes(buff: &[u8]) -> u32 {
    return buff[0] as u32
        | ((buff[1] as u32) << 8)
        | ((buff[2] as u32) << 16)
        | ((buff[3] as u32) << 24);
}

pub fn wrap_cbf_error<T>(r: Result<T, cbf_lite::CbfError>) -> Result<T, MessageError> {
    r.map_err(|_| {
        return MessageError::CannotReadCBF;
    })
}

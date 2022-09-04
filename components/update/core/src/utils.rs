use hbf_lite::BufferReader;
use crate::consts::*;
use crate::messages::{TIMEOUT_ERROR, MessageError};
use uart_channel_api::*;
use storage_api::*;
use userlib::hl::sleep_for;

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
    fn read(
        &self,
        offset: u32,
        dest: &mut [u8],
    ) -> Result<(), hbf_lite::HbfError> {
        // Check offset
        if offset >= self.size {
            return Err(hbf_lite::HbfError::ReadError);
        }
        // Ask to read to storage
        let storage = Storage::new();
        if storage.read_stream(self.base_addr, offset, dest).is_err() {
            return Err(hbf_lite::HbfError::ReadError);
        }
        Ok(())
    }
}

/**
 * Minimum
 */
pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

/**
 * Channel functions
 */
pub fn channel_write_single(
    channel: &mut UartChannel,
    value: u8,
) -> Result<(), MessageError> {
    let buffer: [u8; 1] = [value; 1];
    sleep_for(100);
    Ok(channel
        .write_block(&buffer)
        .map_err(|_| MessageError::ChannelError)?)
}

pub fn channel_write(
    channel: &mut UartChannel,
    buff: &[u8],
) -> Result<(), MessageError> {
    sleep_for(100);
    Ok(channel
        .write_block(&buff)
        .map_err(|_| MessageError::ChannelError)?)
}

pub fn channel_read(
    channel: &mut UartChannel,
    buffer: &mut [u8],
) -> Result<(), MessageError> {
    Ok(channel
        .read_block_timed(buffer, READ_TIMEOUT_TICKS)
        .map_err(|e| {
            if e == ChannelError::ReadTimeOut {
                channel_write_single(channel, TIMEOUT_ERROR);
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


pub fn wrap_hbf_error<T, E>(r: Result<T, E>) -> Result<T, MessageError> {
    r.map_err(|_| {
        return MessageError::InvalidHBF;
    })
}
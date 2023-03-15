// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

use crate::crc::crc8_update;

const PACKET_BUFFER_SIZE: usize = 64;

#[derive(Clone, Copy, Debug)]
pub enum MessageError {
    InvalidSize,
    InvalidCRC,
    InvalidOperation,
    CannotReadHBF,
    NotEnoughSpace,
    FlashError,
    TimeoutError,
    FailedHBFValidation,
    DependencyError,
    MissingDependency,
    IllegalDowngrade,
    //CannotFindComponent = 0xEC,
    //CannotFindVersion = 0xED,
}

impl From<u8> for MessageError {
    fn from(x: u8) -> Self {
        match x {
            0xE1 => Self::InvalidSize,
            0xE2 => Self::InvalidCRC,
            0xE3 => Self::InvalidOperation,
            0xE4 => Self::CannotReadHBF,
            0xE5 => Self::NotEnoughSpace,
            0xE6 => Self::FlashError,
            0xE7 => Self::TimeoutError,
            0xE8 => Self::FailedHBFValidation,
            0xE9 => Self::DependencyError,
            0xEA => Self::MissingDependency,
            0xEB => Self::IllegalDowngrade,
            _ => panic!("Unknown response"),
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

pub trait SerializableMessage<'a> {
    /// Returns an array containing the message as bytes
    fn get_raw(&self) -> Vec<u8>;
}

pub trait FragmentedMessage<'a> {
    fn get_next_fragment(&mut self) -> Option<Vec<u8>>;
    fn get_next_fragment_number(&self) -> Option<usize>;
    fn get_total_fragments(&self) -> usize;
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OperationType {
    ComponentUpdate = 0xCA,
    SystemInfo = 0xCB,
    ComponentErase = 0xCE,
}
impl TryFrom<u8> for OperationType {
    type Error = MessageError;
    fn try_from(value: u8) -> Result<Self, MessageError> {
        match value {
            0xCA => Ok(OperationType::ComponentUpdate),
            0xCB => Ok(OperationType::SystemInfo),
            0xCE => Ok(OperationType::ComponentErase),
            _ => Err(MessageError::InvalidOperation),
        }
    }
}

pub struct HelloMessage {
    operation: OperationType,
}

impl HelloMessage {
    pub fn new(operation: OperationType) -> Self {
        Self {
            operation: operation,
        }
    }
}

impl<'a> SerializableMessage<'a> for HelloMessage {
    fn get_raw(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        buffer.push(self.operation as u8);
        // Compute and append crc
        let mut crc: u8 = 0x00;
        for i in 0..buffer.len() {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer.push(crc);
        buffer
    }
}

pub struct HelloResponseMessage<> {}

impl<'a> HelloResponseMessage<> {
    pub fn from(buffer: &'a [u8]) -> Result<Self, MessageError> {
        // Validate buffer
        Self::validate(buffer)?;
        // Return instance
        Ok(Self {})
    }
    pub const fn get_size() -> usize {
        7
    }
    fn validate(buffer: &'a [u8]) -> Result<OperationType, MessageError> {
        // Check message size
        if buffer.len() != Self::get_size() {
            return Err(MessageError::InvalidSize);
        }
        // Check static data
        if buffer[0] != 'O' as u8
            || buffer[1] != 'L' as u8
            || buffer[2] != 'L' as u8
            || buffer[3] != 'E' as u8
            || buffer[4] != 'H' as u8
        {
            return Err(MessageError::InvalidOperation);
        }
        // Check OP
        let op = OperationType::try_from(buffer[5])?;
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(MessageError::InvalidCRC);
        }
        // Return
        Ok(op)
    }
}

pub struct RawPacket<'a> {
    buffer: &'a [u8],
    current_pos: usize,
}

impl<'a> RawPacket<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer: buffer,
            current_pos: 0,
        }
    }
}

impl<'a> FragmentedMessage<'a> for RawPacket<'a> {
    fn get_next_fragment(&mut self) -> Option<Vec<u8>> {
        // Check if we already consumed the whole buffer
        if self.current_pos >= self.buffer.len() {
            return None;
        }
        // Otherwise extract the new fragment
        let fragment_size =
            core::cmp::min(PACKET_BUFFER_SIZE, self.buffer.len() - self.current_pos);
        let fragment = &self.buffer[self.current_pos..self.current_pos + fragment_size];
        // Append to buffer
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(fragment);
        // Calculate crc
        let mut crc: u8 = 0x00;
        for i in 0..fragment_size {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer.push(crc);
        // Go next in the message
        self.current_pos += fragment_size;
        // Return data
        Some(buffer)
    }

    fn get_next_fragment_number(&self) -> Option<usize> {
        // Check if we already consumed the whole buffer
        if self.current_pos >= self.buffer.len() {
            return None;
        }
        Some(self.current_pos / PACKET_BUFFER_SIZE)
    }

    fn get_total_fragments(&self) -> usize {
        self.buffer.len() / PACKET_BUFFER_SIZE
    }
}

use std::fmt;

use crate::crc::crc8_update;

const PACKET_BUFFER_SIZE: usize = 64;

#[derive(Clone, Copy, Debug)]
pub enum MessageError {
    FlashError,
    InvalidPacket
}

impl From<u8> for MessageError {
    fn from(x: u8) -> Self {
        match x {
            0xE1 => Self::FlashError,
            0xE2 => Self::InvalidPacket,
            a => panic!("Unknown response: {}", a),
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
        let fragment = &self.buffer[self.current_pos..(self.current_pos + fragment_size)];
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

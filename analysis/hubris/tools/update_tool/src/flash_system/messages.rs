// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common_messages::SerializableMessage;
use crate::crc::crc8_update;

#[repr(u8)]
pub enum UpdateMessages {
    SendSectionHeader = 0x01,
    SendNextFragment = 0x02,
    Success = 0xFF
}

pub struct CommandStartMessage {
    total_size: u32
}

impl CommandStartMessage {
    pub fn new(total_size: u32) -> Self {
        Self {
            total_size: total_size
        }
    }
}

impl<'a> SerializableMessage<'a> for CommandStartMessage {
    fn get_raw(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        // Append total size
        let size_bytes = self.total_size.to_le_bytes();
        for i in 0..size_bytes.len() {
            buffer.push(size_bytes[i]);
        }
        // Compute and append crc
        let mut crc: u8 = 0x00;
        for i in 0..buffer.len() {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer.push(crc);
        buffer
    }
}


pub struct SectionHeaderMessage {
    address_relative: u32,
    size: u32
}

impl SectionHeaderMessage {
    pub fn new(address_relative: u32,size: u32) -> Self {
        Self {
            address_relative: address_relative,
            size: size
        }
    }
}

impl<'a> SerializableMessage<'a> for SectionHeaderMessage {
    fn get_raw(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        // Append address_relative
        let address_relative_bytes = self.address_relative.to_le_bytes();
        for i in 0..address_relative_bytes.len() {
            buffer.push(address_relative_bytes[i]);
        }
        let size_bytes = self.size.to_le_bytes();
        for i in 0..size_bytes.len() {
            buffer.push(size_bytes[i]);
        }
        // Compute and append crc
        let mut crc: u8 = 0x00;
        for i in 0..buffer.len() {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer.push(crc);
        buffer
    }
}
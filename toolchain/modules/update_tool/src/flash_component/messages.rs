// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{common_messages::SerializableMessage, crc::crc8_update};

#[repr(u8)]
pub enum ComponentUpdateCommand {
    SendComponentFixedHeader = 0x01,
    SendComponentVariableHeader = 0x02,
    SendComponentPayload = 0x03,
    SendComponentTrailer = 0x04,
    SendNextFragment = 0xA0
}

#[repr(u8)]
pub enum ComponentUpdateResponse {
    Success = 0xFF
}

pub struct FixedHeaderMessage<'a> {
    buffer: &'a [u8]
}

impl<'a> FixedHeaderMessage<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        if buffer.len() != cbf_rs::FIXED_HEADER_SIZE {
            panic!();
        }
        // Return instance
        Self {
            buffer: buffer
        }
    }
}

impl<'a> SerializableMessage<'a> for FixedHeaderMessage<'a> {
    fn get_raw(&self) -> Vec<u8> {
        // Construct buffer
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(self.buffer);
        // Calculate CRC
        let mut crc = 0x00;
        for i in 0..self.buffer.len() {
            crc8_update(&mut crc, self.buffer[i]);
        }
        buffer.push(crc);
        buffer
    }
}


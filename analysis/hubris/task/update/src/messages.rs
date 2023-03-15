// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use userlib::UnwrapLite;

use crate::crc::crc8_update;

#[derive(Debug)]
#[repr(u8)]
pub enum UpdateMessages {
    SendSectionHeader = 0x01,
    SendNextFragment = 0x02,
    Success = 0xFF
}

#[derive(Debug)]
#[repr(u8)]
pub enum UpdateErrors {
    FlashError = 0xE1,
    InvalidPacket = 0xE2,
    Timeout = 0xE3
}

pub struct CommandStartMessage {
    total_size: u32
}

impl<'a> CommandStartMessage {
    pub fn from(buffer: &'a [u8]) -> Result<Self, UpdateErrors> {
        // Validate buffer
        let total_size = Self::validate(buffer)?;
        // Return instance
        Ok(Self {
            total_size: total_size,
        })
    }
    pub const fn get_size() -> usize {
        4+1
    }
    fn validate(buffer: &'a [u8]) -> Result<u32, UpdateErrors> {
        // Check message size
        if buffer.len() != Self::get_size() {
            return Err(UpdateErrors::InvalidPacket);
        }
        // Get size
        let total_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap_lite());
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(UpdateErrors::InvalidPacket);
        }
        // Return
        Ok(total_size)
    }
    pub fn get_image_size(&self) -> u32 {
        self.total_size
    }
}

pub struct HeaderMessage {
    rel_base_address: u32,
    size: u32
}

impl<'a> HeaderMessage {
    pub fn from(buffer: &'a [u8]) -> Result<Self, UpdateErrors> {
        // Validate buffer
        let (rel_base, size) = Self::validate(buffer)?;
        // Return instance
        Ok(Self {
            rel_base_address: rel_base,
            size: size
        })
    }
    pub const fn get_size() -> usize {
        4+4+1
    }
    fn validate(buffer: &'a [u8]) -> Result<(u32,u32), UpdateErrors> {
        // Check message size
        if buffer.len() != Self::get_size() {
            return Err(UpdateErrors::InvalidPacket);
        }
        // Get data
        let rel_base_addr = u32::from_le_bytes(buffer[0..4].try_into().unwrap_lite());
        let size = u32::from_le_bytes(buffer[4..8].try_into().unwrap_lite());
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(UpdateErrors::InvalidPacket);
        }
        // Return
        Ok((rel_base_addr,size))
    }
    pub fn get_section_addr(&self) -> u32 {
        self.rel_base_address
    }
    pub fn get_section_size(&self) -> u32 {
        self.size
    }
}

pub struct RawPacket {
    //buffer: &'a [u8],
}

impl RawPacket {
    pub fn validate(buffer: &[u8]) -> Result<(), UpdateErrors> {
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(UpdateErrors::InvalidPacket);
        }
        // Return
        Ok(())
    }
}

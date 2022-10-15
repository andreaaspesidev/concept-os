use std::fmt::Debug;

use crate::{
    crc::crc8_update,
    utils::{u16_from_le_bytes, u32_from_le_bytes},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ComponentInfoResult {
    NoMoreComponents,
    NeedMoreBytes,
    InvalidMessage,
    InvalidCRC,
}

pub struct ComponentInfoMessage<'a> {
    buffer: &'a [u8],
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ComponentStatus: u16 {
        const NONE = 0;
        /// The hbf of the component is intact
        const HBF_VALID = 1 << 0;
    }
}

impl<'a> ComponentInfoMessage<'a> {
    pub fn from(buffer: &'a [u8]) -> Result<Self, ComponentInfoResult> {
        // Validate buffer
        Self::validate(buffer)?;
        // Return instance
        Ok(Self { buffer: buffer })
    }
    pub const fn min_size() -> usize {
        2
    }
    pub const fn max_size() -> usize {
        17
    }
    pub fn get_component_id(&self) -> u16 {
        u16_from_le_bytes(&self.buffer[0..0 + 2])
    }
    pub fn get_component_version(&self) -> u32 {
        u32_from_le_bytes(&self.buffer[2..2 + 4])
    }
    pub fn get_allocated_flash(&self) -> u32 {
        u32_from_le_bytes(&self.buffer[6..6 + 4])
    }
    pub fn get_allocated_ram(&self) -> u32 {
        u32_from_le_bytes(&self.buffer[10..10 + 4])
    }
    pub fn get_component_status(&self) -> ComponentStatus {
        ComponentStatus::from_bits_truncate(u16_from_le_bytes(&self.buffer[14..14 + 2]))
    }
    fn validate(buffer: &'a [u8]) -> Result<(), ComponentInfoResult> {
        // Check message size
        if buffer.len() == Self::min_size() {
            if buffer[0] == 0x00 && buffer[1] == 0x00 {
                return Err(ComponentInfoResult::NoMoreComponents);
            }
            return Err(ComponentInfoResult::NeedMoreBytes);
        } else if buffer.len() != Self::max_size() {
            return Err(ComponentInfoResult::InvalidMessage);
        }
        // Check component status
        if ComponentStatus::from_bits(u16_from_le_bytes(&buffer[14..14 + 2])).is_none() {
            return Err(ComponentInfoResult::InvalidMessage);
        }
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(ComponentInfoResult::InvalidCRC);
        }
        // Return
        Ok(())
    }
}

impl Debug for ComponentInfoMessage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\nComponent ID: {}\n", &self.get_component_id()))?;
        f.write_fmt(format_args!("\tVersion: {}\n", &self.get_component_version()))?;
        f.write_fmt(format_args!("\tFlash: {}\n", &self.get_allocated_flash()))?;
        f.write_fmt(format_args!("\tRAM: {}\n", &self.get_allocated_ram()))?;
        f.write_fmt(format_args!("\tStatus: {:?}", &self.get_component_status()))
    }
}
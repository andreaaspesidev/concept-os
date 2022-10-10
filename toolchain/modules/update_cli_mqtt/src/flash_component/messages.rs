use crate::{common_messages::SerializableMessage, crc::crc8_update};

#[repr(u8)]
pub enum ComponentUpdateCommand {
    SendComponentMetadata = 0x01,
    SendComponentFixedHeader = 0x02,
    SendComponentVariableHeader = 0x03,
    SendComponentPayload = 0x04,
    SendNextFragment = 0xA0
}

#[repr(u8)]
pub enum ComponentUpdateResponse {
    FailedMetadataCheck = 0xE1,
    NotEnoughSpace = 0xE2,
    CannotStartComponent = 0xE3,
    InvalidHBF = 0xE4,
    FailedHBFValidation = 0xE5,
    GenericFailure = 0xE6,
    Success = 0xFF
}

pub struct FixedHeaderMessage<'a> {
    buffer: &'a [u8]
}

impl<'a> FixedHeaderMessage<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        // Return instance
        Self {
            buffer: buffer
        }
    }
    pub const fn get_size() -> usize {
        hbf_rs::FIXED_HEADER_SIZE + 1
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


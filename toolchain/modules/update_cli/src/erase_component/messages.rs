use crate::{common_messages::SerializableMessage, crc::crc8_update};

#[repr(u8)]
pub enum ComponentEraseCommand {
    SendComponentID = 0x01
}

#[repr(u8)]
pub enum ComponentEraseResponse {
    CannotFindComponent = 0xE1,
    CannotFindVersion = 0xE2,
    GenericFailure = 0xEF,
    Success = 0xFF
}

pub struct ComponentIDMessage {
    component_id: u16,
    component_version: u32
}

impl ComponentIDMessage {
    pub fn new(component_id: u16, component_version: u32) -> Self {
        Self { component_id: component_id, component_version: component_version }
    }
}

impl SerializableMessage<'_> for ComponentIDMessage {
    fn get_raw(&self) -> Vec<u8> {
        let ci_bytes = self.component_id.to_le_bytes();
        let cv_bytes = self.component_version.to_le_bytes();
        // Construct buffer
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&ci_bytes);
        buffer.extend_from_slice(&cv_bytes);
        // Calculate CRC
        let mut crc = 0x00;
        for i in 0..buffer.len() {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer.push(crc);
        buffer
    }
}
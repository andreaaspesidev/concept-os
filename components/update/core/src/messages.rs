use hbf_lite::{BufferReaderImpl, HbfFile};

use crate::{crc::crc8_update, utils::u32_from_le_bytes};

/**
 * Generic enums
 */
#[derive(Debug, Clone, Copy)]
pub enum MessageError {
    InvalidSize,
    InvalidCRC,
    InvalidOperation,
    InvalidHBF,
    NotEnoughSpace,
    FlashError,
    ChannelError,
    TimeoutError,
    FailedHBFValidation,
    CannotFindComponent,
    CannotFindVersion,
}
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum OperationType {
    ComponentUpdate = 0xCA,
    SystemInfo = 0xCB,
    ComponentErase = 0xCE,
}

pub const TIMEOUT_ERROR: u8 = 0xE0;

/**
 * Generic Messages
 */

/// Hello Message
pub struct HelloMessage<'a> {
    operation: OperationType,
    buffer: &'a [u8],
}
impl TryFrom<u8> for OperationType {
    type Error = MessageError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xCA => Ok(OperationType::ComponentUpdate),
            0xCB => Ok(OperationType::SystemInfo),
            0xCE => Ok(OperationType::ComponentErase),
            _ => Err(MessageError::InvalidOperation),
        }
    }
}
impl<'a> HelloMessage<'a> {
    pub fn from(buffer: &'a [u8]) -> Result<Self, MessageError> {
        // Validate buffer
        let op = Self::validate(buffer)?;
        // Return instance
        Ok(Self {
            operation: op,
            buffer: buffer,
        })
    }
    pub const fn get_size() -> usize {
        18
    }
    fn validate(buffer: &'a [u8]) -> Result<OperationType, MessageError> {
        // Check message size
        if buffer.len() != Self::get_size() {
            return Err(MessageError::InvalidSize);
        }
        // Check OP
        let op = OperationType::try_from(buffer[0])?;
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
    pub fn get_operation(&self) -> OperationType {
        self.operation
    }
    pub fn get_aes_key(&self) -> &'a [u8] {
        &self.buffer[1..17]
    }
}

/// Hello Response Message
pub struct HelloResponseMessage {
    operation: OperationType,
}

impl HelloResponseMessage {
    pub fn new(operation: OperationType) -> Self {
        Self {
            operation: operation,
        }
    }
    const fn get_size() -> usize {
        7
    }
    pub fn get_raw(&self) -> [u8; Self::get_size()] {
        let mut buffer: [u8; Self::get_size()] = [0x00; Self::get_size()];
        let mut crc: u8 = 0x00;
        buffer[0] = 'O' as u8;
        buffer[1] = 'L' as u8;
        buffer[2] = 'L' as u8;
        buffer[3] = 'E' as u8;
        buffer[4] = 'H' as u8;
        buffer[5] = self.operation as u8;
        for i in 0..6usize {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer[6] = crc;
        buffer
    }
}

/// Raw Incoming Packet
#[derive(Debug)]
pub struct RawPacket<'a> {
    buffer: &'a [u8],
}

impl<'a> RawPacket<'a> {
    pub fn from(buffer: &'a [u8]) -> Result<Self, MessageError> {
        // Validate buffer
        Self::validate(buffer)?;
        // Return instance
        Ok(Self { buffer: buffer })
    }
    pub fn get_raw(&self) -> &'a [u8] {
        // Already validated
        &self.buffer[0..self.buffer.len() - 1]
    }
    fn validate(buffer: &'a [u8]) -> Result<(), MessageError> {
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(MessageError::InvalidCRC);
        }
        // Return
        Ok(())
    }
}

/**
 * Component Update
 * - Enums
 */
#[repr(u8)]
pub enum ComponentUpdateCommand {
    SendComponentMetadata = 0x01,
    SendComponentFixedHeader = 0x02,
    SendComponentVariableHeader = 0x03,
    SendComponentPayload = 0x04,
    SendNextFragment = 0xA0,
}

#[repr(u8)]
pub enum ComponentUpdateResponse {
    FailedMetadataCheck = 0xE1,
    NotEnoughSpace = 0xE2,
    CannotStartComponent = 0xE3,
    InvalidHBF = 0xE4,
    FailedHBFValidation = 0xE5,
    GenericFailure = 0xE7,
    Success = 0xFF,
}

/**
 * Component Update
 * - Messages
 */
pub struct FixedHeaderMessage<'a> {
    buffer: &'a [u8],
}

impl<'a> FixedHeaderMessage<'a> {
    pub fn from(buffer: &'a [u8]) -> Result<Self, MessageError> {
        // Validate buffer
        Self::validate(buffer)?;
        // Return instance
        Ok(Self { buffer: buffer })
    }
    pub const fn get_size() -> usize {
        hbf_lite::FIXED_HEADER_SIZE + 1
    }
    pub fn get_raw(&self) -> &'a [u8] {
        &self.buffer[0..self.buffer.len() - 1]
    }
    fn validate(buffer: &'a [u8]) -> Result<(), MessageError> {
        // Check message size
        if buffer.len() != Self::get_size() {
            return Err(MessageError::InvalidSize);
        }
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(MessageError::InvalidCRC);
        }
        // Check header
        let buff_reader = BufferReaderImpl::from(&buffer[0..buffer.len() - 1]);
        let hbf = HbfFile::from_reader(&buff_reader);
        if hbf.is_err() {
            return Err(MessageError::InvalidHBF);
        }
        // Return
        Ok(())
    }
}

pub struct ComponentIDPacket<'a> {
    buffer: &'a [u8],
}

impl<'a> ComponentIDPacket<'a> {
    pub fn from(buffer: &'a [u8]) -> Result<Self, MessageError> {
        // Validate buffer
        Self::validate(buffer)?;
        // Return instance
        Ok(Self { buffer: buffer })
    }
    pub const fn get_size() -> usize {
        7
    }
    pub fn get_component_id(&self) -> u16 {
        // Little endian encoding
        self.buffer[0] as u16 | ((self.buffer[1] as u16) << 8)
    }
    pub fn get_component_version(&self) -> u32 {
        u32_from_le_bytes(&self.buffer[2..2+4])
    }
    fn validate(buffer: &'a [u8]) -> Result<(), MessageError> {
        // Check len
        if buffer.len() != Self::get_size() {
            return Err(MessageError::InvalidSize);
        }
        // Check CRC
        let mut crc = 0x00;
        for i in 0..(buffer.len() - 1) {
            crc8_update(&mut crc, buffer[i]);
        }
        if crc != buffer[buffer.len() - 1] {
            return Err(MessageError::InvalidCRC);
        }
        // Return
        Ok(())
    }
}

/**
 * Component Erase
 */
#[repr(u8)]
pub enum ComponentEraseCommand {
    SendComponentID = 0x01,
}

#[repr(u8)]
pub enum ComponentEraseResponse {
    CannotFindComponent = 0xE1,
    CannotFindVersion = 0xE2,
    GenericFailure = 0xEF,
    Success = 0xFF,
}

/**
 * System Info
 */
pub const NO_MORE_COMPONENTS: u16 = 0x0000;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ComponentStatus: u16 {
        const NONE = 0;
        /// The hbf of the component is intact
        const HBF_VALID = 1 << 0;
    }
}

pub struct ComponentInfoMessage {
    component_id: u16,
    component_version: u32,
    allocated_flash: u32,
    allocated_ram: u32,
    component_status: ComponentStatus,
}

impl ComponentInfoMessage {
    pub fn new(
        component_id: u16,
        component_version: u32,
        allocated_flash: u32,
        allocated_ram: u32,
        component_status: ComponentStatus,
    ) -> Self {
        Self {
            component_id: component_id,
            component_version: component_version,
            allocated_flash: allocated_flash,
            allocated_ram: allocated_ram,
            component_status: component_status,
        }
    }
    pub const fn get_size() -> usize {
        17
    }
    pub fn write_to_buffer(&self, buffer: &mut [u8; Self::get_size()]) {
        // Write fields
        let mut pos: usize = 0;
        for b in self.component_id.to_le_bytes() {
            buffer[pos] = b;
            pos += 1;
        }
        for b in self.component_version.to_le_bytes() {
            buffer[pos] = b;
            pos += 1;
        }
        for b in self.allocated_flash.to_le_bytes() {
            buffer[pos] = b;
            pos += 1;
        }
        for b in self.allocated_ram.to_le_bytes() {
            buffer[pos] = b;
            pos += 1;
        }
        for b in self.component_status.bits.to_le_bytes() {
            buffer[pos] = b;
            pos += 1;
        }
        // Compute CRC-8
        let mut crc: u8 = 0x00;
        for i in 0..buffer.len() -1 {
            crc8_update(&mut crc, buffer[i]);
        }
        buffer[buffer.len()-1] = crc;
    }
}
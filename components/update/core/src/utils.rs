use crate::consts::*;
use crate::messages::{MessageError};
use hbf_lite::BufferReader;
use storage_api::*;
use uart_channel_api::*;

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
    fn read(&self, offset: u32, dest: &mut [u8]) -> Result<(), hbf_lite::HbfError> {
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
 * Channel functions
 */
pub fn channel_write_single(channel: &mut UartChannel, value: u8) -> Result<(), MessageError> {
    let buffer: [u8; 1] = [value; 1];
    Ok(channel
        .write_block(&buffer)
        .map_err(|_| MessageError::ChannelError)?)
}

pub fn channel_write(channel: &mut UartChannel, buff: &[u8]) -> Result<(), MessageError> {
    Ok(channel
        .write_block(&buff)
        .map_err(|_| MessageError::ChannelError)?)
}

pub fn channel_ask(
    channel: &mut UartChannel,
    cmd: u8,
    buffer: &mut [u8],
) -> Result<(), MessageError> {
    let buffer_out: [u8; 1] = [cmd; 1];
    Ok(channel
        .transmit_timed(&buffer_out, buffer, READ_TIMEOUT_TICKS)
        .map_err(|e| {
            if e == ChannelError::ReadTimeOut {
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

pub fn wrap_hbf_error<T>(r: Result<T, hbf_lite::HbfError>) -> Result<T, MessageError> {
    r.map_err(|_| {
        return MessageError::CannotReadHBF;
    })
}

/*
/**
 * Component operations
 */

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SearchError {
    CannotFindComponent,
    CannotFindVersion,
    FlashError,
}

#[derive(Debug)]
pub struct SearchResult {
    pub block: GetNthBlockResponse,
    pub component_version: u32,
}

pub fn search_component(
    component_id: u16,
    component_version: Option<u32>,
    storage: &Storage,
) -> Result<SearchResult, SearchError> {
    // Get block stats
    let status_res = storage.report_status();
    if status_res.is_err() {
        return Err(SearchError::FlashError);
    }
    let status = status_res.unwrap();
    let mut component_found: bool = false;
    // Iterate all blocks
    for block_num in 0..status.blocks {
        // Get block
        let block = storage.get_nth_block(block_num).unwrap();

        if block.block_type == BlockType::COMPONENT {
            // Read hbf header
            let mut buff: [u8; hbf_lite::HBF_HEADER_MIN_SIZE] =
                [0x00; hbf_lite::HBF_HEADER_MIN_SIZE];
            if storage
                .read_stream(block.block_base_address, 0, &mut buff)
                .is_err()
            {
                return Err(SearchError::FlashError);
            }
            // Try parse it
            let reader = hbf_lite::BufferReaderImpl::from(&buff);
            let hbf_res = hbf_lite::HbfFile::from_reader(&reader);
            if hbf_res.is_ok() {
                let hbf = hbf_res.unwrap();
                // Check if the id is correct
                let hbf_base_res = hbf.header_base();
                if hbf_base_res.is_err() {
                    continue; // Skip this block
                }
                let hbf_base = hbf_base_res.unwrap();
                // Compare component id
                if hbf_base.component_id() == component_id {
                    component_found = true;
                    if component_version.is_some() {
                        // Compare version
                        if hbf_base.component_version() == component_version.unwrap() {
                            return Ok(SearchResult {
                                block: block,
                                component_version: hbf_base.component_version(),
                            });
                        }
                    } else {
                        return Ok(SearchResult {
                            block: block,
                            component_version: hbf_base.component_version(),
                        });
                    }
                }
            }
        }
    }
    // No block found, return none
    return match component_found {
        true => Err(SearchError::CannotFindVersion),
        false => Err(SearchError::CannotFindComponent),
    };
}
 */
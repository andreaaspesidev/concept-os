#![no_std]

use core::cell::Cell;

use userlib::{flash::BlockType, hl, FromPrimitive, Lease, TaskId, STORAGE_ID};
use zerocopy::{AsBytes, FromBytes};

/**
 * Error Type
 */
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum StorageError {
    BlockIsFinalized = 1,
    OutOfFlash = 2,
    OutOfRam = 3,
    InvalidBlockPointer = 4,
    BlockTooSmall = 5,
    FlashError = 6,
    BadArgument = 7,
    NoBlockAvailable = 8,
    ComponentUnavailable = 9,
}
impl From<u32> for StorageError {
    fn from(x: u32) -> Self {
        match x {
            1 => StorageError::BlockIsFinalized,
            2 => StorageError::OutOfFlash,
            3 => StorageError::OutOfRam,
            4 => StorageError::InvalidBlockPointer,
            5 => StorageError::BlockTooSmall,
            6 => StorageError::FlashError,
            7 => StorageError::BadArgument,
            8 => StorageError::NoBlockAvailable,
            _ => StorageError::ComponentUnavailable,
        }
    }
}
impl From<StorageError> for u32 {
    fn from(x: StorageError) -> Self {
        x as u32
    }
}

/**
 * Operations
 */
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Operation {
    // Allocation Operations
    AllocateComponent = 1,
    AllocateGeneric = 2,
    // Deallocation Operations
    DeallocateBlock = 10,
    // Block Operations
    WriteStream = 20,
    ReadStream = 21,
    FinalizeBlock = 30,
    // Status Operations
    ReportStatus = 40,
    GetNthBlock = 41,
}

/// Component Allocation
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct AllocateComponentRequest {
    pub flash_size: u32,
    pub ram_size: u32,
}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct AllocateComponentResponse {
    pub flash_base_address: u32,
    pub flash_size: u32,
    pub ram_base_address: u32,
    pub ram_size: u32,
}
impl hl::Call for AllocateComponentRequest {
    const OP: u16 = Operation::AllocateComponent as u16;
    type Response = AllocateComponentResponse;
    type Err = StorageError;
}

/// Generic Allocation
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct AllocateGenericRequest {
    pub flash_size: u32,
}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct AllocateGenericResponse {
    pub flash_base_address: u32,
    pub flash_size: u32,
}
impl hl::Call for AllocateGenericRequest {
    const OP: u16 = Operation::AllocateGeneric as u16;
    type Response = AllocateGenericResponse;
    type Err = StorageError;
}

/// Block Deallocation
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct DeallocateBlockRequest {
    pub block_base_address: u32,
}
impl hl::Call for DeallocateBlockRequest {
    const OP: u16 = Operation::DeallocateBlock as u16;
    type Response = ();
    type Err = StorageError;
}

/// Write Stream
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct WriteStreamRequest {
    pub block_base_address: u32, // Data must be in a readable buffer at lease 0
    pub offset: u32,
}
impl hl::Call for WriteStreamRequest {
    const OP: u16 = Operation::WriteStream as u16;
    type Response = ();
    type Err = StorageError;
}

/// Read Stream
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadStreamRequest {
    pub block_base_address: u32, // Data will be put in a writable lease 0
    pub offset: u32,
}
impl hl::Call for ReadStreamRequest {
    const OP: u16 = Operation::ReadStream as u16;
    type Response = ();
    type Err = StorageError;
}

/// Finalize Block
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct FinalizeBlockRequest {
    pub block_base_address: u32,
}
impl hl::Call for FinalizeBlockRequest {
    const OP: u16 = Operation::FinalizeBlock as u16;
    type Response = ();
    type Err = StorageError;
}

/// Status
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReportStatusRequest {}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct ReportStatusResponse {
    pub blocks: u32,
    pub components: u32,
    pub dirty_blocks: u32,
    pub flash_used: u32,
    pub flash_total: u32,
    pub ram_used: u32,
    pub ram_total: u32,
}

impl hl::Call for ReportStatusRequest {
    const OP: u16 = Operation::ReportStatus as u16;
    type Response = ReportStatusResponse;
    type Err = StorageError;
}

/// Get N-th block
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct GetNthBlockRequest {
    pub block_number: u32,
}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct GetNthBlockResponse {
    pub block_base_address: u32,
    pub block_size: u32,
    pub block_type: BlockType,
}
impl hl::Call for GetNthBlockRequest {
    const OP: u16 = Operation::GetNthBlock as u16;
    type Response = GetNthBlockResponse;
    type Err = StorageError;
}

/**
 * Component Interface
 */
pub struct Storage(Cell<TaskId>);

impl Storage {
    pub fn new() -> Self {
        Self {
            0: Cell::new(TaskId(STORAGE_ID)),
        }
    }
    pub fn allocate_component(
        &mut self,
        flash_size: u32,
        ram_size: u32,
    ) -> Result<AllocateComponentResponse, StorageError> {
        hl::send_with_retry(
            &self.0,
            &AllocateComponentRequest {
                flash_size: flash_size,
                ram_size: ram_size,
            },
            &[],
        )
    }
    pub fn allocate_generic(
        &mut self,
        flash_size: u32,
    ) -> Result<AllocateGenericResponse, StorageError> {
        hl::send_with_retry(
            &self.0,
            &AllocateGenericRequest {
                flash_size: flash_size,
            },
            &[],
        )
    }

    pub fn deallocate_block(&mut self, block_base_address: u32) -> Result<(), StorageError> {
        hl::send_with_retry(
            &self.0,
            &DeallocateBlockRequest {
                block_base_address: block_base_address,
            },
            &[],
        )
    }

    pub fn write_stream(
        &mut self,
        block_base_address: u32,
        offset: u32,
        data: &[u8],
    ) -> Result<(), StorageError> {
        hl::send_with_retry(
            &self.0,
            &WriteStreamRequest {
                block_base_address: block_base_address,
                offset: offset,
            },
            &[Lease::read_only(data)],
        )
    }

    pub fn read_stream(
        &self,
        block_base_address: u32,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), StorageError> {
        hl::send_with_retry(
            &self.0,
            &ReadStreamRequest {
                block_base_address: block_base_address,
                offset: offset,
            },
            &[Lease::write_only(buffer)],
        )
    }

    pub fn finalize_block(&mut self, block_base_address: u32) -> Result<(), StorageError> {
        hl::send_with_retry(
            &self.0,
            &FinalizeBlockRequest {
                block_base_address: block_base_address,
            },
            &[],
        )
    }

    pub fn report_status(&self) -> Result<ReportStatusResponse, StorageError> {
        hl::send_with_retry(&self.0, &ReportStatusRequest {}, &[])
    }

    pub fn get_nth_block(&self, block_number: u32) -> Result<GetNthBlockResponse, StorageError> {
        hl::send_with_retry(
            &self.0,
            &GetNthBlockRequest {
                block_number: block_number,
            },
            &[],
        )
    }
}

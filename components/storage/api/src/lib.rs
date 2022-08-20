#![no_std]

use userlib::{hl, TaskId, FromPrimitive, sys_send, Lease};
use zerocopy::{AsBytes,FromBytes};

/**
 * Constants
 */
const STORAGE_TASK_ID: TaskId = TaskId(2);

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
    ComponentUnavailable = 8
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
            _ => StorageError::ComponentUnavailable
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
}

/// Component Allocation
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct AllocateComponentRequest {
    pub flash_size: u32,
    pub ram_size: u32
}
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct AllocateComponentResponse {
    pub flash_base_address: u32,
    pub flash_size: u32,
    pub ram_base_address: u32,
    pub ram_size: u32
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
    pub flash_size: u32
}
#[derive(FromBytes, AsBytes)]
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
    pub block_base_address: u32
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
    pub block_base_address: u32,  // Data must be in a readable buffer at lease 0
    pub offset: u32,
}

/// Read Stream
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadStreamRequest {
    pub block_base_address: u32,  // Data will be put in a writable lease 0
    pub offset: u32
}

/// Finalize Block
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct FinalizeBlockRequest {
    pub block_base_address: u32
}
impl hl::Call for FinalizeBlockRequest {
    const OP: u16 = Operation::FinalizeBlock as u16;
    type Response = ();
    type Err = StorageError;
}

/// Status
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReportStatusRequest {
}
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReportStatusResponse {
    pub blocks: u32,
    pub components: u32,
    pub dirty_blocks: u32,
    pub flash_used: u32,
    pub flash_total: u32,
    pub ram_used: u32,
    pub ram_total: u32
}

impl hl::Call for ReportStatusRequest {
    const OP: u16 = Operation::ReportStatus as u16;
    type Response = ReportStatusResponse;
    type Err = StorageError;
}

/**
 * Component Interface
 */
pub struct Storage();

impl Storage {
    pub fn allocate_component(&mut self, flash_size: u32, ram_size: u32) -> Result<AllocateComponentResponse, StorageError> {
        hl::send(STORAGE_TASK_ID, &AllocateComponentRequest{
            flash_size: flash_size,
            ram_size: ram_size
        })
    }
    pub fn allocate_generic(&mut self, flash_size: u32) -> Result<AllocateGenericResponse, StorageError> {
        hl::send(STORAGE_TASK_ID, &AllocateGenericRequest {
            flash_size: flash_size
        })
    }

    pub fn deallocate_block(&mut self, block_base_address: u32) -> Result<(), StorageError> {
        hl::send(STORAGE_TASK_ID, &DeallocateBlockRequest {
            block_base_address: block_base_address
        })
    }

    pub fn write_stream(&mut self, block_base_address: u32, offset: u32, data: &[u8]) -> Result<(), StorageError> {
        // Missing an appropriate hl function for using leases. Maybe will be introduced in the future
        let message = &WriteStreamRequest{
            block_base_address: block_base_address,
            offset: offset
        };
        let (code, _) = sys_send(STORAGE_TASK_ID, 
            Operation::WriteStream as u16, 
            message.as_bytes(), 
            &mut [],
            &[Lease::read_only(data)]
        );
        if code == 0 {
            Ok(())
        } else {
            return Err(StorageError::from(code));
        }
    }

    pub fn read_stream(&self, block_base_address: u32, offset: u32, buffer: &mut [u8]) -> Result<(), StorageError> {
        // Missing an appropriate hl function for using leases. Maybe will be introduced in the future
        let message = &ReadStreamRequest{
            block_base_address: block_base_address,
            offset: offset
        };
        let (code, _) = sys_send(STORAGE_TASK_ID, 
            Operation::ReadStream as u16, 
            message.as_bytes(), 
            &mut [],
            &[Lease::write_only(buffer)]
        );
        if code == 0 {
            Ok(())
        } else {
            return Err(StorageError::from(code));
        }
    }

    pub fn finalize_block(&mut self, block_base_address: u32) -> Result<(), StorageError> {
        hl::send(STORAGE_TASK_ID, &FinalizeBlockRequest {
            block_base_address: block_base_address
        })
    }

    pub fn report_status(&self) -> Result<ReportStatusResponse, StorageError> {
        hl::send(STORAGE_TASK_ID, &ReportStatusRequest {})
    }
}

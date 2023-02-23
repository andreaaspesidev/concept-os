#![no_std]
#![no_main]

mod flash;

// Import board specific constants
cfg_if::cfg_if! {
    if #[cfg(feature = "board_stm32f303re")] {
        use stm32f303re::*;
    } else if #[cfg(feature = "board_stm32l432kc")] {
        use stm32l432kc::*;
    } else if #[cfg(feature = "board_stm32l476rg")] {
        use stm32l476rg::*;
    } else {
        compile_error!("Board not supported");
    }
}

use flash::FlashInterface;
use flash_allocator::flash::{walker::FlashWalkerImpl, FlashAllocatorImpl, FlashMethods};
use ram_allocator::{AllocatorError, RAMAllocator, RAMAllocatorImpl};
use storage_api::{
    AllocateComponentRequest, AllocateComponentResponse, DeallocateBlockRequest,
    FinalizeBlockRequest, GetNthBlockRequest, GetNthBlockResponse, Operation, ReadStreamRequest,
    ReportStatusRequest, ReportStatusResponse, StorageError, WriteStreamRequest,
};
use userlib::{flash::BlockType, hl::Borrow, *};

const STORAGE_ANALYZE_MASK: u32 = 1;

#[export_name = "main"]
fn main() -> ! {
    sys_log!("[STORAGE] Hello!");
    // Activate task
    kipc::activate_task();
    // Always analyze storage on start-up
    analyze_storage();
    // Message handler
    let recv_handler = |_s: (), op: Operation, msg: hl::Message| -> Result<(), StorageError> {
        match op {
            Operation::AllocateComponent => {
                // Parse message
                let (msg, caller) = msg
                    .fixed::<AllocateComponentRequest, AllocateComponentResponse>()
                    .ok_or(StorageError::BadArgument)?;
                // Allocate Flash segment
                let (flash_base_addr, flash_size) =
                    flash_allocate(msg.flash_size, BlockType::COMPONENT)?;
                // Allocate RAM (+ mark as component)
                let (ram_base_addr, ram_size) = ram_allocate(msg.ram_size, flash_base_addr)?;
                // Respond with data
                caller.reply(AllocateComponentResponse {
                    flash_base_address: flash_base_addr,
                    flash_size: flash_size,
                    ram_base_address: ram_base_addr,
                    ram_size: ram_size,
                });
                Ok(())
            }
            Operation::DeallocateBlock => {
                // Parse message
                let (msg, caller) = msg
                    .fixed::<DeallocateBlockRequest, ()>()
                    .ok_or(StorageError::BadArgument)?;
                // Deallocate block
                // TODO: if block is of a component, ask kernel if the component is stopped
                //       before continuing. Otherwise fail.
                flash_deallocate(msg.block_base_address)?;
                // Respond okay
                caller.reply(());
                Ok(())
            }
            Operation::WriteStream => {
                // Parse message
                let (msg, caller) = msg
                    .fixed_with_leases::<WriteStreamRequest, ()>(1)
                    .ok_or(StorageError::BadArgument)?;
                // Check lease permissions (readable requested)
                let borrow = caller.borrow(0);
                let info = borrow.info().ok_or(StorageError::BadArgument)?;
                if !info.attributes.contains(LeaseAttributes::READ) {
                    return Err(StorageError::BadArgument);
                }
                // Perform the write
                flash_write_stream(msg.block_base_address, msg.offset, &borrow, info.len, msg.flush_after > 0)?;
                // Respond okay
                caller.reply(());
                Ok(())
            }
            Operation::ReadStream => {
                // Parse message
                let (msg, caller) = msg
                    .fixed_with_leases::<ReadStreamRequest, ()>(1)
                    .ok_or(StorageError::BadArgument)?;
                // Check lease permissions (writable requested)
                let borrow = caller.borrow(0);
                let info = borrow.info().ok_or(StorageError::BadArgument)?;
                if !info.attributes.contains(LeaseAttributes::WRITE) {
                    return Err(StorageError::BadArgument);
                }
                // Perform the read
                flash_read_stream(msg.block_base_address, msg.offset, &borrow, info.len)?;
                // Respond okay
                caller.reply(());
                Ok(())
            }
            Operation::FinalizeBlock => {
                // Parse message
                let (msg, caller) = msg
                    .fixed::<FinalizeBlockRequest, ()>()
                    .ok_or(StorageError::BadArgument)?;
                // Deallocate block
                flash_finalize_block(msg.block_base_address)?;
                // Respond okay
                caller.reply(());
                Ok(())
            }
            Operation::ReportStatus => {
                // Parse message
                let (_, caller) = msg
                    .fixed::<ReportStatusRequest, ReportStatusResponse>()
                    .ok_or(StorageError::BadArgument)?;
                // Generate status
                let response = generate_status()?;
                // Respond okay
                caller.reply(response);
                Ok(())
            }
            Operation::GetNthBlock => {
                // Parse message
                let (msg, caller) = msg
                    .fixed::<GetNthBlockRequest, GetNthBlockResponse>()
                    .ok_or(StorageError::BadArgument)?;
                // Search for block
                let (flash_base_addr, flash_size, block_type) = get_nth_block(msg.block_number)?;
                // Respond with data
                caller.reply(GetNthBlockResponse {
                    block_base_address: flash_base_addr,
                    block_size: flash_size,
                    block_type: block_type,
                });
                Ok(())
            }
        }
    };

    // Incoming message buffer
    // Must be as big as the biggest structure of the request
    // In this case at least 3*4 = 12 bytes
    let mut buffer: [u8; 12] = [0; 12];

    // Main loop
    loop {
        // Wait for a command
        hl::recv(
            &mut buffer,
            STORAGE_ANALYZE_MASK,
            (),
            |_s, bits| {
                // Check we got the right one
                if bits & STORAGE_ANALYZE_MASK > 0 {
                    // The kernel indirectly asks to erase a block or validate storage
                    analyze_storage();
                }
            },
            recv_handler,
        );
    }
}

fn analyze_storage() {
    sys_log!("[STORAGE] Analyzing");
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Perform storage analysis
    FlashAllocatorImpl::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_BLOCK_SIZE,
        FLASH_NUM_BLOCKS,
        FLASH_TREE_MAX_LEVEL,
        FLASH_NUM_NODES,
    >::analyze_storage(&mut flash, true);
    sys_log!("[STORAGE] Analysis completed!");
}

fn generate_status() -> Result<ReportStatusResponse, StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Create instance of the response
    let mut response = ReportStatusResponse {
        blocks: 0,
        components: 0,
        dirty_blocks: 0,
        flash_used: 0,
        flash_total: FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_SCAN_ADDR + 1,
        ram_used: 0,
        ram_total: SRAM_END_ADDR - SRAM_START_ADDR - SRAM_RESERVED + 1,
    };
    // Create flash walker
    let walker = FlashWalkerImpl::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >::new(&mut flash);
    // Iterate on each block
    for b in walker {
        response.blocks += 1;
        response.flash_used += b.get_size() + (flash_allocator::flash::HEADER_SIZE as u32);
        if !b.is_finalized() {
            response.dirty_blocks += 1;
        }
        if b.get_type() == BlockType::COMPONENT {
            response.components += 1;
            response.flash_used += 8;
            // Hack to read directly the size, due to borrowing problems using the library
            let mut sram_size_bytes: [u8; 4] = [0x00; 4];
            FlashInterface::new()
                .read(b.get_base_address() + 4, &mut sram_size_bytes)
                .unwrap();
            let sram_size = u32::from_le_bytes(sram_size_bytes);
            response.ram_used += sram_size;
        }
    }

    Ok(response)
}

const STREAM_CHUNK_SIZE: usize = 64;

fn flash_write_stream(
    base_address: u32,
    offset: u32,
    lease: &Borrow,
    total_size: usize,
    flush_after: bool,
) -> Result<(), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Get the block
    let block_res = flash_allocator::flash::utils::get_flash_block::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >(&mut flash, base_address, false);
    if block_res.is_none() {
        return Err(StorageError::InvalidBlockPointer);
    }
    let block = block_res.unwrap();
    // Get the actual position of where to start writing
    let block_start_addr = match block.get_type() {
        BlockType::COMPONENT => block.get_base_address() + 8,
        _ => block.get_base_address(),
    };
    // Calculate if there is enough space
    if total_size as u32 + offset > block.get_size() {
        return Err(StorageError::BlockTooSmall);
    }
    // Perform the write
    // TODO: tune chunk size to affect component performance
    let mut pos: usize = 0;
    let mut buff: [u8; STREAM_CHUNK_SIZE] = [0xFF; STREAM_CHUNK_SIZE];

    while pos < total_size {
        // tbw = min(STREAM_CHUNK_SIZE, total_size - pos)
        let mut tbw: usize = STREAM_CHUNK_SIZE;
        if pos + STREAM_CHUNK_SIZE > total_size {
            tbw = total_size - pos;
        }
        if lease.read_fully_at(pos, &mut buff[0..tbw]).is_none() {
            // Chunk read failed, probably client died.
            return Err(StorageError::BadArgument);
        }
        // Write chunk
        if flash
            .write(block_start_addr + offset + pos as u32, &buff[0..tbw])
            .is_err()
        {
            // Write failed
            return Err(StorageError::FlashError);
        }
        // Increase position
        pos += tbw;
    }
    if flush_after {
        // At the end, to-be-safe, flush write buffer
        if flash.flush_write_buffer().is_err() {
            // Write failed
            return Err(StorageError::FlashError);
        }
    }
    // Return
    Ok(())
}

fn flash_read_stream(
    base_address: u32,
    offset: u32,
    lease: &Borrow,
    total_size: usize,
) -> Result<(), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Get the block
    let block_res = flash_allocator::flash::utils::get_flash_block::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >(&mut flash, base_address, false);
    if block_res.is_none() {
        return Err(StorageError::InvalidBlockPointer);
    }
    let block = block_res.unwrap();
    // Get the actual position of where to start reading
    let block_start_addr = match block.get_type() {
        BlockType::COMPONENT => block.get_base_address() + 8,
        _ => block.get_base_address(),
    };
    // Calculate if there is enough space
    if total_size as u32 + offset > block.get_size() {
        return Err(StorageError::BlockTooSmall);
    }
    // Perform the read
    // TODO: tune chunk size to affect component performance
    let mut pos: usize = 0;
    let mut buff: [u8; STREAM_CHUNK_SIZE] = [0xFF; STREAM_CHUNK_SIZE];

    while pos < total_size {
        // tbr = min(STREAM_CHUNK_SIZE, total_size - pos)
        let mut tbr: usize = STREAM_CHUNK_SIZE;
        if pos + STREAM_CHUNK_SIZE > total_size {
            tbr = total_size - pos;
        }
        // Read chunk
        let read_result = flash.read(block_start_addr + offset + pos as u32, &mut buff[0..tbr]);
        if read_result.is_err() {
            // Read failed
            return Err(StorageError::FlashError);
        }
        // Send the chunk to the client
        if lease.write_fully_at(pos, &buff[0..tbr]).is_none() {
            // Chunk write failed, probably client died.
            return Err(StorageError::BadArgument);
        }
        // Increase position
        pos += tbr;
    }
    // Return
    Ok(())
}

fn flash_finalize_block(base_address: u32) -> Result<(), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Get the block
    let block_res = flash_allocator::flash::utils::get_flash_block::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >(&mut flash, base_address, false);
    if block_res.is_none() {
        return Err(StorageError::InvalidBlockPointer);
    }
    let block = block_res.unwrap();
    // Launch finalization
    let result = flash_allocator::flash::utils::finalize_block::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >(&mut flash, block);
    if result.is_err() {
        return Err(StorageError::BlockIsFinalized);
    }
    Ok(())
}

fn flash_allocate(requested_size: u32, block_type: BlockType) -> Result<(u32, u32), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Instantiate the flash allocator. Inefficient each time, but done this
    // way in order to preserve memory, as we are also using a buddy allocator for the ram
    let mut allocator = FlashAllocatorImpl::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_BLOCK_SIZE,
        FLASH_NUM_BLOCKS,
        FLASH_TREE_MAX_LEVEL,
        FLASH_NUM_NODES,
    >::from_flash(&mut flash, true, true);
    // Get the address
    let result = allocator.allocate(requested_size, block_type);
    if result.is_ok() {
        let block = result.unwrap();
        Ok((block.get_base_address(), block.get_size()))
    } else {
        return Err(StorageError::OutOfFlash);
    }
}

fn flash_deallocate(base_address: u32) -> Result<(), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Instantiate the flash allocator. Inefficient each time, but done this
    // way in order to preserve memory, as we are also using a buddy allocator for the ram
    let mut allocator = FlashAllocatorImpl::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_BLOCK_SIZE,
        FLASH_NUM_BLOCKS,
        FLASH_TREE_MAX_LEVEL,
        FLASH_NUM_NODES,
    >::from_flash(&mut flash, true, true);
    // Get the address
    if allocator.deallocate(base_address).is_ok() {
        Ok(())
    } else {
        return Err(StorageError::InvalidBlockPointer);
    }
}

fn ram_allocate(
    requested_size: u32,
    component_base_address: u32,
) -> Result<(u32, u32), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Instantiate the flash allocator. Inefficient each time, but done this
    // way in order to preserve memory, as we are also using a buddy allocator for the ram
    let mut allocator = RAMAllocatorImpl::<
        SRAM_START_ADDR,
        SRAM_END_ADDR,
        SRAM_BLOCK_SIZE,
        SRAM_NUM_BLOCKS,
        SRAM_TREE_MAX_LEVEL,
        SRAM_NUM_NODES,
        SRAM_RESERVED,
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >::from_flash(&mut flash);
    // Get the address
    let result = allocator.allocate(component_base_address, requested_size);
    if result.is_ok() {
        let block = result.unwrap();
        Ok((block.get_base_address(), block.get_size()))
    } else {
        let err = result.unwrap_err();
        return Err(match err {
            AllocatorError::OutOfRAM => StorageError::OutOfRam,
            AllocatorError::InvalidBlock => StorageError::InvalidBlockPointer,
        });
    }
}

fn get_nth_block(block_number: u32) -> Result<(u32, u32, BlockType), StorageError> {
    // Instantiate the flash operators
    let mut flash = FlashInterface::new();
    // Create flash walker
    let walker = FlashWalkerImpl::<
        FLASH_ALLOCATOR_START_ADDR,
        FLASH_ALLOCATOR_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_TREE_MAX_LEVEL,
    >::new(&mut flash);
    // Iterate on each block
    let mut count: u32 = 0;
    for b in walker {
        if count == block_number {
            // Found the block, return data
            return Ok((b.get_base_address(), b.get_size(), b.get_type()));
        }
        count += 1;
    }
    // No block found
    return Err(StorageError::NoBlockAvailable);
}

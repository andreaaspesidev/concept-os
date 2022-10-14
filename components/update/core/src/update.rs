use crate::consts::*;
use crate::messages::*;
use crate::utils::*;
use hbf_lite::{BufferReaderImpl, HbfFile};
use storage_api::*;
use uart_channel_api::*;
use userlib::flash::BlockType;
use userlib::sys_log;
use userlib::UnwrapLite;

pub fn component_add_update(channel: &mut UartChannel) -> Result<(), MessageError> {
    // Ask fixed header
    let mut hbf_header_buff: [u8; FixedHeaderMessage::get_size()] =
        [0; FixedHeaderMessage::get_size()];
    channel_ask(
        channel,
        ComponentUpdateCommand::SendComponentFixedHeader as u8,
        &mut hbf_header_buff,
    )?;
    // Validate header
    let fhm = FixedHeaderMessage::from(&mut hbf_header_buff)?;
    // Read hbf
    let hbf_reader = BufferReaderImpl::from(fhm.get_raw());
    let hbf = wrap_hbf_error(hbf_lite::HbfFile::from_reader(&hbf_reader))?;
    // Get needed space
    let needed_flash = wrap_hbf_error(hbf.header_base())?.total_size();
    let needed_ram = wrap_hbf_error(hbf.header_main())?.component_min_ram();
    // Request the allocation
    let mut storage = Storage::new();
    // ---> !!!! From this point on, if we fail we must deallocate !!!!
    let allocation = storage
        .allocate_component(needed_flash, needed_ram)
        .map_err(|e| {
            // Fail if no space available
            match e {
                StorageError::OutOfFlash | StorageError::OutOfRam => MessageError::NotEnoughSpace,
                _ => MessageError::FlashError,
            }
        })?;

    // Snap header base
    let header_base = hbf.header_base().unwrap_lite(); // Already read, safe to get
    drop(hbf); // Force ourselves not to use this object anymore

    // Start by flushing hbf header on flash
    // NOTE: we must skip the checksum field, as we will modify the HBF. As last operation, we will write
    // this field and only then we will validate everything. For now, we leave 0xFFFF_FFFF
    let checksum_offset = hbf_lite::HBF_CHECKSUM_OFFSET as usize;
    let original_checksum =
        u32_from_le_bytes(&hbf_header_buff[checksum_offset..checksum_offset + 4]);

    let mut validation_checksum: u32 = 0;
    let mut new_checksum: u32;
    // Change checksum
    for i in 0..4usize {
        hbf_header_buff[checksum_offset + i] = 0x00;
    }
    update_checksum(
        &mut validation_checksum,
        &hbf_header_buff[0..hbf_header_buff.len() - 1],
    ); // TODO: fix this

    // Change again
    for i in 0..4usize {
        hbf_header_buff[checksum_offset + i] = 0xFF;
    }

    let mut curr_pos: u32 = 0;

    deallocate_on_error(
        storage
            .write_stream(
                allocation.flash_base_address,
                curr_pos,
                &hbf_header_buff[0..hbf_header_buff.len() - 1],
            )
            .map_err(|_| MessageError::FlashError),
        &mut storage,
        allocation.flash_base_address,
    )?;

    curr_pos += hbf_header_buff.len() as u32 - 1;

    let mut to_read: usize = hbf_lite::REGION_SIZE * header_base.num_regions() as usize
        + hbf_lite::INTERRUPT_SIZE * header_base.num_interrupts() as usize
        + hbf_lite::RELOC_SIZE * header_base.num_relocations() as usize
        + hbf_lite::DEPENDENCY_SIZE * header_base.num_dependencies() as usize;

    deallocate_on_error(
        channel_write_single(
            channel,
            ComponentUpdateCommand::SendComponentVariableHeader as u8,
        ),
        &mut storage,
        allocation.flash_base_address,
    )?;

    sys_log!("Waiting for variable header");

    deallocate_on_error(
        read_bytes(
            to_read,
            channel,
            &mut storage,
            &mut curr_pos,
            allocation.flash_base_address,
            &mut validation_checksum,
            |_, _| Ok(()),
        ),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // Now, with the whole header intact, let's generate a version of the hbf that is actually
    // able to read all its data
    let flash_reader = FlashReader::from(allocation.flash_base_address, allocation.flash_size);

    sys_log!("Reading HBF from flash");

    let flash_hbf = deallocate_on_error(
        wrap_hbf_error(HbfFile::from_reader(&flash_reader)),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // Before reading the payload, validate the dependencies of this component
    deallocate_on_error(
        validate_version_and_dependencies(&flash_hbf, &mut storage, allocation.flash_base_address),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // From this point on, the validation and new checksum are different
    new_checksum = validation_checksum;

    // Now at the same way, read the payload
    to_read = deallocate_on_error(
        wrap_hbf_error(flash_hbf.payload_size()),
        &mut storage,
        allocation.flash_base_address,
    )? as usize;

    // Calculate needed offsets, create needed vars
    let payload_start_offset = wrap_hbf_error(flash_hbf.get_readonly_payload())?.get_offset();
    let new_base_address: u32 = allocation.flash_base_address + 8 + payload_start_offset;
    let mut reloc_buffer: [u32; RELOC_BUFFER_LEN] = [0; RELOC_BUFFER_LEN];
    let mut used_relocs: usize = 0;
    let mut usable_relocs: usize = 0;
    let mut total_usable_relocs: usize =
        flash_hbf.header_base().unwrap_lite().num_relocations() as usize;
    let mut current_reloc_pos: usize = 0;

    deallocate_on_error(
        channel_write_single(channel, ComponentUpdateCommand::SendComponentPayload as u8),
        &mut storage,
        allocation.flash_base_address,
    )?;

    sys_log!("Waiting for payload");

    deallocate_on_error(
        read_bytes(
            to_read,
            channel,
            &mut storage,
            &mut curr_pos,
            allocation.flash_base_address,
            &mut validation_checksum,
            |buffer, curr_pos| {
                // Launch relocator
                apply_relocs(
                    buffer,
                    curr_pos,
                    new_base_address,
                    &mut reloc_buffer,
                    &mut used_relocs,
                    &mut usable_relocs,
                    &mut current_reloc_pos,
                    &mut total_usable_relocs,
                    &flash_hbf,
                )?;
                // After fixes, compute also the new checksum
                update_checksum(&mut new_checksum, buffer);
                Ok(())
            },
        ),
        &mut storage,
        allocation.flash_base_address,
    )?;

    sys_log!("Checksum validation");

    // Checksum validation
    let new_checksum_bytes = new_checksum.to_le_bytes();
    if validation_checksum != original_checksum
        || storage
            .write_stream(
                allocation.flash_base_address,
                hbf_lite::HBF_CHECKSUM_OFFSET,
                &new_checksum_bytes,
            )
            .is_err()
    {
        // Deallocate the space
        storage
            .deallocate_block(allocation.flash_base_address)
            .unwrap_lite();
        return Err(MessageError::FailedHBFValidation);
    }

    // Validate the HBF (last one, to ensure the library reads it correctly)
    let hbf_validation = deallocate_on_error(
        wrap_hbf_error(flash_hbf.validate()),
        &mut storage,
        allocation.flash_base_address,
    )?;
    if !hbf_validation {
        // Deallocate the space
        storage
            .deallocate_block(allocation.flash_base_address)
            .unwrap();
        return Err(MessageError::FailedHBFValidation);
    }
    // Start component, do stuff ...
    sys_log!("Try to start component");
    let start_result = userlib::kipc::load_component(allocation.flash_base_address);
    if start_result {
        sys_log!("Component started!");
    }
    // Respond (at this point, do not delete the component if we just fail to send the end byte)
    channel_write_single(channel, ComponentUpdateResponse::Success as u8)?;
    Ok(())
}

fn deallocate_on_error<T>(
    r: Result<T, MessageError>,
    storage: &mut Storage,
    block_base_address: u32,
) -> Result<T, MessageError> {
    r.map_err(|e| {
        // Deallocate the space
        storage.deallocate_block(block_base_address).unwrap_lite();
        // Return error
        e
    })
}

fn read_bytes<F>(
    mut bytes_to_read: usize,
    channel: &mut UartChannel,
    storage: &mut Storage,
    curr_pos: &mut u32,
    flash_base: u32,
    validation_checksum: &mut u32,
    mut buffer_process: F,
) -> Result<(), MessageError>
where
    F: FnMut(&mut [u8], u32) -> Result<(), MessageError>,
{
    // As we cannot dynamically allocate ram for variable
    // parts, from this point on we use a small buffer, but
    // directly save in RAM.
    let mut pkt_buffer: [u8; PACKET_BUFFER_SIZE + 1] = [0x00; PACKET_BUFFER_SIZE + 1];
    let mut min_to_read = core::cmp::min(PACKET_BUFFER_SIZE, bytes_to_read);

    while bytes_to_read > 0 {
        // Ask for another packet
        channel_ask(
            channel,
            ComponentUpdateCommand::SendNextFragment as u8,
            &mut pkt_buffer[0..min_to_read + 1],
        )?;
        // Validate this packet
        let pkt = RawPacket::from(&pkt_buffer[0..min_to_read + 1])?;
        // Write this packet content

        // Apply relocations if requested.
        // To this purpose, must move the data in another buffer were we can edit
        let mut data: [u8; PACKET_BUFFER_SIZE] = [0x00; PACKET_BUFFER_SIZE];
        for i in 0..min_to_read {
            data[i] = pkt.get_raw()[i];
        }
        // Before fixes, calculate the old checksum
        update_checksum(validation_checksum, &data[0..min_to_read]);

        // Process the buffer
        buffer_process(&mut data[0..min_to_read], *curr_pos)?;

        // Save stream to flash
        storage
            .write_stream(flash_base, *curr_pos, &data[0..min_to_read])
            .map_err(|_| MessageError::FlashError)?;

        *curr_pos += pkt.get_raw().len() as u32;
        // Update stats
        bytes_to_read -= min_to_read;
        min_to_read = core::cmp::min(PACKET_BUFFER_SIZE, bytes_to_read);
    }

    Ok(())
}

/// Scans the system to verify if all the dependencies of this component
/// are satisfied
fn validate_version_and_dependencies(
    hbf: &HbfFile,
    storage: &mut Storage,
    block_base_address: u32,
) -> Result<(), MessageError> {
    // Iterating for components is expensive, so must be ideally done once.
    // The number of dependencies is unknown at compile time, so we may exploit ordering
    // constraint.
    let hbf_base = wrap_hbf_error(hbf.header_base())?;
    let num_dependencies = hbf_base.num_dependencies();
    // Prepare for iterating over the blocks
    // Get block stats
    let flash_status = storage
        .report_status()
        .map_err(|_| MessageError::FlashError)?;
    // Some dependencies
    let mut current_dep = hbf.dependency_nth(0).unwrap_lite(); // At least one must exist
    let mut current_dep_pos: u16 = 1;
    // Iterate all blocks
    for block_num in 0..flash_status.blocks {
        // Get block
        let block = storage.get_nth_block(block_num).unwrap();
        // Skip the current block!
        if block.block_base_address == block_base_address {
            continue;
        }
        if block.block_type == BlockType::COMPONENT {
            // Read hbf header
            let mut buff: [u8; hbf_lite::HBF_HEADER_MIN_SIZE] =
                [0x00; hbf_lite::HBF_HEADER_MIN_SIZE];
            storage
                .read_stream(block.block_base_address, 0, &mut buff)
                .map_err(|_| MessageError::FlashError)?;
            // Parse it
            let reader = hbf_lite::BufferReaderImpl::from(&buff);
            let comp_hbf =
                hbf_lite::HbfFile::from_reader(&reader).map_err(|_| MessageError::FlashError)?;
            let comp_data = comp_hbf.header_base().unwrap_lite();
            // Check whether this component is an old version of ours
            if comp_data.component_id() == hbf_base.component_id() {
                // Check constraint on greater version
                if comp_data.component_version() >= hbf_base.component_version() {
                    return Err(MessageError::IllegalDowngrade);
                }
            } else if comp_data.component_id() == current_dep.component_id() {
                // Check version
                if current_dep.min_version() > 0
                    && comp_data.component_version() < current_dep.min_version()
                {
                    // Wrong version (lower bound)
                    return Err(MessageError::DependencyError);
                } else if current_dep.max_version() > 0
                    && comp_data.component_version() > current_dep.max_version()
                {
                    // Wrong version (upper bound)
                    return Err(MessageError::DependencyError);
                }
                // Advance with the dependency
                if current_dep_pos < num_dependencies {
                    // Get next one
                    current_dep = hbf.dependency_nth(current_dep_pos).unwrap_lite();
                    current_dep_pos += 1;
                } else {
                    // Got all the dependencies
                    return Ok(());
                }
            }
        }
    }
    // Missing dependency
    return Err(MessageError::MissingDependency);
}

/**
 * As parsing relocations from HBF file is very expensive (lots of syscalls involved to
 * read from flash), and we know relocations are in-order, we can store a sliding-buffer
 * of N relocs, and just iterate over this buffer.
 */
const RELOC_BUFFER_LEN: usize = 16;

fn read_next_relocs(
    reloc_buffer: &mut [u32; RELOC_BUFFER_LEN],
    usable_relocs: &mut usize,
    current_reloc_pos: &mut usize,
    total_usable_relocs: &mut usize,
    hbf: &HbfFile,
) -> Result<(), MessageError> {
    // Get the missing relocations
    let missing_reloc = core::cmp::min(RELOC_BUFFER_LEN, *total_usable_relocs);
    for i in 0..missing_reloc {
        // Read relocation info
        let reloc_num = (*current_reloc_pos + i) as u32;
        let reloc = wrap_hbf_error(hbf.relocation_nth(reloc_num))?;
        // Save in the buffer
        reloc_buffer[i] = reloc.offset();
    }
    // Update the pointers
    *usable_relocs = missing_reloc;
    *total_usable_relocs -= missing_reloc;
    *current_reloc_pos += missing_reloc;
    Ok(())
}

fn apply_relocs(
    buffer: &mut [u8],
    last_written_offset: u32,
    new_dest_address: u32,
    reloc_buffer: &mut [u32; RELOC_BUFFER_LEN],
    used_relocs: &mut usize,
    usable_relocs: &mut usize,
    current_reloc_pos: &mut usize,
    total_usable_relocs: &mut usize,
    hbf: &HbfFile,
) -> Result<(), MessageError> {
    // If we have no relocation still available, ignore
    if *total_usable_relocs == 0usize {
        return Ok(());
    }
    let mut pos: usize = 0usize;
    while pos <= buffer.len() - 4usize {
        // --> Check the status of the relocs buffer
        if *used_relocs == *usable_relocs {
            // Populate again the buffer
            read_next_relocs(
                reloc_buffer,
                usable_relocs,
                current_reloc_pos,
                total_usable_relocs,
                hbf,
            )?;
            *used_relocs = 0usize;
            // Check if we have more relocs
            if *usable_relocs == 0 {
                assert_eq!(*total_usable_relocs, 0);
                break;
            }
        }
        // --> Search for a relocation for this position
        let hbf_rel_pos = last_written_offset + pos as u32;
        // As relocs are in-order, check always only the first element
        if hbf_rel_pos == reloc_buffer[*used_relocs] {
            // Update indices
            *used_relocs += 1usize;
            // Apply the relocation
            let addr = u32_from_le_bytes(&buffer[pos..pos + 4]);
            let new_addr = addr - ORIGINAL_FLASH_ADDR + new_dest_address;
            let new_addr_bytes = new_addr.to_le_bytes();
            for i in 0..4usize {
                buffer[pos + i] = new_addr_bytes[i];
            }
        }
        pos += 4;
    }
    Ok(())
}

fn update_checksum(checksum: &mut u32, bytes: &[u8]) {
    for i in (0..bytes.len()).step_by(4) {
        // Read 4 bytes
        let word: u32 = u32_from_le_bytes(&bytes[i..i + 4]);
        *checksum ^= word;
    }
}

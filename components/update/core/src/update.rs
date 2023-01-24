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
    // -----------------------------
    //    Step 1: Fixed Header
    // -----------------------------
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
    let checksum_offset = hbf.checksum_offset().unwrap_lite();
    drop(hbf); // Force ourselves not to use this object anymore

    let mut validation_checksum: u32 = 0; // Contains the checksum before any relocation is applied, to ensure data is received correctly
    let mut new_checksum: u32; // Contains the new checksum, computed after such relocations

    // Start computing the new checksum from the untouched bytes of the fixed header.
    update_checksum(
        &mut validation_checksum,
        &hbf_header_buff[0..hbf_header_buff.len() - 1],
    );

    // ------------------------------------------------------------------------------
    //    Step 2: Flush such fixed header to flash.
    //            This is needed as we cannot store in SRAM the variable
    //            header, containing the dependencies needed for the validation.
    // -------------------------------------------------------------------------------
    let mut curr_pos: u32 = 0;
    // TODO: this step is critical. We cannot force flush now as otherwise we risk problems
    // due to the write granularity. Still we don't access fields of the main header, so
    // it's safe.
    deallocate_on_error(
        storage
            .write_stream(
                allocation.flash_base_address,
                curr_pos,
                &hbf_header_buff[0..hbf_header_buff.len() - 1],
                false  
            )
            .map_err(|_| MessageError::FlashError),
        &mut storage,
        allocation.flash_base_address,
    )?;
    curr_pos += hbf_header_buff.len() as u32 - 1;

    // -------------------------------------
    //    Step 3: Ask for variable header
    // -------------------------------------
    let mut to_read: usize = hbf_lite::REGION_SIZE * header_base.num_regions() as usize
        + hbf_lite::INTERRUPT_SIZE * header_base.num_interrupts() as usize
        + hbf_lite::RELOC_SIZE * header_base.num_relocations() as usize
        + hbf_lite::DEPENDENCY_SIZE * header_base.num_dependencies() as usize
        + header_base.padding_bytes() as usize;

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
            true   // !! -> critical here, as in the next step all the header should be correctly readeable
        ),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // --------------------------------------------------------------------------------
    //    Step 4: Now the entire header has been received and it's stored on flash.
    //            Perform all the validations (i.e. dependencies check).
    // --------------------------------------------------------------------------------
    let flash_reader = FlashReader::from(allocation.flash_base_address, allocation.flash_size);

    sys_log!("Reading HBF from flash");

    let flash_hbf = deallocate_on_error(
        wrap_hbf_error(HbfFile::from_reader(&flash_reader)),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // Before reading the payload, validate the dependencies of this component
    sys_log!("Checking dependencies");
    deallocate_on_error(
        validate_version_and_dependencies(&flash_hbf, &mut storage, allocation.flash_base_address),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // ------------------------------------------------------------------------
    //    Step 5: Receive the HBF payload, and apply the needed relocations.
    // ------------------------------------------------------------------------

    // From this point on, the validation and new checksum are different.
    // This is because of the relocations applied to the data.
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
    let total_relocs: usize = flash_hbf.header_base().unwrap_lite().num_relocations() as usize;
    let mut total_used_relocs: usize = 0;
    let mut total_usable_relocs: usize = total_relocs;
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
                    &mut total_used_relocs,
                    total_relocs,
                    &flash_hbf,
                )?;
                // After fixes, compute also the new checksum
                update_checksum(&mut new_checksum, buffer);
                Ok(())
            },
            false
        ),
        &mut storage,
        allocation.flash_base_address,
    )?;

    // -----------------------------------------------------------------
    //    Step 6: Receive the HBF trailer, and validate total checksum
    // -----------------------------------------------------------------
    static_assertions::const_assert_eq!(hbf_lite::HBF_TRAILER_SIZE, 4);

    let mut hbf_trailer_buff: [u8; 4] = [0x00; 4];
    sys_log!("Waiting for trailer");

    channel_ask(
        channel,
        ComponentUpdateCommand::SendComponentTrailer as u8,
        &mut hbf_trailer_buff,
    )?;

    // Read the trailer (raw)
    let original_checksum = u32::from_le_bytes(hbf_trailer_buff);
    
    // Validate checksum and write the new checksum in flash
    let new_checksum_bytes = new_checksum.to_le_bytes();
    if validation_checksum != original_checksum
        || storage
            .write_stream(
                allocation.flash_base_address,
                checksum_offset,
                &new_checksum_bytes,
                true    // !!--- important to flush, as later the validation will need the whole hbf stored
            )
            .is_err()
    {
        // Deallocate the space
        storage
            .deallocate_block(allocation.flash_base_address)
            .unwrap_lite();
        return Err(MessageError::FailedHBFValidation);
    }

    // -----------------------------------------------------------------
    //    Step 7: Flush write buffer and validate using library
    // -----------------------------------------------------------------

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
    flush_after: bool,
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
            .write_stream(flash_base, *curr_pos, &data[0..min_to_read], flush_after)
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
    let mut solved_dependencies: u16 = 0;
    // Prepare for iterating over the blocks
    // Get block stats
    let flash_status = storage
        .report_status()
        .map_err(|_| MessageError::FlashError)?;
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
            }
            // Unfortunately, as components are not ordered in flash, we have to iterate over all
            // dependencies for each block.
            if solved_dependencies < num_dependencies {
                // Only makes sense if we miss something
                for dep_num in 0..num_dependencies {
                    let dep = wrap_hbf_error(hbf.dependency_nth(dep_num))?;
                    // Check version
                    if dep.min_version() > 0 && comp_data.component_version() < dep.min_version() {
                        // Wrong version (lower bound)
                        return Err(MessageError::DependencyError);
                    } else if dep.max_version() > 0
                        && comp_data.component_version() > dep.max_version()
                    {
                        // Wrong version (upper bound)
                        return Err(MessageError::DependencyError);
                    }
                    solved_dependencies += 1;
                }
            }
        }
    }
    // At the end, the number of solved dependencies gives the result
    if solved_dependencies == num_dependencies {
        return Ok(());
    } else {
        // Missing dependency
        return Err(MessageError::MissingDependency);
    }
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
    total_used_relocs: &mut usize,
    total_relocs: usize,
    hbf: &HbfFile,
) -> Result<(), MessageError> {
    // If we have no relocation still available, ignore
    if *total_used_relocs == total_relocs {
        sys_log!("Finished relocs");
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
                sys_log!("No more relocs");
                assert_eq!(*total_usable_relocs, 0);
                break;
            }
        }
        // --> Search for a relocation for this position
        let hbf_rel_pos = last_written_offset + pos as u32;
        // As relocs are in-order, check always only the first element
        if hbf_rel_pos == reloc_buffer[*used_relocs] {
            // Update indices
            sys_log!("Used relocation");
            *used_relocs += 1usize;
            *total_used_relocs += 1usize;
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

use crate::consts::*;
use crate::messages::*;
use crate::utils::u32_from_le_bytes;
use crate::utils::wrap_hbf_error;
use crate::utils::FlashReader;
use hbf_lite::HbfHeaderBase;
use hbf_lite::{BufferReaderImpl, HbfFile};
use relocator::Relocator;
use storage_api::*;
use uart_channel_api::*;
use userlib::flash::BlockType;
use userlib::sys_log;
use userlib::UnwrapLite;

/*
   Some objects that will become useful later
*/
/// This object deallocates the provisioned area if an error occurs.
/// It's used for simplicity, still it's necessary to craft a secondary mutable reference
/// to the storage allocator: here it does not constitute a problem, as those methods actually
/// does not mutate any state internally to the allocator (there is no such state currently).
struct UpdateMethods<'a> {
    memory_pointer: u32,
    storage: Storage,
    channel: &'a mut UartChannel,
}

impl<'a> UpdateMethods<'a> {
    pub fn methods_for_requirements(
        needed_flash: u32,
        needed_sram: u32,
        channel: &'a mut UartChannel,
    ) -> Result<(Self, AllocateComponentResponse), StorageError> {
        let mut storage = Storage::new();
        let allocation = storage.allocate_component(needed_flash, needed_sram)?;
        Ok((
            Self {
                memory_pointer: allocation.flash_base_address,
                storage: storage,
                channel: channel,
            },
            allocation,
        ))
    }
    pub fn storage_write_stream(
        &mut self,
        offset: u32,
        data: &[u8],
        flush_after: bool,
    ) -> Result<(), MessageError> {
        self.storage
            .write_stream(self.memory_pointer, offset, data, flush_after)
            .map_err(|_| MessageError::FlashError)
    }

    pub fn storage_read_stream(
        &mut self,
        block_base_address: u32,
        offset: u32,
        data: &mut [u8],
    ) -> Result<(), MessageError> {
        self.storage
            .read_stream(block_base_address, offset, data)
            .map_err(|_| MessageError::FlashError)
    }

    pub fn storage_status(&mut self) -> Result<ReportStatusResponse, MessageError> {
        self.storage
            .report_status()
            .map_err(|_| MessageError::FlashError)
    }

    pub fn storage_get_nth_block(&mut self, nth: u32) -> Result<GetNthBlockResponse, MessageError> {
        self.storage
            .get_nth_block(nth)
            .map_err(|_| MessageError::FlashError)
    }
    pub fn channel_write_single(&mut self, value: u8) -> Result<(), MessageError> {
        crate::utils::channel_write_single(&mut self.channel, value)
    }
    pub fn channel_ask(&mut self, cmd: u8, buffer: &mut [u8]) -> Result<(), MessageError> {
        crate::utils::channel_ask(&mut self.channel, cmd, buffer)
    }

    pub fn deallocate(&mut self) {
        self.storage.deallocate_block(self.memory_pointer).unwrap_lite();
    }
}

struct ChecksumBuff {
    buff: [u8; 4],
    pos: usize,
}

impl ChecksumBuff {
    pub fn new() -> Self {
        Self {
            buff: [0x00; 4],
            pos: 0,
        }
    }
    pub fn compute(&mut self, checksum: &mut u32, src: &[u8]) {
        for i in 0..src.len() {
            // Append to the buffer
            self.buff[self.pos] = src[i];
            self.pos += 1;
            // Check if we got a full word
            if self.pos % 4 == 0 {
                update_checksum(checksum, &mut self.buff);
                self.pos = 0;
            }
        }
    }
}

struct UpdateRelocator<'a, 'hbf, 'um> {
    hbf: &'a HbfFile<'hbf>,
    methods: &'a mut UpdateMethods<'um>,
    num_relocations: usize, // Cached here to simplify code
    checksum: &'a mut u32,
}

impl<'a, 'hbf, 'um> relocator::RelocatorMethods<ChecksumBuff> for UpdateRelocator<'a, 'hbf, 'um> {
    fn read_relocations(
        &self,
        start_index: usize,
        dst: &mut [u32],
        _checksum_buff: &mut ChecksumBuff,
    ) -> Result<usize, ()> {
        // Compute the available relocations
        assert!(start_index < self.num_relocations);
        assert!(dst.len() <= self.num_relocations - start_index);
        // Copy such relocations in the buffer
        for r in 0..dst.len() {
            let reloc = self
                .hbf
                .relocation_nth((start_index + r) as u32)
                .map_err(|_| ())?;
            dst[r] = reloc.value();
        }
        return Ok(dst.len());
    }
    fn flush(
        &mut self,
        position: usize,
        src: &[u8],
        checksum_buff: &mut ChecksumBuff,
    ) -> Result<(), ()> {
        // After fixes, compute also the new checksum
        checksum_buff.compute(self.checksum, src);
        // Flush buffer
        let r = self
            .methods
            .storage_write_stream(position as u32, src, false)
            .map_err(|_| ());
        r
    }
}

fn read_exact_bytes<F, D>(
    methods: &mut UpdateMethods,
    mut bytes_to_read: usize,
    validation_checksum: &mut u32,
    aux_data: &mut D,
    mut buffer_process: F,
) -> Result<(), MessageError>
where
    F: FnMut(&mut UpdateMethods, &mut [u8], &mut D) -> Result<(), MessageError>,
{
    let mut pkt_buffer: [u8; PACKET_BUFFER_SIZE + 1] = [0x00; PACKET_BUFFER_SIZE + 1];
    let mut min_to_read = core::cmp::min(PACKET_BUFFER_SIZE, bytes_to_read);

    while bytes_to_read > 0 {
        // 1. Ask for another packet
        methods.channel_ask(
            ComponentUpdateCommand::SendNextFragment as u8,
            &mut pkt_buffer[0..min_to_read + 1],
        )?;
        // 2. Validate this packet
        RawPacket::validate(&pkt_buffer[0..min_to_read + 1])?;

        // 3. Update the validation checksum with this data
        update_checksum(validation_checksum, &pkt_buffer[0..min_to_read]);

        // 4. Process the buffer (if needed)
        buffer_process(methods, &mut pkt_buffer[0..min_to_read], aux_data)?;

        // 5. Get ready for the next data
        bytes_to_read -= min_to_read;
        min_to_read = core::cmp::min(PACKET_BUFFER_SIZE, bytes_to_read);
    }

    Ok(())
}

/// Scans the system to verify if all the dependencies of this component
/// are satisfied
fn validate_component_version_and_dependencies(
    hbf: &HbfFile,
    methods: &mut UpdateMethods,
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
    let flash_status = methods.storage_status()?;
    // Iterate all blocks
    for block_num in 0..flash_status.blocks {
        // Get block
        let block = methods.storage_get_nth_block(block_num)?;
        // Skip the current block!
        if block.block_base_address == block_base_address {
            continue;
        }
        if block.block_type == BlockType::COMPONENT {
            // Read hbf header
            let mut buff: [u8; hbf_lite::HBF_HEADER_MIN_SIZE] =
                [0x00; hbf_lite::HBF_HEADER_MIN_SIZE];
            methods.storage_read_stream(block.block_base_address, 0, &mut buff)?;
            // Parse it
            let reader = hbf_lite::BufferReaderImpl::from(&buff);
            let comp_hbf = wrap_hbf_error(hbf_lite::HbfFile::from_reader(&reader))?;
            let comp_data = wrap_hbf_error(comp_hbf.header_base())?;
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

fn add_update_core(
    methods: &mut UpdateMethods,
    allocation: &AllocateComponentResponse,
    checksum_offset: u32,
    header_base: &HbfHeaderBase,
    fhm: &FixedHeaderMessage,
) -> Result<(), MessageError> {
    // NOTE: here we have to work with two checksums:
    //       -> validation_checksum: checksum computed on the original data that is sent from the update
    //                               server. Needed to ensure that the original image was received correctly
    //       -> new_checksum: checksum we compute on the modified image (i.e. due to relocations),
    //                        that will replace the original one to ensure at start-up this component
    //                        can pass validation.

    let mut curr_pos: u32 = 0; // Tracks the current position in the image

    let mut validation_checksum: u32 = 0;
    let mut new_checksum: u32;

    // Start computing the new checksum from the untouched bytes of the fixed header.
    update_checksum(
        &mut validation_checksum,
        fhm.get_raw(), //&hbf_header_buff[0..hbf_header_buff.len() - 1],
    );

    // ------------------------------------------------------------------------------
    //    Step 2: Flush such fixed header to flash.
    //            This is needed as we cannot store in SRAM the variable
    //            header, containing the dependencies needed for the validation.
    // -------------------------------------------------------------------------------
    // NOTE: We cannot force flush now as otherwise we risk problems
    // due to the write granularity. Still we don't access fields of the main header, so
    // it's safe.
    methods.storage_write_stream(curr_pos, fhm.get_raw(), false)?;
    curr_pos += fhm.get_raw().len() as u32;

    // -------------------------------------
    //    Step 3: Receive variable header
    // -------------------------------------
    let mut to_read: usize = hbf_lite::REGION_SIZE * header_base.num_regions() as usize
        + hbf_lite::INTERRUPT_SIZE * header_base.num_interrupts() as usize
        + hbf_lite::RELOC_SIZE * header_base.num_relocations() as usize
        + hbf_lite::DEPENDENCY_SIZE * header_base.num_dependencies() as usize
        + header_base.padding_bytes() as usize;

    methods.channel_write_single(ComponentUpdateCommand::SendComponentVariableHeader as u8)?;

    sys_log!("Waiting for variable header");

    read_exact_bytes(
        methods,
        to_read,
        &mut validation_checksum,
        &mut curr_pos,
        |methods, data, curr_pos| {
            // Flush data to storage as it is
            methods.storage_write_stream(*curr_pos, data, false)?;
            *curr_pos += data.len() as u32;
            Ok(())
        },
    )?;
    // Force a flush, as now we must be able to read the whole HBF header
    methods.storage_write_stream(curr_pos, &[], true)?;

    // --------------------------------------------------------------------------------
    //    Step 4: Now the entire header has been received and it's stored on flash.
    //            Perform all the validations (i.e. dependencies check).
    // --------------------------------------------------------------------------------
    let flash_reader = FlashReader::from(allocation.flash_base_address, allocation.flash_size); // TODO: remove the second mutable reference to Storage created here

    sys_log!("Reading HBF from flash");
    let flash_hbf = wrap_hbf_error(HbfFile::from_reader(&flash_reader))?;

    // Before reading the payload, validate the dependencies of this component
    sys_log!("Checking dependencies");
    validate_component_version_and_dependencies(
        &flash_hbf,
        methods,
        allocation.flash_base_address,
    )?;

    // ------------------------------------------------------------------------
    //    Step 5: Receive the HBF payload, and apply the needed relocations.
    // ------------------------------------------------------------------------

    // From this point on, the validation and new checksum are different.
    // This is because of the relocations are going to be applied.
    new_checksum = validation_checksum;

    // Now at the same way, read the payload
    to_read = wrap_hbf_error(flash_hbf.payload_size())? as usize;

    // Prepare the relocator
    let num_relocations = wrap_hbf_error(flash_hbf.header_base())?.num_relocations();
    let payload_start_offset = wrap_hbf_error(flash_hbf.get_readonly_payload())?.get_offset();
    let new_flash_base_address: u32 = allocation.flash_base_address + 8 + payload_start_offset;

    let mut relocator =
        Relocator::<LINKED_FLASH_BASE, LINKED_SRAM_BASE, BUFF_SIZE, RELOC_BUFF_SIZE>::new(
            new_flash_base_address,
            allocation.ram_base_address,
            curr_pos as usize,
            num_relocations as usize,
        );

    // Ask for it
    methods.channel_write_single(ComponentUpdateCommand::SendComponentPayload as u8)?;
    sys_log!("Waiting for payload");

    let mut checksum_buff = ChecksumBuff::new();

    // Read every byte
    read_exact_bytes(
        methods,
        to_read,
        &mut validation_checksum,
        &mut (
            &flash_hbf,
            num_relocations as usize,
            &mut relocator,
            &mut new_checksum,
            &mut checksum_buff,
        ),
        |methods, data, aux_data| {
            // Create relocator methods
            let (hbf, num_relocs, relocator, checksum, checksum_buff) = aux_data;
            let mut reloc_methods = UpdateRelocator {
                hbf: *hbf,
                methods: methods,
                num_relocations: *num_relocs,
                checksum: *checksum,
            };
            // Process buffer
            relocator
                .consume_current_buffer(data, &mut reloc_methods, *checksum_buff)
                .map_err(|_| MessageError::FlashError)?;
            Ok(())
        },
    )?;

    // Finish the relocator operations
    let mut reloc_methods = UpdateRelocator {
        hbf: &flash_hbf,
        methods: methods,
        num_relocations: num_relocations as usize,
        checksum: &mut new_checksum,
    };
    relocator
        .finish(&mut reloc_methods, &mut checksum_buff)
        .map_err(|_| MessageError::FlashError)?;

    // -----------------------------------------------------------------
    //    Step 6: Receive the HBF trailer, and validate total checksum
    // -----------------------------------------------------------------
    static_assertions::const_assert_eq!(hbf_lite::HBF_TRAILER_SIZE, 4);

    let mut hbf_trailer_buff: [u8; 4] = [0x00; 4];
    sys_log!("Waiting for trailer");

    methods.channel_ask(
        ComponentUpdateCommand::SendComponentTrailer as u8,
        &mut hbf_trailer_buff,
    )?;

    // Read the trailer (raw)
    let original_checksum = u32::from_le_bytes(hbf_trailer_buff);

    // Validate checksum and write the new checksum in flash
    let new_checksum_bytes = new_checksum.to_le_bytes();
    if validation_checksum != original_checksum
        || methods
            .storage_write_stream(
                checksum_offset,
                &new_checksum_bytes,
                true, // !!--- important to flush, as later the validation will need the whole hbf stored
            )
            .is_err()
    {
        return Err(MessageError::FailedHBFValidation);
    }

    // -----------------------------------------------------------------
    //    Step 7: Flush write buffer and validate using library
    // -----------------------------------------------------------------

    // Validate the HBF (last one, to ensure the library reads it correctly)
    if !wrap_hbf_error(flash_hbf.validate())? {
        return Err(MessageError::FailedHBFValidation);
    }
    Ok(())
}

pub fn component_add_update(channel: &mut UartChannel) -> Result<(), MessageError> {
    // -----------------------------
    //    Step 1: Fixed Header
    // -----------------------------
    let mut hbf_header_buff: [u8; FixedHeaderMessage::get_size()] =
        [0; FixedHeaderMessage::get_size()];
    crate::utils::channel_ask(
        channel,
        ComponentUpdateCommand::SendComponentFixedHeader as u8,
        &mut hbf_header_buff,
    )?;
    // Validate header
    let fhm = FixedHeaderMessage::from(&mut hbf_header_buff)?;
    // Read hbf
    let hbf_reader = BufferReaderImpl::from(fhm.get_raw());
    let hbf = wrap_hbf_error(hbf_lite::HbfFile::from_reader(&hbf_reader))?;
    // Cache only the needed values
    let header_base = wrap_hbf_error(hbf.header_base())?;
    let needed_flash = wrap_hbf_error(hbf.header_base())?.total_size();
    let needed_ram = wrap_hbf_error(hbf.header_main())?.component_min_ram();
    let checksum_offset = wrap_hbf_error(hbf.checksum_offset())?; // safe call
    drop(hbf); // Force ourselves not to use this object anymore

    // Request the allocation
    let (mut methods, allocation) =
        UpdateMethods::methods_for_requirements(needed_flash, needed_ram, channel).map_err(
            |e| // Fail if no space available
        match e {
            StorageError::OutOfFlash | StorageError::OutOfRam => MessageError::NotEnoughSpace,
            _ => MessageError::FlashError,
        },
        )?;
    // Process everything
    add_update_core(
        &mut methods,
        &allocation,
        checksum_offset,
        &header_base,
        &fhm,
    )
    .map_err(|e| {
        // Whatever, deallocate the block
        methods.deallocate();
        // Return the error
        e
    })?;
    // Start component, do stuff ...
    sys_log!("Try to start component");
    if !userlib::kipc::load_component(allocation.flash_base_address) {
        return Err(MessageError::CannotStartComponent);
    }
    sys_log!("Component started!");
    // Respond (at this point, do not delete the component if we just fail to send the end byte)
    methods.channel_write_single(ComponentUpdateResponse::Success as u8)
}

fn update_checksum(checksum: &mut u32, bytes: &[u8]) {
    for i in (0..bytes.len()).step_by(4) {
        // Read 4 bytes
        let word: u32 = u32_from_le_bytes(&bytes[i..i + 4]);
        *checksum ^= word;
    }
}

use crate::{
    startup::{with_irq_table, HUBRIS_STORAGE_ANALYZE_NOTIFICATION},
    sys_log,
    task::{NotificationSet, Task},
    utils::{get_task, get_task_mut},
};
use abi::{
    flash::BlockType, u32_from_le_bytes_raw, InterruptOwner, RegionAttributes,
    RegionDescriptor, TaskDescriptor, TaskFlags, HUBRIS_MAX_IRQS,
    HUBRIS_MAX_SUPPORTED_TASKS, REGIONS_PER_TASK,
};
use flash_allocator::flash::FlashBlock;
use hbf_lite::{BufferReaderImpl, HbfFile};
use heapless::{FnvIndexMap, Vec};
use unwrap_lite::UnwrapLite;

pub fn populate_kernel_structures(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
) {
    // Get an iterator for the flash
    let flash_walker = crate::arch::get_flash_walker();
    // Iterate, to find HBFs
    for b in flash_walker {
        if !b.is_finalized() {
            // Mark as dismissed, will be cleaned from the storage component
            // when it starts
            /*unsafe {
                if crate::arch::dismiss_block(b.get_base_address()).is_err() {
                    panic!(
                        "Cannot dismiss non finalized block at: {}",
                        b.get_base_address()
                    );
                }
            }*/
            sys_log!(
                "Not finalized block found at: {}",
                b.get_base_address()
            );
            continue;
        }
        // Look into only finalized blocks of components
        if b.is_finalized() && b.get_type() == BlockType::COMPONENT {
            // Load the component
            let task = get_task_from_block(b, true).unwrap();
            let task_id = task.descriptor().component_id();
            add_task_to_system(task_map, irq_map, task, task_id).unwrap();
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LoadError {
    InvalidBlockPointer,
    InvalidBlock,
    MalformedHBF,
    TooManyIRQs,
    TooManyTasks,
}

fn get_task_from_block(block: FlashBlock, validate: bool) -> Result<Task, LoadError> {
    // Let's create an abstraction to read its bytes
    let raw_block_bytes = unsafe {
        core::slice::from_raw_parts(
            (block.get_base_address() + 8) as *const u8,
            block.get_size() as usize,
        )
    };
    let block_reader = BufferReaderImpl::from(raw_block_bytes);
    // Let's read the hbf
    let hbf_parse = hbf_lite::HbfFile::from_reader(&block_reader);
    if hbf_parse.is_err() {
        sys_log!("Malformed HBF at {:#010x}", block.get_base_address());
        return Err(LoadError::MalformedHBF);
    }
    let hbf = hbf_parse.unwrap();
    if validate {
        // Validate this hbf
        let hbf_valid = hbf.validate().unwrap_or(false);
        if !hbf_valid {
            sys_log!("Malformed HBF at {:#010x}", block.get_base_address());
            return Err(LoadError::MalformedHBF);
        }
    }
    Ok(process_hbf(
        &hbf,
        block.get_nominal_base_address(),
        block.get_base_address(),
        block.get_nominal_size(),
    ))
}

fn process_hbf(
    hbf: &HbfFile,
    block_nominal_base_address: u32,
    block_base_address: u32,
    block_nominal_size: u32,
) -> Task {
    // Create a new instance of Task
    let task_desc = TaskDescriptor::new(block_base_address, block_nominal_size); // Nominal size is actually bigger than needed. It's only used for hbf reading so here it's okay
    let mut regions: Vec<RegionDescriptor, REGIONS_PER_TASK> = Vec::new();
    // Create a region for the SRAM
    let sram_base: u32 = unsafe { u32_from_le_bytes_raw(block_base_address) };
    let sram_size: u32 =
        unsafe { u32_from_le_bytes_raw(block_base_address + 4) };
    let sram_region = RegionDescriptor {
        base: sram_base,
        size: sram_size,
        attributes: RegionAttributes::READ
            | RegionAttributes::WRITE
            | RegionAttributes::EXECUTE,
    };
    regions.push(sram_region).unwrap();
    // Create a sregion for the FLASH
    let flash_region = RegionDescriptor {
        base: block_nominal_base_address,
        size: block_nominal_size,
        attributes: RegionAttributes::READ
            | RegionAttributes::WRITE
            | RegionAttributes::EXECUTE,
    };
    regions.push(flash_region).unwrap();
    let hbf_base = hbf.header_base().unwrap();
    // Append all the other regions
    for region_num in 0..hbf_base.num_regions() {
        let region = hbf.region_nth(region_num).unwrap();
        regions
            .push(RegionDescriptor {
                base: region.base_address(),
                size: region.size(),
                attributes: unsafe {
                    RegionAttributes::from_bits_unchecked(
                        region.attributes().bits(),
                    )
                },
            })
            .unwrap();
    }
    // Extract the data section
    let data_section = hbf.get_data_payload().unwrap_lite();
    let mut data_section_slice: &'static [u8] = &[];
    if data_section.is_some() {
        let ds = data_section.unwrap_lite();
        let data_address = block_base_address + 8 + ds.get_offset();
        data_section_slice = unsafe {
            core::slice::from_raw_parts(
                data_address as *const u8,
                ds.size() as usize,
            )
        };
    }
    // Create the task structure
    Task::from_descriptor(&task_desc, &regions, data_section_slice)
}

fn remove_task_from_system(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
    task_id: u16,
) {
    // Start by flushing IRQs
    let task_search = task_map.get(&task_id);
    if task_search.is_none() {
        return; // Simply ignore
    }
    let task = task_search.unwrap_lite();
    for interrupt_num in 0..task.descriptor().num_interrupts() {
        let interrupt = task.descriptor().interrupt_nth(interrupt_num);
        irq_map.remove(&interrupt.irq_num).unwrap_lite();
    }
    // Mark the corresponding block for removal
    unsafe {
        crate::arch::dismiss_block(task.descriptor().get_descriptor_block())
            .unwrap_lite();
    }
    // Remove the task
    task_map.remove(&task_id).unwrap_lite();
}

fn add_task_to_system(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
    task: Task,
    use_id: u16,
) -> Result<(), LoadError> {
    // First, check if this component already exists
    let search_result = task_map.get(&use_id);
    if search_result.is_some() {
        // Check the versions, if this is newer let's override everything
        let other_task = search_result.unwrap_lite();
        if task.descriptor().component_version()
            > other_task.descriptor().component_version()
        {
            sys_log!("Found an newer task for {}", use_id);
            // Delete the old task
            remove_task_from_system(task_map, irq_map, use_id);
        } else {
            sys_log!("Found an older task for {}", use_id);
            // Ignore this task
            return Ok(()); // TODO: maybe an error is better here?
        }
    }
    // Add the IRQs managed by this component
    let num_irqs = task.descriptor().num_interrupts();
    for interrupt_num in 0..num_irqs {
        let interrupt = task.descriptor().interrupt_nth(interrupt_num);
        // Append the IRQ
        match irq_map.insert(
            interrupt.irq_num,
            InterruptOwner {
                task_id: use_id,
                notification: interrupt.notification,
            },
        ) {
            Ok(old_val) => {
                if old_val.is_some() {
                    // Another component registered this IRQ, panic!
                    panic!("Duplicated IRQ: {}", interrupt.irq_num);
                }
            }
            Err(_) => {
                // TODO: clean-up
                return Err(LoadError::TooManyIRQs);
            }
        };
    }
    // Insert the component
    match task_map.insert(use_id, task) {
        Ok(old_val) => {
            if old_val.is_some() {
                // Should be impossible, still
                panic!("The specified ID already exists");
            }
        }
        Err(_) => {
            // TODO: clean-up interrupts, recover old task ones
            // if this was live insertion
            return Err(LoadError::TooManyTasks);
        }
    };
    Ok(())
}

/// Insert a component in the system, and executes it
pub fn load_component_at(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
    block_base_address: u32,
) -> Result<(), LoadError> {
    // Try to read this header (already checked for addresses)
    let block_header_search =
        crate::arch::get_flash_block(block_base_address, false);
    if block_header_search.is_none() {
        return Err(LoadError::InvalidBlock);
    }
    let block = block_header_search.unwrap_lite();
    // Check if this block has a component (but allow unfinalized blocks!)
    if block.get_type() != BlockType::COMPONENT {
        return Err(LoadError::InvalidBlock);
    }
    // Load the component
    let load_result = get_task_from_block(block, false); // Assume already validated
    if load_result.is_err() {
        return Err(load_result.unwrap_err());
    }
    let mut task = load_result.unwrap();
    // Check if an older component with this ID exist
    let nominal_id = task.descriptor().component_id();
    let task_search = task_map.get_mut(&nominal_id);
    if task_search.is_some() {
        let old_task = task_search.unwrap();
        // Remove all its irqs, after disabling them
        for interrupt_num in 0..old_task.descriptor().num_interrupts() {
            let interrupt = old_task.descriptor().interrupt_nth(interrupt_num);
            crate::arch::disable_irq(interrupt.irq_num);
            irq_map.remove(&interrupt.irq_num).unwrap();
        }
        // If the old component support it, now it can state transfer.
        // Otherwise is simply stopped.
        old_task.begin_state_transfer();
    }
    // Initialize the task for update
    task.begin_update();
    // Actually add the component to the map
    let new_id = abi::UPDATE_TEMP_ID;
    let res = add_task_to_system(task_map, irq_map, task, new_id);
    if res.is_ok() {
        // Initialize task
        let task = task_map.get_mut(&new_id).unwrap_lite();
        crate::arch::reinitialize(task);
    }
    res
}

pub fn revert_update(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
) {
    sys_log!("Reverting update for timeout");
    // Search for the new component
    let task_new = task_map.get(&abi::UPDATE_TEMP_ID);
    if task_new.is_none() {
        return; // Ignore
    }
    let nominal_id = task_new.unwrap_lite().descriptor().component_id();
    // Delete the new task
    with_irq_table(|irq_map| {
        remove_task_from_system(task_map, irq_map, abi::UPDATE_TEMP_ID);
    });
    // Get the old one
    let old_task = task_map.get_mut(&nominal_id);
    if old_task.is_none() {
        // Remove the new version
        get_task_mut(task_map, abi::STORAGE_ID)
            .post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
        return; // Ignore, we have nothing to revert to
    }
    // Re-map IRQs of the old one. Do not reenable them, as the task will restart
    // and enable them itself.
    with_irq_table(|irq_map| {
        let task = get_task(task_map, nominal_id);
        for interrupt_num in 0..task.descriptor().num_interrupts() {
            let interrupt = task.descriptor().interrupt_nth(interrupt_num);
            irq_map
                .insert(
                    interrupt.irq_num,
                    InterruptOwner {
                        task_id: nominal_id,
                        notification: interrupt.notification,
                    },
                )
                .unwrap_lite();
        }
    });
    // Reset the old one
    let old_task = get_task_mut(task_map, nominal_id);
    old_task.reinitialize();
    if old_task
        .descriptor()
        .flags()
        .contains(TaskFlags::START_AT_BOOT)
    {
        old_task.set_healthy_state(abi::SchedState::Runnable);
    }
    // Remove the new version
    get_task_mut(task_map, abi::STORAGE_ID)
        .post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
}

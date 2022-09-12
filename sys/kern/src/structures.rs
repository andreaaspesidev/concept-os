use crate::{
    flash::FlashReader,
    startup::with_irq_table,
    sys_log,
    task::Task,
    utils::{get_task, get_task_mut},
};
use abi::{
    flash::BlockType, u32_from_le_bytes_raw, InterruptOwner, RegionAttributes,
    RegionDescriptor, TaskDescriptor, TaskFlags, HUBRIS_MAX_IRQS,
    HUBRIS_MAX_SUPPORTED_TASKS, REGIONS_PER_TASK,
};
use flash_allocator::flash::{walker::FlashWalkerImpl, FlashBlock};
use hbf_lite::{BufferReaderImpl, HbfFile};
use heapless::{FnvIndexMap, Vec};
use unwrap_lite::UnwrapLite;

pub fn populate_kernel_structures(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
) {
    // For now, create a mock flash interface
    let mut flash_methods =
        FlashReader::<FLASH_ALLOCATOR_START_SCAN_ADDR, FLASH_END_ADDR>::new();
    // Get an iterator for the flash
    let flash_walker = FlashWalkerImpl::<
        FLASH_START_ADDR,
        FLASH_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_NUM_SLOTS,
        FLASH_BLOCK_SIZE,
        FLASH_FLAG_SIZE,
    >::new(&mut flash_methods);
    // Iterate, to find HBFs
    for b in flash_walker {
        if !b.is_finalized() {
            panic!("Non finalized block found at: {}", b.get_base_address());
        }
        if b.get_type() != BlockType::COMPONENT {
            panic!("Non component block found at: {}", b.get_base_address());
        }
        // Look into only finalized blocks of components
        if b.is_finalized() && b.get_type() == BlockType::COMPONENT {
            // Load the component
            let task = get_task_from_block(b).unwrap();
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

fn get_task_from_block(block: FlashBlock) -> Result<Task, LoadError> {
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
    // Validate this hbf
    let hbf_valid = hbf.validate().unwrap_or(false);
    if !hbf_valid {
        sys_log!("Malformed HBF at {:#010x}", block.get_base_address());
        return Err(LoadError::MalformedHBF);
    }
    Ok(process_hbf(
        &hbf,
        block.get_base_address(),
        block.get_size() + 12 + 8,
    ))
}

fn process_hbf(
    hbf: &HbfFile,
    block_base_address: u32,
    block_size: u32,
) -> Task {
    // Create a new instance of Task
    let task_desc = TaskDescriptor::new(block_base_address);
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
        base: block_base_address - 12,
        size: block_size,
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

fn add_task_to_system(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
    task: Task,
    use_id: u16,
) -> Result<(), LoadError> {
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

pub fn load_component_at(
    task_map: &mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>,
    irq_map: &mut FnvIndexMap<u32, InterruptOwner, HUBRIS_MAX_IRQS>,
    block_base_address: u32,
) -> Result<(), LoadError> {
    // Check if at this address we can find a valid block
    if block_base_address < FLASH_ALLOCATOR_START_SCAN_ADDR
        || block_base_address > FLASH_END_ADDR
    {
        return Err(LoadError::InvalidBlockPointer);
    }
    // For now, create a mock flash interface
    let flash_methods =
        FlashReader::<FLASH_ALLOCATOR_START_SCAN_ADDR, FLASH_END_ADDR>::new();
    // Try to read this header
    let block_header_search = flash_allocator::flash::utils::get_flash_block::<
        FLASH_START_ADDR,
        FLASH_END_ADDR,
        FLASH_ALLOCATOR_START_SCAN_ADDR,
        FLASH_NUM_SLOTS,
        FLASH_BLOCK_SIZE,
        FLASH_FLAG_SIZE,
    >(&flash_methods, block_base_address, false);
    if block_header_search.is_none() {
        return Err(LoadError::InvalidBlock);
    }
    let block = block_header_search.unwrap_lite();
    // Check if this block has a component (but allow unfinalized blocks!)
    if block.get_type() != BlockType::COMPONENT {
        return Err(LoadError::InvalidBlock);
    }
    // Load the component
    let load_result = get_task_from_block(block);
    if load_result.is_err() {
        return Err(load_result.unwrap_err());
    }
    let mut task = load_result.unwrap();
    // Check if an older component with this ID exist
    let nominal_id = task.descriptor().component_id();
    let task_search = task_map.get_mut(&nominal_id);
    //let mut can_state_transfer: bool = false;
    if task_search.is_some() {
        // If it has the update handler, force it
        let old_task = task_search.unwrap();
        // Remove all its irqs
        for interrupt_num in 0..old_task.descriptor().num_interrupts() {
            let interrupt = old_task.descriptor().interrupt_nth(interrupt_num);
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
    // Cancel all irqs of the new one
    with_irq_table(|irq_map| {
        let task = get_task(task_map, abi::UPDATE_TEMP_ID);
        for interrupt_num in 0..task.descriptor().num_interrupts() {
            let interrupt = task.descriptor().interrupt_nth(interrupt_num);
            irq_map.remove(&interrupt.irq_num).unwrap_lite();
        }
    });
    // Cancel the new one
    task_map.remove(&abi::UPDATE_TEMP_ID).unwrap_lite();

    // Get the old one
    let old_task = task_map.get_mut(&nominal_id);
    if old_task.is_none() {
        return; // Ignore, we have nothing to revert to
    }
    // Re-map IRQs of the old one
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
}

pub const FLASH_ALLOCATOR_START_SCAN_ADDR: u32 = 0x0804_0000;
pub const FLASH_START_ADDR: u32 = 0x0804_0000;
pub const FLASH_END_ADDR: u32 = 0x0807_FFFF;
pub const FLASH_BLOCK_SIZE: usize = 2048;
pub const FLASH_FLAG_SIZE: usize = 2; // 2 bytes
pub const FLASH_NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1
pub const FLASH_PAGE_SIZE: u32 = 2048;

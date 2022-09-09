// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Kernel startup.

use abi::flash::BlockType;
use abi::{
    u32_from_le_bytes_raw, InterruptOwner, RegionAttributes, RegionDescriptor,
    TaskDescriptor, HUBRIS_MAX_IRQS, HUBRIS_MAX_SUPPORTED_TASKS,
    REGIONS_PER_TASK,
};
use flash_allocator::flash::walker::FlashWalkerImpl;
use hbf_lite::{BufferReaderImpl, HbfFile};
use heapless::{FnvIndexMap, Vec};
use unwrap_lite::UnwrapLite;

use crate::atomic::AtomicExt;
use crate::flash::FlashReader;
use crate::task::Task;
use core::sync::atomic::{AtomicBool, Ordering};

/// Tracks when a mutable reference to the task table is floating around in
/// kernel code, to prevent production of a second one. This forms a sort of
/// ad-hoc Mutex around the task table.
///
/// Notice that this begins life initialized to `true`. This prevents use of
/// `with_task_table` et al before the kernel is properly started. We set it to
/// `false` late in `start_kernel`.
static TASK_TABLE_IN_USE: AtomicBool = AtomicBool::new(true);

pub const HUBRIS_FAULT_NOTIFICATION: u32 = 1;

// These new structures becomes defined in the kernel, instead of being
// generated during build process.

/// Structure of Task structures, sorted by Task ID
static mut TASK_MAP: FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS> =
    FnvIndexMap::new();

/// Structure of IRQs, in order to get to the tasks
pub static mut IRQ_TO_TASK: FnvIndexMap<
    u32,            // IRQ
    InterruptOwner, // Task
    HUBRIS_MAX_IRQS,
> = FnvIndexMap::new();

macro_rules! sys_log {
    ($s:expr) => {
        unsafe {
            let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[0];
            cortex_m::iprintln!(stim, $s);
        }
    };
    ($s:expr, $($tt:tt)*) => {
        unsafe {
            let stim = &mut (*cortex_m::peripheral::ITM::ptr()).stim[0];
            cortex_m::iprintln!(stim, $s, $($tt)*);
        }
    };
}

/// The main kernel entry point.
///
/// We currently expect an application to provide its own `main`-equivalent
/// function, which does basic hardware setup and then calls this function.
///
/// Parameters:
///
/// - `tick_divisor`: a platform-specific way of converting "machine ticks" into
///   "kernel ticks." On ARM M-profile, this is CPU cycles per tick, where a
///   tick is typically a millisecond.
///
/// # Safety
///
/// This function has architecture-specific requirements for safe use -- on ARM,
/// for instance, it must be called from the main (interrupt) stack in
/// privileged mode.
///
/// This function may not be called reentrantly or from multiple cores.
pub unsafe fn start_kernel(tick_divisor: u32) -> ! {
    // Set our clock frequency so debuggers can find it as needed
    //
    // Safety: TODO it is not clear that this operation needs to be unsafe.
    unsafe {
        crate::arch::set_clock_freq(tick_divisor);
    }

    // Load structures from flash
    sync_kernel_structures();

    // Get a safe reference
    let task_map = unsafe { &mut TASK_MAP };

    // Debug!
    sys_log!("--------- Kernel Start ----------");
    // Print components
    for (cid, task) in task_map.iter() {
        sys_log!("- Component with ID: {}", cid);
        // Print component regions
        sys_log!("  Regions:");
        for r in task.region_table() {
            sys_log!("  -Addr: {:#010x}", r.base);
            sys_log!("   Size: {}", r.size);
            sys_log!("   Attr: {:?}", r.attributes);
        }
    }
    // Print interrupts
    let interr_map = unsafe { &mut IRQ_TO_TASK };
    for (irq, owner) in interr_map.iter() {
        sys_log!(
            "- IRQ {} mapped to cid {} on bits {:#010x}",
            irq,
            owner.task_id,
            owner.notification
        );
    }

    // With that done, set up initial register state etc.
    for (_, task) in task_map.iter_mut() {
        crate::arch::reinitialize(task);
    }

    // Great! Pick our first task. We'll act like we're scheduling after the
    // last task, which will cause a scan from 0 on.
    let first_task_id = crate::task::select(
        *task_map.keys().last().expect("No Component Loaded"),
        &task_map,
    );

    let first_task = task_map.get_mut(&first_task_id).expect("Wrong task id");

    // Setup memory protection for this task
    crate::arch::apply_memory_protection(first_task);

    // Mark the table as not in use, so we can now issue `with_task_table`
    TASK_TABLE_IN_USE.store(false, Ordering::Release);

    crate::arch::start_first_task(tick_divisor, first_task)
}

fn sync_kernel_structures() {
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
            // Let's create an abstraction to read its bytes
            let raw_block_bytes = unsafe {
                core::slice::from_raw_parts(
                    (b.get_base_address() + 8) as *const u8,
                    b.get_size() as usize,
                )
            };
            let block_reader = BufferReaderImpl::from(raw_block_bytes);
            // Let's read the hbf
            let hbf_parse = hbf_lite::HbfFile::from_reader(&block_reader);
            if hbf_parse.is_err() {
                panic!("Malformed HBF at {:#010x}", b.get_base_address());
                continue; // Skip
            }
            let hbf = hbf_parse.unwrap();
            // Validate this hbf
            let hbf_valid = hbf.validate().unwrap_or(false);
            if !hbf_valid {
                panic!("Malformed HBF at {:#010x}", b.get_base_address());
                continue; // Skip malformed hbf
            }
            // Process hbf
            update_with_hbf(&hbf, b.get_base_address(), b.get_size() + 12 + 8);
        }
    }
}

fn update_with_hbf(hbf: &HbfFile, block_base_address: u32, block_size: u32) {
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
        attributes: RegionAttributes::READ | RegionAttributes::WRITE | RegionAttributes::EXECUTE,
    };
    regions.push(sram_region).unwrap();
    // Create a sregion for the FLASH
    let flash_region = RegionDescriptor {
        base: block_base_address - 12,
        size: block_size,
        attributes: RegionAttributes::READ | RegionAttributes::WRITE | RegionAttributes::EXECUTE,
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
    // Append the task
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
    let task = Task::from_descriptor(&task_desc, &regions, data_section_slice);
    // Add the IRQs managed by this component
    for interrupt_num in 0..hbf_base.num_interrupts() {
        let interrupt = hbf.interrupt_nth(interrupt_num).unwrap();
        // Append the IRQ
        unsafe {
            let insert_result = IRQ_TO_TASK.insert(
                interrupt.irq_number(),
                InterruptOwner {
                    task_id: task_desc.component_id(),
                    notification: interrupt.notification_mask(),
                },
            );
            if insert_result.is_ok() {
                if insert_result.unwrap().is_some() {
                    // Another component registered this IRQ, panic!
                    panic!("Duplicated IRQ: {}", interrupt.irq_number());
                }
            } else {
                // No more space for IRQs
                panic!("Cannot add more IRQs!");
            }
        }
    }
    // Finally add the component
    unsafe {
        let insert_result = TASK_MAP.insert(task_desc.component_id(), task);
        if insert_result.is_ok() {
            if insert_result.unwrap().is_some() {
                // Another component with the same ID.
                // Ask to terminate, for now panic
                panic!(
                    "Duplicated component found: {}",
                    task_desc.component_id()
                );
            }
        } else {
            // No more space to store components, panic!
            panic!("Cannot add more components!");
        }
    }
}

/// Runs `body` with a reference to the task table.
///
/// To preserve uniqueness of the `&mut` reference passed into `body`, this
/// function will detect any attempts to call it recursively and panic.
pub(crate) fn with_task_table<R>(
    body: impl FnOnce(&mut FnvIndexMap<u16, Task, HUBRIS_MAX_SUPPORTED_TASKS>) -> R,
) -> R {
    if TASK_TABLE_IN_USE.swap_polyfill(true, Ordering::Acquire) {
        panic!(); // recursive use of with_task_table
    }
    // Safety: we have observed `TASK_TABLE_IN_USE` being false, which means the
    // task table is initialized (note that at reset it starts out true) and
    // that we're not already within a call to with_task_table. Thus, we can
    // produce a reference to the task table without aliasing, and we can be
    // confident that the memory it's pointing to is initialized.

    let task_map_ptr = unsafe { &mut TASK_MAP };

    let r = body(task_map_ptr);

    // Mark we are no more using the structure
    TASK_TABLE_IN_USE.store(false, Ordering::Release);

    r
}

pub const FLASH_ALLOCATOR_START_SCAN_ADDR: u32 = 0x0804_0000;
pub const FLASH_START_ADDR: u32 = 0x0804_0000;
pub const FLASH_END_ADDR: u32 = 0x0807_FFFF;
pub const FLASH_BLOCK_SIZE: usize = 2048;
pub const FLASH_FLAG_SIZE: usize = 2; // 2 bytes
pub const FLASH_NUM_SLOTS: usize = 7 + 1; // clog2(NUM_BLOCKS) + 1
pub const FLASH_PAGE_SIZE: u32 = 2048;

use crate::{task::Task, log::sys_log, structures::{TaskIndexes, KHash}};

use abi::{HUBRIS_MAX_IRQS, HUBRIS_MAX_SUPPORTED_TASKS, InterruptOwner};
use unwrap_lite::UnwrapLite;

pub fn log_task(task: &Task) {
    sys_log!(
        "- Component with ID: {} [ident: {}] [orig: {}]",
        task.id(),
        task.current_identifier().0,
        task.descriptor().component_id()
    );
    // Print component regions
    sys_log!("  Regions:");
    for r in task.region_table().into_iter() {
        sys_log!("  -Addr: {:#010x}", r.base);
        sys_log!("   Size: {}", r.size);
        sys_log!("   Attr: {:?}", r.attributes);
    }
    // Print component irqs
    sys_log!("  Interrupts:");
    for interrupt_num in 0..task.descriptor().num_interrupts() {
        let interrupt = task.descriptor().interrupt_nth(interrupt_num);
        sys_log!("  -IRQ: {}", interrupt.irq_num);
        sys_log!("   Mask: {:#010x}", interrupt.notification);
    }
    // Print entrypoint
    sys_log!("  Entrypoint at: {:#010x}", task.descriptor().entry_point());
}

pub fn log_structures(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    irq_map: &mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>,
) {
    // Print components
    for cindex in task_map.indexes().into_iter() {
        log_task(&task_list[*cindex]);
    }
    // Print interrupts
    for irq in irq_map.keys().into_iter() {
        let owner = irq_map.get(*irq).unwrap_lite();
        sys_log!(
            "- IRQ {} mapped to cid {} on bits {:#010x}",
            irq,
            owner.task_id,
            owner.notification
        );
    }
}

use kern::profiling::EventsTable;
use stm32l4::stm32l4x6 as device;

/**
 * Performance Analysis
 * ------------------------------
 * GPIO ports are used. In particular, a part of Port C is used:
 * - PC0 rising: syscall_enter_profile
 * - PC0 falling: syscall_exit_profile
 * - PC1 rising: secondary_syscall_enter_profile
 * - PC1 falling: secondary_syscall_exit_profile
 * - PC2 rising: isr_enter_profile
 * - PC2 falling: isr_exit_profile
 * - PC3 rising: timer_isr_enter_profile
 * - PC3 falling: timer_isr_exit_profile
 *
 * - PC4 (syscall number) bit0
 * - PC5 (syscall number) bit1
 * - PC6 (syscall number) bit2
 * - PC7 (syscall number) bit3
 */

// Create a new static instance of the EventsTable
static EVENT_TABLE: EventsTable = EventsTable {
    syscall_enter: syscall_enter_profile,
    syscall_exit: syscall_exit_profile,
    secondary_syscall_enter: secondary_syscall_enter_profile,
    secondary_syscall_exit: secondary_syscall_exit_profile,
    isr_enter: isr_enter_profile,
    isr_exit: isr_exit_profile,
    timer_isr_enter: timer_isr_enter_profile,
    timer_isr_exit: timer_isr_exit_profile,
    context_switch: context_switch,
};

pub fn configure_profiling() {
    // Turn on GPIO C
    let rcc = unsafe { &*device::RCC::ptr() };
    let pmask: u32 = 1 << 2; // GPIOC
    rcc.ahb2enr
        .modify(|r, w| unsafe { w.bits(r.bits() | pmask) });
    // Setup GPIO C
    let gpioc = unsafe { &*device::GPIOC::PTR };
    // -> set highest speed on all pins
    gpioc.ospeedr.modify(|_, w| {
        w.ospeedr0()
            .very_high_speed()
            .ospeedr1()
            .very_high_speed()
            .ospeedr2()
            .very_high_speed()
            .ospeedr3()
            .very_high_speed()
            .ospeedr4()
            .very_high_speed()
            .ospeedr5()
            .very_high_speed()
            .ospeedr6()
            .very_high_speed()
            .ospeedr7()
            .very_high_speed()
    });
    // -> set push-pull mode for all pins
    gpioc.otyper.modify(|_, w| {
        w.ot0()
            .push_pull()
            .ot1()
            .push_pull()
            .ot2()
            .push_pull()
            .ot3()
            .push_pull()
            .ot4()
            .push_pull()
            .ot5()
            .push_pull()
            .ot6()
            .push_pull()
            .ot7()
            .push_pull()
    }); // Default mode
        // -> set direction as output
    gpioc.moder.modify(|_, w| {
        w.moder0()
            .output()
            .moder1()
            .output()
            .moder2()
            .output()
            .moder3()
            .output()
            .moder4()
            .output()
            .moder5()
            .output()
            .moder6()
            .output()
            .moder7()
            .output()
    });
    // Register the event table
    kern::profiling::configure_events_table(&EVENT_TABLE);
}

fn syscall_enter_profile(number: u32) {
    let gpioc = unsafe { &*device::GPIOC::PTR };
    // Write the lowest 4 bits of the syscall number + mark syscall start
    let syscall_4bits = ((number & 0b1111) << 4) as u32;
    gpioc.odr.modify(|r, w| unsafe {w.bits(
        (r.bits() & (0b0000_u32 << 4_u32)) | syscall_4bits | 1
    )});
}
fn syscall_exit_profile() {
    // Set PC0 to low
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr0().clear_bit());
}

fn secondary_syscall_enter_profile() {
    // Set PC1 to high
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr1().set_bit());
}
fn secondary_syscall_exit_profile() {
    // Set PC1 to low
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr1().clear_bit());
}

fn isr_enter_profile() {
    // Set PC2 to high
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr2().set_bit());
}
fn isr_exit_profile() {
    // Set PC2 to high
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr2().clear_bit());
}

fn timer_isr_enter_profile() {
    // Set PC3 to high
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr3().set_bit());
}
fn timer_isr_exit_profile() {
    // Set PC3 to low
    let gpioc = unsafe { &*device::GPIOC::PTR };
    gpioc.odr.modify(|_, w| w.odr3().clear_bit());
}

fn context_switch(_component_index: u16) {
    // Unused
}

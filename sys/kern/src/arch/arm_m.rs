// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Architecture support for ARMv7.
//!
//! # ARM-M timer
//!
//! We use the system tick timer as the kernel timer, but it's only suitable for
//! producing periodic interrupts -- its counter is small and only counts down.
//! So, at each SysTick interrupt, we increment the `TICKS` global that contains
//! the real kernel timestamp. This has the downside that we take regular
//! interrupts to maintain `TICKS`, but has the upside that we don't need
//! special SoC support for timing.
//!
//! # Notes on ARM-M interrupts
//!
//! For performance and (believe it or not) simplicity, this implementation uses
//! several different interrupt service routines:
//!
//! - `SVCall` implements the `SVC` instruction used to make syscalls.
//! - `SysTick` handles interrupts from the System Tick Timer, used to maintain
//! the kernel timestamp.
//! - `PendSV` handles deferred context switches from interrupts.
//!
//! The first two are expected; the last one's a bit odd and deserves an
//! explanation.
//!
//! It has to do with interrupt latency.
//!
//! On any interrupt, the processor stacks a small subset of machine state and
//! then calls our ISR. Our ISR is a normal Rust function, and follows the
//! normal (C) calling convention: there are some registers that it can use
//! without saving, and there are others it must save first. When the ISR
//! returns, it restores any registers it saved.
//!
//! This is great, as long as the code you're returning to is the *same code
//! that called you* -- but in the case of a context switch, it isn't.
//!
//! There's another problem, which is that we'd like to be able to read the
//! values of some of the user registers for syscall arguments and the like...
//! but if we rely on the automatic saving to put them somewhere on the stack,
//! that "somewhere" is opaque and we can't manipulate it.
//!
//! And so, if you want to be able to inspect callee registers (beyond `r0`
//! through `r3`) or switch tasks, you need to do something more elaborate than
//! the basic hardware interrupt behavior: you need to carefully deposit all
//! user state into the `Task`, and then read it back on the way out (possibly
//! from a different `Task` if the context has switched).
//!
//! This is relatively costly, so it's only appropriate to do this in an ISR
//! that you believe will result in a context switch. `SVCall` usually does --
//! our most-used system calls are blocking. `SysTick` usually *does not* -- it
//! will cause a context switch only when it causes a higher-priority timer to
//! fire, which is a sometimes thing. And most hardware interrupt handlers are
//! also not guaranteed to cause a context switch immediately.
//!
//! So, we do the full save/restore sequence around `SVCall` (see the assembly
//! code in that function), but *not* around `SysTick`, and not around other
//! hardware IRQs. Instead, if one of those routines discovers that a context
//! switch is required, it pokes a register that sets the `PendSV` interrupt
//! pending.
//!
//! `PendSV` is intended for this exact use. It will kick in when our ISR exits
//! (i.e. it won't preempt our ISR, but follow it) and perform the full
//! save/restore sequence around invoking the scheduler.
//!
//! We didn't invent this idea -- it's covered in most books on the Cortex-M.
//! We might later decide that most ISRs (including ticks) tend to trigger
//! context switches, and just always do full save/restore, eliminating PendSV.
//! We'll see.

use core::sync::atomic::{AtomicBool, AtomicPtr, AtomicU32, Ordering};
use core::arch::asm;
use zerocopy::FromBytes;

use crate::atomic::AtomicExt;
use crate::startup::with_task_table;
use crate::task;
use crate::time::Timestamp;
use crate::umem::USlice;
use abi::FaultInfo;
use abi::FaultSource;
use unwrap_lite::UnwrapLite;

macro_rules! uassert {
    ($cond : expr) => {
        if !$cond {
            panic!("Assertion failed!");
        }
    };
}

/// On ARMvx-M we have to use a global to record the current task pointer, since
/// we don't have a scratch register.
#[no_mangle]
static CURRENT_TASK_PTR: AtomicPtr<task::Task> =
    AtomicPtr::new(core::ptr::null_mut());

/// To allow our clock frequency to be easily determined from a debugger, we
/// store it in memory.
#[no_mangle]
static CLOCK_FREQ_KHZ: AtomicU32 = AtomicU32::new(0);

/// ARMvx-M volatile registers that must be saved across context switches.
#[repr(C)]
#[derive(Debug, Default)]
pub struct SavedState {
    // NOTE: the following fields must be kept contiguous!
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    psp: u32,
    exc_return: u32,

    // gosh it would sure be nice if cfg_if were legal here
    s16: u32,
    s17: u32,
    s18: u32,
    s19: u32,
    s20: u32,
    s21: u32,
    s22: u32,
    s23: u32,
    s24: u32,
    s25: u32,
    s26: u32,
    s27: u32,
    s28: u32,
    s29: u32,
    s30: u32,
    s31: u32,
    // NOTE: the above fields must be kept contiguous!
}

/// Map the volatile registers to (architecture-independent) syscall argument
/// and return slots.
impl task::ArchState for SavedState {
    fn stack_pointer(&self) -> u32 {
        self.psp
    }

    /// Reads syscall argument register 0.
    fn arg0(&self) -> u32 {
        self.r4
    }
    fn arg1(&self) -> u32 {
        self.r5
    }
    fn arg2(&self) -> u32 {
        self.r6
    }
    fn arg3(&self) -> u32 {
        self.r7
    }
    fn arg4(&self) -> u32 {
        self.r8
    }
    fn arg5(&self) -> u32 {
        self.r9
    }
    fn arg6(&self) -> u32 {
        self.r10
    }

    fn syscall_descriptor(&self) -> u32 {
        self.r11
    }

    /// Writes syscall return argument 0.
    fn ret0(&mut self, x: u32) {
        self.r4 = x
    }
    fn ret1(&mut self, x: u32) {
        self.r5 = x
    }
    fn ret2(&mut self, x: u32) {
        self.r6 = x
    }
    fn ret3(&mut self, x: u32) {
        self.r7 = x
    }
    fn ret4(&mut self, x: u32) {
        self.r8 = x
    }
    fn ret5(&mut self, x: u32) {
        self.r9 = x
    }
}

/// Stuff placed on the stack at exception entry whether or not an FPU is
/// present.
#[derive(Debug, FromBytes, Default)]
#[repr(C)]
pub struct BaseExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}

/// Extended version for FPU.
#[derive(Debug, FromBytes, Default)]
#[repr(C)]
pub struct ExtendedExceptionFrame {
    base: BaseExceptionFrame,
    fpu_regs: [u32; 16],
    fpscr: u32,
    reserved: u32,
}

/// Initially we just set the Thumb Mode bit, the minimum required.
const INITIAL_PSR: u32 = 1 << 24;

/// We don't really care about the initial FPU mode; 0 is reasonable.
const INITIAL_FPSCR: u32 = 0;

// Because debuggers need to know the clock frequency to set the SWO clock
// scaler that enables ITM, and because ITM is particularly useful when
// debugging boot failures, this should be set as early in boot as it can
// be.
pub unsafe fn set_clock_freq(tick_divisor: u32) {
    CLOCK_FREQ_KHZ.store(tick_divisor, Ordering::Relaxed);
}

pub fn reinitialize(task: &mut task::Task) {
    *task.save_mut() = SavedState::default();
    // The initial stack points to the top of the sram available of this component
    // ---------------------- SRAM HIGH ADDRESSES
    // |  exception_frame   | <- initial_stack
    // |       STACK        |
    // |       ....         |
    // |       .data        |
    // ---------------------- SRAM LOW ADDRESSES

    let initial_stack = task.descriptor().initial_stack();

    // Modern ARMvX-M machines require 8-byte stack alignment. Make sure that's
    // still true. Note that this carries the risk of panic on task re-init if
    // the task table is corrupted -- this is deliberate.
    uassert!(initial_stack & 0x7 == 0);

    // The remaining state is stored on the stack.
    // Use checked operations to get a reference to the exception frame.
    let frame_size = core::mem::size_of::<ExtendedExceptionFrame>();
    // The subtract below can overflow if the task table is corrupt -- let's
    // make that failure a little easier to read:
    uassert!(initial_stack as usize >= frame_size);
    // Ok. Generate a uslice for the task's starting stack frame.
    let mut frame_uslice: USlice<ExtendedExceptionFrame> =
        USlice::from_raw(initial_stack as usize - frame_size, 1).unwrap_lite();

    // Get the region, that will always be the first one by construction
    let sram_region = *task.region_table().first().unwrap_lite();

    let mut stack_uslice: USlice<u32> = USlice::from_raw(
        sram_region.base as usize,
        (initial_stack as usize - frame_size - sram_region.base as usize) >> 4, //? >> 2
    )
    .unwrap_lite();

    // Before we set our frame, find the region that contains our initial stack
    // pointer, and zap the region from the base to the stack pointer with a
    // distinct (and storied) pattern.
    let zap = task.try_write(&mut stack_uslice).unwrap_lite();
    for word in zap.iter_mut() {
        *word = 0xbaddcafe;
    }

    // Now copy the .data section in ram
    let mut data_uslice: USlice<u8> =
        USlice::from_raw(sram_region.base as usize, task.data_section().len())
            .unwrap_lite();
            
    let data_section = task.data_section().clone();
    let data_raw = task.try_write(&mut data_uslice).unwrap_lite();
    

    // Load .data in SRAM
    for i in 0..data_raw.len() {
        data_raw[i] = data_section[i];
    }

    // Setup frame and pointers
    let entry_point = task.descriptor().entry_point();
    let frame = &mut task.try_write(&mut frame_uslice).unwrap_lite()[0];

    // Conservatively/defensively zero the entire frame.
    *frame = ExtendedExceptionFrame::default();
    // Now fill in the bits we actually care about.
    frame.base.pc = entry_point | 1; // for thumb
    frame.base.xpsr = INITIAL_PSR;
    frame.base.lr = 0xFFFF_FFFF; // trap on return from main
    frame.fpscr = INITIAL_FPSCR;

    // Set the initial stack pointer, *not* to the stack top, but to the base of
    // this frame.
    task.save_mut().psp = frame as *const _ as u32;

    // Set the ram base in R9, as required by ROPI-RWPI
    task.save_mut().r9 = sram_region.base;

    // Finally, record the EXC_RETURN we'll use to enter the task.
    task.save_mut().exc_return = EXC_RETURN_CONST;
}

pub fn force_task_update_handler(task: &mut task::Task) {
    // In order to force the task to execute the update handler,
    // we have to overwrite the PC saved in the exception frame saved on the stack
    let update_handler = task.get_update_handler().unwrap_lite();
    // Get the current task stack
    let current_stack = task.save().psp;
    // The frame will be on top of the current stack pointer
    let frame_address = current_stack as usize;
    let mut frame_uslice: USlice<ExtendedExceptionFrame> = USlice::from_raw(frame_address, 1).unwrap_lite();
    // Let's write the new pc
    let frame = &mut task.try_write(&mut frame_uslice).unwrap_lite()[0];
    frame.base.pc = update_handler;
}

pub fn mark_task_dying(task: &mut task::Task) {
    task.set_dying_at(now());
}

pub fn apply_memory_protection(task: &task::Task) {
    // We are manufacturing authority to interact with the MPU here, because we
    // can't thread a cortex-specific peripheral through an
    // architecture-independent API. This approach might bear revisiting later.
    let mpu = unsafe {
        // At least by not taking a &mut we're confident we're not violating
        // aliasing....
        &*cortex_m::peripheral::MPU::PTR
    };

    for (i, region) in task.region_table().iter().enumerate() {
        let rbar = (i as u32)  // region number
            | (1 << 4)  // honor the region number
            | region.base;
        let ratts = region.attributes;
        let xn = !ratts.contains(abi::RegionAttributes::EXECUTE);
        // These AP encodings are chosen such that we never deny *privileged*
        // code (i.e. us) access to the memory.
        let ap = if ratts.contains(abi::RegionAttributes::WRITE) {
            0b011
        } else if ratts.contains(abi::RegionAttributes::READ) {
            0b010
        } else {
            0b001
        };
        // Set the TEX/SCB bits to configure memory type, caching policy, and
        // shareability (with other cores or masters). See table B3-13 in the
        // ARMv7-M ARM. (Settings are identical on v6-M but the sharability and
        // TEX bits tend to be ignored.)
        let (tex, scb) = if ratts.contains(abi::RegionAttributes::DEVICE) {
            // Device memory.
            (0b000, 0b001)
        } else if ratts.contains(abi::RegionAttributes::DMA) {
            // Conservative settings for normal memory assuming that DMA might
            // be a problem:
            // - Outer and inner non-cacheable.
            // - Shared.
            (0b001, 0b100)
        } else {
            // Aggressive settings for normal memory assume that it is used only
            // by this processor:
            // - Outer and inner write-back
            // - Read and write allocate.
            // - Not shared.
            (0b001, 0b011)
        };
        // On v6/7-M the MPU expresses size of a region in log2 form _minus
        // one._ So, the minimum allowed size of 32 bytes is represented as 4,
        // because `2**(4 + 1) == 32`.
        //
        // We store sizes in the region table in an architecture-independent
        // form (number of bytes) because it simplifies basically everything
        // else but this routine. Here we must convert between the two -- and
        // quickly, because this is called on every context switch.
        //
        // The image-generation tools check at build time that region sizes are
        // powers of two. So, we can assume that the size has a single 1 bit. We
        // can cheaply compute log2 of this by counting trailing zeroes, but
        // ARMv7-M doesn't have a native instruction for that -- only leading
        // zeroes. The equivalent using leading zeroes is
        //
        //   log2(N) = bits_in_word - 1 - clz(N)
        //
        // Because we want log2 _minus one_ we compute it as...
        //
        //   log2_m1(N) = bits_in_word - 2 - clz(N)
        //
        // If the size is zero or one, this subtraction will underflow. This
        // should not occur in a valid image, but could occur due to runtime
        // flash corruption. Any region size under 32 bytes is illegal on
        // ARMv7-M anyway, so panicking is better than triggering possibly
        // undefined hardware behavior.
        //
        // On ARMv6-M, there is no CLZ instruction either. This winds up
        // generating decent intrinsic code for `leading_zeros` so we'll live
        // with it.
        let l2size = 30 - region.size.leading_zeros();

        let rasr = (xn as u32) << 28
            | ap << 24
            | tex << 19
            | scb << 16
            | l2size << 1
            | (1 << 0); // enable
        unsafe {
            mpu.rbar.write(rbar);
            mpu.rasr.write(rasr);
        }
    }
}

pub fn start_first_task(tick_divisor: u32, task: &mut task::Task) -> ! {
    // Enable faults and set fault/exception priorities to reasonable settings.
    // Our goal here is to keep the kernel non-preemptive, which means the
    // kernel entry points (SVCall, PendSV, SysTick, interrupt handlers) must be
    // at one priority level. Fault handlers need to be higher priority,
    // however, so that we can detect faults in the kernel.
    //
    // Safety: this is actually fairly safe. We're purely lowering priorities
    // from their defaults, so it can't cause any surprise preemption or
    // anything. But these operations are `unsafe` in the `cortex_m` crate.
    unsafe {
        let scb = &*cortex_m::peripheral::SCB::PTR;
        // Faults on, on the processors that distinguish faults. This
        // distinguishes the following faults from HardFault:
        //
        // - ARMv7+: MEMFAULT, BUSFAULT, USGFAULT
        // - ARMv8: SECUREFAULT
        scb.shcsr.modify(|x| x | 0b111 << 16);

        // Set fault and standard exception priorities.
        // Set priority of Usage, Bus, MemManage to 0 (highest
        // configurable).
        scb.shpr[0].write(0x00);
        scb.shpr[1].write(0x00);
        scb.shpr[2].write(0x00);
        // Set priority of SVCall to 0xFF (lowest configurable).
        scb.shpr[7].write(0xFF);
        // SysTick and PendSV also to 0xFF
        scb.shpr[10].write(0xFF);
        scb.shpr[11].write(0xFF);

        // ARM's default disposition is that division by zero doesn't
        // actually fail, but rather returns 0. (!)  It's unclear how
        // placating this kind of programmatic sloppiness doesn't ultimately
        // end in tears; we explicitly configure ourselves to trap on any
        // divide by zero.
        const DIV_0_TRP: u32 = 1 << 4;
        scb.ccr.modify(|x| x | DIV_0_TRP);

        // Configure the priority of all external interrupts so that they can't
        // preempt the kernel.
        let nvic = &*cortex_m::peripheral::NVIC::PTR;

        // How many IRQs have we got on ARMv7+? This information is
        // stored in a separate area of the address space, away from the
        // NVIC, and is (presumably due to an oversight) not present in
        // the cortex_m API, so let's fake it.
        let ictr = (0xe000_e004 as *const u32).read_volatile();
        // This gives interrupt count in blocks of 32, minus 1, so there
        // are always at least 32 interrupts.
        let irq_block_count = (ictr as usize & 0xF) + 1;
        let irq_count = irq_block_count * 32;
        // Blindly poke all the interrupts to 0xFF. IPR registers on
        // ARMv7/8 are modeled as `u8` by `cortex_m`, unlike on ARMv6.
        // We're explicit with the `u8` suffix below to ensure that we
        // notice if this changes.
        for i in 0..irq_count {
            nvic.ipr[i].write(0xFFu8);
        }
    }

    // Safety: this, too, is safe in practice but unsafe in API.
    unsafe {
        // Configure the timer.
        let syst = &*cortex_m::peripheral::SYST::PTR;
        // Program reload value.
        syst.rvr.write(tick_divisor - 1);
        // Clear current value.
        syst.cvr.write(0);
        // Enable counter and interrupt.
        syst.csr.modify(|v| v | 0b111);
    }
    // We are manufacturing authority to interact with the MPU here, because we
    // can't thread a cortex-specific peripheral through an
    // architecture-independent API. This approach might bear revisiting later.
    let mpu = unsafe {
        // At least by not taking a &mut we're confident we're not violating
        // aliasing....
        &*cortex_m::peripheral::MPU::PTR
    };

    const ENABLE: u32 = 0b001;
    const PRIVDEFENA: u32 = 0b100;
    // Safety: this has no memory safety implications. The worst it can do is
    // cause us to fault, which is safe. The register API doesn't know this.
    unsafe {
        mpu.ctrl.write(ENABLE | PRIVDEFENA);
    }

    CURRENT_TASK_PTR.store(task, Ordering::Relaxed);

    // Safety: this is setting the Process (task) stack pointer, which has no
    // effect _assuming_ this code is running on the Main (kernel) stack.
    unsafe {
        cortex_m::register::psp::write(task.save().psp);
    }

    // Run the final pre-kernel assembly sequence to set up the kernel
    // environment!
    //
    // Our basic goal here is to flip into Handler mode (i.e. interrupt state)
    // so that we can switch Thread mode (not-interrupt state) to unprivileged
    // and running off the Process Stack Pointer. The easiest way to do this on
    // ARM-M is by entering Handler mode by a trap. We use SVC, which we also
    // use for system calls; the SVC entry sequence (also in this file) has code
    // to detect this condition and do kernel startup rather than processing it
    // as a syscall.
    unsafe {
        asm!("
            @ Restore callee-save registers.
            ldm {task}, {{r4-r11}}
            @ Trap into the kernel.
            svc #0xFF
            @ noreturn generates a UDF here in case that should return.
            ",
            task = in(reg) &task.save().r4,
            options(noreturn),
        )
    }
}

/// Handler that gets linked into the vector table for the Supervisor Call (SVC)
/// instruction. (Name is dictated by the `cortex_m` crate.)
#[allow(non_snake_case)]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn SVCall() {
    // TODO: could shave several cycles off SVC entry with more careful ordering
    // of instructions below, though the precise details depend on how complex
    // of an M-series processor you're targeting -- so I've punted on this for
    // the time being.

    // All the syscall handlers use the same strategy, but the implementation
    // differs on different profile variants.
    //
    // First, we inspect LR, which on exception entry contains bits describing
    // the _previous_ (interrupted) processor state. We can use this to detect
    // if the SVC came from the Main (interrupt) stack. This only happens once,
    // during startup, so we vector to a different routine in this case.
    //
    // We then store the calling task's context into the TCB.
    //
    // Then, we call into `syscall_entry`.
    //
    // After that, we repeat the same steps in the opposite order to restore
    // task context (possibly for a different task!).
    unsafe {
        asm!("
            @ Inspect LR to figure out the caller's mode.
            mov r0, lr
            mov r1, #0xFFFFFFF3
            bic r0, r1
            @ Is the call coming from thread mode + main stack, i.e.
            @ from the kernel startup routine?
            cmp r0, #0x8
            @ If so, this is startup; jump ahead. The common case falls
            @ through because branch-not-taken tends to be faster on small
            @ cores.
            beq 1f

            @ store volatile state.
            @ first, get a pointer to the current task.
            movw r0, #:lower16:CURRENT_TASK_PTR
            movt r0, #:upper16:CURRENT_TASK_PTR
            ldr r1, [r0]
            movs r2, r1
            @ fetch the process-mode stack pointer.
            @ fetching into r12 means the order in the stm below is right.
            mrs r12, PSP
            @ now, store volatile registers, plus the PSP in r12, plus LR.
            stm r2!, {{r4-r12, lr}}
            vstm r2, {{s16-s31}}

            @ syscall number is passed in r11. Move it into r0 to pass it as
            @ an argument to the handler, then call the handler.
            movs r0, r11
            bl syscall_entry

            @ we're returning back to *some* task, maybe not the same one.
            movw r0, #:lower16:CURRENT_TASK_PTR
            movt r0, #:upper16:CURRENT_TASK_PTR
            ldr r0, [r0]
            @ restore volatile registers, plus load PSP into r12
            ldm r0!, {{r4-r12, lr}}
            vldm r0, {{s16-s31}}
            msr PSP, r12

            @ resume
            bx lr

        1:  @ starting up the first task.
            movs r0, #1         @ get bitmask to...
            msr CONTROL, r0     @ ...shed privs from thread mode.
                                @ note: now barrier here because exc return
                                @ serves as barrier

            mov lr, {exc_return}    @ materialize EXC_RETURN value to
                                    @ return into thread mode, PSP, FP on

            bx lr                   @ branch into user mode
            ",
            exc_return = const EXC_RETURN_CONST as u32,
            options(noreturn),
        )
    }
}

/// Records the address of `task` as the current user task.
///
/// # Safety
///
/// This records a pointer that aliases `task`. As long as you don't read that
/// pointer while you have access to `task`, and as long as the `task` being
/// stored is actually in the task table, you'll be okay.
pub unsafe fn set_current_task(task: &mut task::Task) {
    CURRENT_TASK_PTR.store(task, Ordering::Relaxed);
}

/// Reads the tick counter.
pub fn now() -> Timestamp {
    // Recall that we expect the systick interrupt cannot preempt kernel code,
    // so we're safe to read this in two nonatomic parts here.
    Timestamp::from([
        TICKS[0].load(Ordering::Relaxed),
        TICKS[1].load(Ordering::Relaxed),
    ])
}

/// Kernel global for tracking the current timestamp, measured in ticks.
///
/// This is a pair of `AtomicU32` because (1) we want the interior mutability of
/// the atomic types but (2) ARMv7-M doesn't have any 64-bit atomic operations.
/// We access this only from contexts where we can't be preempted, so, the fact
/// that it's split across two words is ok.
///
/// `TICKS[0]` is the least significant part, `TICKS[1]` the most significant.
static TICKS: [AtomicU32; 2] = {
    #[allow(clippy::declare_interior_mutable_const)]
    const ZERO: AtomicU32 = AtomicU32::new(0);
    [ZERO; 2]
};

/// Handler that gets linked into the vector table for the System Tick Timer
/// overflow interrupt. (Name is dictated by the `cortex_m` crate.)
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SysTick() {
    crate::profiling::event_timer_isr_enter();
    with_task_table(|tasks| {
        // Load the time before this tick event.
        let t0 = TICKS[0].load(Ordering::Relaxed);
        let t1 = TICKS[1].load(Ordering::Relaxed);

        // Advance the kernel's notion of time by adding 1. Laboriously.
        let (t0, t1) = if let Some(t0p) = t0.checked_add(1) {
            // Incrementing t0 did not roll over, no need to update t1.
            TICKS[0].store(t0p, Ordering::Relaxed);
            (t0p, t1)
        } else {
            // Incrementing t0 overflowed. We need to also increment t1. We use
            // normal checked addition for this, not wrapping, because this
            // should not be able to overflow under normal operation, and would
            // almost certainly indicate state corruption that we'd like to
            // discover.
            TICKS[0].store(0, Ordering::Relaxed);
            TICKS[1].store(t1 + 1, Ordering::Relaxed);
            (0, t1 + 1)
        };

        // Process any timers.
        let now = Timestamp::from([t0, t1]);
        let switch = task::process_timers(tasks, now);

        // If any timers fired, we need to defer a context switch, because the entry
        // sequence to this ISR doesn't save state correctly for efficiency.
        if switch != task::NextTask::Same {
            pend_context_switch_from_isr();
        }
    });
    crate::profiling::event_timer_isr_exit();
}

fn pend_context_switch_from_isr() {
    // This sets the bit to pend a PendSV interrupt. PendSV will happen after
    // the current ISR (and any chained ISRs) returns, and perform the context
    // switch.
    cortex_m::peripheral::SCB::set_pendsv();
}

#[allow(non_snake_case)]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    unsafe {
        asm!(
            "
            @ store volatile state.
            @ first, get a pointer to the current task.
            movw r0, #:lower16:CURRENT_TASK_PTR
            movt r0, #:upper16:CURRENT_TASK_PTR
            ldr r1, [r0]
            @ fetch the process-mode stack pointer.
            @ fetching into r12 means the order in the stm below is right.
            mrs r12, PSP
            @ now, store volatile registers, plus the PSP in r12, plus LR.
            stm r1!, {{r4-r12, lr}}
            vstm r1, {{s16-s31}}

            bl pendsv_entry

            @ we're returning back to *some* task, maybe not the same one.
            movw r0, #:lower16:CURRENT_TASK_PTR
            movt r0, #:upper16:CURRENT_TASK_PTR
            ldr r0, [r0]
            @ restore volatile registers, plus load PSP into r12
            ldm r0!, {{r4-r12, lr}}
            vldm r0, {{s16-s31}}
            msr PSP, r12

            @ resume
            bx lr
            ",
            options(noreturn),
        );
    }
}

/// The Rust side of the PendSV handler, after all volatile registers have been
/// saved somewhere predictable.
#[no_mangle]
unsafe extern "C" fn pendsv_entry() {
    crate::profiling::event_secondary_syscall_enter();

    let current = CURRENT_TASK_PTR.load(Ordering::Relaxed);
    uassert!(!current.is_null()); // irq before kernel started?

    // Safety: we're dereferencing the current task pointer, which we're
    // trusting the rest of this module to maintain correctly.
    let current_id =
        u16::from(unsafe { (*current).descriptor().component_id() });

    with_task_table(|tasks| {
        let next_id = task::select(current_id, tasks);
        let next_task =
            tasks.get_mut(&next_id).expect("Cannot find component ID!");
        apply_memory_protection(next_task);
        // Safety: next comes from the task table and we don't use it again
        // until next kernel entry, so we meet set_current_task's requirements.
        unsafe {
            set_current_task(next_task);
        }
    });
    crate::profiling::event_secondary_syscall_exit();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler() {
    crate::profiling::event_isr_enter();
    // We can cheaply get the identity of the interrupt that called us from the
    // bottom 9 bits of IPSR.
    //
    // Safety: we're just reading the PSR.
    let exception_num = unsafe {
        let mut ipsr: u32;
        asm!(
            "mrs {}, IPSR",
            out(reg) ipsr,
            options(pure, nomem, preserves_flags, nostack),
        );
        ipsr & 0x1FF
    };

    // The first 16 exceptions are architecturally defined; vendor hardware
    // interrupts start at 16.
    match exception_num {
        // 1=Reset is not handled this way
        2 => panic!("NMI"),
        // 3=HardFault is handled elsewhere
        // 4=MemManage is handled below
        // 5=BusFault is handled below
        // 6=UsageFault is handled below
        // 7-10 are currently reserved
        // 11=SVCall is handled above by its own handler
        12 => panic!("DebugMon"),
        // 13 is currently reserved
        // 14=PendSV is handled above by its own handler
        // 15=SysTick is handled above by its own handler
        x if x >= 16 => {
            // Hardware interrupt
            let irq_num = exception_num - 16;
            let irq_to_task = unsafe{&crate::startup::IRQ_TO_TASK};
            let owner = irq_to_task
                .get(&irq_num)
                .unwrap_or_else(|| panic!("unhandled IRQ {}", irq_num));

            let switch = with_task_table(|tasks| {
                disable_irq(irq_num);

                // Now, post the notification and return the
                // scheduling hint.
                let n = task::NotificationSet(owner.notification);
                tasks
                    .get_mut(&owner.task_id)
                    .expect("Cannot find component ID")
                    .post(n)
            });
            if switch {
                pend_context_switch_from_isr()
            }
        }

        _ => panic!("unknown exception {}", exception_num),
    }
    crate::profiling::event_isr_exit();
}

pub fn disable_irq(n: u32) {
    // Disable the interrupt by poking the Interrupt Clear Enable Register.
    let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
    let reg_num = (n / 32) as usize;
    let bit_mask = 1 << (n % 32);
    unsafe {
        nvic.icer[reg_num].write(bit_mask);
    }
}

pub fn enable_irq(n: u32) {
    // Enable the interrupt by poking the Interrupt Set Enable Register.
    let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
    let reg_num = (n / 32) as usize;
    let bit_mask = 1 << (n % 32);
    unsafe {
        nvic.iser[reg_num].write(bit_mask);
    }
}

#[repr(u8)]
#[allow(dead_code)]
enum FaultType {
    MemoryManagement = 4,
    BusFault = 5,
    UsageFault = 6,
}

#[naked]
unsafe extern "C" fn configurable_fault() {
    unsafe {
        asm!(
            "
            @ Read the current task pointer.
            movw r0, #:lower16:CURRENT_TASK_PTR
            movt r0, #:upper16:CURRENT_TASK_PTR
            ldr r0, [r0]
            mrs r12, PSP

            @ Now, to aid those who will debug what induced this fault, save our
            @ context.  Some of our context (namely, r0-r3, r12, LR, the return
            @ address and the xPSR) is already on our stack as part of the fault;
            @ we'll store our remaining registers, plus the PSP (now in r12), plus
            @ exc_return (now in LR) into the save region in the current task.
            @ Note that we explicitly refrain from saving the floating point
            @ registers here:  touching the floating point registers will induce
            @ a lazy save on the stack, which is clearly bad news if we have
            @ overflowed our stack!  We do want to ultimately save them to aid
            @ debuggability, however, so we pass the address to which they should
            @ be saved to our fault handler, which will take the necessary
            @ measures to save them safely.  Finally, note that deferring the
            @ save to later in handle_fault assumes that the floating point
            @ registers are not in fact touched before determmining the fault type
            @ and disabling lazy saving accordingly; should that assumption not
            @ hold, we will need to be (ironically?) less lazy about disabling
            @ lazy saving...
            mov r2, r0
            stm r2!, {{r4-r12, lr}}

            @ Pull our fault number out of IPSR, allowing for program text to be
            @ shared across all configurable faults.  (Note that the exception
            @ number is the bottom 9 bits, but we need only look at the bottom 4
            @ bits as this handler is only used for exceptions with numbers less
            @ than 16.)
            mrs r1, IPSR
            and r1, r1, #0xf
            bl handle_fault

            @ Our task has changed; reload it.
            movw r0, #:lower16:CURRENT_TASK_PTR
            movt r0, #:upper16:CURRENT_TASK_PTR
            ldr r0, [r0]

            @ Restore volatile registers, plus load PSP into r12
            ldm r0!, {{r4-r12, lr}}
            vldm r0, {{s16-s31}}
            msr PSP, r12

            @ resume
            bx lr
            ",
            options(noreturn),
        );
    }
}

/// Initial entry point for handling a memory management fault.
#[allow(non_snake_case)]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn MemoryManagement() {
    // Safety: this is merely a call (a tailcall, really) to a different handler
    // -- we're doing it this way simply because the other handler does context
    // save, so we can't go up into Rust here.
    unsafe { asm!("b {0}", sym configurable_fault, options(noreturn)) }
}

/// Initial entry point for handling a bus fault.
#[allow(non_snake_case)]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn BusFault() {
    // Safety: this is merely a call (a tailcall, really) to a different handler
    // -- we're doing it this way simply because the other handler does context
    // save, so we can't go up into Rust here.
    unsafe { asm!("b {0}", sym configurable_fault, options(noreturn)) }
}

/// Initial entry point for handling a usage fault.
#[allow(non_snake_case)]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn UsageFault() {
    // Safety: this is merely a call (a tailcall, really) to a different handler
    // -- we're doing it this way simply because the other handler does context
    // save, so we can't go up into Rust here.
    unsafe { asm!("b {0}", sym configurable_fault, options(noreturn)) }
}

bitflags::bitflags! {
    /// Bits in the Configurable Fault Status Register.
    #[repr(transparent)]
    struct Cfsr: u32 {
        // Bits 0-7: MMFSR (Memory Management Fault Status Register)
        const IACCVIOL = 1 << 0;
        const DACCVIOL = 1 << 1;
        // MMFSR bit 2 reserved
        const MUNSTKERR = 1 << 3;
        const MSTKERR = 1 << 4;
        const MLSPERR = 1 << 5;
        // MMFSR bit 6 reserved
        const MMARVALID = 1 << 7;

        // Bits 8-15: BFSR (Bus Fault Status Register)
        const IBUSERR = 1 << (8 + 0);
        const PRECISERR = 1 << (8 + 1);
        const IMPRECISERR = 1 << (8 + 2);
        const UNSTKERR = 1 << (8 + 3);
        const STKERR = 1 << (8 + 4);
        const LSPERR = 1 << (8 + 5);
        // BFSR bit 6 reserved
        const BFARVALID = 1 << (8 + 7);

        // Bits 16-31: UFSR (Usage Fault Status Register)
        const UNDEFINSTR = 1 << (16 + 0);
        const INVSTATE = 1 << (16 + 1);
        const INVPC = 1 << (16 + 2);
        const NOCP = 1 << (16 + 3);

        #[cfg(armv8m)]
        const STKOF = 1 << (16 + 4);

        // UFSR bits 4-7 reserved on ARMv7-M -- 5-7 on ARMv8-M
        const UNALIGNED = 1 << (16 + 8);
        const DIVBYZERO = 1 << (16 + 9);

        // UFSR bits 10-31 reserved
    }
}

/// Common implementation of fault handling.
///
/// # Safety
///
/// Requirements for using this safely include:
///
/// - Call this on the way into the kernel from a (naked) ISR, not from within
///   the kernel Rust code.
/// - Ensure that `task` is a pointer to an initialized, aligned Task in the
///   task table.
/// - Ensure that `fpsave` points to that task's floating point save area.
#[no_mangle]
unsafe extern "C" fn handle_fault(
    task: *mut task::Task,
    fault_type: FaultType,
    fpsave: *mut u32,
) {
    // To diagnose the fault, we're going to need access to the System Control
    // Block. Pull such access from thin air.
    //
    // Safety: this is dereferencing the raw pointer produced by SCB::ptr. We
    // trust that the returned pointer is valid (non-null, aligned). The
    // resulting reference is to a static-scoped Sync thing, and it's a shared
    // reference, so we shouldn't be breaking any rules by doing this. Arguably
    // this should be available as a safe operation in the cortex_m crate, but
    // that crate comes with _ideas_ about peripheral ownership management.
    let scb = unsafe { &*cortex_m::peripheral::SCB::PTR };
    let cfsr = Cfsr::from_bits_truncate(scb.cfsr.read());

    // Who faulted? Collect some parameters from the task.
    //
    // Safety: we're dereferencing the raw `task` pointer passed in. Our
    // contract requires that it be valid. We immediately throw away the result
    // of dereferencing it, as it would otherwise alias the task table obtained
    // later.
    let (exc_return, psp, id) = unsafe {
        let t = &(*task);
        (
            t.save().exc_return,
            t.save().psp,
            t.descriptor().component_id(),
        )
    };
    let from_thread_mode = exc_return & 0b1000 != 0;

    if !from_thread_mode {
        // Uh. This fault originates from the kernel. Let's try to make the
        // panic as clear and as information-rich as possible, while trying
        // to not consume unnecessary program text (i.e., it isn't worth
        // conditionally printing MMFAR or BFAR only on a MemoryManagement
        // fault or a BusFault, respectively).  In that vein, note that we
        // promote our fault type to a u32 to not pull in the Display trait
        // for either FaultType or u8.
        panic!(
            "Kernel fault {}: \
            CFSR={:#010x}, MMFAR={:#010x}, BFAR={:#010x}",
            (fault_type as u8) as u32,
            cfsr.bits(),
            scb.mmfar.read(),
            scb.bfar.read(),
        );
    }

    let (fault, stackinvalid) = match fault_type {
        FaultType::MemoryManagement => {
            if cfsr.contains(Cfsr::MSTKERR) {
                // If we have an MSTKERR, we know very little other than the
                // fact that the user's stack pointer is so trashed that we
                // can't store through it.  (In particular, we seem to have no
                // way at getting at our faulted PC.)
                (FaultInfo::StackOverflow { address: psp }, true)
            } else if cfsr.contains(Cfsr::IACCVIOL) {
                (FaultInfo::IllegalText, false)
            } else {
                (
                    FaultInfo::MemoryAccess {
                        address: if cfsr.contains(Cfsr::MMARVALID) {
                            Some(scb.mmfar.read())
                        } else {
                            None
                        },
                        source: FaultSource::User,
                    },
                    false,
                )
            }
        }

        FaultType::BusFault => (
            FaultInfo::BusError {
                address: if cfsr.contains(Cfsr::BFARVALID) {
                    Some(scb.bfar.read())
                } else {
                    None
                },
                source: FaultSource::User,
            },
            false,
        ),

        FaultType::UsageFault => (
            if cfsr.contains(Cfsr::DIVBYZERO) {
                FaultInfo::DivideByZero
            } else if cfsr.contains(Cfsr::UNDEFINSTR) {
                FaultInfo::IllegalInstruction
            } else {
                FaultInfo::InvalidOperation(cfsr.bits())
            },
            false,
        ),
    };

    // Because we are responsible for clearing all conditions, we write back
    // the value of CFSR that we read
    //
    // Safety: this is a traditional write-one-to-clear register that, when
    // written, clears recorded fault states. It is not at _all_ clear why its
    // write function is unsafe.
    unsafe {
        scb.cfsr.write(cfsr.bits());
    }

    if stackinvalid {
        // We know that we have an invalid stack; to prevent our subsequent
        // save of the dead task's floating point registers from storing
        // floating point registers to the invalid stack, we explicitly clear
        // the Lazy Stack Preservation Active bit in our Floating Point
        // Context Control register.
        const LSPACT: u32 = 1 << 0;
        unsafe {
            let fpu = &*cortex_m::peripheral::FPU::PTR;
            fpu.fpccr.modify(|x| x & !LSPACT);
        }
    }

    // It's safe to store our floating point registers; store them now to
    // preserve as much state as possible for debugging.
    //
    // Safety: asm! is always unsafe, obvs, but in this case as long as fpsave
    // points to a correctly aligned area large enough to store 16 floats -- a
    // property our caller is required to ensure -- this is ok.
    unsafe {
        asm!("vstm {0}, {{s16-s31}}", in(reg) fpsave);
    }

    // We are now going to force a fault on our current task and directly
    // switch to a task to run.  (It may be tempting to use PendSV here,
    // but that won't work on ARMv8-M in the presence of MPU faults on
    // PSP:  even with PendSV pending, ARMv8-M will generate a MUNSTKERR
    // when returning from an exception with a PSP that generates an MPU
    // fault!)
    with_task_table(|tasks| {
        let next_id = match task::force_fault(tasks, id, fault) {
            task::NextTask::Specific(i) => i,
            task::NextTask::Other => task::select(id, tasks),
            task::NextTask::Same => id,
        };

        if next_id == id {
            panic!("attempt to return to Task #{} after fault", id);
        }

        let next_task =
            tasks.get_mut(&next_id).expect("Cannot find component ID");
        apply_memory_protection(next_task);
        // Safety: this leaks a pointer aliasing next into static scope, but
        // we're not going to read it back until the next kernel entry, so we
        // won't be aliasing/racing.
        unsafe {
            set_current_task(next_task);
        }
    });
}

impl AtomicExt for AtomicBool {
    type Primitive = bool;

    #[inline(always)]
    fn swap_polyfill(
        &self,
        value: Self::Primitive,
        ordering: Ordering,
    ) -> Self::Primitive {
        self.swap(value, ordering)
    }
}

pub const EXC_RETURN_CONST: u32 = 0xFFFFFFED; // Unsafe

// Constants that may change depending on configuration
//include!(concat!(env!("ROOT"), "/consts.rs"));

#![no_std]
#![no_main]

use rcc_api::RCCError;
use uart_channel_api::*;
use userlib::{hl::Caller, *};

#[cfg(feature = "stm32f303re")]
use stm32f3::stm32f303 as device;

// Baudrate used during communication
const BAUDRATE: u32 = 115_200;
const USART_IRQ_MASK: u32 = 0b0000_0000_0000_0001;
const DMA1_CH6_IRQ_MASK: u32 = 0b0000_0000_0000_0010;
const TIMEOUT_MASK: u32 = 0b1000_0000_0000_0000;

// Driver state
struct Transmitter {
    caller: hl::Caller<()>,
    borrow_num: usize,
    len: usize,
    pos: usize,
}
struct Receiver {
    caller: hl::Caller<()>,
    borrow_num: usize,
    len: usize,
    pos: usize,
}

struct DriverState {
    pub receiver_state: ReceiverState,
    pub pending_transmitter: Option<Transmitter>,
}

struct ReceiverState {
    pub pending_receiver: Option<Receiver>,
    pub current_read_pos: usize,
}

#[export_name = "main"]
fn main() -> ! {
    // From thin air, pluck a pointer to the USART register block.
    //
    // Safety: this is needlessly unsafe in the API. The USART is essentially a
    // static, and we access it through a & reference so aliasing is not a
    // concern. Were it literally a static, we could just reference it.
    let usart = unsafe { &*device::USART2::ptr() };
    // DMA1
    let dma1 = unsafe { &*device::DMA1::ptr() };

    setup_usart(usart).unwrap();
    setup_gpio().unwrap();
    setup_dma(dma1, usart).unwrap();

    // Turn on our interrupt. We haven't enabled any interrupt sources at the
    // USART side yet, so this won't trigger notifications yet.
    sys_irq_control(USART_IRQ_MASK, true);
    sys_irq_control(DMA1_CH6_IRQ_MASK, true);

    // Construct driver state
    let mut state = DriverState {
        receiver_state: ReceiverState {
            pending_receiver: None,
            current_read_pos: 1, // Ask for some reason, the first byte we read is 0x00
        },
        pending_transmitter: None,
    };

    // Main loop
    let mut recv_buff: [u8; 8] = [0x00; 8];
    let mut frame_recovery: bool = true;
    loop {
        hl::recv(
            &mut recv_buff,
            USART_IRQ_MASK | DMA1_CH6_IRQ_MASK | TIMEOUT_MASK,
            &mut state,
            |state_ref, bits| {
                // Timer IRQ
                if bits & TIMEOUT_MASK != 0 {
                    // Timeout for read expired
                    if state_ref.receiver_state.pending_receiver.is_some() {
                        core::mem::replace(&mut state_ref.receiver_state.pending_receiver, None)
                            .unwrap()
                            .caller
                            .reply_fail(ChannelError::ReadTimeOut);
                    }
                }
                // UART IRQ
                if bits & USART_IRQ_MASK != 0 {
                    // Handling an interrupt. To allow for spurious interrupts,
                    // check the individual conditions we care about, and
                    // unconditionally re-enable the IRQ at the end of the handler.

                    let usart_isr = usart.isr.read();

                    // Transmit the old way
                    if usart_isr.txe().bit_is_set() {
                        // TX register empty. Do we need to send something?
                        step_transmit(
                            &usart,
                            &mut state_ref.pending_transmitter,
                            &mut state_ref.receiver_state.pending_receiver,
                        );
                    }

                    if usart_isr.idle().bit_is_set() {
                        // IDLE, we have to flush RX buffer
                        // -> get the number of bytes still to be read of DMA
                        let remaining_rx = dma1.ch6.ndtr.read().bits() as usize;
                        if remaining_rx > 0 && remaining_rx < RX_BUFFER_SIZE {
                            // Still something to read (otherwise TC will be called)
                            dma_receive_callback(
                                &mut state_ref.receiver_state,
                                RX_BUFFER_SIZE - remaining_rx,
                                dma1,
                                usart,
                            );
                        }
                        // Clear bit
                        usart.icr.modify(|_, w| w.idlecf().set_bit());
                    }

                    // Frame error
                    if usart_isr.fe().bit_is_set() {
                        if !frame_recovery {
                            panic!("UART Frame Error");
                        }
                        // For this time, just reset the error.
                        // This is needed as for some reason it happens to fire
                        // after the peripheral is configured. Not enough time to
                        // further investigate at the moment, maybe wait some flag
                        // will fix it.
                        usart.icr.modify(|_, w| w.fecf().set_bit());
                        frame_recovery = false;
                    }

                    // Overrun error: happens only if we mess up with the DMA
                    // otherwise it's impossibile.
                    if usart_isr.ore().bit_is_set() {
                        // Something happened
                        panic!("UART Overrun");
                    }

                    // Enable again interrupts
                    sys_irq_control(USART_IRQ_MASK, true);
                }
                // DMA IRQ
                if bits & DMA1_CH6_IRQ_MASK != 0 {
                    // DMA fired interrupt (RX)
                    let ch6_isr = dma1.isr.read();
                    if ch6_isr.htif6().bit_is_set() {
                        // Clear the flag
                        dma1.ifcr.write(|w| w.chtif6().set_bit());
                        // Half transfer complete!
                        dma_receive_callback(
                            &mut state_ref.receiver_state,
                            RX_BUFFER_SIZE / 2,
                            dma1,
                            usart,
                        );
                    } else if ch6_isr.tcif6().bit_is_set() {
                        // Clear the flag
                        dma1.ifcr.write(|w| w.ctcif6().set_bit());
                        // Full transfer complete
                        dma_receive_callback(
                            &mut state_ref.receiver_state,
                            RX_BUFFER_SIZE,
                            dma1,
                            usart,
                        );
                    } else if ch6_isr.teif6().bit_is_set() {
                        // Error
                        panic!("Got error on DMA");
                    }

                    // Enable again interrupt
                    sys_irq_control(DMA1_CH6_IRQ_MASK, true);
                }
            },
            |state_ref, op, msg| match op {
                Operation::WriteBlock => {
                    // Validate lease count and buffer sizes first.
                    let ((), caller) = msg.fixed_with_leases(1).ok_or(ChannelError::BadArgument)?;

                    // Deny incoming writes if we're already running one.
                    if state_ref.pending_transmitter.is_some() {
                        return Err(ChannelError::ChannelBusy);
                    }

                    // Check borrow
                    let borrow = caller.borrow(0);
                    let info = borrow.info().ok_or(ChannelError::BadArgument)?;
                    if !info.attributes.contains(LeaseAttributes::READ) || info.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // Perform setup
                    setup_transmit(state_ref, usart, caller, info.len, 0)?;

                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
                Operation::ReadBlock => {
                    // Validate lease count and buffer sizes first.
                    let ((), caller) = msg.fixed_with_leases(1).ok_or(ChannelError::BadArgument)?;

                    // Deny incoming reads if we're already running too many.
                    if state_ref.receiver_state.pending_receiver.is_some() {
                        return Err(ChannelError::ChannelBusy);
                    }
                    // Check borrow
                    let borrow = caller.borrow(0);
                    let info = borrow.info().ok_or(ChannelError::BadArgument)?;
                    if !info.attributes.contains(LeaseAttributes::WRITE) || info.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // Perform setup
                    setup_read(state_ref, usart, dma1, caller, info.len, 0)?;

                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
                Operation::ReadBlockTimed => {
                    // Validate lease count and buffer sizes first.
                    let (msg, caller) = msg
                        .fixed_with_leases::<ReadBlockTimedRequest, ()>(1)
                        .ok_or(ChannelError::BadArgument)?;

                    // Deny incoming reads if we're already running too many.
                    if state_ref.receiver_state.pending_receiver.is_some() {
                        return Err(ChannelError::ChannelBusy);
                    }
                    // Check borrow
                    let borrow = caller.borrow(0);
                    let info = borrow.info().ok_or(ChannelError::BadArgument)?;
                    if !info.attributes.contains(LeaseAttributes::WRITE) || info.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // Setup
                    setup_timed_read(
                        state_ref,
                        usart,
                        dma1,
                        caller,
                        info.len,
                        0,
                        msg.timeout_ticks,
                    )?;

                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
                Operation::TransmitTimed => {
                    // Validate lease count and buffer sizes first.
                    let (msg, caller) = msg
                        .fixed_with_leases::<TransmitTimedRequest, ()>(2)
                        .ok_or(ChannelError::BadArgument)?;

                    // Check both requisites before proceding
                    if state_ref.receiver_state.pending_receiver.is_some() {
                        return Err(ChannelError::ChannelBusy);
                    }
                    if state_ref.pending_transmitter.is_some() {
                        return Err(ChannelError::ChannelBusy);
                    }
                    // Check timeout
                    if msg.timeout_ticks == 0 {
                        return Err(ChannelError::BadArgument);
                    }
                    // Check leases
                    // [0] Data out
                    // [1] Data in
                    let borrow_out = caller.borrow(0);
                    let info_out = borrow_out.info().ok_or(ChannelError::BadArgument)?;
                    if !info_out.attributes.contains(LeaseAttributes::READ) || info_out.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }
                    let borrow_in = caller.borrow(1);
                    let info_in = borrow_in.info().ok_or(ChannelError::BadArgument)?;
                    if !info_in.attributes.contains(LeaseAttributes::WRITE) || info_in.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // First, setup the reception part
                    setup_timed_read(
                        state_ref,
                        usart,
                        dma1,
                        caller.clone(),
                        info_in.len,
                        1,
                        msg.timeout_ticks,
                    )?;
                    // Then, prepare a transmission
                    setup_transmit(state_ref, usart, caller.clone(), info_out.len, 0)?;

                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
            },
        )
    }
}

fn setup_timed_read(
    state_ref: &mut DriverState,
    usart: &device::usart2::RegisterBlock,
    dma1: &device::dma1::RegisterBlock,
    caller: Caller<()>,
    rx_len: usize,
    borrow_num: usize,
    timeout_ticks: u32,
) -> Result<(), ChannelError> {
    setup_read(state_ref, usart, dma1, caller, rx_len, borrow_num)?;
    let deadline = sys_get_timer().now + timeout_ticks as u64 + 1;
    sys_set_timer(Some(deadline), TIMEOUT_MASK);
    Ok(())
}

fn setup_read(
    state_ref: &mut DriverState,
    usart: &device::usart2::RegisterBlock,
    dma1: &device::dma1::RegisterBlock,
    caller: Caller<()>,
    rx_len: usize,
    borrow_num: usize,
) -> Result<(), ChannelError> {
    // Prepare for a transfer
    state_ref.receiver_state.pending_receiver = Some(Receiver {
        caller: caller,
        pos: 0,
        len: rx_len,
        borrow_num: borrow_num,
    });

    // Enable reception
    dma_receive_to_idle(dma1, usart);

    Ok(())
}

fn setup_transmit(
    state_ref: &mut DriverState,
    usart: &device::usart2::RegisterBlock,
    caller: Caller<()>,
    tx_len: usize,
    borrow_num: usize,
) -> Result<(), ChannelError> {
    // Prepare for a transfer
    state_ref.pending_transmitter = Some(Transmitter {
        caller: caller,
        pos: 0,
        len: tx_len,
        borrow_num: borrow_num,
    });

    // Enable transmit interrupts
    usart.cr1.modify(|_, w| w.txeie().enabled());
    Ok(())
}

fn setup_usart(usart: &device::usart2::RegisterBlock) -> Result<(), RCCError> {
    // Enable clock and leave reset
    // Turn on clock and leave reset
    let mut rcc = rcc_api::RCC::new();
    rcc.enable_clock(rcc_api::Peripheral::USART2)?;
    rcc.leave_reset(rcc_api::Peripheral::USART2)?;

    // The UART has clock and is out of reset, but isn't actually on until we:
    usart.cr1.write(|w| w.ue().enabled());

    // Work out our baud rate divisor.
    #[cfg(feature = "stm32f303re")]
    {
        const CLOCK_HZ: u32 = 36_000_000; // PLCK1
        usart
            .brr
            .write(|w| w.brr().bits((CLOCK_HZ / BAUDRATE) as u16));
    }
    // Enable the transmitter.
    usart.cr1.modify(|_, w| w.te().enabled());
    // Enable the receiver.
    usart.cr1.modify(|_, w| w.re().enabled());

    Ok(())
}

/// Write USART2 on GPIOA (pin 2,3)
fn setup_gpio() -> Result<(), RCCError> {
    // TODO: the fact that we interact with GPIOA directly here is an expedient
    // hack, but control of the GPIOs should probably be centralized somewhere.
    let gpioa = unsafe { &*device::GPIOA::ptr() };

    // Turn on clock and leave reset
    let mut rcc = rcc_api::RCC::new();
    rcc.enable_clock(rcc_api::Peripheral::GPIOA)?;
    rcc.leave_reset(rcc_api::Peripheral::GPIOA)?;

    // Setup Alternate Function 7
    gpioa
        .moder
        .modify(|_, w| w.moder2().alternate().moder3().alternate());
    gpioa.afrl.modify(|_, w| w.afrl2().af7().afrl3().af7());

    Ok(())
}

// RX_BUFFER_SIZE must be a multiple of the cache line of the device.
// On cortex-m4, cache seems not to exist (https://stackoverflow.com/questions/57377260/is-there-a-cache-in-the-arm-cortex-m4)
// On cortex-m7, is 32 bytes
// For more info, see https://community.st.com/s/question/0D53W00001Z9K9TSAV/maintaining-cpu-data-cache-coherence-for-dma-buffers
const RX_BUFFER_SIZE: usize = 128;
static mut RX_BUFFER: [u8; RX_BUFFER_SIZE] = [0xAA; RX_BUFFER_SIZE];

/**
 * DMA Support
 * USART2_RX -> DMA1 - Channel6
 * USART2_TX -> DMA1 - Channel7
 * (pag 272/1141)
 */
fn setup_dma(
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart2::RegisterBlock,
) -> Result<(), RCCError> {
    // Turn on clock
    let mut rcc = rcc_api::RCC::new();
    rcc.enable_clock(rcc_api::Peripheral::DMA1)?;

    // Configure DMA
    configure_dma_rx(dma1, usart);

    Ok(())
}

fn configure_dma_rx(dma1: &device::dma1::RegisterBlock, usart: &device::usart2::RegisterBlock) {
    // Disable the channel (tbs)
    dma1.ch6.cr.modify(|_, w| w.en().clear_bit());
    // Clear all interrupts
    dma1.ifcr.write(|w| w.cgif6().set_bit());
    // Set periph. address (RDR register)
    dma1.ch6
        .par
        .write(|w| unsafe { w.bits(usart.rdr.as_ptr() as u32) });
    // Set the mem. address (RX_Buffer)
    dma1.ch6
        .mar
        .write(|w| unsafe { w.bits(RX_BUFFER.as_mut_ptr() as u32) });
    // Set data length (number of elements to be received)
    dma1.ch6
        .ndtr
        .write(|w| unsafe { w.bits(RX_BUFFER_SIZE as u32) });
    // Set the transfer direction
    dma1.ch6.cr.modify(|_, w| w.dir().clear_bit());
    // Set channel priority
    dma1.ch6.cr.modify(|_, w| w.pl().very_high());
    // Set circular mode
    dma1.ch6.cr.modify(|_, w| w.circ().set_bit());
    // Set data length
    dma1.ch6.cr.modify(|_, w| w.psize().bits8());
    dma1.ch6.cr.modify(|_, w| w.msize().bits8());
    // Set increment mode
    dma1.ch6.cr.modify(|_, w| w.minc().set_bit());

    // Enable right interrupts (Half-transfer, Transfer-complete, Transfer-error)
    dma1.ch6
        .cr
        .modify(|_, w| w.htie().set_bit().tcie().set_bit().teie().set_bit());

    // Start DMA Channel
    dma1.ch6.cr.modify(|_, w| w.en().set_bit());
}

fn dma_receive_to_idle(_: &device::dma1::RegisterBlock, usart: &device::usart2::RegisterBlock) {
    if usart.cr3.read().dmar().bit_is_set() {
        return; // Already active
    }
    // Enable UART parity error interrupt (even if we don't use it now)
    usart.cr1.modify(|_, w| w.peie().set_bit());
    // Enable UART error interrupt (frame error, noise error, overrun error)
    usart.cr3.modify(|_, w| w.eie().set_bit());
    // Enable the DMA transfer
    usart.cr3.modify(|_, w| w.dmar().set_bit());
    // Turn on IDLE interrupt
    usart.cr1.modify(|_, w| w.idleie().set_bit());
}

/*
fn dma_stop_receive(
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart2::RegisterBlock,
) {
    // Disable UART parity error interrupt (even if we don't use it now)
    usart.cr1.modify(|_, w| w.peie().clear_bit());
    // Disable UART error interrupt (frame error, noise error, overrun error)
    usart.cr3.modify(|_, w| w.eie().clear_bit());
    // Disable the DMA transfer
    usart.cr3.modify(|_, w| w.dmar().clear_bit());
    // Turn off IDLE interrupt
    usart.cr1.modify(|_, w| w.idleie().clear_bit());
} */

fn dma_receive_callback(
    rec_state: &mut ReceiverState,
    available_up_to: usize,
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart2::RegisterBlock,
) {
    // Something changed?
    if rec_state.current_read_pos != available_up_to {
        // Flush cache, as we are not using DMA-marked memory
        // -> not needed on Cortex-M4 (but on M7 yes!)
        // From this point on, should be safe to read from the buffer
        if available_up_to > rec_state.current_read_pos {
            let received_bytes = available_up_to - rec_state.current_read_pos;
            // New data on the top to read, simple case
            rx_update_caller(
                &mut rec_state.pending_receiver,
                unsafe {
                    &RX_BUFFER
                        [rec_state.current_read_pos..rec_state.current_read_pos + received_bytes]
                },
                dma1,
                usart,
            );
        } else {
            // There could be a top part
            let received_bytes = RX_BUFFER_SIZE - rec_state.current_read_pos;
            if received_bytes > 0 {
                rx_update_caller(
                    &mut rec_state.pending_receiver,
                    unsafe {
                        &RX_BUFFER[rec_state.current_read_pos
                            ..rec_state.current_read_pos + received_bytes]
                    },
                    dma1,
                    usart,
                );
            }
            if available_up_to > 0 {
                // Bottom part available
                rx_update_caller(
                    &mut rec_state.pending_receiver,
                    unsafe { &RX_BUFFER[0..available_up_to] },
                    dma1,
                    usart,
                );
            }
        }
        // Update pointer
        rec_state.current_read_pos = available_up_to;
    }
}

fn rx_update_caller(
    receiver: &mut Option<Receiver>,
    data: &[u8],
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart2::RegisterBlock,
) {
    // Handler for every end of reception
    fn end_reception(
        _: &device::dma1::RegisterBlock,
        _: &device::usart2::RegisterBlock,
        receiver: &mut Option<Receiver>,
    ) -> hl::Caller<()> {
        // Cancel timer, or the new settings won't stick.
        sys_set_timer(None, TIMEOUT_MASK);
        // For now, avoid disabling interrupts. Otherwise we have problems with overrun
        // Return the caller
        core::mem::replace(receiver, None).unwrap().caller
    }

    // Get the receiver, or ignore
    let rx = if let Some(rx) = receiver {
        rx
    } else {
        return;
    };

    // Copy data in the buffer of caller
    let need_bytes = min(rx.len - rx.pos, data.len());
    // Try write data
    if rx
        .caller
        .borrow(rx.borrow_num)
        .write_fully_at(rx.pos, &data[0..need_bytes])
        .is_some()
    {
        rx.pos += need_bytes;
        if rx.pos == rx.len {
            // Success
            end_reception(dma1, usart, receiver).reply(());
        }
    } else {
        end_reception(dma1, usart, receiver).reply_fail(ChannelError::BadArgument);
    }
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn step_transmit(
    usart: &device::usart1::RegisterBlock,
    transmitter: &mut Option<Transmitter>,
    receiver: &mut Option<Receiver>,
) {
    // Clearer than just using replace:
    fn end_transmission(
        usart: &device::usart1::RegisterBlock,
        transmitter: &mut Option<Transmitter>,
    ) -> hl::Caller<()> {
        // Disable transmit interrupt
        usart.cr1.modify(|_, w| w.txeie().disabled());
        // Clear transmitter and return the caller field
        core::mem::replace(transmitter, None).unwrap().caller
    }

    // Get the transmitter, or ignore
    let tx = if let Some(tx) = transmitter {
        tx
    } else {
        return;
    };

    if let Some(byte) = tx.caller.borrow(tx.borrow_num).read_at::<u8>(tx.pos) {
        // Stuff byte into transmitter.
        #[cfg(feature = "stm32f303re")]
        usart.tdr.write(|w| w.tdr().bits(u16::from(byte)));

        tx.pos += 1;
        if tx.pos == tx.len {
            let caller = end_transmission(usart, transmitter);
            if !is_transmit_mode(&caller, receiver) {
                // Otherwise tell the caller the operation is finished
                caller.reply(());
            }
        }
    } else {
        let caller = end_transmission(usart, transmitter);
        if is_transmit_mode(&caller, receiver) {
            // Cancel also the read operation
            *receiver = None;
        }
        caller.reply_fail(ChannelError::BadArgument);
    }
}

fn is_transmit_mode(caller: &Caller<()>, receiver: &Option<Receiver>) -> bool {
    if receiver.is_some() {
        if caller.task_id() == receiver.as_ref().unwrap().caller.task_id() {
            return true;
        }
    }
    return false;
}

/*fn step_receive(
    usart: &device::usart1::RegisterBlock,
    receivers: &mut Vec<Receiver, MAX_QUEUE_LENGTH>,
) {
    fn end_reception(
        usart: &device::usart1::RegisterBlock,
        state: &mut Vec<Receiver, MAX_QUEUE_LENGTH>,
        rec_index: usize,
    ) -> hl::Caller<()> {
        // Delete receiver
        let rec = state.swap_remove(rec_index);
        // Clear interrupt only if we have no receivers
        if state.is_empty() {
            usart.cr1.modify(|_, w| w.rxneie().disabled());
        }
        // Return the caller
        rec.caller
    }

    // Read data
    #[cfg(feature = "stm32f303re")]
    let data = (usart.rdr.read().bits() & 0xFF) as u8; // Keep only the first 8 bits

    for index in 0..receivers.len() {
        // Get the receiver
        let receiver = &mut receivers[index];
        // Try write data
        if receiver.caller.borrow(0).write_at(receiver.pos, data).is_some() {
            receiver.pos += 1;
            if receiver.pos == receiver.len {
                // Success
                end_reception(usart, receivers, index).reply(());
            }
        } else {
            end_reception(usart, receivers, index).reply_fail(ChannelError::BadArgument);
        }
    }
}*/

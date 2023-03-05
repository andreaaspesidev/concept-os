#![no_std]
#![no_main]

use rcc_api::RCCError;
use uart_channel_api::*;
use userlib::{hl::Caller, *};

#[cfg(feature = "board_stm32f303re")]
use stm32f303re::device;

#[cfg(feature = "board_stm32l432kc")]
use stm32l432kc::device;

#[cfg(feature = "board_stm32l476rg")]
use stm32l476rg::device;

// Baudrate used during communication
const BAUDRATE: u32 = 115_200;
const USART_IRQ_MASK: u32 = 0b0000_0000_0000_0001;
const DMA1_CH3_IRQ_MASK: u32 = 0b0000_0000_0000_0010;
const TIMEOUT_MASK: u32 = 0b1000_0000_0000_0000;

// Configuration of this module
cfg_if::cfg_if! {
    if #[cfg(feature = "multi-support")] {
        const MAX_TRANSMITTERS: usize = 2; // Number of components allowed to trasmit simult.
        const MAX_RECEIVERS: usize = 2;    // Number of components allowed to wait for data simult.
        use heapless::Vec;
    }
}

// Driver state
cfg_if::cfg_if! {
    if #[cfg(feature = "multi-support")] {
        struct Transmitter {
            caller: hl::Caller<()>,
            id: u16,
            borrow_num: usize,
            len: usize,
            pos: usize,
            crc: u8
        }
        struct Receiver {
            caller: hl::Caller<()>,
            id: u16,
            borrow_num: usize,
            len: usize,
            pos: usize,
            deadline: Option<u64>
        }
        struct ReceiverState {
            pub receivers: Vec<Receiver, MAX_RECEIVERS>,
            pub header_data_buff: [u8; 4],
            pub header_data_buff_pos: usize,
            pub preamble_cnt: usize,
            pub crc8: u8,
            pub current_read_pos: usize,
            pub last_channel_id: Option<u16>,
            pub last_packet_len: u16,
            pub received_bytes: u16
        }
        struct TransmitterState {
            pub transmitters: Vec<Transmitter, MAX_TRANSMITTERS>,
        }
        struct DriverState {
            pub receiver_state: ReceiverState,
            pub transmitter_state: TransmitterState,
        }
    } else {
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
            pos: usize
        }
        struct ReceiverState {
            pub pending_receiver: Option<Receiver>,
            pub current_read_pos: usize,
        }
        struct DriverState {
            pub receiver_state: ReceiverState,
            pub pending_transmitter: Option<Transmitter>,
        }
    }
}

fn update_handler() -> ! {
    // Deconfigure everything, especially DMA
    //let usart = unsafe { &*device::USART2::ptr() };
    //let dma1 = unsafe { &*device::DMA1::ptr() };
    //dma_stop_receive(dma1, usart);
    // Power down and reset everything
    let mut rcc = rcc_api::RCC::new();
    rcc.enter_reset(rcc_api::Peripheral::DMA1).ok();
    rcc.enter_reset(rcc_api::Peripheral::USART3).ok();
    rcc.disable_clock(rcc_api::Peripheral::DMA1).ok();
    rcc.disable_clock(rcc_api::Peripheral::USART3).ok();
    hl::transfer_state(1u32);
}

#[export_name = "main"]
fn main() -> ! {
    // Wait for the old component to terminate (if any)
    let mut state_buff: [u8; 4] = [0; 4];
    let got_state = hl::get_state(&mut state_buff, (), |_s, _d: &u32| {}).is_ok();
    // For this component, it makes little sense and a lot of effort to transfer the state
    // In fact, reconfiguration of peripherals is still needed for DMA.
    kipc::activate_task();
    kipc::set_update_support(true);
    // From thin air, pluck a pointer to the USART register block.
    //
    // Safety: this is needlessly unsafe in the API. The USART is essentially a
    // static, and we access it through a & reference so aliasing is not a
    // concern. Were it literally a static, we could just reference it.
    let usart = unsafe { &*device::USART3::ptr() };
    // DMA1
    let dma1 = unsafe { &*device::DMA1::ptr() };

    setup_usart(usart).unwrap();
    setup_gpio().unwrap();
    setup_dma(dma1, usart).unwrap();

    // Turn on our interrupt. We haven't enabled any interrupt sources at the
    // USART side yet, so this won't trigger notifications yet.
    sys_irq_control(USART_IRQ_MASK, true);
    sys_irq_control(DMA1_CH3_IRQ_MASK, true);

    // Construct driver state
    #[cfg(feature = "multi-support")]
    let mut state = DriverState {
        receiver_state: ReceiverState {
            receivers: Vec::new(),
            current_read_pos: match got_state {
                true => 0,
                false => 1,
            },
            last_channel_id: None,
            last_packet_len: 0,
            header_data_buff: [0x00; 4],
            header_data_buff_pos: 0,
            preamble_cnt: 0,
            crc8: 0,
            received_bytes: 0,
        },
        transmitter_state: TransmitterState {
            transmitters: Vec::new(),
        },
    };
    #[cfg(not(feature = "multi-support"))]
    let mut state = DriverState {
        receiver_state: ReceiverState {
            pending_receiver: None,
            current_read_pos: match got_state {
                true => 0,
                false => 1,
            }, // Ask for some reason, the first byte we read is 0x00
        },
        pending_transmitter: None,
    };

    // Main loop
    let mut recv_buff: [u8; 12] = [0x00; 12];
    let mut frame_recovery: bool = true;
    loop {
        hl::recv(
            &mut recv_buff,
            USART_IRQ_MASK | DMA1_CH3_IRQ_MASK | TIMEOUT_MASK | STATE_TRANSFER_REQUESTED_MASK,
            &mut state,
            |state_ref, bits| {
                // Check if state transfer
                if bits & STATE_TRANSFER_REQUESTED_MASK != 0 {
                    // Call the update handler
                    update_handler();
                }
                // Timer IRQ
                if bits & TIMEOUT_MASK != 0 {
                    // Timeout for read expired
                    #[cfg(feature = "multi-support")]
                    cancel_expired(state_ref);

                    #[cfg(not(feature = "multi-support"))]
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
                        step_transmit(&usart, state_ref);
                    }

                    if usart_isr.idle().bit_is_set() {
                        // IDLE, we have to flush RX buffer
                        // -> get the number of bytes still to be read of DMA
                        #[cfg(feature = "board_stm32f303re")]
                        let remaining_rx = dma1.ch3.ndtr.read().bits() as usize;
                        #[cfg(any(feature = "board_stm32l432kc", feature = "board_stm32l476rg"))]
                        let remaining_rx = dma1.cndtr3.read().bits() as usize;

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
                        usart.icr.write(|w| w.idlecf().set_bit());
                    }

                    // Frame error
                    if usart_isr.fe().bit_is_set() {
                        if !frame_recovery {
                            panic!();
                        }
                        // For this time, just reset the error.
                        // This is needed as for some reason it happens to fire
                        // after the peripheral is configured. Not enough time to
                        // further investigate at the moment, maybe wait some flag
                        // will fix it.
                        usart.icr.write(|w| w.fecf().set_bit());
                        frame_recovery = false;
                    }

                    // Overrun error: happens only if we mess up with the DMA
                    // otherwise it's impossibile.
                    if usart_isr.ore().bit_is_set() {
                        // Something happened
                        panic!();
                    }

                    // Enable again interrupts
                    sys_irq_control(USART_IRQ_MASK, true);
                }
                // DMA IRQ
                if bits & DMA1_CH3_IRQ_MASK != 0 {
                    // DMA fired interrupt (RX)
                    let isr = dma1.isr.read();
                    if isr.htif3().bit_is_set() {
                        // Clear the flag
                        dma1.ifcr.write(|w| w.chtif3().set_bit());
                        // Half transfer complete!
                        dma_receive_callback(
                            &mut state_ref.receiver_state,
                            RX_BUFFER_SIZE / 2,
                            dma1,
                            usart,
                        );
                    } else if isr.tcif3().bit_is_set() {
                        // Clear the flag
                        dma1.ifcr.write(|w| w.ctcif3().set_bit());
                        // Full transfer complete
                        dma_receive_callback(
                            &mut state_ref.receiver_state,
                            RX_BUFFER_SIZE,
                            dma1,
                            usart,
                        );
                    } else if isr.teif3().bit_is_set() {
                        // Error
                        panic!();
                    }

                    // Enable again interrupt
                    sys_irq_control(DMA1_CH3_IRQ_MASK, true);
                }
            },
            |state_ref, op, msg| match op {
                Operation::WriteBlock => {
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "multi-support")] {
                            // Validate lease count and buffer sizes first.
                            let (msg, caller) = msg.fixed_with_leases::<WriteBlockRequest, ()>(1).ok_or(ChannelError::BadArgument)?;
                            let channel_id = msg.channel_id as u16;
                            // Check if we can add the writer
                            if !can_add_writer(state_ref, &caller, channel_id) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        } else {
                            // Validate lease count and buffer sizes first.
                            let ((), caller) = msg.fixed_with_leases(1).ok_or(ChannelError::BadArgument)?;
                            // Check if we can add the writer
                            if !can_add_writer(state_ref, &caller) {
                                return Err(ChannelError::BadArgument);
                            }
                        }
                    }

                    // Check borrow
                    let borrow = caller.borrow(0);
                    let info = borrow.info().ok_or(ChannelError::BadArgument)?;
                    if !info.attributes.contains(LeaseAttributes::READ) || info.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // Perform setup
                    #[cfg(feature = "multi-support")]
                    setup_transmit(state_ref, usart, caller, info.len, 0, channel_id)?;
                    #[cfg(not(feature = "multi-support"))]
                    setup_transmit(state_ref, usart, caller, info.len, 0)?;

                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
                Operation::ReadBlock => {
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "multi-support")] {
                            // Validate lease count and buffer sizes first.
                            let (msg, caller) = msg.fixed_with_leases::<ReadBlockRequest, ()>(1).ok_or(ChannelError::BadArgument)?;
                            let channel_id = msg.channel_id as u16;
                            // Check if we can add the reader
                            if !can_add_reader(state_ref, &caller, channel_id) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        } else {
                            // Validate lease count and buffer sizes first.
                            let ((), caller) = msg.fixed_with_leases(1).ok_or(ChannelError::BadArgument)?;
                            // Check if we can add the reader
                            if !can_add_reader(state_ref, &caller) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        }
                    }

                    // Check borrow
                    let borrow = caller.borrow(0);
                    let info = borrow.info().ok_or(ChannelError::BadArgument)?;
                    if !info.attributes.contains(LeaseAttributes::WRITE) || info.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // Perform setup
                    #[cfg(feature = "multi-support")]
                    setup_read(
                        state_ref, usart, dma1, caller, info.len, 0, None, channel_id,
                    )?;
                    #[cfg(not(feature = "multi-support"))]
                    setup_read(state_ref, usart, dma1, caller, info.len, 0)?;

                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
                Operation::ReadBlockTimed => {
                    // Validate lease count and buffer sizes first.
                    let (msg, caller) = msg
                        .fixed_with_leases::<ReadBlockTimedRequest, ()>(1)
                        .ok_or(ChannelError::BadArgument)?;

                    cfg_if::cfg_if! {
                        if #[cfg(feature = "multi-support")] {
                            let channel_id = msg.channel_id as u16;
                            // Check if we can add the reader
                            if !can_add_reader(state_ref, &caller, channel_id) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        } else {
                            // Check if we can add the reader
                            if !can_add_reader(state_ref, &caller) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        }
                    }

                    // Check borrow
                    let borrow = caller.borrow(0);
                    let info = borrow.info().ok_or(ChannelError::BadArgument)?;
                    if !info.attributes.contains(LeaseAttributes::WRITE) || info.len == 0 {
                        return Err(ChannelError::BadArgument);
                    }

                    // Setup
                    #[cfg(feature = "multi-support")]
                    setup_timed_read(
                        state_ref,
                        usart,
                        dma1,
                        caller,
                        info.len,
                        0,
                        msg.timeout_ticks,
                        channel_id,
                    )?;
                    #[cfg(not(feature = "multi-support"))]
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
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "multi-support")] {
                            let channel_id = msg.channel_id as u16;
                            // Check if we can add as reader
                            if !can_add_reader(state_ref, &caller, channel_id) {
                                return Err(ChannelError::ChannelBusy);
                            }
                            // Check if we can add as writer
                            if !can_add_writer(state_ref, &caller, channel_id) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        } else {
                            // Check if we can add as reader
                            if !can_add_reader(state_ref, &caller) {
                                return Err(ChannelError::ChannelBusy);
                            }
                            // Check if we can add as writer
                            if !can_add_writer(state_ref, &caller) {
                                return Err(ChannelError::ChannelBusy);
                            }
                        }
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
                    cfg_if::cfg_if! {
                        if #[cfg(feature = "multi-support")] {
                            setup_timed_read(
                                state_ref,
                                usart,
                                dma1,
                                caller.clone(),
                                info_in.len,
                                1,
                                msg.timeout_ticks,
                                channel_id
                            )?;
                            // Then, prepare a transmission
                            setup_transmit(state_ref, usart, caller.clone(), info_out.len, 0, channel_id)?;
                        } else {
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
                        }
                    }
                    // We'll do the rest as interrupts arrive.
                    Ok(())
                }
            },
        )
    }
}

#[cfg(feature = "multi-support")]
fn can_add_reader(state_ref: &mut DriverState, caller: &Caller<()>, channel_id: u16) -> bool {
    // It could happen that an updated component issue again a read. In this case,
    // discard the old request as surely it's no more valid.
    let mut can_add = !state_ref.receiver_state.receivers.is_full();
    state_ref.receiver_state.receivers.retain(|rec| {
        if rec.caller.task_id().component_id() == caller.task_id().component_id() {
            // Delete this element
            can_add = true;
            return false;
        } else if rec.id == channel_id {
            // It's not allowed to share channel ids
            can_add = false;
        }
        return true;
    });
    return can_add;
}
#[cfg(not(feature = "multi-support"))]
fn can_add_reader(state_ref: &mut DriverState, caller: &Caller<()>) -> bool {
    // It could happen that an updated component issue again a read. In this case,
    // discard the old request as surely it's no more valid.
    if let Some(rec) = &state_ref.receiver_state.pending_receiver {
        if rec.caller.task_id().component_id() == caller.task_id().component_id() {
            // No need to delete it first
            return true;
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(feature = "multi-support")]
fn can_add_writer(state_ref: &mut DriverState, caller: &Caller<()>, channel_id: u16) -> bool {
    // It could happen that an updated component issue again a read. In this case,
    // discard the old request as surely it's no more valid.
    let mut can_add = !state_ref.transmitter_state.transmitters.is_full();
    state_ref.transmitter_state.transmitters.retain(|tr| {
        if tr.caller.task_id().component_id() == caller.task_id().component_id() {
            // Delete this element
            can_add = true;
            return false;
        } else if tr.id == channel_id {
            // It's not allowed to share channel ids
            can_add = false;
        }
        return true;
    });
    return can_add;
}
#[cfg(not(feature = "multi-support"))]
fn can_add_writer(state_ref: &mut DriverState, caller: &Caller<()>) -> bool {
    // It could happen that an updated component issue again a read. In this case,
    // discard the old request as surely it's no more valid.
    if let Some(tr) = &state_ref.pending_transmitter {
        if tr.caller.task_id().component_id() == caller.task_id().component_id() {
            // No need to delete it first
            return true;
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(feature = "multi-support")]
fn cancel_expired(state_ref: &mut DriverState) {
    // Get the current time
    let current = sys_get_timer().now;
    let receivers = &mut state_ref.receiver_state.receivers;
    let mut next_deadline: Option<u64> = None;
    // Iterate over all the receivers, by keeping only the not-expired ones
    receivers.retain(|receiver| {
        if let Some(d_time) = receiver.deadline {
            if d_time <= current {
                // This receiver faulted
                receiver
                    .caller
                    .clone()
                    .reply_fail(ChannelError::ReadTimeOut);
                // Do not retain the element
                return false;
            } else {
                // Remember to update the deadline
                next_deadline = Some(match next_deadline {
                    Some(d) => core::cmp::min(d, d_time),
                    None => d_time,
                });
            }
        }
        return true;
    });
    // If we have a new deadline, apply it
    if let Some(d) = next_deadline {
        sys_set_timer(Some(d), TIMEOUT_MASK);
    }
}

#[cfg(feature = "multi-support")]
fn update_deadline(receiver_state: &mut ReceiverState) {
    let mut next_deadline: Option<u64> = None;
    let receivers = &receiver_state.receivers;
    for receiver in receivers {
        if let Some(d_time) = receiver.deadline {
            next_deadline = Some(match next_deadline {
                Some(d) => core::cmp::min(d, d_time),
                None => d_time,
            });
        }
    }
    // Apply the deadline, or cancel the timer if it's no more required
    sys_set_timer(next_deadline, TIMEOUT_MASK);
}

#[cfg(feature = "multi-support")]
fn setup_timed_read(
    state_ref: &mut DriverState,
    usart: &device::usart1::RegisterBlock,
    dma1: &device::dma1::RegisterBlock,
    caller: Caller<()>,
    rx_len: usize,
    borrow_num: usize,
    timeout_ticks: u32,
    channel_id: u16,
) -> Result<(), ChannelError> {
    let timer_state = sys_get_timer();
    let deadline = timer_state.now + timeout_ticks as u64 + 1;
    // Setup read
    setup_read(
        state_ref,
        usart,
        dma1,
        caller,
        rx_len,
        borrow_num,
        Some(deadline),
        channel_id,
    )?;
    // If we already got a deadline, check whether this one happens before
    if let Some(c_deadline) = timer_state.deadline {
        if deadline < c_deadline {
            // Change with this deadline
            sys_set_timer(Some(deadline), TIMEOUT_MASK);
        }
    } else {
        // No deadline, setup this one
        sys_set_timer(Some(deadline), TIMEOUT_MASK);
    }
    Ok(())
}
#[cfg(not(feature = "multi-support"))]
fn setup_timed_read(
    state_ref: &mut DriverState,
    usart: &device::usart1::RegisterBlock,
    dma1: &device::dma1::RegisterBlock,
    caller: Caller<()>,
    rx_len: usize,
    borrow_num: usize,
    timeout_ticks: u32,
) -> Result<(), ChannelError> {
    let timer_state = sys_get_timer();
    let deadline = timer_state.now + timeout_ticks as u64 + 1;
    setup_read(state_ref, usart, dma1, caller, rx_len, borrow_num)?;
    sys_set_timer(Some(deadline), TIMEOUT_MASK);
    Ok(())
}

#[cfg(feature = "multi-support")]
fn setup_read(
    state_ref: &mut DriverState,
    usart: &device::usart1::RegisterBlock,
    dma1: &device::dma1::RegisterBlock,
    caller: Caller<()>,
    rx_len: usize,
    borrow_num: usize,
    deadline: Option<u64>,
    channel_id: u16,
) -> Result<(), ChannelError> {
    // Prepare for a transfer
    state_ref
        .receiver_state
        .receivers
        .push(Receiver {
            id: channel_id,
            caller: caller,
            pos: 0,
            len: rx_len,
            borrow_num: borrow_num,
            deadline: deadline,
        })
        .unwrap_lite();

    // Enable reception
    dma_receive_to_idle(dma1, usart);

    Ok(())
}
#[cfg(not(feature = "multi-support"))]
fn setup_read(
    state_ref: &mut DriverState,
    usart: &device::usart1::RegisterBlock,
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

#[cfg(feature = "multi-support")]
fn setup_transmit(
    state_ref: &mut DriverState,
    usart: &device::usart1::RegisterBlock,
    caller: Caller<()>,
    tx_len: usize,
    borrow_num: usize,
    channel_id: u16,
) -> Result<(), ChannelError> {
    // Prepare for a transfer
    state_ref
        .transmitter_state
        .transmitters
        .push(Transmitter {
            id: channel_id,
            caller: caller,
            pos: 0,
            len: tx_len,
            borrow_num: borrow_num,
            crc: 0,
        })
        .unwrap_lite();

    // Enable transmit interrupts
    usart.cr1.modify(|_, w| w.txeie().enabled());
    Ok(())
}
#[cfg(not(feature = "multi-support"))]
fn setup_transmit(
    state_ref: &mut DriverState,
    usart: &device::usart1::RegisterBlock,
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

/*
 * ----------------
 *      Setup
 * ----------------
*/
fn setup_usart(usart: &device::usart1::RegisterBlock) -> Result<(), RCCError> {
    // Enable clock and leave reset
    // Turn on clock and leave reset
    let mut rcc = rcc_api::RCC::new();
    rcc.enable_clock(rcc_api::Peripheral::USART3)?;
    rcc.leave_reset(rcc_api::Peripheral::USART3)?;

    // The UART has clock and is out of reset, but isn't actually on until we:
    usart.cr1.write(|w| w.ue().enabled());

    // Work out our baud rate divisor.
    #[cfg(feature = "board_stm32f303re")]
    {
        const CLOCK_HZ: u32 = 36_000_000; // PLCK1
        usart
            .brr
            .write(|w| w.brr().bits((CLOCK_HZ / BAUDRATE) as u16));
    }
    #[cfg(feature = "board_stm32l476rg")]
    {
        const CLOCK_HZ: u32 = 80_000_000; // PLCK1
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

#[cfg(any(feature = "board_stm32f303re", feature = "board_stm32l476rg"))]
/// Write USART3 on GPIOC (pin 10,11)
fn setup_gpio() -> Result<(), RCCError> {
    // TODO: the fact that we interact with GPIOC directly here is an expedient
    // hack, but control of the GPIOs should probably be centralized somewhere.
    let gpioc = unsafe { &*device::GPIOC::ptr() };

    // Turn on clock and leave reset
    let mut rcc = rcc_api::RCC::new();
    rcc.enable_clock(rcc_api::Peripheral::GPIOC)?;
    rcc.leave_reset(rcc_api::Peripheral::GPIOC)?;

    // Setup Alternate Function 7
    gpioc
        .moder
        .modify(|_, w| w.moder10().alternate().moder11().alternate());
    gpioc.afrh.modify(|_, w| w.afrh10().af7().afrh11().af7());

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
 * USART3_RX -> DMA1 - Channel3
 * USART3_TX -> DMA1 - Channel2
 * (pag 272/1141)
 */
#[cfg(any(
    feature = "board_stm32f303re",
    feature = "board_stm32l476rg"
))]
fn setup_dma(
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart1::RegisterBlock,
) -> Result<(), RCCError> {
    // Turn on clock
    let mut rcc = rcc_api::RCC::new();
    rcc.enable_clock(rcc_api::Peripheral::DMA1)?;
    #[cfg(feature = "board_stm32l476rg")]
    rcc.enter_reset(rcc_api::Peripheral::DMA1)?;
    #[cfg(feature = "board_stm32l476rg")]
    rcc.leave_reset(rcc_api::Peripheral::DMA1)?;

    // Configure DMA
    configure_dma_rx(dma1, usart);

    Ok(())
}
#[cfg(feature = "board_stm32f303re")]
fn configure_dma_rx(dma1: &device::dma1::RegisterBlock, usart: &device::usart1::RegisterBlock) {
    // Disable the channel (tbs)
    dma1.ch3.cr.modify(|_, w| w.en().clear_bit());
    // Clear all interrupts
    dma1.ifcr.write(|w| w.cgif3().set_bit());
    // Set periph. address (RDR register)
    dma1.ch3
        .par
        .write(|w| unsafe { w.bits(usart.rdr.as_ptr() as u32) });
    // Set the mem. address (RX_Buffer)
    dma1.ch3
        .mar
        .write(|w| unsafe { w.bits(RX_BUFFER.as_mut_ptr() as u32) });
    // Set data length (number of elements to be received)
    dma1.ch3
        .ndtr
        .write(|w| unsafe { w.bits(RX_BUFFER_SIZE as u32) });
    // Set the transfer direction
    dma1.ch3.cr.modify(|_, w| w.dir().clear_bit());
    // Set channel priority
    dma1.ch3.cr.modify(|_, w| w.pl().very_high());
    // Set circular mode
    dma1.ch3.cr.modify(|_, w| w.circ().set_bit());
    // Set data length
    dma1.ch3.cr.modify(|_, w| w.psize().bits8());
    dma1.ch3.cr.modify(|_, w| w.msize().bits8());
    // Set increment mode
    dma1.ch3.cr.modify(|_, w| w.minc().set_bit());

    // Enable right interrupts (Half-transfer, Transfer-complete, Transfer-error)
    dma1.ch3
        .cr
        .modify(|_, w| w.htie().set_bit().tcie().set_bit().teie().set_bit());

    // Start DMA Channel
    dma1.ch3.cr.modify(|_, w| w.en().set_bit());
}

#[cfg(feature = "board_stm32l476rg")]
fn configure_dma_rx(dma1: &device::dma1::RegisterBlock, usart: &device::usart1::RegisterBlock) {
    // Disable the channel (tbs)
    dma1.ccr3.modify(|_, w| w.en().clear_bit());
    // Clear all interrupts
    dma1.ifcr.write(|w| w.cgif3().set_bit());

    // Select USART3_RX for Channel 3
    // See: RM0351/pag.339 (L476)
    dma1.cselr.modify(|_, w| w.c3s().bits(0b0010));

    // Set periph. address (RDR register)
    dma1.cpar3
        .write(|w| unsafe { w.bits(usart.rdr.as_ptr() as u32) });
    // Set the mem. address (RX_Buffer)
    dma1.cmar3
        .write(|w| unsafe { w.bits(RX_BUFFER.as_mut_ptr() as u32) });
    // Set data length (number of elements to be received)
    dma1.cndtr3
        .write(|w| unsafe { w.bits(RX_BUFFER_SIZE as u32) });
    // Set the transfer direction
    dma1.ccr3.modify(|_, w| w.dir().clear_bit());
    // Set channel priority
    dma1.ccr3.modify(|_, w| w.pl().very_high());
    // Set circular mode
    dma1.ccr3.modify(|_, w| w.circ().set_bit());
    // Set data length
    dma1.ccr3.modify(|_, w| w.psize().bits8());
    dma1.ccr3.modify(|_, w| w.msize().bits8());
    // Set increment mode
    dma1.ccr3.modify(|_, w| w.minc().set_bit());

    // Enable right interrupts (Half-transfer, Transfer-complete, Transfer-error)
    dma1.ccr3
        .modify(|_, w| w.htie().set_bit().tcie().set_bit().teie().set_bit());

    // Start DMA Channel
    dma1.ccr3.modify(|_, w| w.en().set_bit());
}

fn dma_receive_to_idle(_: &device::dma1::RegisterBlock, usart: &device::usart1::RegisterBlock) {
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

/*fn dma_stop_receive(dma1: &device::dma1::RegisterBlock, usart: &device::usart1::RegisterBlock) {
    // Disable DMA channel
    dma1.ch3.cr.modify(|_, w| w.en().clear_bit());
    // Disable UART parity error interrupt (even if we don't use it now)
    usart.cr1.modify(|_, w| w.peie().clear_bit());
    // Disable UART error interrupt (frame error, noise error, overrun error)
    usart.cr3.modify(|_, w| w.eie().clear_bit());
    // Disable the DMA transfer
    usart.cr3.modify(|_, w| w.dmar().clear_bit());
    // Turn off IDLE interrupt
    usart.cr1.modify(|_, w| w.idleie().clear_bit());
}*/

/*
 * ------------------------
 *      Data callbacks
 * ------------------------
*/

fn dma_receive_callback(
    rec_state: &mut ReceiverState,
    available_up_to: usize,
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart1::RegisterBlock,
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
                rec_state,
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
                    rec_state,
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
                    rec_state,
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

#[cfg(feature = "multi-support")]
const PREAMBLE_BYTE: u8 = 0b10101010;
#[cfg(feature = "multi-support")]
const PREAMBLE_LEN: usize = 4;

#[cfg(feature = "multi-support")]
fn rx_update_caller(
    receiver_state: &mut ReceiverState,
    mut data: &[u8],
    _dma1: &device::dma1::RegisterBlock,
    _usart: &device::usart1::RegisterBlock,
) {
    // We want to receive packets in the format
    //   +----------+------------+---------------+----------+--------+
    //   | Preamble | Channel ID | Packet Length |   Data   | CRC-8  |
    //   +----------+------------+---------------+----------+--------+
    //   | 4 bytes  | 2 bytes    | 2 bytes       | pl_bytes | 1 byte |
    //   +----------+------------+---------------+----------+--------+
    while data.len() > 0 {
        // Check whether we are currently receiving a packet
        if receiver_state.last_channel_id.is_none() {
            // In case we are waiting for a new packet, check whether we already
            // received the preamble.
            if receiver_state.preamble_cnt != PREAMBLE_LEN {
                // Search for the preamble of a packet
                // As it may come in (at least) two different calls,
                // we need to keep track of such bytes
                while data[0] == PREAMBLE_BYTE {
                    // Advance in data
                    data = &data[1..];
                    // Mark as arrived
                    receiver_state.preamble_cnt += 1;
                    if receiver_state.preamble_cnt == PREAMBLE_LEN {
                        // Got it!
                        break;
                    }
                    // If no more data, then wait next data
                    if data.len() == 0 {
                        return;
                    }
                }
                // We can get here in two cases: we found all the preamble bytes,
                // or we mistaken a byte for a preamble byte
                if receiver_state.preamble_cnt != PREAMBLE_LEN {
                    // Discard the byte
                    data = &data[1..];
                    // Restart from the beginning
                    receiver_state.preamble_cnt = 0;
                    continue; // Retry on the remaining data
                }
            }
            // If we get here, then we received at least the preamble.
            // Now it's time to receive the header (Channel ID + Packet Length)
            // Compute how many bytes are still missing
            let missing_header_len = core::cmp::min(
                receiver_state.header_data_buff.len() - receiver_state.header_data_buff_pos,
                data.len(),
            );
            // Copy the missing bytes in the buffer.
            // We need this buffer as such header could arrive fragmented in two
            // different calls.
            for i in 0..missing_header_len {
                receiver_state.header_data_buff[receiver_state.header_data_buff_pos + i] = data[i];
            }
            receiver_state.header_data_buff_pos += missing_header_len;
            data = &data[missing_header_len..]; // Skip these bytes

            // Check whether we got the whole header
            if receiver_state.header_data_buff_pos < receiver_state.header_data_buff.len() {
                // Surely here data.len() == 0, so wait for more data
                return;
            }

            // It's now time to decode such header
            receiver_state.last_channel_id = Some(u16::from_be_bytes(
                receiver_state.header_data_buff[0..2]
                    .try_into()
                    .unwrap_lite(),
            ));
            receiver_state.last_packet_len = u16::from_be_bytes(
                receiver_state.header_data_buff[2..4]
                    .try_into()
                    .unwrap_lite(),
            ) + 1; // +1 for the CRC-8

            // Initialize structure fields for this packet
            receiver_state.header_data_buff_pos = 0;
            receiver_state.received_bytes = 0;
            receiver_state.crc8 = 0;

            // Start computing CRC on header
            for i in 0..4 {
                crc8_update(&mut receiver_state.crc8, receiver_state.header_data_buff[i]);
            }
            continue; // Redo the check, as data.len() could be 0 now
        }
        // At this point, we are receiving the data bytes associated with the packet
        // Let's initialize some variables
        let c_id = receiver_state.last_channel_id.unwrap_lite(); // Channel ID
        let mut update_deadlines: bool = false; // Whether we removed some component
        let mut consumed_bytes: usize = 0; // Number of bytes of the packet given to the component
        let last_packet_slice: bool; // Marked when the crc of the packet is in view
        let mut packet_failed: bool = false; // In case we have a CRC failure

        // Compute the bytes we still need to receive for this packet (included CRC field)
        let to_be_received =
            (receiver_state.last_packet_len - receiver_state.received_bytes) as usize;

        // Only some of these are available in this round, so let's compute those
        let available_bytes = core::cmp::min(data.len(), to_be_received);

        // Check if we have the CRC in range.
        // In this case, we are ready to close the packet
        if to_be_received - available_bytes == 0 {
            // Finish computing the CRC on the bytes,
            // in case we have more than just the CRC field
            if available_bytes > 1 {
                for i in 0..(available_bytes - 1) {
                    crc8_update(&mut receiver_state.crc8, data[i]);
                }
            }
            // The CRC will always be the last byte of the available ones
            // for this packet.
            let crc8 = data[available_bytes - 1];

            // Check whether the packet is valid
            if crc8 != receiver_state.crc8 {
                // Failed validation!
                // This slice of the packet is discarded and marked as consumed.
                packet_failed = true;
                data = &data[available_bytes..];
                consumed_bytes += available_bytes;
            }
            // In any case, mark this is the last slice of the packet
            last_packet_slice = true;
            // Search for a new preamble
            receiver_state.last_channel_id = None;
            receiver_state.preamble_cnt = 0;
        } else {
            // We do not see the end of the packet in this slice,
            // just receive it and compute the CRC on all the data
            for i in 0..available_bytes {
                crc8_update(&mut receiver_state.crc8, data[i]);
            }
            last_packet_slice = false;
        }
        // Send this data to components, eventually removing them
        // from the listeners if they completed reception or they fail.
        receiver_state.receivers.retain_mut(|rx| {
            if rx.id == c_id {
                // Manage basic recovery for simple packet.
                if packet_failed {
                    // Let's reason on the possible outcomes:
                    // -> Part of the packet was already consumed by a prev. call of this component.
                    //    In this case, let's hope it has an higher level CRC.
                    // -> All data of this packet was copied to an active receiver.
                    //    We might have copied data here the prev. round. Just decrement the received bytes
                    //    in rx.pos, exploiting the fact received_bytes is incremented only after this loop
                    //    so it represents the bytes consumed up to the prev. round.
                    let discard_many =
                        core::cmp::min(rx.pos, receiver_state.received_bytes as usize);
                    // -1 on receiver_state.received_bytes is not needed,
                    // as surely in the prev. round we did not reached the CRC field
                    rx.pos -= discard_many;
                    // Skip this round, keeping the listener
                    return true;
                }
                // Compute how much data of the packet the component actually requested.
                // We admit waiting for less than the packet size, and even for more.
                // In the first case, the remaining packet is discarded, in the last multiple
                // packets are joined to fill the buffer
                let need_bytes = core::cmp::min(
                    rx.len - rx.pos,
                    match last_packet_slice {
                        true => available_bytes - 1,
                        false => available_bytes,
                    },
                );
                // Copy the data to the buffer of the listener
                if rx
                    .caller
                    .borrow(rx.borrow_num)
                    .write_fully_at(rx.pos, &data[0..need_bytes])
                    .is_some()
                {
                    // Consume this data
                    rx.pos += need_bytes;
                    data = &data[need_bytes..];
                    consumed_bytes += need_bytes;
                    // Check whether we completed the reception
                    if rx.pos == rx.len {
                        // Signal success
                        rx.caller.clone().reply(());
                        update_deadlines = true;
                        // Remove this listener
                        return false;
                    }
                    return true;
                } else {
                    // An exception occurred!
                    // Discard this data (and later the packet!)
                    data = &data[need_bytes..];
                    consumed_bytes += need_bytes;
                    // Signal failure
                    rx.caller.clone().reply_fail(ChannelError::BadArgument);
                    update_deadlines = true;
                    // Remove this listener
                    return false;
                }
            }
            // Otherwise this is not the component we are looking for,
            // so keep it!
            return true;
        });
        // In any case, consume the packet data in order not to lose the header tracking.
        if consumed_bytes < available_bytes {
            match last_packet_slice {
                false => {
                    data = &data[(available_bytes - consumed_bytes)..];
                }
                true => {
                    data = &data[(available_bytes - consumed_bytes - 1)..];
                }
            }
        }
        // Increment the counter of the consumed bytes
        receiver_state.received_bytes += available_bytes as u16;
        // Update deadlines
        if update_deadlines {
            update_deadline(receiver_state);
        }
    }
}

#[cfg(not(feature = "multi-support"))]
fn rx_update_caller(
    receiver_state: &mut ReceiverState,
    data: &[u8],
    dma1: &device::dma1::RegisterBlock,
    usart: &device::usart1::RegisterBlock,
) {
    // Handler for every end of reception
    fn end_reception(
        _: &device::dma1::RegisterBlock,
        _: &device::usart1::RegisterBlock,
        receiver: &mut Option<Receiver>,
    ) -> hl::Caller<()> {
        // Cancel timer, or the new settings won't stick.
        sys_set_timer(None, TIMEOUT_MASK);
        // For now, avoid disabling interrupts. Otherwise we have problems with overrun
        // Return the caller
        core::mem::replace(receiver, None).unwrap().caller
    }

    let receiver = &mut receiver_state.pending_receiver;

    // Get the receiver, or ignore
    let rx = if let Some(rx) = receiver {
        rx
    } else {
        return;
    };

    // Copy data in the buffer of caller
    let need_bytes = core::cmp::min(rx.len - rx.pos, data.len());
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

#[cfg(feature = "multi-support")]
fn step_transmit(usart: &device::usart1::RegisterBlock, state_ref: &mut DriverState) {
    // Clearer than just using replace:
    fn end_transmission(
        usart: &device::usart1::RegisterBlock,
        transmitter_state: &mut TransmitterState,
    ) -> hl::Caller<()> {
        // Disable transmit interrupt
        usart.cr1.modify(|_, w| w.txeie().disabled());
        // Clear transmitter and return the caller field
        transmitter_state.transmitters.swap_remove(0).caller
    }

    // Get a transmitter, or ignore
    let tx = if let Some(tx) = state_ref.transmitter_state.transmitters.first_mut() {
        tx
    } else {
        return;
    };

    // We have the preamble to be sent out first
    if tx.pos < PREAMBLE_LEN {
        // Stuff byte into transmitter.
        #[cfg(any(
            feature = "board_stm32f303re",
            feature = "board_stm32l476rg"
        ))]
        usart.tdr.write(|w| w.tdr().bits(u16::from(PREAMBLE_BYTE)));
        tx.pos += 1;
        return;
    } else if tx.pos < PREAMBLE_LEN + 2 {
        let c_id_bytes = tx.id.to_be_bytes();
        let byte = c_id_bytes[tx.pos - PREAMBLE_LEN];
        // Stuff byte into transmitter.
        #[cfg(any(
            feature = "board_stm32f303re",
            feature = "board_stm32l476rg"
        ))]
        usart.tdr.write(|w| w.tdr().bits(u16::from(byte)));
        // Update CRC
        crc8_update(&mut tx.crc, byte);
        tx.pos += 1;
        return;
    } else if tx.pos < PREAMBLE_LEN + 4 {
        let c_len_bytes = (tx.len as u16).to_be_bytes();
        let byte = c_len_bytes[tx.pos - 2 - PREAMBLE_LEN];
        // Stuff byte into transmitter.
        #[cfg(any(
            feature = "board_stm32f303re",
            feature = "board_stm32l476rg"
        ))]
        usart.tdr.write(|w| w.tdr().bits(u16::from(byte)));
        // Update CRC
        crc8_update(&mut tx.crc, byte);
        tx.pos += 1;
        return;
    } else if tx.pos - 4 - PREAMBLE_LEN == tx.len {
        // Send out the CRC
        #[cfg(any(
            feature = "board_stm32f303re",
            feature = "board_stm32l476rg"
        ))]
        usart.tdr.write(|w| w.tdr().bits(u16::from(tx.crc)));
        tx.pos += 1;
        return;
    } else if tx.pos - 4 - PREAMBLE_LEN > tx.len {
        // Finish transmission
        let caller = end_transmission(usart, &mut state_ref.transmitter_state);
        if !is_transmit_mode(&caller, &state_ref.receiver_state) {
            // Otherwise tell the caller the operation is finished
            caller.reply(());
        }
        return;
    }
    // Now we can actually send out data
    if let Some(byte) = tx
        .caller
        .borrow(tx.borrow_num)
        .read_at::<u8>(tx.pos - 4 - PREAMBLE_LEN)
    {
        // Update CRC
        crc8_update(&mut tx.crc, byte);
        // Stuff byte into transmitter.
        #[cfg(any(
            feature = "board_stm32f303re",
            feature = "board_stm32l476rg"
        ))]
        usart.tdr.write(|w| w.tdr().bits(u16::from(byte)));
        tx.pos += 1;
    } else {
        let caller = end_transmission(usart, &mut state_ref.transmitter_state);
        if is_transmit_mode(&caller, &state_ref.receiver_state) {
            // Cancel also the read operation
            state_ref.receiver_state.receivers.retain(|rx| {
                return rx.caller.task_id() != caller.task_id();
            });
        }
        caller.reply_fail(ChannelError::BadArgument);
    }
}

#[cfg(not(feature = "multi-support"))]
fn step_transmit(usart: &device::usart1::RegisterBlock, state_ref: &mut DriverState) {
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

    let transmitter = &mut state_ref.pending_transmitter;
    let receiver = &mut state_ref.receiver_state.pending_receiver;

    // Get the transmitter, or ignore
    let tx = if let Some(tx) = transmitter {
        tx
    } else {
        return;
    };

    if let Some(byte) = tx.caller.borrow(tx.borrow_num).read_at::<u8>(tx.pos) {
        // Stuff byte into transmitter.
        #[cfg(any(
            feature = "board_stm32f303re",
            feature = "board_stm32l476rg"
        ))]
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

#[cfg(feature = "multi-support")]
fn is_transmit_mode(caller: &Caller<()>, receiver_state: &ReceiverState) -> bool {
    for rx in &receiver_state.receivers {
        if caller.task_id() == rx.caller.task_id() {
            return true;
        }
    }
    return false;
}
#[cfg(not(feature = "multi-support"))]
fn is_transmit_mode(caller: &Caller<()>, receiver: &Option<Receiver>) -> bool {
    if receiver.is_some() {
        if caller.task_id() == receiver.as_ref().unwrap().caller.task_id() {
            return true;
        }
    }
    return false;
}

/// Optimized Dallas (now Maxim) iButton 8-bit CRC calculation.
/// Polynomial: x^8 + x^5 + x^4 + 1 (0x8C)
/// Initial value: 0x0
pub fn crc8_update(crc: &mut u8, byte: u8) {
    let mut tmp = (*crc) ^ byte;
    for _ in 0..8u8 {
        if tmp & 0x01 == 1 {
            tmp = (tmp >> 1) ^ 0x8C;
        } else {
            tmp >>= 1;
        }
    }
    *crc = tmp;
}

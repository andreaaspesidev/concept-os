// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

use flash::Flash;

mod crc;
mod flash;
mod messages;

use channel_api::UartChannel;
use messages::{
    CommandStartMessage, HeaderMessage, RawPacket, UpdateErrors, UpdateMessages,
};
use userlib::{task_slot, sys_log};
extern crate userlib;

task_slot!(CHANNEL, channel);

const TIMEOUT_MS: u32 = 10000;
const PACKET_BUFFER_SIZE: usize = 64;
const CHANNEL_ID: u16 = 5;

#[export_name = "main"]
fn main() -> ! {
    // Get instance
    let mut channel = channel_api::UartChannel::new(CHANNEL.get_task_id());
    let mut flash = flash::Flash::new();
    // flash.force_bank1();

    sys_log!("Update Online!");
    
    loop {
        // Wait for the start command
        let mut data: [u8; CommandStartMessage::get_size()] =
            [0x00; CommandStartMessage::get_size()];
        if channel.read_block(CHANNEL_ID, &mut data).is_ok() {
            // Parse message
            if let Ok(msg) = CommandStartMessage::from(&data) {
                // Start the process
                update_process(&mut channel, msg.get_image_size() as usize, &mut flash).unwrap();
            }
        }
    }
}

fn update_process(
    channel: &mut UartChannel,
    total_size: usize,
    flash: &mut Flash,
) -> Result<(), UpdateErrors> {
    // Prepare the flash by erasing the update bank
    flash.erase_update_bank();
    // Receive data
    let mut received_bytes: usize = 0;
    // While we have data to receive
    while received_bytes < total_size {
        // Ask for the program header and wait for the response
        let mut header_buff: [u8; HeaderMessage::get_size()] =
            [0; HeaderMessage::get_size()];
        // Launch the read operation
        channel
            .transmit_timed(
                CHANNEL_ID,
                &[UpdateMessages::SendSectionHeader as u8],
                &mut header_buff,
                TIMEOUT_MS,
            )
            .map_err(|_| UpdateErrors::Timeout)?;
        // Decode packet
        let header = HeaderMessage::from(&header_buff)?;
        sys_log!("Got section");
        // Process it
        process_section(channel, &header, flash)?;
        sys_log!("Processed section");
        // Update stats
        received_bytes += header.get_section_size() as usize;
    }
    // Send okay
    channel.write_block(CHANNEL_ID, &[UpdateMessages::Success as u8]).unwrap();
    sys_log!("Swapping banks");
    // Swap banks
    flash.swap_banks().map_err(|_| UpdateErrors::FlashError)?;
    // Restart device
    userlib::kipc::system_restart();
}

static_assertions::const_assert_eq!(PACKET_BUFFER_SIZE % 8, 0); // Assert multiple of 64 bits (8 bytes)

fn process_section(
    channel: &mut UartChannel,
    header: &HeaderMessage,
    flash: &mut Flash,
) -> Result<(), UpdateErrors> {
    let mut received_bytes: usize = 0;
    let total_bytes = header.get_section_size() as usize;
    let mut current_addr: u32 = header.get_section_addr();
    while received_bytes < total_bytes {
        // Check how much data we can read
        let to_read: usize =
            core::cmp::min(PACKET_BUFFER_SIZE, total_bytes - received_bytes)
                + 1;
        // Ask for the next fragment
        let mut pkt_buff: [u8; PACKET_BUFFER_SIZE + 1] =
            [0x00; PACKET_BUFFER_SIZE + 1];
        channel
            .transmit_timed(
                CHANNEL_ID,
                &[UpdateMessages::SendNextFragment as u8],
                &mut pkt_buff[0..to_read],
                TIMEOUT_MS,
            )
            .map_err(|_| UpdateErrors::Timeout)?;
        // Validate packet
        RawPacket::validate(&pkt_buff[0..to_read])?;
        // Write data using the correct alignment.
        // For simplicity, we assume to already have the correct alignments (64 bits)
        let available_bytes = to_read - 1;
        assert!(available_bytes % 8 == 0);
        let words = available_bytes / 8;
        for i in 0..words {
            let start_i = i * 8;
            let word =
                u64::from_le_bytes(pkt_buff[start_i..(start_i + 8)].try_into().unwrap());
            // Flash write
            flash
                .write_to_update_bank(current_addr, word)
                .map_err(|_| UpdateErrors::FlashError)?;
            // Increment address
            current_addr += 8;
        }
        // Update stats
        received_bytes += available_bytes;
    }
    Ok(())
}
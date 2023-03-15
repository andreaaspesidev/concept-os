// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod messages;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use crate::common_messages::*;
use crate::utils::*;

use self::messages::ComponentInfoMessage;
use self::messages::ComponentInfoResult;

pub fn info(
    channel_in_consumer: Receiver<u8>,
    channel_out_producer: Sender<Vec<u8>>,
    verbose: bool,
) {
    // Send hello message
    let hello_msg = HelloMessage::new(OperationType::SystemInfo);
    channel_write(&channel_out_producer, &hello_msg.get_raw());
    // Read hello response
    let mut buff: [u8; HelloResponseMessage::get_size()] = [0x00; HelloResponseMessage::get_size()];
    channel_read(&channel_in_consumer, &mut buff);
    // Validate hello response
    HelloResponseMessage::from(&buff).expect("Wrong response from device at HELLO");
    if verbose {
        println!("Got HELLO!");
    }
    read_system_info(&channel_in_consumer, &channel_out_producer);
}

fn read_system_info(channel_in_consumer: &Receiver<u8>, _channel_out_producer: &Sender<Vec<u8>>) {
    println!("----------- System Status -----------");
    let mut at_least_one: bool = false;
    loop {
        // Start two read the first bytes
        let mut buff: [u8; ComponentInfoMessage::max_size()] =
            [0x00; ComponentInfoMessage::max_size()];
        channel_read(
            &channel_in_consumer,
            &mut buff[..ComponentInfoMessage::min_size()],
        );
        let parse_res = ComponentInfoMessage::from(&buff[..ComponentInfoMessage::min_size()]);
        if parse_res.is_err() {
            match parse_res.unwrap_err() {
                ComponentInfoResult::NoMoreComponents => break, // Finished
                ComponentInfoResult::InvalidMessage | ComponentInfoResult::InvalidCRC => {
                    panic!("Got invalid message")
                }
                ComponentInfoResult::NeedMoreBytes => {
                    // Read missing bytes
                    channel_read(
                        &channel_in_consumer,
                        &mut buff[ComponentInfoMessage::min_size()..],
                    );
                    // Try to parse now
                    let msg_result = ComponentInfoMessage::from(&buff);
                    if msg_result.is_err() {
                        panic!("Cannot read component status message");
                    }
                    let msg = msg_result.unwrap();
                    println!("{:?}", msg);
                    at_least_one = true;
                }
            }
        }
    }
    if !at_least_one {
        println!("\n\tNo components found!");
    }
    println!("\n----------- ------------- -----------");
}

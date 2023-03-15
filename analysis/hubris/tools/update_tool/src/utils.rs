// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crossbeam_channel::{Sender, Receiver};

pub fn channel_write(mqtt_out_producer: &Sender<Vec<u8>>, buffer: &[u8]) {
    mqtt_out_producer.send(buffer.to_vec()).unwrap();
}

pub fn channel_flush_read(mqtt_in_consumer: &Receiver<u8>) {
    while !mqtt_in_consumer.is_empty() {
        mqtt_in_consumer.recv().unwrap();
    }
}

pub fn channel_read(mqtt_in_consumer: &Receiver<u8>, buffer: &mut [u8]) {
    // Wait to have enough bytes
    let mut pos: usize = 0;
    loop {
        let data = mqtt_in_consumer.recv().unwrap();
        buffer[pos] = data;
        pos += 1;
        if pos == buffer.len() {
            return;
        }
    }
}
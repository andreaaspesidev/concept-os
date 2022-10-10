use crossbeam_channel::{Sender, Receiver};
use std::io::{Read, Write};

pub fn serial_write(mqtt_out_producer: &Sender<Vec<u8>>, buffer: &[u8]) {
    mqtt_out_producer.send(buffer.to_vec()).unwrap();
}

pub fn flush_read(mqtt_in_consumer: &Receiver<u8>) {
    while !mqtt_in_consumer.is_empty() {
        mqtt_in_consumer.recv().unwrap();
    }
}

pub fn serial_read(mqtt_in_consumer: &Receiver<u8>, buffer: &mut [u8]) {
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

pub fn u32_from_le_bytes(buffer: &[u8]) -> u32 {
    return (buffer[0] as u32)
        | (buffer[1] as u32) << 8
        | (buffer[2] as u32) << 16
        | (buffer[3] as u32) << 24;
}

pub fn u16_from_le_bytes(buffer: &[u8]) -> u16 {
    return (buffer[0] as u16) | (buffer[1] as u16) << 8;
}
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

pub fn u32_from_le_bytes(buffer: &[u8]) -> u32 {
    return (buffer[0] as u32) | (buffer[1] as u32) << 8 | (buffer[2] as u32) << 16 | (buffer[3] as u32) << 24;
}

pub fn u16_from_le_bytes(buffer: &[u8]) -> u16 {
    return (buffer[0] as u16) | (buffer[1] as u16) << 8;
}

#[cfg(feature = "uart")]
pub fn openocd_board_to_chip(board: &String) -> String {
    // Try to extract the type (stm32f303re -> stm32 f3 -> stm32f3x.cfg)
    let board_line = &board[5..5 + 2];
    if board_line.starts_with("f") {
        return String::from(format!("{}x", &board[0..7]));
    } else if board_line.starts_with("l") {
        return String::from(format!("{}x", &board[0..7]));
    }
    panic!("Board not supported");
}
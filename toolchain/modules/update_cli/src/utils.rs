use std::io::{Write, Read};
use serialport::{TTYPort, SerialPort};

pub fn serial_write(serial: &mut TTYPort, buffer: &[u8]) {
    //std::thread::sleep(std::time::Duration::from_millis(100));
    serial.write_all(buffer).unwrap();
    serial.flush().unwrap();
    //for i in 0..buffer.len() {
    //    std::thread::sleep(std::time::Duration::from_millis(10));
    //    serial.write_all(&buffer[i..i+1]).unwrap();
    //    serial.flush().unwrap();
    //}
}

pub fn flush_read(serial: &mut TTYPort) {
    let mut buff: [u8;1] = [0x00;1];
    while serial.bytes_to_read().unwrap() > 0 {
        serial.read(&mut buff).unwrap();
    }
}

pub fn serial_read(serial: &mut TTYPort, buffer: &mut [u8]) {
    // Wait to have enough bytes
    let mut pos: usize = 0;
    let mut buff: [u8;1] = [0x00;1];
    loop {  
        let size = match serial.read(&mut buff) {
            Ok(x) => x,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (0), // Ignore
            Err(_) => {panic!("Failed to read from serial!")}
        };
        if size == 1 {
            buffer[pos] = buff[0];
            pos += 1;
            if pos == buffer.len() {
                break;
            }
        }
    }
}

pub fn u32_from_le_bytes(buffer: &[u8]) -> u32 {
    return (buffer[0] as u32) | (buffer[1] as u32) << 8 | (buffer[2] as u32) << 16 | (buffer[3] as u32) << 24;
}

pub fn u16_from_le_bytes(buffer: &[u8]) -> u16 {
    return (buffer[0] as u16) | (buffer[1] as u16) << 8;
}

pub fn openocd_board_to_chip(board: &String) -> String {
    // Try to extract the type (stm32f303re -> stm32 f3 -> stm32f3x.cfg)
    let board_line = &board[5..5 + 2];
    if board_line.starts_with("f") {
        return String::from(format!("{}x", &board[0..7]));
    }
    panic!("Board not supported");
}
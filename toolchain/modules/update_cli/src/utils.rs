use std::io::{Write, Read};
use serialport::{TTYPort, SerialPort};

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

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
    loop {
        match serial.read_exact(buffer) {
            Ok(_) => break,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (), // Ignore
            Err(_) => {panic!("Failed to read from serial!")}
        }
    }
}

pub fn u32_from_le_bytes(buffer: &[u8]) -> u32 {
    return (buffer[0] as u32) | (buffer[1] as u32) << 8 | (buffer[2] as u32) << 16 | (buffer[3] as u32) << 24;
}

pub fn u16_from_le_bytes(buffer: &[u8]) -> u16 {
    return (buffer[0] as u16) | (buffer[1] as u16) << 8;
}
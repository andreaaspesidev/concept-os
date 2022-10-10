#![no_std]
#![no_main]

use bthermo_api::{OutputType, Program, RepeatType, TimeStructure, MAX_PROGRAMS};
use userlib::*;
use zerocopy::{AsBytes, LayoutVerified};

const CMD_READ_RTC: u8 = 'c' as u8;
const CMD_SET_RTC: u8 = 's' as u8;
const CMD_READ_TEMP: u8 = 't' as u8;
const CMD_GET_OUTPUTS: u8 = 'o' as u8;
const CMD_GET_PROGRAMS: u8 = 'p' as u8;
const CMD_SET_PROGRAM: u8 = 'n' as u8;
const CMD_REM_PROGRAM: u8 = 'd' as u8;

#[export_name = "main"]
fn main() -> ! {
    // Activate task
    kipc::activate_task();
    
    // Create an instance of balancino
    let mut bthermo = bthermo_api::BThermo::new();

    // Listen for commands packet on serial
    let mut serial = uart_channel_api::UartChannel::new();

    // Main loop
    let mut in_buffer: [u8; 1] = [0; 1];
    loop {
        serial.read_block(&mut in_buffer).unwrap();
        match in_buffer[0] {
            CMD_READ_RTC => {
                let data = bthermo.read_rtc();
                if data.is_err() {
                    serial.write_block(&[CMD_READ_TEMP, 0x01]).unwrap();
                    continue;
                }
                let mut response_pkt: [u8; 9] = [0x00; 9];
                response_pkt[0] = CMD_READ_RTC;
                let time = data.unwrap().time;
                let data_bytes: &[u8] = time.as_bytes();
                for i in 0..data_bytes.len() {
                    response_pkt[1 + i] = data_bytes[i];
                }
                // Write response
                serial.write_block(&response_pkt).unwrap();
            }
            CMD_SET_RTC => {
                let mut request_pkt: [u8; 8] = [0x00; 8];
                if serial.read_block_timed(&mut request_pkt, 5000).is_err() {
                    // Ignore, request timeout
                    continue;
                }
                let day = request_pkt[0];
                let month = request_pkt[1];
                let year_bytes: [u8; 2] = [request_pkt[2], request_pkt[3]];
                let year = u16::from_le_bytes(year_bytes);
                let week_day = request_pkt[4];
                let hour = request_pkt[5];
                let min = request_pkt[6];
                let sec = request_pkt[7];
                // Generate response
                let mut response_pkt: [u8; 2] = [CMD_SET_RTC, 0x00];
                if bthermo
                    .set_rtc(sec, min, hour, week_day, day, month, year)
                    .is_ok()
                {
                    response_pkt[1] = 0x00;
                } else {
                    response_pkt[1] = 0x01;
                }
                // Send response
                serial.write_block(&response_pkt).unwrap();
            }
            CMD_READ_TEMP => {
                let data = bthermo.read_temperature();
                if data.is_err() {
                    serial.write_block(&[CMD_READ_TEMP, 0x01]).unwrap();
                    continue;
                }
                let mut response_pkt: [u8; 5] = [0x00; 5];
                response_pkt[0] = CMD_READ_TEMP;
                let temp_bytes = data.unwrap().temperature.to_le_bytes();
                response_pkt[1..].copy_from_slice(&temp_bytes);
                // Send response
                serial.write_block(&response_pkt).unwrap();
            }
            CMD_GET_OUTPUTS => {
                let result = bthermo.get_output_status();
                if result.is_err() {
                    serial.write_block(&[CMD_GET_OUTPUTS, 0x01]).unwrap();
                    continue;
                }
                let data = result.unwrap();
                let mut response_pkt: [u8; 5] = [0x00; 5];
                response_pkt[0] = CMD_GET_OUTPUTS;
                for i in 0..data.outputs_active.len() {
                    response_pkt[1 + i] = data.outputs_active[i]
                }
                // Send response
                serial.write_block(&response_pkt).unwrap();
            }
            CMD_GET_PROGRAMS => {
                let data = bthermo.get_programs().unwrap();
                let programs = &data.programs[0..data.num_valid as usize];
                // We must allocate the whole space, then we will send only the needed one
                // This is required as we do not have an allocator
                const PROGRAM_SIZE: usize = core::mem::size_of::<Program>();
                const RESPONSE_SIZE: usize = 1 + PROGRAM_SIZE * MAX_PROGRAMS;
                let mut response_pkt: [u8; RESPONSE_SIZE] = [0x00; RESPONSE_SIZE];
                response_pkt[0] = CMD_GET_PROGRAMS;
                for i in 0..programs.len() {
                    // Get program bytes
                    let prog_bytes: &[u8] = programs[i].as_bytes();
                    for j in 0..prog_bytes.len() {
                        response_pkt[1 + i * PROGRAM_SIZE + j] = prog_bytes[j];
                    }
                }
                // Write response
                serial
                    .write_block(&response_pkt[0..1 + PROGRAM_SIZE * data.num_valid as usize])
                    .unwrap();
            }
            CMD_SET_PROGRAM => {
                // Read program data
                const TIME_SIZE: usize = core::mem::size_of::<TimeStructure>();
                const REQUEST_SIZE: usize = core::mem::size_of::<TimeStructure>() * 2 + 4 + 1 + 1;
                let mut request_pkt: [u8; REQUEST_SIZE] = [0x00; REQUEST_SIZE];
                if serial.read_block_timed(&mut request_pkt, 5000).is_err() {
                    // Ignore, request timeout
                    continue;
                }
                let from_time = LayoutVerified::<_, TimeStructure>::new(&request_pkt[0..TIME_SIZE])
                    .unwrap()
                    .into_ref();
                let to_time =
                    LayoutVerified::<_, TimeStructure>::new(&request_pkt[TIME_SIZE..2 * TIME_SIZE])
                        .unwrap()
                        .into_ref();
                let ptr = 2 * TIME_SIZE;
                let temperature_setpoint = f32::from_le_bytes([
                    request_pkt[ptr],
                    request_pkt[ptr + 1],
                    request_pkt[ptr + 2],
                    request_pkt[ptr + 3],
                ]);
                let output = OutputType::try_from(request_pkt[ptr + 4]);
                let repeat = RepeatType::try_from(request_pkt[ptr + 5]);
                if output.is_err() || repeat.is_err() {
                    serial.write_block(&[CMD_SET_PROGRAM, 0x01]).unwrap();
                    continue;
                }
                let result = bthermo.set_program(
                    *from_time,
                    *to_time,
                    temperature_setpoint,
                    output.unwrap(),
                    repeat.unwrap(),
                );
                if result.is_err() {
                    serial.write_block(&[CMD_SET_PROGRAM, 0x02]).unwrap();
                    continue;
                } else {
                    serial.write_block(&[CMD_SET_PROGRAM, 0x00]).unwrap();
                    continue;
                }
            }
            CMD_REM_PROGRAM => {
                // Read program id
                let mut request_pkt: [u8; 1] = [0x00; 1];
                if serial.read_block_timed(&mut request_pkt, 5000).is_err() {
                    // Ignore, request timeout
                    continue;
                }
                let program_id: u8 = request_pkt[0];
                let result = bthermo.remove_program(program_id);
                let mut response_pkt: [u8; 2] = [0x00; 2];
                response_pkt[0] = CMD_REM_PROGRAM;
                if result.is_ok() {
                    response_pkt[1] = 0x00;
                } else {
                    response_pkt[1] = 0x01;
                }
                serial.write_block(&response_pkt).unwrap();
            }
            _ => {
                // Unknown command
                serial.write_block(&[0x00]).unwrap();
            }
        }
    }
}

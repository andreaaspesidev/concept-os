#![no_std]
#![no_main]

use bthermo_api::{OutputType, Program, RepeatType, TimeStructure, MAX_PROGRAMS, NUM_TEMPERATURES};
use channel_api::UartChannel;
use userlib::*;
use zerocopy::{AsBytes, LayoutVerified};

task_slot!(BTHERMO,bthermo);
task_slot!(CHANNEL,channel);

const SERIAL_CHANNEL: u16 = 11;
const CHANNEL_TIMEOUT_MS: u32 = 5000;

const CMD_READ_RTC: u8 = 'c' as u8;
const CMD_SET_RTC: u8 = 's' as u8;
const CMD_READ_TEMP: u8 = 't' as u8;
const CMD_GET_OUTPUTS: u8 = 'o' as u8;
const CMD_GET_PROGRAMS: u8 = 'p' as u8;
const CMD_SET_PROGRAM: u8 = 'n' as u8;
const CMD_REM_PROGRAM: u8 = 'd' as u8;

#[export_name = "main"]
fn main() -> ! {
    // Create an instance of balancino
    let mut bthermo = bthermo_api::BThermo::new(BTHERMO.get_task_id());

    // Listen for commands packet on serial
    let mut serial = channel_api::UartChannel::new(CHANNEL.get_task_id());

    sys_log!("[CONTROLLERv1] Online!");

    // Main loop
    let mut in_buffer: [u8; 1] = [0; 1];
    loop {
        // Wait for the command type
        channel_read_no_timeout(&mut serial, &mut in_buffer);
        // Analyze command
        match in_buffer[0] {
            CMD_READ_RTC => {
                let data = bthermo.read_rtc();
                if data.is_err() {
                    channel_write(&mut serial, &[CMD_READ_TEMP, 0x01]);
                    continue;
                }
                let mut response_pkt: [u8; 9] = [0x00; 9];
                response_pkt[0] = CMD_READ_RTC;
                let time = data.unwrap_lite().time;
                let data_bytes: &[u8] = time.as_bytes();
                for i in 0..data_bytes.len() {
                    response_pkt[1 + i] = data_bytes[i];
                }
                // Write response
                channel_write(&mut serial, &response_pkt);
            }
            CMD_SET_RTC => {
                let mut request_pkt: [u8; 8] = [0x00; 8];
                if !channel_read(&mut serial, &mut request_pkt) {
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
                if bthermo
                    .set_rtc(sec, min, hour, week_day, day, month, year)
                    .is_ok()
                {
                    channel_write(&mut serial, &[CMD_SET_RTC, 0x00]);
                } else {
                    channel_write(&mut serial, &[CMD_SET_RTC, 0x01]);
                }
            }
            CMD_READ_TEMP => {
                let data = bthermo.read_temperature();
                if data.is_err() {
                    channel_write(&mut serial, &[CMD_READ_TEMP, 0x01]);
                    continue;
                }
                let mut response_pkt: [u8; NUM_TEMPERATURES * 4 + 4 + 1] =
                    [0x00; NUM_TEMPERATURES * 4 + 4 + 1];
                response_pkt[0] = CMD_READ_TEMP;
                let temp_data = data.unwrap_lite();
                let temp_bytes = temp_data.as_bytes();
                for i in 0..temp_bytes.len() {
                    response_pkt[i + 1] = temp_bytes[i];
                }
                // Send response
                channel_write(&mut serial, &response_pkt);
            }
            CMD_GET_OUTPUTS => {
                let result = bthermo.get_output_status();
                if result.is_err() {
                    channel_write(&mut serial, &[CMD_GET_OUTPUTS, 0x01]);
                    continue;
                }
                let data = result.unwrap_lite();
                let mut response_pkt: [u8; 5] = [0x00; 5];
                response_pkt[0] = CMD_GET_OUTPUTS;
                for i in 0..data.outputs_active.len() {
                    response_pkt[1 + i] = data.outputs_active[i]
                }
                // Send response
                channel_write(&mut serial, &response_pkt);
            }
            CMD_GET_PROGRAMS => {
                let data = bthermo.get_programs().unwrap_lite();
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
                channel_write(
                    &mut serial,
                    &response_pkt[0..1 + PROGRAM_SIZE * data.num_valid as usize],
                );
            }
            CMD_SET_PROGRAM => {
                // Read program data
                const TIME_SIZE: usize = core::mem::size_of::<TimeStructure>();
                const REQUEST_SIZE: usize = core::mem::size_of::<TimeStructure>() * 2 + 4 + 1 + 1;
                let mut request_pkt: [u8; REQUEST_SIZE] = [0x00; REQUEST_SIZE];
                if !channel_read(&mut serial, &mut request_pkt) {
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
                    channel_write(&mut serial, &[CMD_SET_PROGRAM, 0x01]);
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
                    channel_write(&mut serial, &[CMD_SET_PROGRAM, 0x02]);
                } else {
                    channel_write(&mut serial, &[CMD_SET_PROGRAM, 0x00]);
                }
            }
            CMD_REM_PROGRAM => {
                // Read program id
                let mut request_pkt: [u8; 1] = [0x00; 1];
                if !channel_read(&mut serial, &mut request_pkt) {
                    // Ignore, request timeout
                    continue;
                }
                let program_id: u8 = request_pkt[0];
                let result = bthermo.remove_program(program_id);
                if result.is_ok() {
                    channel_write(&mut serial, &[CMD_REM_PROGRAM, 0x00]);
                } else {
                    channel_write(&mut serial, &[CMD_REM_PROGRAM, 0x01]);
                }
            }
            _ => {
                // Unknown command
                channel_write(&mut serial, &[0x00]);
            }
        }
    }
}

fn channel_write(serial: &mut UartChannel, data: &[u8]) {
    serial.write_block(SERIAL_CHANNEL, data).unwrap_lite();
}

fn channel_read(serial: &mut UartChannel, data: &mut [u8]) -> bool {
    serial
        .read_block_timed(SERIAL_CHANNEL, data, CHANNEL_TIMEOUT_MS)
        .is_ok()
}

fn channel_read_no_timeout(serial: &mut UartChannel, data: &mut [u8]) {
    serial.read_block(SERIAL_CHANNEL, data).unwrap_lite()
}

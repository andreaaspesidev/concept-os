#![no_std]
#![no_main]

mod ds3231;
mod history;
mod i2c;
mod outputs;
mod programs;
mod state;
mod tmp117;

use bthermo_api::{
    GetOutputStatusResponse, GetProgramsResponse, Operation, OutputType, Program, ReadRTCResponse,
    ReadTemperatureResponse, RemoveProgramRequest, RepeatType, SetProgramRequest,
    SetProgramResponse, SetRTCRequest, ThermoError, TimeStructure, MAX_PROGRAMS,
};
use rcc_api::RCC;
use state::{StateManager, TransferableState};
use userlib::{FromPrimitive, *};
use zerocopy::{AsBytes, LayoutVerified};

use crate::history::History;

const TIMER_IRQ_MASK: u32 = 1 << 0;
const SLEEP_MAX_TIME_MS: u64 = 500;

#[export_name = "main"]
fn main() -> ! {
    // Prepare for state migration
    let mut transfer_buffer: [u8; core::mem::size_of::<TransferableState>()] =
        [0x00; core::mem::size_of::<TransferableState>()];
    let mut prev_state: Option<&TransferableState> = None;
    let transfer_result = hl::get_state(&mut transfer_buffer, (), |_, data: &TransferableState| {
        prev_state = Some(data);
    });

    // Create a safe instance of the RCC
    let mut rcc = rcc_api::RCC::new();
    // Create the I2C channel
    let mut i2c = i2c::I2C_Channel::new();
    // Create instance of the RTC
    let mut rtc = ds3231::DS3231::new();
    // Create instance of Thermometer
    let mut thermo = tmp117::TMP117::new();
    // Create an instance of outputs
    let mut output_controller = outputs::OutputController::new();

    // Initialize hardware
    if transfer_result.is_err() {
        start_up_routine(
            &mut rcc,
            &mut i2c,
            &mut rtc,
            &mut thermo,
            &mut output_controller,
        );
    }

    // Activate (always better after hardware configuration)
    kipc::activate_task();

    // Enable state migration
    kipc::set_update_support(true);

    // Create an instance of the state manager
    let mut state_manager = StateManager::new(prev_state);

    // Create an instance of the history
    let mut history = History::new();

    // Create an instance of the program controller
    let mut program_manager = programs::ProgramManager::new();

    // Main loop
    loop {
        // Acquire data from sensors
        let time_result = rtc.read_sensor(&mut i2c);
        if time_result.is_err() {
            error_loop(ThermoError::RTCNotConnected, Some(&mut state_manager));
        }
        let temp_result = thermo.read_temperature(&mut i2c);
        if temp_result.is_err() {
            error_loop(ThermoError::TempNotConnected, Some(&mut state_manager));
        }
        let current_time = time_result.unwrap_lite();
        let current_temp = temp_result.unwrap_lite();
        let current_ticks = sys_get_timer().now;
        // Update stats
        history.add_temperature(current_ticks, current_temp, &mut state_manager);
        // Process the control loop
        program_manager.run(
            &current_time,
            current_temp,
            &mut output_controller,
            &mut state_manager,
        );
        // Process requests, until the timeout expires
        process_requests(
            &current_time,
            &mut i2c,
            &mut rtc,
            &output_controller,
            &mut program_manager,
            &mut history,
            &mut state_manager,
        );
    }
}

fn start_up_routine(
    rcc: &mut RCC,
    i2c: &mut i2c::I2C_Channel,
    rtc: &mut ds3231::DS3231,
    thermo: &mut tmp117::TMP117,
    outputs: &mut outputs::OutputController,
) {
    // Initialize hardware
    let init_result = init_hardware(rcc, i2c, rtc, thermo, outputs);

    // If initialization fails, just respond to every request with the error
    if let Err(err) = init_result {
        error_loop(err, None);
    }
}

/// Whenever we are not able to communicate with a sensor, just give up
/// and respond with the error to any incoming requests
fn error_loop(err: ThermoError, state_manager: Option<&mut StateManager>) {
    if state_manager.is_none() {
        // Disable state transfer, in case was enabled
        kipc::set_update_support(false);
    }
    loop {
        let result = sys_recv(&mut [], STATE_TRANSFER_REQUESTED_MASK, None);
        if result.is_ok() {
            let msg = result.unwrap_lite();
            // Check if is a state transfer request
            if msg.sender == TaskId::KERNEL {
                if msg.operation & STATE_TRANSFER_REQUESTED_MASK > 0 {
                    // If we have a state to transfer, let's transfer it
                    if let Some(sm) = state_manager {
                        // State transfer requested
                        hl::transfer_state(sm.get_transferable_state());
                    }
                }
                // Continue with the loop, ignore
                continue;
            }
            // Just send back the error
            sys_reply(msg.sender, err.into(), &[]);
        }
    }
}

/// Initializes hardware peripherals. It was separated from the constructor since
/// it might not be called upon update, as the hardware can be assumed to have already been configured
fn init_hardware(
    rcc: &mut RCC,
    i2c: &mut i2c::I2C_Channel,
    rtc: &mut ds3231::DS3231,
    thermo: &mut tmp117::TMP117,
    outputs: &mut outputs::OutputController,
) -> Result<(), ThermoError> {
    i2c.init_hardware(rcc)?;
    rtc.init_hardware(i2c)?;
    thermo.init_hardware(i2c)?;
    outputs.init_hardware(rcc)?;
    Ok(())
}

fn process_requests(
    current_time: &TimeStructure,
    i2c: &mut i2c::I2C_Channel,
    rtc: &mut ds3231::DS3231,
    output_controller: &outputs::OutputController,
    program_manager: &mut programs::ProgramManager,
    history: &mut History,
    state_manager: &mut StateManager,
) {
    let mut request_buffer: [u8; 32] = [0; 32];
    // Pause to read commands
    sys_set_timer(
        Some(sys_get_timer().now + SLEEP_MAX_TIME_MS),
        TIMER_IRQ_MASK,
    );
    loop {
        let result = sys_recv(
            &mut request_buffer,
            TIMER_IRQ_MASK | STATE_TRANSFER_REQUESTED_MASK,
            None,
        );
        if result.is_ok() {
            // Parse message
            let rm = result.unwrap_lite();
            // Check if the timeout expired
            if rm.sender == TaskId::KERNEL {
                if rm.operation & STATE_TRANSFER_REQUESTED_MASK > 0 {
                    // State transfer requested
                    hl::transfer_state(state_manager.get_transferable_state());
                }
                // Reset timer
                sys_set_timer(None, TIMER_IRQ_MASK);
                return;
            }
            // Handle operation
            let operation = Operation::from_u32(rm.operation);
            if operation.is_some() {
                match operation.unwrap() {
                    Operation::ReadRTC => {
                        let response = ReadRTCResponse {
                            time: current_time.clone(),
                        };
                        sys_reply(rm.sender, 0, response.as_bytes());
                    }
                    Operation::SetRTC => {
                        let msg = LayoutVerified::<_, SetRTCRequest>::new(
                            &request_buffer[..rm.message_len],
                        )
                        .unwrap_lite()
                        .into_ref();
                        let time = msg.time;
                        let result = rtc.set_date(
                            i2c,
                            time.sec,
                            time.min,
                            time.hour,
                            time.week_day,
                            time.day,
                            time.month,
                            time.year,
                        );
                        if result.is_ok() {
                            sys_reply(rm.sender, 0, &[]);
                        } else {
                            sys_reply(rm.sender, ThermoError::RTCNotConnected.into(), &[]);
                        }
                    }
                    Operation::ReadTemperature => {
                        let response = ReadTemperatureResponse {
                            history: history.get_temperatures(state_manager),
                            operation_value: history.perform_operation(state_manager),
                        };
                        sys_reply(rm.sender, 0, response.as_bytes());
                    }
                    Operation::GetOutputs => {
                        let response = GetOutputStatusResponse {
                            outputs_active: output_controller.get_status().map(|s| s as u8),
                        };
                        sys_reply(rm.sender, 0, response.as_bytes());
                    }
                    Operation::GetPrograms => {
                        let programs = &state_manager.get_program_state().programs;
                        // Create an array for the response data
                        let mut response_programs: [Program; MAX_PROGRAMS] =
                            [Program::default(); MAX_PROGRAMS];
                        for (i, p) in programs.iter().enumerate() {
                            response_programs[i].program_id = p.program_id;
                            response_programs[i].from_time = p.from_time;
                            response_programs[i].to_time = p.to_time;
                            response_programs[i].temperature_setpoint = p.temperature_setpoint;
                            response_programs[i].output = p.output as u8;
                            response_programs[i].repeat = p.repeat as u8;
                        }
                        let response = GetProgramsResponse {
                            num_valid: programs.len() as u16,
                            programs: response_programs,
                        };
                        sys_reply(rm.sender, 0, response.as_bytes());
                    }
                    Operation::SetProgram => {
                        let msg = LayoutVerified::<_, SetProgramRequest>::new(
                            &request_buffer[..rm.message_len],
                        )
                        .unwrap_lite()
                        .into_ref();

                        let program: Program = msg.program;
                        let result = program_manager.add_program(
                            program.from_time,
                            program.to_time,
                            program.temperature_setpoint,
                            OutputType::try_from(program.output).unwrap_lite(),
                            RepeatType::try_from(program.repeat).unwrap_lite(),
                            state_manager,
                        );
                        match result {
                            Ok(p_id) => {
                                let response = SetProgramResponse { program_id: p_id };
                                sys_reply(rm.sender, 0, response.as_bytes());
                            }
                            Err(_) => {
                                sys_reply(rm.sender, ThermoError::NoMoreSpace.into(), &[]);
                            }
                        }
                    }
                    Operation::RemoveProgram => {
                        let msg = LayoutVerified::<_, RemoveProgramRequest>::new(
                            &request_buffer[..rm.message_len],
                        )
                        .unwrap_lite()
                        .into_ref();
                        let program_id: u8 = msg.program_id;
                        let result = program_manager.remove_program(program_id, state_manager);
                        if result.is_ok() {
                            sys_reply(rm.sender, 0, &[]);
                        } else {
                            sys_reply(rm.sender, ThermoError::MissingProgram.into(), &[]);
                        }
                    }
                }
            } else {
                sys_reply(rm.sender, ThermoError::BadArgument.into(), &[]);
            }
        }
    }
}

use bthermo_api::{OutputType, RepeatType, TimeStructure};
use userlib::UnwrapLite;

use crate::{
    outputs::OutputController,
    state::{ProgramEntry, StateManager},
};

/**
 * bTermo Programs
 *
 * This is the programmable logic of the bTermo. At the moment it's very simple
 * and leverages both the RTC (Real Time Clock) and the Temperature sensor (of course!).
 *
 * The template followed by a program is the following:
 * <FROM_TIME> <TO_TIME> <TEMPERATURE> <OUTPUT> <REPEAT>
 *
 * where:
 * - <FROM_TIME> is start time this program is active (time structure, from the second to the year)
 * - <TO_TIME> is start time this program is not more active (time structure, from the second to the year)
 * - <TEMPERATURE> is the setpoint temperature
 * - <OUTPUT> can be:
 *      - OUT1: to control output 1
 *      - OUT2: to control output 2
 *      - OUT3: to control output 3
 *      - OUT4: to control output 4
 * - <REPEAT> can be:
 *      - NO_REPEAT: if this program is never to be repeated, apart from the specified period
 *      - EVERY_DAY: if this program is to be repeated every day
 *      - EVERY_WEEK: if this program is to be repeated every week
 *      - EVERY_MONTH: if this program is to be repeated every month
 *      - EVERY_YEAR: if this program is to be repeated every year
 *
 * Each program is identified by a number, to allow its deletion.
 * As regards implementation, I use an u16, where each program is identified by a bit.
 * To get the next available program number, just search the lowest bit set
 */

#[derive(Debug, Clone, Copy)]
pub enum ProgramError {
    NoMoreSpace,
}

pub struct ProgramManager {}

impl ProgramManager {
    pub fn new() -> Self {
        Self {}
    }

    fn get_next_id(&mut self, state_manager: &mut StateManager) -> u8 {
        let program_state = state_manager.get_program_state_mut();
        // implementation of ffs
        let value_32 = program_state.program_bits as u32;
        let ffs = (31 - (value_32 & value_32.wrapping_neg()).leading_zeros()) as u16;
        // Unset the bit
        program_state.program_bits &= !(1 << ffs);
        return ffs as u8;
    }
    fn free_id(&mut self, program_id: u8, state_manager: &mut StateManager) {
        assert!(program_id < 16);
        let program_state = state_manager.get_program_state_mut();
        program_state.program_bits |= 1 << program_id;
    }

    /// Adds a new program to the system, if there is space from a new one.
    /// In this case, returns the identifier of the program
    pub fn add_program(
        &mut self,
        from_time: TimeStructure,
        to_time: TimeStructure,
        temperature_setpoint: f32,
        output: OutputType,
        repeat: RepeatType,
        state_manager: &mut StateManager,
    ) -> Result<u8, ProgramError> {
        // Check if we have enough space
        if state_manager.get_program_state().programs.is_full() {
            return Err(ProgramError::NoMoreSpace);
        }
        // Check dates makes sense
        /*if from_time >= to_time {
            return Err(ProgramError::InvalidDates);
        }*/
        // Get a new id
        let program_id = self.get_next_id(state_manager);
        // Add the program
        state_manager
            .get_program_state_mut()
            .programs
            .push(ProgramEntry {
                program_id,
                from_time,
                to_time,
                temperature_setpoint,
                output,
                repeat,
            })
            .unwrap_lite(); // As we already checked there is space
                            // Return the identifier
        Ok(program_id)
    }
    /// Removes a program
    /// (more efficient with Maps, but seems over-complicated here)
    pub fn remove_program(
        &mut self,
        program_id: u8,
        state_manager: &mut StateManager,
    ) -> Result<(), ()> {
        let program_state = state_manager.get_program_state_mut();
        // Get program index
        let mut program_index: Option<usize> = None;
        for (index, program) in program_state.programs.iter().enumerate() {
            if program.program_id == program_id {
                program_index = Some(index);
                break;
            }
        }
        // Remove the program
        if let Some(index) = program_index {
            program_state.programs.swap_remove(index);
            // Free the id
            self.free_id(program_id, state_manager);
            return Ok(());
        }
        return Err(());
    }

    fn program_active(
        current_time: &TimeStructure,
        program_from_time: &TimeStructure,
        program_to_time: &TimeStructure,
        repeat: RepeatType,
    ) -> bool {
        match repeat {
            RepeatType::EveryDay => {
                // Compare only sec, min, hour
                let current_time_s: u32 = extract_up_to_hour(current_time);
                let from_time_s: u32 = extract_up_to_hour(program_from_time);
                let to_time_s: u32 = extract_up_to_hour(program_to_time);
                return current_time_s >= from_time_s && current_time_s < to_time_s;
            }
            RepeatType::EveryWeek => {
                // EveryDay + compare week day
                let current_time_s: u32 = extract_up_to_weekday(current_time);
                let from_time_s: u32 = extract_up_to_weekday(program_from_time);
                let to_time_s: u32 = extract_up_to_weekday(program_to_time);
                return current_time_s >= from_time_s && current_time_s < to_time_s;
            }
            RepeatType::EveryMonth => {
                // EveryDay + compare day
                let current_time_s = extract_up_to_day(current_time);
                let from_time_s = extract_up_to_day(program_from_time);
                let to_time_s = extract_up_to_day(program_to_time);
                return current_time_s >= from_time_s && current_time_s < to_time_s;
            }
            RepeatType::EveryYear => {
                // EveryMonth + compare month
                let current_time_s = extract_up_to_month(current_time);
                let from_time_s = extract_up_to_month(program_from_time);
                let to_time_s = extract_up_to_month(program_to_time);
                return current_time_s >= from_time_s && current_time_s < to_time_s;
            }
            RepeatType::NoRepeat => {
                // EveryMonth + compare year
                let current_time_s = extract_up_to_year(current_time);
                let from_time_s = extract_up_to_year(program_from_time);
                let to_time_s = extract_up_to_year(program_to_time);
                return current_time_s >= from_time_s && current_time_s < to_time_s;
            }
        }
    }

    /// Routine that checks and executes all programs
    pub fn run(
        &mut self,
        current_time: &TimeStructure,
        current_temperature: f32,
        outputs: &mut OutputController,
        state_manager: &mut StateManager,
    ) {
        let mut out_state: [bool; 4] = [false; 4];
        for program in &state_manager.get_program_state().programs {
            if Self::program_active(
                current_time,
                &program.from_time,
                &program.to_time,
                program.repeat,
            ) {
                // Program is active, check temperature
                if current_temperature < program.temperature_setpoint {
                    // Turn on output
                    out_state[program.output as usize - 1] = true;
                }
            }
        }
        // Apply outputs state
        for (index, state) in out_state.iter().enumerate() {
            let out_num = index + 1;
            match *state {
                true => outputs.turn_on(out_num.into()),
                false => outputs.turn_off(out_num.into()),
            }
        }
    }
}

fn extract_up_to_hour(ts: &TimeStructure) -> u32 {
    return ts.hour as u32 * 3600 + ts.min as u32 * 60 + ts.sec as u32;
}

fn extract_up_to_weekday(ts: &TimeStructure) -> u32 {
    return ts.week_day as u32 * 24 * 3600 + extract_up_to_hour(ts);
}

fn extract_up_to_day(ts: &TimeStructure) -> u32 {
    return ts.day as u32 * 24 * 3600 + extract_up_to_hour(ts);
}

fn extract_up_to_month(ts: &TimeStructure) -> u32 {
    // Convert day and month in day of year.
    // See: http://howardhinnant.github.io/date_algorithms.html
    let day_of_year =
        (153 * (match ts.month > 2 {
            true => ts.month as u32 - 3,
            false => ts.month as u32 + 9,
        }) + 2)
            / 5
            + ts.day as u32
            - 1;
    // Now just get back the seconds
    // 365 * 24 * 3600 < u32::MAX;
    return day_of_year * 24 * 3600 + extract_up_to_hour(ts);
}

/// Valid only when year > 1970 (Unix epoch)
fn extract_up_to_year(ts: &TimeStructure) -> u32 {
    // number of days since civil 1970-01-01
    let mut year = ts.year as u32;
    year -= (ts.month <= 2) as u32;
    let era = year / 400;
    let yoe = year - era * 400;
    let doy =
        (153 * (match ts.month > 2 {
            true => ts.month as u32 - 3,
            false => ts.month as u32 + 9,
        }) + 2)
            / 5
            + ts.day as u32
            - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe - 719468;
    return days * 24 * 3600 + extract_up_to_hour(ts);
}

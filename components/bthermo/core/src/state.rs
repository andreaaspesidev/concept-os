use bthermo_api::{OutputType, Program, RepeatType, TimeStructure};
use heapless::Vec;
use userlib::UnwrapLite;
use zerocopy::{AsBytes, FromBytes};

#[repr(C)]
#[derive(FromBytes, AsBytes)]
pub struct TransferableState {
    pub num_valid: u16,
    pub program_bits: u16,
    pub programs: [Program; MAX_PROGRAMS],
}

const MAX_PROGRAMS: usize = 16;

/// Manages application state, in order to enable state transfer.
/// The state during a transfer must be in fact serializable into bytes,
/// and this could limit application choice of state.
///
/// Here the two aspects are decoupled, and a transfer-ready state
/// is generated only when requested.
pub struct StateManager {
    programs: Vec<ProgramEntry, MAX_PROGRAMS>,
    program_bits: u16,
}

// Note: do not derive debug on this, as it contains a f32.
// This would lead to huge memory occupation for the f32 formatting,
// due to a bug in the standard library.
#[derive(Clone, Copy)]
pub struct ProgramEntry {
    pub program_id: u8,
    pub from_time: TimeStructure,
    pub to_time: TimeStructure,
    pub temperature_setpoint: f32,
    pub output: OutputType,
    pub repeat: RepeatType,
}

impl StateManager {
    pub fn new(prev_state: Option<&TransferableState>) -> Self {
        let mut programs: Vec<ProgramEntry, MAX_PROGRAMS> = Vec::new();
        let mut program_bits: u16 = 0xFFFF;
        if let Some(state) = prev_state {
            for p_index in 0..state.num_valid as usize {
                let program = state.programs[p_index];
                programs.push(ProgramEntry{
                    program_id: program.program_id,
                    from_time: program.from_time,
                    to_time: program.to_time,
                    temperature_setpoint: program.temperature_setpoint,
                    output: OutputType::try_from(program.output).unwrap_lite(),
                    repeat: RepeatType::try_from(program.repeat).unwrap_lite(),
                }).unwrap_lite();
            }
            program_bits = state.program_bits;
        }
        Self {
            programs: programs,
            program_bits: program_bits,
        }
    }

    pub fn get_programs(&self) -> &Vec<ProgramEntry, MAX_PROGRAMS> {
        &self.programs
    }

    pub fn get_programs_mut(&mut self) -> &mut Vec<ProgramEntry, MAX_PROGRAMS> {
        &mut self.programs
    }

    pub fn get_program_bits_mut(&mut self) -> &mut u16 {
        &mut self.program_bits
    }

    pub fn get_transferable_state(&self) -> TransferableState {
        let mut response_programs: [Program; MAX_PROGRAMS] = [Program::default(); MAX_PROGRAMS];
        for (i, p) in self.programs.iter().enumerate() {
            response_programs[i].program_id = p.program_id;
            response_programs[i].from_time = p.from_time;
            response_programs[i].to_time = p.to_time;
            response_programs[i].temperature_setpoint = p.temperature_setpoint;
            response_programs[i].output = p.output as u8;
            response_programs[i].repeat = p.repeat as u8;
        }
        TransferableState {
            num_valid: self.programs.len() as u16,
            program_bits: self.program_bits,
            programs: response_programs,
        }
    }
}

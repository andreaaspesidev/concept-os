use bthermo_api::{OutputType, Program, RepeatType, TimeStructure, MAX_PROGRAMS, NUM_TEMPERATURES};
use heapless::Vec;
use ringbuffer::{ConstGenericRingBuffer, RingBufferExt};
use userlib::UnwrapLite;
use zerocopy::{AsBytes, FromBytes};

#[repr(C)]
#[derive(FromBytes, AsBytes)]
pub struct TransferableState {
    pub program_bits: u16,
    pub programs: [Program; MAX_PROGRAMS],
    pub num_prog_valid: u16,
    pub temperatures: [f32; NUM_TEMPERATURES],
}

/// Manages application state, in order to enable state transfer.
/// The state during a transfer must be in fact serializable into bytes,
/// and this could limit application choice of state.
///
/// Here the two aspects are decoupled, and a transfer-ready state
/// is generated only when requested.
pub struct StateManager {
    program_state: ProgramState,
    history_state: HistoryState,
}

pub struct ProgramState {
    pub programs: Vec<ProgramEntry, MAX_PROGRAMS>,
    pub program_bits: u16,
}

pub struct HistoryState {
    pub circular_buffer: ConstGenericRingBuffer<f32, NUM_TEMPERATURES>,
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
        let mut temperatures: ConstGenericRingBuffer<f32, NUM_TEMPERATURES> =
            ConstGenericRingBuffer::new();
        if let Some(state) = prev_state {
            // Copy programs from old state
            for p_index in 0..state.num_prog_valid as usize {
                let program = state.programs[p_index];
                programs
                    .push(ProgramEntry {
                        program_id: program.program_id,
                        from_time: program.from_time,
                        to_time: program.to_time,
                        temperature_setpoint: program.temperature_setpoint,
                        output: OutputType::try_from(program.output).unwrap_lite(),
                        repeat: RepeatType::try_from(program.repeat).unwrap_lite(),
                    })
                    .unwrap_lite();
            }
            // Copy temperatures from old state
            temperatures.extend(state.temperatures);
            // Copy program bits from old state
            program_bits = state.program_bits;
        }
        Self {
            program_state: ProgramState {
                programs: programs,
                program_bits: program_bits,
            },
            history_state: HistoryState {
                circular_buffer: temperatures,
            },
        }
    }

    pub fn get_program_state(&self) -> &ProgramState {
        &self.program_state
    }

    pub fn get_program_state_mut(&mut self) -> &mut ProgramState {
        &mut self.program_state
    }

    pub fn get_history_state(&self) -> &HistoryState {
        &self.history_state
    }

    pub fn get_history_state_mut(&mut self) -> &mut HistoryState {
        &mut self.history_state
    }

    pub fn get_transferable_state(&self) -> TransferableState {
        let mut response_programs: [Program; MAX_PROGRAMS] = [Program::default(); MAX_PROGRAMS];
        for (i, p) in self.program_state.programs.iter().enumerate() {
            response_programs[i].program_id = p.program_id;
            response_programs[i].from_time = p.from_time;
            response_programs[i].to_time = p.to_time;
            response_programs[i].temperature_setpoint = p.temperature_setpoint;
            response_programs[i].output = p.output as u8;
            response_programs[i].repeat = p.repeat as u8;
        }
        let mut response_temperatures: [f32; NUM_TEMPERATURES] = [0.0; NUM_TEMPERATURES];
        for i in 0..NUM_TEMPERATURES {
            response_temperatures[i] = self
                .history_state
                .circular_buffer
                .get(i as isize - NUM_TEMPERATURES as isize)
                .map(|e| *e)
                .unwrap_or(0.0);
        }
        
        for i in 0..NUM_TEMPERATURES {
            response_temperatures[i] = self
                .history_state
                .circular_buffer
                .get(i as isize - NUM_TEMPERATURES as isize)
                .map(|e| *e)
                .unwrap_or(0.0);
        }
        TransferableState {
            num_prog_valid: self.program_state.programs.len() as u16,
            program_bits: self.program_state.program_bits,
            programs: response_programs,
            temperatures: response_temperatures,
        }
    }
}

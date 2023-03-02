use bthermo_api::{
    OutputType, Program, RepeatType, TimeStructure, MAX_PROGRAMS,
    NUM_TEMPERATURES,
};
use heapless::Vec;
use ringbuffer::ConstGenericRingBuffer;
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
    pub fn new() -> Self {
        let programs: Vec<ProgramEntry, MAX_PROGRAMS> = Vec::new();
        let program_bits: u16 = 0xFFFF;
        let temperatures: ConstGenericRingBuffer<f32, NUM_TEMPERATURES> =
            ConstGenericRingBuffer::new();
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
}

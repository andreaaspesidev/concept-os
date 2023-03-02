#![no_std]

use core::cell::Cell;
use userlib::{hl, FromPrimitive, TaskId};
use zerocopy::{AsBytes, FromBytes};

/**
 * Constants
 */
pub const MAX_PROGRAMS: usize = 16;
pub const NUM_TEMPERATURES: usize = 16;

/**
 * Enums
 */
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RepeatType {
    NoRepeat = 0,
    EveryDay = 1,
    EveryWeek = 2,
    EveryMonth = 3,
    EveryYear = 4,
}

impl TryFrom<u8> for RepeatType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RepeatType::NoRepeat),
            1 => Ok(RepeatType::EveryDay),
            2 => Ok(RepeatType::EveryWeek),
            3 => Ok(RepeatType::EveryMonth),
            4 => Ok(RepeatType::EveryYear),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OutputType {
    OUT1 = 1,
    OUT2 = 2,
    OUT3 = 3,
    OUT4 = 4,
}
impl Default for OutputType {
    fn default() -> Self {
        Self::OUT1
    }
}

impl TryFrom<u8> for OutputType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::OUT1),
            2 => Ok(Self::OUT2),
            3 => Ok(Self::OUT3),
            4 => Ok(Self::OUT4),
            _ => Err(()),
        }
    }
}

impl From<OutputType> for usize {
    fn from(x: OutputType) -> Self {
        match x {
            OutputType::OUT1 => 1,
            OutputType::OUT2 => 2,
            OutputType::OUT3 => 3,
            OutputType::OUT4 => 4,
        }
    }
}
impl Into<OutputType> for usize {
    fn into(self) -> OutputType {
        match self {
            1 => OutputType::OUT1,
            2 => OutputType::OUT2,
            3 => OutputType::OUT3,
            4 => OutputType::OUT4,
            _ => panic!(""),
        }
    }
}

/**
 * Response Structures
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, PartialOrd, AsBytes, FromBytes)]
pub struct TimeStructure {
    pub sec: u8,
    pub min: u8,
    pub hour: u8,
    pub week_day: u8,
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Default, AsBytes, FromBytes)]
pub struct Program {
    pub program_id: u8,
    pub from_time: TimeStructure,
    pub to_time: TimeStructure,
    pub temperature_setpoint: f32,
    pub output: u8, // As currently the default hubris api do not support serialization of enums
    pub repeat: u8, // As currently the default hubris api do not support serialization of enums
}

/**
 * Error Type
 */
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum ThermoError {
    BadArgument = 1,
    TempNotConnected = 2,
    RTCNotConnected = 3,
    NoMoreSpace = 4,
    MissingProgram = 5,
    ComponentUnavailable = 100,
}
impl From<u32> for ThermoError {
    fn from(x: u32) -> Self {
        match x {
            1 => ThermoError::BadArgument,
            2 => ThermoError::TempNotConnected,
            3 => ThermoError::RTCNotConnected,
            4 => ThermoError::NoMoreSpace,
            5 => ThermoError::MissingProgram,
            _ => ThermoError::ComponentUnavailable,
        }
    }
}
impl From<ThermoError> for u32 {
    fn from(x: ThermoError) -> Self {
        x as u32
    }
}

/**
 * Operations
 */
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Operation {
    ReadRTC = 1,
    SetRTC = 2,
    ReadTemperature = 3,
    GetOutputs = 4,
    GetPrograms = 5,
    SetProgram = 6,
    RemoveProgram = 7,
}

/// ReadRTC
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadRTCRequest {}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadRTCResponse {
    pub time: TimeStructure,
}
impl hl::Call for ReadRTCRequest {
    const OP: u16 = Operation::ReadRTC as u16;
    type Response = ReadRTCResponse;
    type Err = ThermoError;
}

/// SetRTC
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct SetRTCRequest {
    pub time: TimeStructure,
}
impl hl::Call for SetRTCRequest {
    const OP: u16 = Operation::SetRTC as u16;
    type Response = ();
    type Err = ThermoError;
}

/// ReadTemperature
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadTemperatureRequest {}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct ReadTemperatureResponse {
    pub history: [f32; NUM_TEMPERATURES],
    pub operation_value: f32,
}
impl hl::Call for ReadTemperatureRequest {
    const OP: u16 = Operation::ReadTemperature as u16;
    type Response = ReadTemperatureResponse;
    type Err = ThermoError;
}

/// GetOutputs
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct GetOutputStatusRequest {}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct GetOutputStatusResponse {
    pub outputs_active: [u8; 4],
}
impl hl::Call for GetOutputStatusRequest {
    const OP: u16 = Operation::GetOutputs as u16;
    type Response = GetOutputStatusResponse;
    type Err = ThermoError;
}

/// GetPrograms
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct GetProgramsRequest {}
#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct GetProgramsResponse {
    pub num_valid: u16,
    pub programs: [Program; MAX_PROGRAMS],
}
impl hl::Call for GetProgramsRequest {
    const OP: u16 = Operation::GetPrograms as u16;
    type Response = GetProgramsResponse;
    type Err = ThermoError;
}

/// SetProgram
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct SetProgramRequest {
    pub program: Program,
}
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct SetProgramResponse {
    pub program_id: u8,
}
impl hl::Call for SetProgramRequest {
    const OP: u16 = Operation::SetProgram as u16;
    type Response = SetProgramResponse;
    type Err = ThermoError;
}

/// RemoveProgram
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct RemoveProgramRequest {
    pub program_id: u8,
}
impl hl::Call for RemoveProgramRequest {
    const OP: u16 = Operation::RemoveProgram as u16;
    type Response = ();
    type Err = ThermoError;
}

/**
 * Component Interface
 */
pub struct BThermo(Cell<TaskId>);

impl BThermo {
    pub fn new(task_id: TaskId) -> Self {
        Self {
            0: Cell::new(task_id),
        }
    }
    pub fn read_rtc(&mut self) -> Result<ReadRTCResponse, ThermoError> {
        hl::send_with_retry(&self.0, &ReadRTCRequest {}, &[])
    }
    pub fn set_rtc(
        &mut self,
        sec: u8,
        min: u8,
        hour: u8,
        week_day: u8,
        day: u8,
        month: u8,
        year: u16,
    ) -> Result<(), ThermoError> {
        hl::send_with_retry(
            &self.0,
            &SetRTCRequest {
                time: TimeStructure {
                    sec,
                    min,
                    hour,
                    week_day,
                    day,
                    month,
                    year,
                },
            },
            &[],
        )
    }
    pub fn read_temperature(&mut self) -> Result<ReadTemperatureResponse, ThermoError> {
        hl::send_with_retry(&self.0, &ReadTemperatureRequest {}, &[])
    }
    pub fn get_output_status(&mut self) -> Result<GetOutputStatusResponse, ThermoError> {
        hl::send_with_retry(&self.0, &GetOutputStatusRequest {}, &[])
    }
    pub fn get_programs(&mut self) -> Result<GetProgramsResponse, ThermoError> {
        hl::send_with_retry(&self.0, &GetProgramsRequest {}, &[])
    }
    pub fn set_program(
        &mut self,
        from_time: TimeStructure,
        to_time: TimeStructure,
        temperature_setpoint: f32,
        output: OutputType,
        repeat: RepeatType,
    ) -> Result<SetProgramResponse, ThermoError> {
        hl::send_with_retry(
            &self.0,
            &SetProgramRequest {
                program: Program {
                    program_id: 0,
                    from_time: from_time,
                    to_time: to_time,
                    temperature_setpoint: temperature_setpoint,
                    output: output as u8,
                    repeat: repeat as u8,
                },
            },
            &[],
        )
    }
    pub fn remove_program(&mut self, program_id: u8) -> Result<(), ThermoError> {
        hl::send_with_retry(
            &self.0,
            &RemoveProgramRequest {
                program_id: program_id,
            },
            &[],
        )
    }
}

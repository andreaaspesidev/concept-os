//! A driver for the STM32F3/4 Reset and Clock Controller (RCC).
//!
//! This driver puts the system into a reasonable initial state, and then fields
//! requests to alter settings on behalf of other drivers. This prevents us from
//! needing to map the popular registers in the RCC into every driver task.
//! 
//! Adapted from Hubris/stm32fx-rcc

#![no_std]

use userlib::{hl, TaskId, FromPrimitive};
use zerocopy::{AsBytes,FromBytes};

// Import device-specific constants/structures/functions
cfg_if::cfg_if! {
    // STM32F3
    if #[cfg(feature = "stm32f303re")] {
        mod stm32f303re;
        use stm32f303re::*;
    } else {
        compile_error!("Board not supported");
    }
}

/**
 * Constants
 */
const RCC_TASK_ID: TaskId = TaskId(2);

/**
 * Error Type
 */
#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum RCCError {
    InvalidPeripheral = 1,
    BadArgument = 2,
    ComponentUnavailable = 3
}
impl From<u32> for RCCError {
    fn from(x: u32) -> Self {
        match x {
            1 => RCCError::InvalidPeripheral,
            2 => RCCError::BadArgument,
            _ => RCCError::ComponentUnavailable
        }
    }
}
impl From<RCCError> for u32 {
    fn from(x: RCCError) -> Self {
        x as u32
    }
}

/**
 * Operations
 */
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Operation {
    EnableClock = 1,
    DisableClock = 2,
    EnterReset = 3,
    LeaveReset = 4
}

// Not complete, just some needed
#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Peripheral {
    TIM1 = 0,
    TIM2 = 1,
    TIM3 = 2,
    TIM4 = 3,
    TIM6 = 4,
    TIM7 = 5,
    TIM8 = 6,
    WWD = 7,
    SPI1 = 8,
    SPI2 = 9,
    SPI3 = 10,
    SPI4 = 11,
    USART1 = 12,
    USART2 = 13,
    USART3 = 14,
    UART4 = 15,
    UART5 = 16,
    I2C1 = 17,
    I2C2 = 18,
    I2C3 = 19,
    USB = 20,
    CAN = 21,
    DAC1 = 22,
    DAC2 = 23,
    PWR = 24,
    DMA1 = 25,
    DMA2 = 26,
    SRAM = 27,
    CRC = 28,
    GPIOA = 29,
    GPIOB = 30,
    GPIOC = 31,
    GPIOD = 32,
    GPIOE = 33,
    GPIOF = 34,
    GPIOH = 35,
    GPIOG = 36,
}

/// Enable clock
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct EnableClockRequest {
    pub bus: u32,
    pub bit: u32
}
impl hl::Call for EnableClockRequest {
    const OP: u16 = Operation::EnableClock as u16;
    type Response = ();
    type Err = RCCError;
}

/// Disable clock
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct DisableClockRequest {
    pub bus: u32,
    pub bit: u32
}
impl hl::Call for DisableClockRequest {
    const OP: u16 = Operation::DisableClock as u16;
    type Response = ();
    type Err = RCCError;
}

/// Enter reset
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct EnterResetRequest {
    pub bus: u32,
    pub bit: u32
}
impl hl::Call for EnterResetRequest {
    const OP: u16 = Operation::EnterReset as u16;
    type Response = ();
    type Err = RCCError;
}

/// Leave reset
#[derive(FromBytes, AsBytes)]
#[repr(C)]
pub struct LeaveResetRequest {
    pub bus: u32,
    pub bit: u32
}
impl hl::Call for LeaveResetRequest {
    const OP: u16 = Operation::LeaveReset as u16;
    type Response = ();
    type Err = RCCError;
}

// Bus structure
#[derive(FromPrimitive)]
pub enum Bus {
    AHB1 = 0,
    AHB2 = 1,
    AHB3 = 2,
    APB1 = 3,
    APB2 = 4
}

// API Class
pub struct RCC();

impl RCC {
    pub fn new() -> Self {
        Self {}
    }
    pub fn enable_clock(&mut self, peripheral: Peripheral) -> Result<(),RCCError> {
        let (bus, bit) = clock_mapping(peripheral)?;
        hl::send(RCC_TASK_ID, &EnableClockRequest{
            bus: bus as u32,
            bit: bit
        })
    }
    pub fn disable_clock(&mut self, peripheral: Peripheral) -> Result<(),RCCError> {
        let (bus, bit) = clock_mapping(peripheral)?;
        hl::send(RCC_TASK_ID, &DisableClockRequest{
            bus: bus as u32,
            bit: bit
        })
    }
    pub fn enter_reset(&mut self, peripheral: Peripheral) -> Result<(),RCCError> {
        let (bus, bit) = reset_mapping(peripheral)?;
        hl::send(RCC_TASK_ID, &EnterResetRequest{
            bus: bus as u32,
            bit: bit
        })
    }
    pub fn leave_reset(&mut self, peripheral: Peripheral) -> Result<(),RCCError> {
        let (bus, bit) = reset_mapping(peripheral)?;
        hl::send(RCC_TASK_ID, &LeaveResetRequest{
            bus: bus as u32,
            bit: bit
        })
    }
}
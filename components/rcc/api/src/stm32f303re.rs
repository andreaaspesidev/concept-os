use crate::{Peripheral,Bus, RCCError};

// Peripheral mapping
pub fn clock_mapping(p: Peripheral) -> Result<(Bus, u32), RCCError> {
    match p {
        // 9.4.8 APB1 peripheral clock enable register (RCC_APB1ENR)
        Peripheral::TIM2 => Ok((Bus::APB1, 0)),
        Peripheral::TIM3 => Ok((Bus::APB1, 1)),
        Peripheral::TIM4 => Ok((Bus::APB1, 2)),
        Peripheral::TIM6 => Ok((Bus::APB1, 4)),
        Peripheral::TIM7 => Ok((Bus::APB1, 5)),
        Peripheral::WWD => Ok((Bus::APB1, 11)),
        Peripheral::SPI2 => Ok((Bus::APB1, 14)),
        Peripheral::SPI3 => Ok((Bus::APB1, 15)),
        Peripheral::USART2 => Ok((Bus::APB1, 17)),
        Peripheral::USART3 => Ok((Bus::APB1, 18)),
        Peripheral::UART4 => Ok((Bus::APB1, 19)),
        Peripheral::UART5 => Ok((Bus::APB1, 20)),
        Peripheral::I2C1 => Ok((Bus::APB1, 21)),
        Peripheral::I2C2 => Ok((Bus::APB1, 22)),
        Peripheral::USB => Ok((Bus::APB1, 23)),
        Peripheral::CAN => Ok((Bus::APB1, 25)),
        Peripheral::DAC2 => Ok((Bus::APB1, 26)),
        Peripheral::PWR => Ok((Bus::APB1, 28)),
        Peripheral::DAC1 => Ok((Bus::APB1, 29)),
        Peripheral::I2C3 => Ok((Bus::APB1, 30)),
        // 9.4.7 APB2 peripheral clock enable register (RCC_APB2ENR)
        Peripheral::TIM1 => Ok((Bus::APB2, 11)),
        Peripheral::SPI1 => Ok((Bus::APB2, 12)),
        Peripheral::TIM8 => Ok((Bus::APB2, 13)),
        Peripheral::USART1 => Ok((Bus::APB2, 14)),
        Peripheral::SPI4 => Ok((Bus::APB2, 15)),
        // 9.4.6 AHB peripheral clock enable register (RCC_AHBENR)
        Peripheral::DMA1 => Ok((Bus::AHB1, 0)),
        Peripheral::DMA2 => Ok((Bus::AHB1, 1)),
        Peripheral::SRAM => Ok((Bus::AHB1, 2)),
        Peripheral::CRC => Ok((Bus::AHB1, 6)),
        Peripheral::GPIOH => Ok((Bus::AHB1, 16)),
        Peripheral::GPIOA => Ok((Bus::AHB1, 17)),
        Peripheral::GPIOB => Ok((Bus::AHB1, 18)),
        Peripheral::GPIOC => Ok((Bus::AHB1, 19)),
        Peripheral::GPIOD => Ok((Bus::AHB1, 20)),
        Peripheral::GPIOE => Ok((Bus::AHB1, 21)),
        Peripheral::GPIOF => Ok((Bus::AHB1, 22)),
        Peripheral::GPIOG => Ok((Bus::AHB1, 23)),
    }
}

pub fn reset_mapping(p: Peripheral) -> Result<(Bus, u32), RCCError> {
    match p {
        // 9.4.5 APB1 peripheral reset register (RCC_APB1RSTR)
        Peripheral::TIM2 => Ok((Bus::APB1, 0)),
        Peripheral::TIM3 => Ok((Bus::APB1, 1)),
        Peripheral::TIM4 => Ok((Bus::APB1, 2)),
        Peripheral::TIM6 => Ok((Bus::APB1, 4)),
        Peripheral::TIM7 => Ok((Bus::APB1, 5)),
        Peripheral::WWD => Ok((Bus::APB1, 11)),
        Peripheral::SPI2 => Ok((Bus::APB1, 14)),
        Peripheral::SPI3 => Ok((Bus::APB1, 15)),
        Peripheral::USART2 => Ok((Bus::APB1, 17)),
        Peripheral::USART3 => Ok((Bus::APB1, 18)),
        Peripheral::UART4 => Ok((Bus::APB1, 19)),
        Peripheral::UART5 => Ok((Bus::APB1, 20)),
        Peripheral::I2C1 => Ok((Bus::APB1, 21)),
        Peripheral::I2C2 => Ok((Bus::APB1, 22)),
        Peripheral::USB => Ok((Bus::APB1, 23)),
        Peripheral::CAN => Ok((Bus::APB1, 25)),
        Peripheral::DAC2 => Ok((Bus::APB1, 26)),
        Peripheral::PWR => Ok((Bus::APB1, 28)),
        Peripheral::DAC1 => Ok((Bus::APB1, 29)),
        Peripheral::I2C3 => Ok((Bus::APB1, 30)),
        // 9.4.4 APB2 peripheral reset register (RCC_APB2RSTR)
        Peripheral::TIM1 => Ok((Bus::APB2, 11)),
        Peripheral::SPI1 => Ok((Bus::APB2, 12)),
        Peripheral::TIM8 => Ok((Bus::APB2, 13)),
        Peripheral::USART1 => Ok((Bus::APB2, 14)),
        Peripheral::SPI4 => Ok((Bus::APB2, 15)),
        // 9.4.11 AHB peripheral reset register (RCC_AHBRSTR)
        Peripheral::GPIOH => Ok((Bus::AHB1, 16)),
        Peripheral::GPIOA => Ok((Bus::AHB1, 17)),
        Peripheral::GPIOB => Ok((Bus::AHB1, 18)),
        Peripheral::GPIOC => Ok((Bus::AHB1, 19)),
        Peripheral::GPIOD => Ok((Bus::AHB1, 20)),
        Peripheral::GPIOE => Ok((Bus::AHB1, 21)),
        Peripheral::GPIOF => Ok((Bus::AHB1, 22)),
        Peripheral::GPIOG => Ok((Bus::AHB1, 23)),
        _ => Err(RCCError::BadArgument)
    }
}
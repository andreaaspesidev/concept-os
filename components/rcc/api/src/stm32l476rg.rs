use crate::{Peripheral,Bus, RCCError};

// Peripheral mapping
pub fn clock_mapping(p: Peripheral) -> Result<(Bus, u32), RCCError> {
    match p {
        // 6.4.16 AHB1 peripheral clock enable register (RCC_AHB1ENR)
        Peripheral::DMA1 => Ok((Bus::AHB1, 0)),
        Peripheral::DMA2 => Ok((Bus::AHB1, 1)),

        // 6.4.17 AHB2 peripheral clock enable register (RCC_AHB2ENR)
        Peripheral::GPIOA => Ok((Bus::AHB2, 0)),
        Peripheral::GPIOB => Ok((Bus::AHB2, 1)),
        Peripheral::GPIOC => Ok((Bus::AHB2, 2)),
        Peripheral::GPIOD => Ok((Bus::AHB2, 3)),
        Peripheral::GPIOE => Ok((Bus::AHB2, 4)),
        Peripheral::GPIOF => Ok((Bus::AHB2, 5)),
        Peripheral::GPIOG => Ok((Bus::AHB2, 6)),
        Peripheral::GPIOH => Ok((Bus::AHB2, 7)),

        // 6.4.19 APB1 peripheral clock enable register 1 (RCC_APB1ENR1)
        Peripheral::TIM2 => Ok((Bus::APB1, 0)),
        Peripheral::TIM3 => Ok((Bus::APB1, 1)),
        Peripheral::TIM4 => Ok((Bus::APB1, 2)),
        Peripheral::TIM5 => Ok((Bus::APB1, 3)),
        Peripheral::TIM6 => Ok((Bus::APB1, 4)),
        Peripheral::TIM7 => Ok((Bus::APB1, 5)),

        Peripheral::USART2 => Ok((Bus::APB1, 17)),
        Peripheral::USART3 => Ok((Bus::APB1, 18)),
        Peripheral::UART4 => Ok((Bus::APB1, 19)),
        _ => Err(RCCError::BadArgument)
    }
}

pub fn reset_mapping(p: Peripheral) -> Result<(Bus, u32), RCCError> {
    match p {
        // 6.4.10 AHB1 peripheral reset register (RCC_AHB1RSTR)
        Peripheral::DMA1 => Ok((Bus::AHB1, 0)),
        Peripheral::DMA2 => Ok((Bus::AHB1, 1)),

        // 6.4.11 AHB2 peripheral reset register (RCC_AHB2RSTR)
        Peripheral::GPIOA => Ok((Bus::AHB2, 0)),
        Peripheral::GPIOB => Ok((Bus::AHB2, 1)),
        Peripheral::GPIOC => Ok((Bus::AHB2, 2)),
        Peripheral::GPIOD => Ok((Bus::AHB2, 3)),
        Peripheral::GPIOE => Ok((Bus::AHB2, 4)),
        Peripheral::GPIOF => Ok((Bus::AHB2, 5)),
        Peripheral::GPIOG => Ok((Bus::AHB2, 6)),
        Peripheral::GPIOH => Ok((Bus::AHB2, 7)),

        // 6.4.13 APB1 peripheral reset register 1 (RCC_APB1RSTR1)
        Peripheral::TIM2 => Ok((Bus::APB1, 0)),
        Peripheral::TIM3 => Ok((Bus::APB1, 1)),
        Peripheral::TIM4 => Ok((Bus::APB1, 2)),
        Peripheral::TIM5 => Ok((Bus::APB1, 3)),
        Peripheral::TIM6 => Ok((Bus::APB1, 4)),
        Peripheral::TIM7 => Ok((Bus::APB1, 5)),

        Peripheral::USART2 => Ok((Bus::APB1, 17)),
        Peripheral::USART3 => Ok((Bus::APB1, 18)),
        Peripheral::UART4 => Ok((Bus::APB1, 19)),
        _ => Err(RCCError::BadArgument)
    }
}
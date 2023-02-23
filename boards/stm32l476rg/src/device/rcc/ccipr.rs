#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFSDMSEL` reader - DFSDM clock source selection"]
pub type DFSDMSEL_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMSEL` writer - DFSDM clock source selection"]
pub type DFSDMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "Field `SWPMI1SEL` reader - SWPMI1 clock source selection"]
pub type SWPMI1SEL_R = crate::BitReader<bool>;
#[doc = "Field `SWPMI1SEL` writer - SWPMI1 clock source selection"]
pub type SWPMI1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCIPR_SPEC, bool, O>;
#[doc = "ADCs clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSEL_A {
    #[doc = "0: No clock selected"]
    NoClock = 0,
    #[doc = "1: PLLSAI1 clock selected"]
    Pllsai1 = 1,
    #[doc = "2: PLLSAI2 clock selected (only for STM32L47x/L48x/L49x/L4Ax devices)"]
    Pllsai2 = 2,
    #[doc = "3: SYSCLK clock selected"]
    Sysclk = 3,
}
impl From<ADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCSEL` reader - ADCs clock source selection"]
pub type ADCSEL_R = crate::FieldReader<u8, ADCSEL_A>;
impl ADCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSEL_A {
        match self.bits {
            0 => ADCSEL_A::NoClock,
            1 => ADCSEL_A::Pllsai1,
            2 => ADCSEL_A::Pllsai2,
            3 => ADCSEL_A::Sysclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADCSEL_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Pllsai1`"]
    #[inline(always)]
    pub fn is_pllsai1(&self) -> bool {
        *self == ADCSEL_A::Pllsai1
    }
    #[doc = "Checks if the value of the field is `Pllsai2`"]
    #[inline(always)]
    pub fn is_pllsai2(&self) -> bool {
        *self == ADCSEL_A::Pllsai2
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == ADCSEL_A::Sysclk
    }
}
#[doc = "Field `ADCSEL` writer - ADCs clock source selection"]
pub type ADCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, ADCSEL_A, 2, O>;
impl<'a, const O: u8> ADCSEL_W<'a, O> {
    #[doc = "No clock selected"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(ADCSEL_A::NoClock)
    }
    #[doc = "PLLSAI1 clock selected"]
    #[inline(always)]
    pub fn pllsai1(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pllsai1)
    }
    #[doc = "PLLSAI2 clock selected (only for STM32L47x/L48x/L49x/L4Ax devices)"]
    #[inline(always)]
    pub fn pllsai2(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pllsai2)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(ADCSEL_A::Sysclk)
    }
}
#[doc = "48 MHz clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK48SEL_A {
    #[doc = "0: HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)"]
    Hsi48 = 0,
    #[doc = "1: PLLSAI1 clock selected"]
    Pllsai1 = 1,
    #[doc = "2: PLL clock selected"]
    Pll = 2,
    #[doc = "3: MSI clock selected"]
    Msi = 3,
}
impl From<CLK48SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK48SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK48SEL` reader - 48 MHz clock source selection"]
pub type CLK48SEL_R = crate::FieldReader<u8, CLK48SEL_A>;
impl CLK48SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK48SEL_A {
        match self.bits {
            0 => CLK48SEL_A::Hsi48,
            1 => CLK48SEL_A::Pllsai1,
            2 => CLK48SEL_A::Pll,
            3 => CLK48SEL_A::Msi,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Hsi48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == CLK48SEL_A::Hsi48
    }
    #[doc = "Checks if the value of the field is `Pllsai1`"]
    #[inline(always)]
    pub fn is_pllsai1(&self) -> bool {
        *self == CLK48SEL_A::Pllsai1
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CLK48SEL_A::Pll
    }
    #[doc = "Checks if the value of the field is `Msi`"]
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == CLK48SEL_A::Msi
    }
}
#[doc = "Field `CLK48SEL` writer - 48 MHz clock source selection"]
pub type CLK48SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, CLK48SEL_A, 2, O>;
impl<'a, const O: u8> CLK48SEL_W<'a, O> {
    #[doc = "HSI48 clock selected (only for STM32L41x/L42x/L43x/L44x/L45x/L46x/L49x/L4Ax devices, otherwise no clock selected)"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(CLK48SEL_A::Hsi48)
    }
    #[doc = "PLLSAI1 clock selected"]
    #[inline(always)]
    pub fn pllsai1(self) -> &'a mut W {
        self.variant(CLK48SEL_A::Pllsai1)
    }
    #[doc = "PLL clock selected"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CLK48SEL_A::Pll)
    }
    #[doc = "MSI clock selected"]
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(CLK48SEL_A::Msi)
    }
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub type SAI2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub type SAI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub type SAI1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub type SAI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Low power timer 2 clock source selection"]
pub use LPTIM1SEL_A as LPTIM2SEL_A;
#[doc = "Field `LPTIM2SEL` reader - Low power timer 2 clock source selection"]
pub use LPTIM1SEL_R as LPTIM2SEL_R;
#[doc = "Field `LPTIM2SEL` writer - Low power timer 2 clock source selection"]
pub use LPTIM1SEL_W as LPTIM2SEL_W;
#[doc = "Low power timer 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: LSI clock selected"]
    Lsi = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
impl LPTIM1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::Pclk,
            1 => LPTIM1SEL_A::Lsi,
            2 => LPTIM1SEL_A::Hsi16,
            3 => LPTIM1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIM1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, LPTIM1SEL_A, 2, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Pclk)
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
}
#[doc = "I2C3 clock source selection"]
pub use I2C1SEL_A as I2C3SEL_A;
#[doc = "I2C2 clock source selection"]
pub use I2C1SEL_A as I2C2SEL_A;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub use I2C1SEL_R as I2C3SEL_R;
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub use I2C1SEL_R as I2C2SEL_R;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub use I2C1SEL_W as I2C3SEL_W;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub use I2C1SEL_W as I2C2SEL_W;
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub type I2C1SEL_R = crate::FieldReader<u8, I2C1SEL_A>;
impl I2C1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::Pclk),
            1 => Some(I2C1SEL_A::Sysclk),
            2 => Some(I2C1SEL_A::Hsi16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL_A::Hsi16
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, I2C1SEL_A, 2, O>;
impl<'a, const O: u8> I2C1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Hsi16)
    }
}
#[doc = "LPUART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = crate::FieldReader<u8, LPUART1SEL_A>;
impl LPUART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1SEL_A {
        match self.bits {
            0 => LPUART1SEL_A::Pclk,
            1 => LPUART1SEL_A::Sysclk,
            2 => LPUART1SEL_A::Hsi16,
            3 => LPUART1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPUART1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == LPUART1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPUART1SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL_A::Lse
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub type LPUART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, LPUART1SEL_A, 2, O>;
impl<'a, const O: u8> LPUART1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Lse)
    }
}
#[doc = "Field `UART5SEL` reader - UART5 clock source selection"]
pub type UART5SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART5SEL` writer - UART5 clock source selection"]
pub type UART5SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `UART4SEL` reader - UART4 clock source selection"]
pub type UART4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART4SEL` writer - UART4 clock source selection"]
pub type UART4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
#[doc = "USART3 clock source selection"]
pub use USART1SEL_A as USART3SEL_A;
#[doc = "USART2 clock source selection"]
pub use USART1SEL_A as USART2SEL_A;
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub use USART1SEL_R as USART3SEL_R;
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub use USART1SEL_R as USART2SEL_R;
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub use USART1SEL_W as USART3SEL_W;
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub use USART1SEL_W as USART2SEL_W;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART1SEL_A {
    #[doc = "0: PCLK clock selected"]
    Pclk = 0,
    #[doc = "1: SYSCLK clock selected"]
    Sysclk = 1,
    #[doc = "2: HSI16 clock selected"]
    Hsi16 = 2,
    #[doc = "3: LSE clock selected"]
    Lse = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub type USART1SEL_R = crate::FieldReader<u8, USART1SEL_A>;
impl USART1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::Pclk,
            1 => USART1SEL_A::Sysclk,
            2 => USART1SEL_A::Hsi16,
            3 => USART1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SEL_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == USART1SEL_A::Hsi16
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL_A::Lse
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub type USART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, USART1SEL_A, 2, O>;
impl<'a, const O: u8> USART1SEL_W<'a, O> {
    #[doc = "PCLK clock selected"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::Pclk)
    }
    #[doc = "SYSCLK clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::Sysclk)
    }
    #[doc = "HSI16 clock selected"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(USART1SEL_A::Hsi16)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::Lse)
    }
}
impl R {
    #[doc = "Bit 31 - DFSDM clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&self) -> DFSDMSEL_R {
        DFSDMSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - SWPMI1 clock source selection"]
    #[inline(always)]
    pub fn swpmi1sel(&self) -> SWPMI1SEL_R {
        SWPMI1SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - DFSDM clock source selection"]
    #[inline(always)]
    pub fn dfsdmsel(&mut self) -> DFSDMSEL_W<31> {
        DFSDMSEL_W::new(self)
    }
    #[doc = "Bit 30 - SWPMI1 clock source selection"]
    #[inline(always)]
    pub fn swpmi1sel(&mut self) -> SWPMI1SEL_W<30> {
        SWPMI1SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W<28> {
        ADCSEL_W::new(self)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<26> {
        CLK48SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<24> {
        SAI2SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<22> {
        SAI1SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<20> {
        LPTIM2SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<16> {
        I2C3SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<14> {
        I2C2SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<12> {
        I2C1SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<10> {
        LPUART1SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W<8> {
        UART5SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W<6> {
        UART4SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W<4> {
        USART3SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCIPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

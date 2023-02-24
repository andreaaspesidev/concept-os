#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System clock Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: HSI selected as system clock"]
    Hsi = 0,
    #[doc = "1: HSE selected as system clock"]
    Hse = 1,
    #[doc = "2: PLL selected as system clock"]
    Pll = 2,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SW` reader - System clock Switch"]
pub type SW_R = crate::FieldReader<u8, SW_A>;
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SW_A> {
        match self.bits {
            0 => Some(SW_A::Hsi),
            1 => Some(SW_A::Hse),
            2 => Some(SW_A::Pll),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW_A::Pll
    }
}
#[doc = "Field `SW` writer - System clock Switch"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, SW_A, 2, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::Hsi)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::Hse)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::Pll)
    }
}
#[doc = "System Clock Switch Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "0: HSI oscillator used as system clock"]
    Hsi = 0,
    #[doc = "1: HSE oscillator used as system clock"]
    Hse = 1,
    #[doc = "2: PLL used as system clock"]
    Pll = 2,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub type SWS_R = crate::FieldReader<u8, SWS_A>;
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWS_A> {
        match self.bits {
            0 => Some(SWS_A::Hsi),
            1 => Some(SWS_A::Hse),
            2 => Some(SWS_A::Pll),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWS_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWS_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWS_A::Pll
    }
}
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: SYSCLK not divided"]
    Div1 = 0,
    #[doc = "8: SYSCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SYSCLK divided by 256"]
    Div256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    Div512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<u8, HPRE_A>;
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::Div1),
            8 => Some(HPRE_A::Div2),
            9 => Some(HPRE_A::Div4),
            10 => Some(HPRE_A::Div8),
            11 => Some(HPRE_A::Div16),
            12 => Some(HPRE_A::Div64),
            13 => Some(HPRE_A::Div128),
            14 => Some(HPRE_A::Div256),
            15 => Some(HPRE_A::Div512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::Div256
    }
    #[doc = "Checks if the value of the field is `Div512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::Div512
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, HPRE_A, 4, O>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::Div1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::Div2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::Div4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::Div8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::Div16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::Div64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::Div128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::Div256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::Div512)
    }
}
#[doc = "APB Low speed prescaler (APB1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPRE1_A {
    #[doc = "0: HCLK not divided"]
    Div1 = 0,
    #[doc = "4: HCLK divided by 2"]
    Div2 = 4,
    #[doc = "5: HCLK divided by 4"]
    Div4 = 5,
    #[doc = "6: HCLK divided by 8"]
    Div8 = 6,
    #[doc = "7: HCLK divided by 16"]
    Div16 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, PPRE1_A>;
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            0 => Some(PPRE1_A::Div1),
            4 => Some(PPRE1_A::Div2),
            5 => Some(PPRE1_A::Div4),
            6 => Some(PPRE1_A::Div8),
            7 => Some(PPRE1_A::Div16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1_A::Div16
    }
}
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PPRE1_A, 3, O>;
impl<'a, const O: u8> PPRE1_W<'a, O> {
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1_A::Div1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1_A::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1_A::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1_A::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1_A::Div16)
    }
}
#[doc = "APB high speed prescaler (APB2)"]
pub use PPRE1_A as PPRE2_A;
#[doc = "Field `PPRE2` reader - APB high speed prescaler (APB2)"]
pub use PPRE1_R as PPRE2_R;
#[doc = "Field `PPRE2` writer - APB high speed prescaler (APB2)"]
pub use PPRE1_W as PPRE2_W;
#[doc = "PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: HSI divided by 2 selected as PLL input clock"]
    HsiDiv2 = 0,
    #[doc = "1: HSI divided by PREDIV selected as PLL input clock"]
    HsiDivPrediv = 1,
    #[doc = "2: HSE divided by PREDIV selected as PLL input clock"]
    HseDivPrediv = 2,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::FieldReader<u8, PLLSRC_A>;
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLSRC_A> {
        match self.bits {
            0 => Some(PLLSRC_A::HsiDiv2),
            1 => Some(PLLSRC_A::HsiDivPrediv),
            2 => Some(PLLSRC_A::HseDivPrediv),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HsiDiv2`"]
    #[inline(always)]
    pub fn is_hsi_div2(&self) -> bool {
        *self == PLLSRC_A::HsiDiv2
    }
    #[doc = "Checks if the value of the field is `HsiDivPrediv`"]
    #[inline(always)]
    pub fn is_hsi_div_prediv(&self) -> bool {
        *self == PLLSRC_A::HsiDivPrediv
    }
    #[doc = "Checks if the value of the field is `HseDivPrediv`"]
    #[inline(always)]
    pub fn is_hse_div_prediv(&self) -> bool {
        *self == PLLSRC_A::HseDivPrediv
    }
}
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PLLSRC_A, 2, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div2(self) -> &'a mut W {
        self.variant(PLLSRC_A::HsiDiv2)
    }
    #[doc = "HSI divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HsiDivPrediv)
    }
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hse_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HseDivPrediv)
    }
}
#[doc = "HSE divider for PLL entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLXTPRE_A {
    #[doc = "0: HSE clock not divided"]
    Div1 = 0,
    #[doc = "1: HSE clock divided by 2"]
    Div2 = 1,
}
impl From<PLLXTPRE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLXTPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PLLXTPRE_R = crate::BitReader<PLLXTPRE_A>;
impl PLLXTPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLXTPRE_A {
        match self.bits {
            false => PLLXTPRE_A::Div1,
            true => PLLXTPRE_A::Div2,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRE_A::Div2
    }
}
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PLLXTPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PLLXTPRE_A, O>;
impl<'a, const O: u8> PLLXTPRE_W<'a, O> {
    #[doc = "HSE clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::Div1)
    }
    #[doc = "HSE clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::Div2)
    }
}
#[doc = "PLL Multiplication Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLMUL_A {
    #[doc = "0: PLL input clock x2"]
    Mul2 = 0,
    #[doc = "1: PLL input clock x3"]
    Mul3 = 1,
    #[doc = "2: PLL input clock x4"]
    Mul4 = 2,
    #[doc = "3: PLL input clock x5"]
    Mul5 = 3,
    #[doc = "4: PLL input clock x6"]
    Mul6 = 4,
    #[doc = "5: PLL input clock x7"]
    Mul7 = 5,
    #[doc = "6: PLL input clock x8"]
    Mul8 = 6,
    #[doc = "7: PLL input clock x9"]
    Mul9 = 7,
    #[doc = "8: PLL input clock x10"]
    Mul10 = 8,
    #[doc = "9: PLL input clock x11"]
    Mul11 = 9,
    #[doc = "10: PLL input clock x12"]
    Mul12 = 10,
    #[doc = "11: PLL input clock x13"]
    Mul13 = 11,
    #[doc = "12: PLL input clock x14"]
    Mul14 = 12,
    #[doc = "13: PLL input clock x15"]
    Mul15 = 13,
    #[doc = "14: PLL input clock x16"]
    Mul16 = 14,
    #[doc = "15: PLL input clock x16"]
    Mul16x = 15,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub type PLLMUL_R = crate::FieldReader<u8, PLLMUL_A>;
impl PLLMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLMUL_A {
        match self.bits {
            0 => PLLMUL_A::Mul2,
            1 => PLLMUL_A::Mul3,
            2 => PLLMUL_A::Mul4,
            3 => PLLMUL_A::Mul5,
            4 => PLLMUL_A::Mul6,
            5 => PLLMUL_A::Mul7,
            6 => PLLMUL_A::Mul8,
            7 => PLLMUL_A::Mul9,
            8 => PLLMUL_A::Mul10,
            9 => PLLMUL_A::Mul11,
            10 => PLLMUL_A::Mul12,
            11 => PLLMUL_A::Mul13,
            12 => PLLMUL_A::Mul14,
            13 => PLLMUL_A::Mul15,
            14 => PLLMUL_A::Mul16,
            15 => PLLMUL_A::Mul16x,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Mul2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMUL_A::Mul2
    }
    #[doc = "Checks if the value of the field is `Mul3`"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL_A::Mul3
    }
    #[doc = "Checks if the value of the field is `Mul4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL_A::Mul4
    }
    #[doc = "Checks if the value of the field is `Mul5`"]
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMUL_A::Mul5
    }
    #[doc = "Checks if the value of the field is `Mul6`"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL_A::Mul6
    }
    #[doc = "Checks if the value of the field is `Mul7`"]
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMUL_A::Mul7
    }
    #[doc = "Checks if the value of the field is `Mul8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL_A::Mul8
    }
    #[doc = "Checks if the value of the field is `Mul9`"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMUL_A::Mul9
    }
    #[doc = "Checks if the value of the field is `Mul10`"]
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMUL_A::Mul10
    }
    #[doc = "Checks if the value of the field is `Mul11`"]
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMUL_A::Mul11
    }
    #[doc = "Checks if the value of the field is `Mul12`"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL_A::Mul12
    }
    #[doc = "Checks if the value of the field is `Mul13`"]
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMUL_A::Mul13
    }
    #[doc = "Checks if the value of the field is `Mul14`"]
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMUL_A::Mul14
    }
    #[doc = "Checks if the value of the field is `Mul15`"]
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMUL_A::Mul15
    }
    #[doc = "Checks if the value of the field is `Mul16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL_A::Mul16
    }
    #[doc = "Checks if the value of the field is `Mul16x`"]
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        *self == PLLMUL_A::Mul16x
    }
}
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, PLLMUL_A, 4, O>;
impl<'a, const O: u8> PLLMUL_W<'a, O> {
    #[doc = "PLL input clock x2"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul2)
    }
    #[doc = "PLL input clock x3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul3)
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul4)
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul5)
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul6)
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul7)
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul8)
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul9)
    }
    #[doc = "PLL input clock x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul10)
    }
    #[doc = "PLL input clock x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul11)
    }
    #[doc = "PLL input clock x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul12)
    }
    #[doc = "PLL input clock x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul13)
    }
    #[doc = "PLL input clock x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul14)
    }
    #[doc = "PLL input clock x15"]
    #[inline(always)]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul15)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul16)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul16x)
    }
}
#[doc = "USB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPRE_A {
    #[doc = "0: PLL clock is divided by 1.5"]
    Div15 = 0,
    #[doc = "1: PLL clock is not divided"]
    Div1 = 1,
}
impl From<USBPRE_A> for bool {
    #[inline(always)]
    fn from(variant: USBPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBPRE` reader - USB prescaler"]
pub type USBPRE_R = crate::BitReader<USBPRE_A>;
impl USBPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPRE_A {
        match self.bits {
            false => USBPRE_A::Div15,
            true => USBPRE_A::Div1,
        }
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == USBPRE_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == USBPRE_A::Div1
    }
}
#[doc = "Field `USBPRE` writer - USB prescaler"]
pub type USBPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, USBPRE_A, O>;
impl<'a, const O: u8> USBPRE_W<'a, O> {
    #[doc = "PLL clock is divided by 1.5"]
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut W {
        self.variant(USBPRE_A::Div15)
    }
    #[doc = "PLL clock is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(USBPRE_A::Div1)
    }
}
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO_A {
    #[doc = "0: MCO output disabled, no clock on MCO"]
    NoMco = 0,
    #[doc = "2: Internal low speed (LSI) oscillator clock selected"]
    Lsi = 2,
    #[doc = "3: External low speed (LSE) oscillator clock selected"]
    Lse = 3,
    #[doc = "4: System clock selected"]
    Sysclk = 4,
    #[doc = "5: Internal RC 8 MHz (HSI) oscillator clock selected"]
    Hsi = 5,
    #[doc = "6: External 4-32 MHz (HSE) oscillator clock selected"]
    Hse = 6,
    #[doc = "7: PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    Pll = 7,
}
impl From<MCO_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type MCO_R = crate::FieldReader<u8, MCO_A>;
impl MCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO_A> {
        match self.bits {
            0 => Some(MCO_A::NoMco),
            2 => Some(MCO_A::Lsi),
            3 => Some(MCO_A::Lse),
            4 => Some(MCO_A::Sysclk),
            5 => Some(MCO_A::Hsi),
            6 => Some(MCO_A::Hse),
            7 => Some(MCO_A::Pll),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoMco`"]
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        *self == MCO_A::NoMco
    }
    #[doc = "Checks if the value of the field is `Lsi`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO_A::Lsi
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO_A::Lse
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO_A::Hse
    }
    #[doc = "Checks if the value of the field is `Pll`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO_A::Pll
    }
}
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type MCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO_A, 3, O>;
impl<'a, const O: u8> MCO_W<'a, O> {
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn no_mco(self) -> &'a mut W {
        self.variant(MCO_A::NoMco)
    }
    #[doc = "Internal low speed (LSI) oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO_A::Lsi)
    }
    #[doc = "External low speed (LSE) oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO_A::Lse)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO_A::Sysclk)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO_A::Hsi)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO_A::Hse)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO_A::Pll)
    }
}
#[doc = "I2S external clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2SSRC_A {
    #[doc = "0: System clock used as I2S clock source"]
    Sysclk = 0,
    #[doc = "1: External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    Ckin = 1,
}
impl From<I2SSRC_A> for bool {
    #[inline(always)]
    fn from(variant: I2SSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2SSRC` reader - I2S external clock source selection"]
pub type I2SSRC_R = crate::BitReader<I2SSRC_A>;
impl I2SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSRC_A {
        match self.bits {
            false => I2SSRC_A::Sysclk,
            true => I2SSRC_A::Ckin,
        }
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2SSRC_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Ckin`"]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == I2SSRC_A::Ckin
    }
}
#[doc = "Field `I2SSRC` writer - I2S external clock source selection"]
pub type I2SSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, I2SSRC_A, O>;
impl<'a, const O: u8> I2SSRC_W<'a, O> {
    #[doc = "System clock used as I2S clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2SSRC_A::Sysclk)
    }
    #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut W {
        self.variant(I2SSRC_A::Ckin)
    }
}
#[doc = "Microcontroller Clock Output Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCOPRE_A {
    #[doc = "0: MCO is divided by 1"]
    Div1 = 0,
    #[doc = "1: MCO is divided by 2"]
    Div2 = 1,
    #[doc = "2: MCO is divided by 4"]
    Div4 = 2,
    #[doc = "3: MCO is divided by 8"]
    Div8 = 3,
    #[doc = "4: MCO is divided by 16"]
    Div16 = 4,
    #[doc = "5: MCO is divided by 32"]
    Div32 = 5,
    #[doc = "6: MCO is divided by 64"]
    Div64 = 6,
    #[doc = "7: MCO is divided by 128"]
    Div128 = 7,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCOPRE` reader - Microcontroller Clock Output Prescaler"]
pub type MCOPRE_R = crate::FieldReader<u8, MCOPRE_A>;
impl MCOPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCOPRE_A {
        match self.bits {
            0 => MCOPRE_A::Div1,
            1 => MCOPRE_A::Div2,
            2 => MCOPRE_A::Div4,
            3 => MCOPRE_A::Div8,
            4 => MCOPRE_A::Div16,
            5 => MCOPRE_A::Div32,
            6 => MCOPRE_A::Div64,
            7 => MCOPRE_A::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MCOPRE_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MCOPRE_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MCOPRE_A::Div128
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller Clock Output Prescaler"]
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, MCOPRE_A, 3, O>;
impl<'a, const O: u8> MCOPRE_W<'a, O> {
    #[doc = "MCO is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div1)
    }
    #[doc = "MCO is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div2)
    }
    #[doc = "MCO is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div4)
    }
    #[doc = "MCO is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div8)
    }
    #[doc = "MCO is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div16)
    }
    #[doc = "MCO is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div32)
    }
    #[doc = "MCO is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div64)
    }
    #[doc = "MCO is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div128)
    }
}
#[doc = "Do not divide PLL to MCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLNODIV_A {
    #[doc = "0: PLL is divided by 2 for MCO"]
    Div2 = 0,
    #[doc = "1: PLL is not divided for MCO"]
    Div1 = 1,
}
impl From<PLLNODIV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLNODIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLNODIV` reader - Do not divide PLL to MCO"]
pub type PLLNODIV_R = crate::BitReader<PLLNODIV_A>;
impl PLLNODIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLNODIV_A {
        match self.bits {
            false => PLLNODIV_A::Div2,
            true => PLLNODIV_A::Div1,
        }
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLNODIV_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLNODIV_A::Div1
    }
}
#[doc = "Field `PLLNODIV` writer - Do not divide PLL to MCO"]
pub type PLLNODIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PLLNODIV_A, O>;
impl<'a, const O: u8> PLLNODIV_W<'a, O> {
    #[doc = "PLL is divided by 2 for MCO"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLNODIV_A::Div2)
    }
    #[doc = "PLL is not divided for MCO"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLNODIV_A::Div1)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 15:16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - USB prescaler"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 23 - I2S external clock source selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Do not divide PLL to MCO"]
    #[inline(always)]
    pub fn pllnodiv(&self) -> PLLNODIV_R {
        PLLNODIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - APB high speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 15:16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<15> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<17> {
        PLLXTPRE_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W<18> {
        PLLMUL_W::new(self)
    }
    #[doc = "Bit 22 - USB prescaler"]
    #[inline(always)]
    pub fn usbpre(&mut self) -> USBPRE_W<22> {
        USBPRE_W::new(self)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&mut self) -> MCO_W<24> {
        MCO_W::new(self)
    }
    #[doc = "Bit 23 - I2S external clock source selection"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<23> {
        I2SSRC_W::new(self)
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    #[doc = "Bit 31 - Do not divide PLL to MCO"]
    #[inline(always)]
    pub fn pllnodiv(&mut self) -> PLLNODIV_W<31> {
        PLLNODIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register (RCC_CFGR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

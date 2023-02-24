#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC channel1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN1_A {
    #[doc = "0: DAC Channel X disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X enabled"]
    Enabled = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - DAC channel1 enable"]
pub type EN1_R = crate::BitReader<EN1_A>;
impl EN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::Disabled,
            true => EN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1_A::Enabled
    }
}
#[doc = "Field `EN1` writer - DAC channel1 enable"]
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN1_A, O>;
impl<'a, const O: u8> EN1_W<'a, O> {
    #[doc = "DAC Channel X disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::Disabled)
    }
    #[doc = "DAC Channel X enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::Enabled)
    }
}
#[doc = "DAC channel1 trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN1_A {
    #[doc = "0: DAC Channel X trigger disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X trigger enabled"]
    Enabled = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN1` reader - DAC channel1 trigger enable"]
pub type TEN1_R = crate::BitReader<TEN1_A>;
impl TEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::Disabled,
            true => TEN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1_A::Enabled
    }
}
#[doc = "Field `TEN1` writer - DAC channel1 trigger enable"]
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEN1_A, O>;
impl<'a, const O: u8> TEN1_W<'a, O> {
    #[doc = "DAC Channel X trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::Disabled)
    }
    #[doc = "DAC Channel X trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::Enabled)
    }
}
#[doc = "DAC channel1 trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: TIM6_TRGO event trigger for DAC conversion, if TEN is enabled"]
    Tim6Trgo = 0,
    #[doc = "1: TIM8_TRGO"]
    Tim8Trgo = 1,
    #[doc = "2: TIM7_TRGO (Note: Reserved on STM32L45xxx and STM32L46xxx devices)"]
    Tim7Trgo = 2,
    #[doc = "3: TIM5_TRGO"]
    Tim5Trgo = 3,
    #[doc = "4: TIM2_TRGO"]
    Tim2Trgo = 4,
    #[doc = "5: TIM4_TRGO"]
    Tim4Trgo = 5,
    #[doc = "6: External pin"]
    Exti9 = 6,
    #[doc = "7: Software triger"]
    Swtrig = 7,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSEL1` reader - DAC channel1 trigger selection"]
pub type TSEL1_R = crate::FieldReader<u8, TSEL1_A>;
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEL1_A {
        match self.bits {
            0 => TSEL1_A::Tim6Trgo,
            1 => TSEL1_A::Tim8Trgo,
            2 => TSEL1_A::Tim7Trgo,
            3 => TSEL1_A::Tim5Trgo,
            4 => TSEL1_A::Tim2Trgo,
            5 => TSEL1_A::Tim4Trgo,
            6 => TSEL1_A::Exti9,
            7 => TSEL1_A::Swtrig,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Tim6Trgo`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1_A::Tim6Trgo
    }
    #[doc = "Checks if the value of the field is `Tim8Trgo`"]
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == TSEL1_A::Tim8Trgo
    }
    #[doc = "Checks if the value of the field is `Tim7Trgo`"]
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1_A::Tim7Trgo
    }
    #[doc = "Checks if the value of the field is `Tim5Trgo`"]
    #[inline(always)]
    pub fn is_tim5_trgo(&self) -> bool {
        *self == TSEL1_A::Tim5Trgo
    }
    #[doc = "Checks if the value of the field is `Tim2Trgo`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1_A::Tim2Trgo
    }
    #[doc = "Checks if the value of the field is `Tim4Trgo`"]
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == TSEL1_A::Tim4Trgo
    }
    #[doc = "Checks if the value of the field is `Exti9`"]
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1_A::Exti9
    }
    #[doc = "Checks if the value of the field is `Swtrig`"]
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == TSEL1_A::Swtrig
    }
}
#[doc = "Field `TSEL1` writer - DAC channel1 trigger selection"]
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, TSEL1_A, 3, O>;
impl<'a, const O: u8> TSEL1_W<'a, O> {
    #[doc = "TIM6_TRGO event trigger for DAC conversion, if TEN is enabled"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim6Trgo)
    }
    #[doc = "TIM8_TRGO"]
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim8Trgo)
    }
    #[doc = "TIM7_TRGO (Note: Reserved on STM32L45xxx and STM32L46xxx devices)"]
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim7Trgo)
    }
    #[doc = "TIM5_TRGO"]
    #[inline(always)]
    pub fn tim5_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim5Trgo)
    }
    #[doc = "TIM2_TRGO"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim2Trgo)
    }
    #[doc = "TIM4_TRGO"]
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim4Trgo)
    }
    #[doc = "External pin"]
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::Exti9)
    }
    #[doc = "Software triger"]
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut W {
        self.variant(TSEL1_A::Swtrig)
    }
}
#[doc = "DAC channel1 noise/triangle wave generation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVE1_A {
    #[doc = "0: Wave generation disabled"]
    Disabled = 0,
    #[doc = "1: Noise wave generation enabled"]
    Noise = 1,
    #[doc = "2: Triangle wave generation enabled"]
    Triangle = 2,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_R = crate::FieldReader<u8, WAVE1_A>;
impl WAVE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE1_A> {
        match self.bits {
            0 => Some(WAVE1_A::Disabled),
            1 => Some(WAVE1_A::Noise),
            2 => Some(WAVE1_A::Triangle),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Noise`"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1_A::Noise
    }
    #[doc = "Checks if the value of the field is `Triangle`"]
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1_A::Triangle
    }
}
#[doc = "Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable"]
pub type WAVE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, WAVE1_A, 2, O>;
impl<'a, const O: u8> WAVE1_W<'a, O> {
    #[doc = "Wave generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE1_A::Disabled)
    }
    #[doc = "Noise wave generation enabled"]
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE1_A::Noise)
    }
    #[doc = "Triangle wave generation enabled"]
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE1_A::Triangle)
    }
}
#[doc = "DAC channel1 mask/amplitude selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MAMP1_A {
    #[doc = "0: Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    Amp1 = 0,
    #[doc = "1: Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    Amp3 = 1,
    #[doc = "2: Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    Amp7 = 2,
    #[doc = "3: Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    Amp15 = 3,
    #[doc = "4: Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    Amp31 = 4,
    #[doc = "5: Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    Amp63 = 5,
    #[doc = "6: Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    Amp127 = 6,
    #[doc = "7: Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    Amp255 = 7,
    #[doc = "8: Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    Amp511 = 8,
    #[doc = "9: Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    Amp1023 = 9,
    #[doc = "10: Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    Amp2047 = 10,
    #[doc = "11: Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    Amp4095 = 11,
}
impl From<MAMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: MAMP1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MAMP1` reader - DAC channel1 mask/amplitude selector"]
pub type MAMP1_R = crate::FieldReader<u8, MAMP1_A>;
impl MAMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAMP1_A> {
        match self.bits {
            0 => Some(MAMP1_A::Amp1),
            1 => Some(MAMP1_A::Amp3),
            2 => Some(MAMP1_A::Amp7),
            3 => Some(MAMP1_A::Amp15),
            4 => Some(MAMP1_A::Amp31),
            5 => Some(MAMP1_A::Amp63),
            6 => Some(MAMP1_A::Amp127),
            7 => Some(MAMP1_A::Amp255),
            8 => Some(MAMP1_A::Amp511),
            9 => Some(MAMP1_A::Amp1023),
            10 => Some(MAMP1_A::Amp2047),
            11 => Some(MAMP1_A::Amp4095),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Amp1`"]
    #[inline(always)]
    pub fn is_amp1(&self) -> bool {
        *self == MAMP1_A::Amp1
    }
    #[doc = "Checks if the value of the field is `Amp3`"]
    #[inline(always)]
    pub fn is_amp3(&self) -> bool {
        *self == MAMP1_A::Amp3
    }
    #[doc = "Checks if the value of the field is `Amp7`"]
    #[inline(always)]
    pub fn is_amp7(&self) -> bool {
        *self == MAMP1_A::Amp7
    }
    #[doc = "Checks if the value of the field is `Amp15`"]
    #[inline(always)]
    pub fn is_amp15(&self) -> bool {
        *self == MAMP1_A::Amp15
    }
    #[doc = "Checks if the value of the field is `Amp31`"]
    #[inline(always)]
    pub fn is_amp31(&self) -> bool {
        *self == MAMP1_A::Amp31
    }
    #[doc = "Checks if the value of the field is `Amp63`"]
    #[inline(always)]
    pub fn is_amp63(&self) -> bool {
        *self == MAMP1_A::Amp63
    }
    #[doc = "Checks if the value of the field is `Amp127`"]
    #[inline(always)]
    pub fn is_amp127(&self) -> bool {
        *self == MAMP1_A::Amp127
    }
    #[doc = "Checks if the value of the field is `Amp255`"]
    #[inline(always)]
    pub fn is_amp255(&self) -> bool {
        *self == MAMP1_A::Amp255
    }
    #[doc = "Checks if the value of the field is `Amp511`"]
    #[inline(always)]
    pub fn is_amp511(&self) -> bool {
        *self == MAMP1_A::Amp511
    }
    #[doc = "Checks if the value of the field is `Amp1023`"]
    #[inline(always)]
    pub fn is_amp1023(&self) -> bool {
        *self == MAMP1_A::Amp1023
    }
    #[doc = "Checks if the value of the field is `Amp2047`"]
    #[inline(always)]
    pub fn is_amp2047(&self) -> bool {
        *self == MAMP1_A::Amp2047
    }
    #[doc = "Checks if the value of the field is `Amp4095`"]
    #[inline(always)]
    pub fn is_amp4095(&self) -> bool {
        *self == MAMP1_A::Amp4095
    }
}
#[doc = "Field `MAMP1` writer - DAC channel1 mask/amplitude selector"]
pub type MAMP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, MAMP1_A, 4, O>;
impl<'a, const O: u8> MAMP1_W<'a, O> {
    #[doc = "Unmask bit0 of LFSR/ triangle amplitude equal to 1"]
    #[inline(always)]
    pub fn amp1(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp1)
    }
    #[doc = "Unmask bits\\[1:0\\]
of LFSR/ triangle amplitude equal to 3"]
    #[inline(always)]
    pub fn amp3(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp3)
    }
    #[doc = "Unmask bits\\[2:0\\]
of LFSR/ triangle amplitude equal to 7"]
    #[inline(always)]
    pub fn amp7(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp7)
    }
    #[doc = "Unmask bits\\[3:0\\]
of LFSR/ triangle amplitude equal to 15"]
    #[inline(always)]
    pub fn amp15(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp15)
    }
    #[doc = "Unmask bits\\[4:0\\]
of LFSR/ triangle amplitude equal to 31"]
    #[inline(always)]
    pub fn amp31(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp31)
    }
    #[doc = "Unmask bits\\[5:0\\]
of LFSR/ triangle amplitude equal 63"]
    #[inline(always)]
    pub fn amp63(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp63)
    }
    #[doc = "Unmask bits\\[6:0\\]
of LFSR/ triangle amplitude equal to 127"]
    #[inline(always)]
    pub fn amp127(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp127)
    }
    #[doc = "Unmask bits\\[7:0\\]
of LFSR/ triangle amplitude equal to 255"]
    #[inline(always)]
    pub fn amp255(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp255)
    }
    #[doc = "Unmask bits\\[8:0\\]
of LFSR/ triangle amplitude equal to 511"]
    #[inline(always)]
    pub fn amp511(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp511)
    }
    #[doc = "Unmask bits\\[9:0\\]
of LFSR/ triangle amplitude equal to 1023"]
    #[inline(always)]
    pub fn amp1023(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp1023)
    }
    #[doc = "Unmask bits\\[10:0\\]
of LFSR/ triangle amplitude equal to 2047"]
    #[inline(always)]
    pub fn amp2047(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp2047)
    }
    #[doc = "Unmask bits\\[11:0\\]
of LFSR/ triangle amplitude equal to 4095"]
    #[inline(always)]
    pub fn amp4095(self) -> &'a mut W {
        self.variant(MAMP1_A::Amp4095)
    }
}
#[doc = "DAC channel1 DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN1_A {
    #[doc = "0: DAC Channel X DMA mode disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA mode enabled"]
    Enabled = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN1` reader - DAC channel1 DMA enable"]
pub type DMAEN1_R = crate::BitReader<DMAEN1_A>;
impl DMAEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::Disabled,
            true => DMAEN1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1_A::Enabled
    }
}
#[doc = "Field `DMAEN1` writer - DAC channel1 DMA enable"]
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAEN1_A, O>;
impl<'a, const O: u8> DMAEN1_W<'a, O> {
    #[doc = "DAC Channel X DMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Disabled)
    }
    #[doc = "DAC Channel X DMA mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Enabled)
    }
}
#[doc = "DAC channel1 DMA Underrun Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDRIE1_A {
    #[doc = "0: DAC Channel X DMA Underrun Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DAC Channel X DMA Underrun Interrupt enabled"]
    Enabled = 1,
}
impl From<DMAUDRIE1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDRIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDRIE1` reader - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_R = crate::BitReader<DMAUDRIE1_A>;
impl DMAUDRIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDRIE1_A {
        match self.bits {
            false => DMAUDRIE1_A::Disabled,
            true => DMAUDRIE1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAUDRIE1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAUDRIE1_A::Enabled
    }
}
#[doc = "Field `DMAUDRIE1` writer - DAC channel1 DMA Underrun Interrupt enable"]
pub type DMAUDRIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAUDRIE1_A, O>;
impl<'a, const O: u8> DMAUDRIE1_W<'a, O> {
    #[doc = "DAC Channel X DMA Underrun Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::Disabled)
    }
    #[doc = "DAC Channel X DMA Underrun Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAUDRIE1_A::Enabled)
    }
}
#[doc = "DAC Channel 1 calibration enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN1_A {
    #[doc = "0: DAC Channel X Normal operating mode"]
    Normal = 0,
    #[doc = "1: DAC Channel X calibration mode"]
    Calibration = 1,
}
impl From<CEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN1` reader - DAC Channel 1 calibration enable"]
pub type CEN1_R = crate::BitReader<CEN1_A>;
impl CEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN1_A {
        match self.bits {
            false => CEN1_A::Normal,
            true => CEN1_A::Calibration,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CEN1_A::Normal
    }
    #[doc = "Checks if the value of the field is `Calibration`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == CEN1_A::Calibration
    }
}
#[doc = "Field `CEN1` writer - DAC Channel 1 calibration enable"]
pub type CEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CEN1_A, O>;
impl<'a, const O: u8> CEN1_W<'a, O> {
    #[doc = "DAC Channel X Normal operating mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CEN1_A::Normal)
    }
    #[doc = "DAC Channel X calibration mode"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(CEN1_A::Calibration)
    }
}
#[doc = "DAC Channel 2 calibration enable"]
pub use CEN1_A as CEN2_A;
#[doc = "Field `CEN2` reader - DAC Channel 2 calibration enable"]
pub use CEN1_R as CEN2_R;
#[doc = "Field `CEN2` writer - DAC Channel 2 calibration enable"]
pub use CEN1_W as CEN2_W;
#[doc = "DAC channel2 DMA enable"]
pub use DMAEN1_A as DMAEN2_A;
#[doc = "Field `DMAEN2` reader - DAC channel2 DMA enable"]
pub use DMAEN1_R as DMAEN2_R;
#[doc = "Field `DMAEN2` writer - DAC channel2 DMA enable"]
pub use DMAEN1_W as DMAEN2_W;
#[doc = "DAC channel2 DMA underrun interrupt enable"]
pub use DMAUDRIE1_A as DMAUDRIE2_A;
#[doc = "Field `DMAUDRIE2` reader - DAC channel2 DMA underrun interrupt enable"]
pub use DMAUDRIE1_R as DMAUDRIE2_R;
#[doc = "Field `DMAUDRIE2` writer - DAC channel2 DMA underrun interrupt enable"]
pub use DMAUDRIE1_W as DMAUDRIE2_W;
#[doc = "DAC channel2 enable"]
pub use EN1_A as EN2_A;
#[doc = "Field `EN2` reader - DAC channel2 enable"]
pub use EN1_R as EN2_R;
#[doc = "Field `EN2` writer - DAC channel2 enable"]
pub use EN1_W as EN2_W;
#[doc = "DAC channel2 mask/amplitude selector"]
pub use MAMP1_A as MAMP2_A;
#[doc = "Field `MAMP2` reader - DAC channel2 mask/amplitude selector"]
pub use MAMP1_R as MAMP2_R;
#[doc = "Field `MAMP2` writer - DAC channel2 mask/amplitude selector"]
pub use MAMP1_W as MAMP2_W;
#[doc = "DAC channel2 trigger enable"]
pub use TEN1_A as TEN2_A;
#[doc = "Field `TEN2` reader - DAC channel2 trigger enable"]
pub use TEN1_R as TEN2_R;
#[doc = "Field `TEN2` writer - DAC channel2 trigger enable"]
pub use TEN1_W as TEN2_W;
#[doc = "DAC channel2 trigger selection"]
pub use TSEL1_A as TSEL2_A;
#[doc = "Field `TSEL2` reader - DAC channel2 trigger selection"]
pub use TSEL1_R as TSEL2_R;
#[doc = "Field `TSEL2` writer - DAC channel2 trigger selection"]
pub use TSEL1_W as TSEL2_W;
#[doc = "DAC channel2 noise/triangle wave generation enable"]
pub use WAVE1_A as WAVE2_A;
#[doc = "Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable"]
pub use WAVE1_R as WAVE2_R;
#[doc = "Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable"]
pub use WAVE1_W as WAVE2_W;
impl R {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&self) -> DMAUDRIE1_R {
        DMAUDRIE1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&self) -> CEN1_R {
        CEN1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie2(&self) -> DMAUDRIE2_R {
        DMAUDRIE2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable"]
    #[inline(always)]
    pub fn cen2(&self) -> CEN2_R {
        CEN2_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 enable"]
    #[inline(always)]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    #[doc = "Bit 2 - DAC channel1 trigger enable"]
    #[inline(always)]
    pub fn ten1(&mut self) -> TEN1_W<2> {
        TEN1_W::new(self)
    }
    #[doc = "Bits 3:5 - DAC channel1 trigger selection"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W<3> {
        TSEL1_W::new(self)
    }
    #[doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave1(&mut self) -> WAVE1_W<6> {
        WAVE1_W::new(self)
    }
    #[doc = "Bits 8:11 - DAC channel1 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp1(&mut self) -> MAMP1_W<8> {
        MAMP1_W::new(self)
    }
    #[doc = "Bit 12 - DAC channel1 DMA enable"]
    #[inline(always)]
    pub fn dmaen1(&mut self) -> DMAEN1_W<12> {
        DMAEN1_W::new(self)
    }
    #[doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie1(&mut self) -> DMAUDRIE1_W<13> {
        DMAUDRIE1_W::new(self)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration enable"]
    #[inline(always)]
    pub fn cen1(&mut self) -> CEN1_W<14> {
        CEN1_W::new(self)
    }
    #[doc = "Bit 16 - DAC channel2 enable"]
    #[inline(always)]
    pub fn en2(&mut self) -> EN2_W<16> {
        EN2_W::new(self)
    }
    #[doc = "Bit 18 - DAC channel2 trigger enable"]
    #[inline(always)]
    pub fn ten2(&mut self) -> TEN2_W<18> {
        TEN2_W::new(self)
    }
    #[doc = "Bits 19:21 - DAC channel2 trigger selection"]
    #[inline(always)]
    pub fn tsel2(&mut self) -> TSEL2_W<19> {
        TSEL2_W::new(self)
    }
    #[doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn wave2(&mut self) -> WAVE2_W<22> {
        WAVE2_W::new(self)
    }
    #[doc = "Bits 24:27 - DAC channel2 mask/amplitude selector"]
    #[inline(always)]
    pub fn mamp2(&mut self) -> MAMP2_W<24> {
        MAMP2_W::new(self)
    }
    #[doc = "Bit 28 - DAC channel2 DMA enable"]
    #[inline(always)]
    pub fn dmaen2(&mut self) -> DMAEN2_W<28> {
        DMAEN2_W::new(self)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn dmaudrie2(&mut self) -> DMAUDRIE2_W<29> {
        DMAUDRIE2_W::new(self)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration enable"]
    #[inline(always)]
    pub fn cen2(&mut self) -> CEN2_W<30> {
        CEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

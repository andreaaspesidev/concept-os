#[doc = "Register `COMP4_CSR` reader"]
pub struct R(crate::R<COMP4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP4_CSR` writer"]
pub struct W(crate::W<COMP4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP4_CSR_SPEC>;
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
impl From<crate::W<COMP4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator 4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4EN_A {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<COMP4EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4EN` reader - Comparator 4 enable"]
pub type COMP4EN_R = crate::BitReader<COMP4EN_A>;
impl COMP4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4EN_A {
        match self.bits {
            false => COMP4EN_A::Disabled,
            true => COMP4EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP4EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP4EN_A::Enabled
    }
}
#[doc = "Field `COMP4EN` writer - Comparator 4 enable"]
pub type COMP4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP4_CSR_SPEC, COMP4EN_A, O>;
impl<'a, const O: u8> COMP4EN_W<'a, O> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP4EN_A::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP4EN_A::Enabled)
    }
}
#[doc = "Comparator 4 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP4INMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: PA4 or DAC1_CH1 output if enabled"]
    Pa4Dac1Ch1 = 4,
    #[doc = "5: DAC1_CH2"]
    Dac1Ch2 = 5,
    #[doc = "7: PB2"]
    Pb2 = 7,
}
impl From<COMP4INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP4INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP4INMSEL` reader - Comparator 4 inverting input selection"]
pub type COMP4INMSEL_R = crate::FieldReader<u8, COMP4INMSEL_A>;
impl COMP4INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP4INMSEL_A> {
        match self.bits {
            0 => Some(COMP4INMSEL_A::OneQuarterVref),
            1 => Some(COMP4INMSEL_A::OneHalfVref),
            2 => Some(COMP4INMSEL_A::ThreeQuarterVref),
            3 => Some(COMP4INMSEL_A::Vref),
            4 => Some(COMP4INMSEL_A::Pa4Dac1Ch1),
            5 => Some(COMP4INMSEL_A::Dac1Ch2),
            7 => Some(COMP4INMSEL_A::Pb2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP4INMSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP4INMSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP4INMSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP4INMSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `Pa4Dac1Ch1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP4INMSEL_A::Pa4Dac1Ch1
    }
    #[doc = "Checks if the value of the field is `Dac1Ch2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP4INMSEL_A::Dac1Ch2
    }
    #[doc = "Checks if the value of the field is `Pb2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == COMP4INMSEL_A::Pb2
    }
}
#[doc = "Field `COMP4INMSEL` writer - Comparator 4 inverting input selection"]
pub type COMP4INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP4_CSR_SPEC, u8, COMP4INMSEL_A, 3, O>;
impl<'a, const O: u8> COMP4INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::Vref)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::Pa4Dac1Ch1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::Dac1Ch2)
    }
    #[doc = "PB2"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(COMP4INMSEL_A::Pb2)
    }
}
#[doc = "Comparator 4 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP4OUTSEL_A {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 break input 2"]
    Timer1breakInput2 = 2,
    #[doc = "6: Timer 3 input capture 3"]
    Timer3inputCapture3 = 6,
    #[doc = "8: Timer 15 input capture 2"]
    Timer15inputCapture2 = 8,
    #[doc = "10: Timer 15 OCREF_CLR input"]
    Timer15ocrefClearInput = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    Timer3ocrefClearInput = 11,
}
impl From<COMP4OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP4OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP4OUTSEL` reader - Comparator 4 output selection"]
pub type COMP4OUTSEL_R = crate::FieldReader<u8, COMP4OUTSEL_A>;
impl COMP4OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP4OUTSEL_A> {
        match self.bits {
            0 => Some(COMP4OUTSEL_A::NoSelection),
            1 => Some(COMP4OUTSEL_A::Timer1breakInput),
            2 => Some(COMP4OUTSEL_A::Timer1breakInput2),
            6 => Some(COMP4OUTSEL_A::Timer3inputCapture3),
            8 => Some(COMP4OUTSEL_A::Timer15inputCapture2),
            10 => Some(COMP4OUTSEL_A::Timer15ocrefClearInput),
            11 => Some(COMP4OUTSEL_A::Timer3ocrefClearInput),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoSelection`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP4OUTSEL_A::NoSelection
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP4OUTSEL_A::Timer1breakInput
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP4OUTSEL_A::Timer1breakInput2
    }
    #[doc = "Checks if the value of the field is `Timer3inputCapture3`"]
    #[inline(always)]
    pub fn is_timer3input_capture3(&self) -> bool {
        *self == COMP4OUTSEL_A::Timer3inputCapture3
    }
    #[doc = "Checks if the value of the field is `Timer15inputCapture2`"]
    #[inline(always)]
    pub fn is_timer15input_capture2(&self) -> bool {
        *self == COMP4OUTSEL_A::Timer15inputCapture2
    }
    #[doc = "Checks if the value of the field is `Timer15ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer15ocref_clear_input(&self) -> bool {
        *self == COMP4OUTSEL_A::Timer15ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer3ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP4OUTSEL_A::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP4OUTSEL` writer - Comparator 4 output selection"]
pub type COMP4OUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP4_CSR_SPEC, u8, COMP4OUTSEL_A, 4, O>;
impl<'a, const O: u8> COMP4OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::Timer1breakInput)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::Timer1breakInput2)
    }
    #[doc = "Timer 3 input capture 3"]
    #[inline(always)]
    pub fn timer3input_capture3(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::Timer3inputCapture3)
    }
    #[doc = "Timer 15 input capture 2"]
    #[inline(always)]
    pub fn timer15input_capture2(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::Timer15inputCapture2)
    }
    #[doc = "Timer 15 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer15ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::Timer15ocrefClearInput)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP4OUTSEL_A::Timer3ocrefClearInput)
    }
}
#[doc = "Comparator 4 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4POL_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP4POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4POL` reader - Comparator 4 output polarity"]
pub type COMP4POL_R = crate::BitReader<COMP4POL_A>;
impl COMP4POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4POL_A {
        match self.bits {
            false => COMP4POL_A::NotInverted,
            true => COMP4POL_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP4POL_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP4POL_A::Inverted
    }
}
#[doc = "Field `COMP4POL` writer - Comparator 4 output polarity"]
pub type COMP4POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP4_CSR_SPEC, COMP4POL_A, O>;
impl<'a, const O: u8> COMP4POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP4POL_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP4POL_A::Inverted)
    }
}
#[doc = "Comparator 4 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP4_BLANKING_A {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "1: TIM3 OC4 selected as blanking source"]
    Tim3oc4 = 1,
    #[doc = "3: TIM15 OC1 selected as blanking source"]
    Tim15oc1 = 3,
}
impl From<COMP4_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP4_BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP4_BLANKING` reader - Comparator 4 blanking source"]
pub type COMP4_BLANKING_R = crate::FieldReader<u8, COMP4_BLANKING_A>;
impl COMP4_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP4_BLANKING_A> {
        match self.bits {
            0 => Some(COMP4_BLANKING_A::NoBlanking),
            1 => Some(COMP4_BLANKING_A::Tim3oc4),
            3 => Some(COMP4_BLANKING_A::Tim15oc1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoBlanking`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP4_BLANKING_A::NoBlanking
    }
    #[doc = "Checks if the value of the field is `Tim3oc4`"]
    #[inline(always)]
    pub fn is_tim3oc4(&self) -> bool {
        *self == COMP4_BLANKING_A::Tim3oc4
    }
    #[doc = "Checks if the value of the field is `Tim15oc1`"]
    #[inline(always)]
    pub fn is_tim15oc1(&self) -> bool {
        *self == COMP4_BLANKING_A::Tim15oc1
    }
}
#[doc = "Field `COMP4_BLANKING` writer - Comparator 4 blanking source"]
pub type COMP4_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP4_CSR_SPEC, u8, COMP4_BLANKING_A, 3, O>;
impl<'a, const O: u8> COMP4_BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP4_BLANKING_A::NoBlanking)
    }
    #[doc = "TIM3 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc4(self) -> &'a mut W {
        self.variant(COMP4_BLANKING_A::Tim3oc4)
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc1(self) -> &'a mut W {
        self.variant(COMP4_BLANKING_A::Tim15oc1)
    }
}
#[doc = "Comparator 4 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP4OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4OUT` reader - Comparator 4 output"]
pub type COMP4OUT_R = crate::BitReader<COMP4OUT_A>;
impl COMP4OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4OUT_A {
        match self.bits {
            false => COMP4OUT_A::Low,
            true => COMP4OUT_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP4OUT_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP4OUT_A::High
    }
}
#[doc = "Comparator 4 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP4LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    Locked = 1,
}
impl From<COMP4LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP4LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP4LOCK` reader - Comparator 4 lock"]
pub type COMP4LOCK_R = crate::BitReader<COMP4LOCK_A>;
impl COMP4LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP4LOCK_A {
        match self.bits {
            false => COMP4LOCK_A::Unlocked,
            true => COMP4LOCK_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP4LOCK_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP4LOCK_A::Locked
    }
}
#[doc = "Field `COMP4LOCK` writer - Comparator 4 lock"]
pub type COMP4LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP4_CSR_SPEC, COMP4LOCK_A, O>;
impl<'a, const O: u8> COMP4LOCK_W<'a, O> {
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP4LOCK_A::Unlocked)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP4LOCK_A::Locked)
    }
}
#[doc = "Field `COMP4WINMODE` reader - Comparator 4 window mode"]
pub type COMP4WINMODE_R = crate::BitReader<bool>;
#[doc = "Field `COMP4WINMODE` writer - Comparator 4 window mode"]
pub type COMP4WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP4_CSR_SPEC, bool, O>;
#[doc = "Field `COMP4MODE` reader - Comparator 4 mode"]
pub type COMP4MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP4MODE` writer - Comparator 4 mode"]
pub type COMP4MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP4_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP4INPSEL` reader - Comparator 4 non inverted input"]
pub type COMP4INPSEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP4INPSEL` writer - Comparator 4 non inverted input"]
pub type COMP4INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP4_CSR_SPEC, bool, O>;
#[doc = "Field `COMP4HYST` reader - Comparator 4 hysteresis"]
pub type COMP4HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP4HYST` writer - Comparator 4 hysteresis"]
pub type COMP4HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP4_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP4INMSEL3` reader - Comparator 4 inverting input selection"]
pub type COMP4INMSEL3_R = crate::BitReader<bool>;
#[doc = "Field `COMP4INMSEL3` writer - Comparator 4 inverting input selection"]
pub type COMP4INMSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP4_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&self) -> COMP4EN_R {
        COMP4EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel(&self) -> COMP4INMSEL_R {
        COMP4INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4outsel(&self) -> COMP4OUTSEL_R {
        COMP4OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&self) -> COMP4POL_R {
        COMP4POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&self) -> COMP4_BLANKING_R {
        COMP4_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output"]
    #[inline(always)]
    pub fn comp4out(&self) -> COMP4OUT_R {
        COMP4OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&self) -> COMP4LOCK_R {
        COMP4LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn comp4winmode(&self) -> COMP4WINMODE_R {
        COMP4WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&self) -> COMP4MODE_R {
        COMP4MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input"]
    #[inline(always)]
    pub fn comp4inpsel(&self) -> COMP4INPSEL_R {
        COMP4INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&self) -> COMP4HYST_R {
        COMP4HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel3(&self) -> COMP4INMSEL3_R {
        COMP4INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&mut self) -> COMP4EN_W<0> {
        COMP4EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel(&mut self) -> COMP4INMSEL_W<4> {
        COMP4INMSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4outsel(&mut self) -> COMP4OUTSEL_W<10> {
        COMP4OUTSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&mut self) -> COMP4POL_W<15> {
        COMP4POL_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&mut self) -> COMP4_BLANKING_W<18> {
        COMP4_BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&mut self) -> COMP4LOCK_W<31> {
        COMP4LOCK_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn comp4winmode(&mut self) -> COMP4WINMODE_W<9> {
        COMP4WINMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&mut self) -> COMP4MODE_W<2> {
        COMP4MODE_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input"]
    #[inline(always)]
    pub fn comp4inpsel(&mut self) -> COMP4INPSEL_W<7> {
        COMP4INPSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&mut self) -> COMP4HYST_W<16> {
        COMP4HYST_W::new(self)
    }
    #[doc = "Bit 22 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4inmsel3(&mut self) -> COMP4INMSEL3_W<22> {
        COMP4INMSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp4_csr](index.html) module"]
pub struct COMP4_CSR_SPEC;
impl crate::RegisterSpec for COMP4_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp4_csr::R](R) reader structure"]
impl crate::Readable for COMP4_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp4_csr::W](W) writer structure"]
impl crate::Writable for COMP4_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for COMP4_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader<COMP2EN_A>;
impl COMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2EN_A {
        match self.bits {
            false => COMP2EN_A::Disabled,
            true => COMP2EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN_A::Enabled
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2EN_A, O>;
impl<'a, const O: u8> COMP2EN_W<'a, O> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::Enabled)
    }
}
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2INMSEL_A {
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
    #[doc = "6: PA2"]
    Pa2 = 6,
}
impl From<COMP2INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2INMSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INMSEL_R = crate::FieldReader<u8, COMP2INMSEL_A>;
impl COMP2INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2INMSEL_A> {
        match self.bits {
            0 => Some(COMP2INMSEL_A::OneQuarterVref),
            1 => Some(COMP2INMSEL_A::OneHalfVref),
            2 => Some(COMP2INMSEL_A::ThreeQuarterVref),
            3 => Some(COMP2INMSEL_A::Vref),
            4 => Some(COMP2INMSEL_A::Pa4Dac1Ch1),
            5 => Some(COMP2INMSEL_A::Dac1Ch2),
            6 => Some(COMP2INMSEL_A::Pa2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INMSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INMSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `Pa4Dac1Ch1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP2INMSEL_A::Pa4Dac1Ch1
    }
    #[doc = "Checks if the value of the field is `Dac1Ch2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP2INMSEL_A::Dac1Ch2
    }
    #[doc = "Checks if the value of the field is `Pa2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == COMP2INMSEL_A::Pa2
    }
}
#[doc = "Field `COMP2INMSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, COMP2INMSEL_A, 3, O>;
impl<'a, const O: u8> COMP2INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::Vref)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::Pa4Dac1Ch1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::Dac1Ch2)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::Pa2)
    }
}
#[doc = "Comparator 2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2OUTSEL_A {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 break input 2"]
    Timer1breakInput2 = 2,
    #[doc = "6: Timer 1 OCREF_CLR input"]
    Timer1ocrefClearInput = 6,
    #[doc = "7: Timer 1 input capture 1"]
    Timer1inputCapture1 = 7,
    #[doc = "8: Timer 2 input capture 4"]
    Timer2inputCapture4 = 8,
    #[doc = "9: Timer 2 OCREF_CLR input"]
    Timer2ocrefClearInput = 9,
    #[doc = "10: Timer 3 input capture 1"]
    Timer3inputCapture1 = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    Timer3ocrefClearInput = 11,
}
impl From<COMP2OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub type COMP2OUTSEL_R = crate::FieldReader<u8, COMP2OUTSEL_A>;
impl COMP2OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2OUTSEL_A> {
        match self.bits {
            0 => Some(COMP2OUTSEL_A::NoSelection),
            1 => Some(COMP2OUTSEL_A::Timer1breakInput),
            2 => Some(COMP2OUTSEL_A::Timer1breakInput2),
            6 => Some(COMP2OUTSEL_A::Timer1ocrefClearInput),
            7 => Some(COMP2OUTSEL_A::Timer1inputCapture1),
            8 => Some(COMP2OUTSEL_A::Timer2inputCapture4),
            9 => Some(COMP2OUTSEL_A::Timer2ocrefClearInput),
            10 => Some(COMP2OUTSEL_A::Timer3inputCapture1),
            11 => Some(COMP2OUTSEL_A::Timer3ocrefClearInput),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoSelection`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP2OUTSEL_A::NoSelection
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1breakInput
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1breakInput2
    }
    #[doc = "Checks if the value of the field is `Timer1ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer1inputCapture1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer1inputCapture1
    }
    #[doc = "Checks if the value of the field is `Timer2inputCapture4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer2inputCapture4
    }
    #[doc = "Checks if the value of the field is `Timer2ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer2ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer3inputCapture1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer3inputCapture1
    }
    #[doc = "Checks if the value of the field is `Timer3ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::Timer3ocrefClearInput
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, COMP2OUTSEL_A, 4, O>;
impl<'a, const O: u8> COMP2OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1breakInput)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1breakInput2)
    }
    #[doc = "Timer 1 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1ocrefClearInput)
    }
    #[doc = "Timer 1 input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer1inputCapture1)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer2inputCapture4)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer2ocrefClearInput)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer3inputCapture1)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::Timer3ocrefClearInput)
    }
}
#[doc = "Comparator 2 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2POL_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP2POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader<COMP2POL_A>;
impl COMP2POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2POL_A {
        match self.bits {
            false => COMP2POL_A::NotInverted,
            true => COMP2POL_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POL_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POL_A::Inverted
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2POL_A, O>;
impl<'a, const O: u8> COMP2POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::Inverted)
    }
}
#[doc = "Comparator 2 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP2_BLANKING_A {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "1: TIM1 OC5 selected as blanking source"]
    Tim1oc5 = 1,
    #[doc = "2: TIM2 OC3 selected as blanking source"]
    Tim2oc3 = 2,
    #[doc = "3: TIM3 OC3 selected as blanking source"]
    Tim3oc3 = 3,
}
impl From<COMP2_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2_BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub type COMP2_BLANKING_R = crate::FieldReader<u8, COMP2_BLANKING_A>;
impl COMP2_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2_BLANKING_A> {
        match self.bits {
            0 => Some(COMP2_BLANKING_A::NoBlanking),
            1 => Some(COMP2_BLANKING_A::Tim1oc5),
            2 => Some(COMP2_BLANKING_A::Tim2oc3),
            3 => Some(COMP2_BLANKING_A::Tim3oc3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoBlanking`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP2_BLANKING_A::NoBlanking
    }
    #[doc = "Checks if the value of the field is `Tim1oc5`"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == COMP2_BLANKING_A::Tim1oc5
    }
    #[doc = "Checks if the value of the field is `Tim2oc3`"]
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        *self == COMP2_BLANKING_A::Tim2oc3
    }
    #[doc = "Checks if the value of the field is `Tim3oc3`"]
    #[inline(always)]
    pub fn is_tim3oc3(&self) -> bool {
        *self == COMP2_BLANKING_A::Tim3oc3
    }
}
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub type COMP2_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, COMP2_BLANKING_A, 3, O>;
impl<'a, const O: u8> COMP2_BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::NoBlanking)
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::Tim1oc5)
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::Tim2oc3)
    }
    #[doc = "TIM3 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc3(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::Tim3oc3)
    }
}
#[doc = "Comparator 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP2OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type COMP2OUT_R = crate::BitReader<COMP2OUT_A>;
impl COMP2OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2OUT_A {
        match self.bits {
            false => COMP2OUT_A::Low,
            true => COMP2OUT_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP2OUT_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP2OUT_A::High
    }
}
#[doc = "Comparator 2 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    Locked = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK_A>;
impl COMP2LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LOCK_A {
        match self.bits {
            false => COMP2LOCK_A::Unlocked,
            true => COMP2LOCK_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP2LOCK_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP2LOCK_A::Locked
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, COMP2LOCK_A, O>;
impl<'a, const O: u8> COMP2LOCK_W<'a, O> {
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::Unlocked)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::Locked)
    }
}
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 non inverted input"]
pub type COMP2INPSEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 non inverted input"]
pub type COMP2INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
#[doc = "Field `COMP2WINMODE` reader - Comparator 2 window mode"]
pub type COMP2WINMODE_R = crate::BitReader<bool>;
#[doc = "Field `COMP2WINMODE` writer - Comparator 2 window mode"]
pub type COMP2WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP2INMSEL3` reader - Comparator 2 inverting input selection"]
pub type COMP2INMSEL3_R = crate::BitReader<bool>;
#[doc = "Field `COMP2INMSEL3` writer - Comparator 2 inverting input selection"]
pub type COMP2INMSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 2 window mode"]
    #[inline(always)]
    pub fn comp2winmode(&self) -> COMP2WINMODE_R {
        COMP2WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel3(&self) -> COMP2INMSEL3_R {
        COMP2INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&mut self) -> COMP2EN_W<0> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W<4> {
        COMP2INMSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<10> {
        COMP2OUTSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&mut self) -> COMP2POL_W<15> {
        COMP2POL_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<18> {
        COMP2_BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<31> {
        COMP2LOCK_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<2> {
        COMP2MODE_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input"]
    #[inline(always)]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W<7> {
        COMP2INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 2 window mode"]
    #[inline(always)]
    pub fn comp2winmode(&mut self) -> COMP2WINMODE_W<9> {
        COMP2WINMODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<16> {
        COMP2HYST_W::new(self)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel3(&mut self) -> COMP2INMSEL3_W<22> {
        COMP2INMSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

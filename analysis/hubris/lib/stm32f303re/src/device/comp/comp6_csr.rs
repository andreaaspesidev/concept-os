#[doc = "Register `COMP6_CSR` reader"]
pub struct R(crate::R<COMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_CSR` writer"]
pub struct W(crate::W<COMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_CSR_SPEC>;
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
impl From<crate::W<COMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6EN_A {
    #[doc = "0: Comparator disabled"]
    Disabled = 0,
    #[doc = "1: Comparator enabled"]
    Enabled = 1,
}
impl From<COMP6EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6EN` reader - Comparator 6 enable"]
pub type COMP6EN_R = crate::BitReader<COMP6EN_A>;
impl COMP6EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6EN_A {
        match self.bits {
            false => COMP6EN_A::Disabled,
            true => COMP6EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP6EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP6EN_A::Enabled
    }
}
#[doc = "Field `COMP6EN` writer - Comparator 6 enable"]
pub type COMP6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, COMP6EN_A, O>;
impl<'a, const O: u8> COMP6EN_W<'a, O> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::Disabled)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::Enabled)
    }
}
#[doc = "Comparator 6 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP6INMSEL_A {
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
    #[doc = "7: PB15"]
    Pb15 = 7,
}
impl From<COMP6INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP6INMSEL` reader - Comparator 6 inverting input selection"]
pub type COMP6INMSEL_R = crate::FieldReader<u8, COMP6INMSEL_A>;
impl COMP6INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6INMSEL_A> {
        match self.bits {
            0 => Some(COMP6INMSEL_A::OneQuarterVref),
            1 => Some(COMP6INMSEL_A::OneHalfVref),
            2 => Some(COMP6INMSEL_A::ThreeQuarterVref),
            3 => Some(COMP6INMSEL_A::Vref),
            4 => Some(COMP6INMSEL_A::Pa4Dac1Ch1),
            5 => Some(COMP6INMSEL_A::Dac1Ch2),
            7 => Some(COMP6INMSEL_A::Pb15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP6INMSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP6INMSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `Pa4Dac1Ch1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP6INMSEL_A::Pa4Dac1Ch1
    }
    #[doc = "Checks if the value of the field is `Dac1Ch2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP6INMSEL_A::Dac1Ch2
    }
    #[doc = "Checks if the value of the field is `Pb15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == COMP6INMSEL_A::Pb15
    }
}
#[doc = "Field `COMP6INMSEL` writer - Comparator 6 inverting input selection"]
pub type COMP6INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, COMP6INMSEL_A, 3, O>;
impl<'a, const O: u8> COMP6INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Vref)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Pa4Dac1Ch1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Dac1Ch2)
    }
    #[doc = "PB15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Pb15)
    }
}
#[doc = "Comparator 6 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP6OUTSEL_A {
    #[doc = "0: No selection"]
    NoSelection = 0,
    #[doc = "1: Timer 1 break input"]
    Timer1breakInput = 1,
    #[doc = "2: Timer 1 break input 2"]
    Timer1breakInput2 = 2,
    #[doc = "6: Timer 2 input capture 2"]
    Timer2inputCapture2 = 6,
    #[doc = "8: Timer 2 OCREF_CLR input"]
    Timer2ocrefClearInput = 8,
    #[doc = "9: Timer 16 OCREF_CLR input"]
    Timer16ocrefClearInput = 9,
    #[doc = "10: Timer 16 input capture 1"]
    Timer16inputCapture1 = 10,
}
impl From<COMP6OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP6OUTSEL` reader - Comparator 6 output selection"]
pub type COMP6OUTSEL_R = crate::FieldReader<u8, COMP6OUTSEL_A>;
impl COMP6OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6OUTSEL_A> {
        match self.bits {
            0 => Some(COMP6OUTSEL_A::NoSelection),
            1 => Some(COMP6OUTSEL_A::Timer1breakInput),
            2 => Some(COMP6OUTSEL_A::Timer1breakInput2),
            6 => Some(COMP6OUTSEL_A::Timer2inputCapture2),
            8 => Some(COMP6OUTSEL_A::Timer2ocrefClearInput),
            9 => Some(COMP6OUTSEL_A::Timer16ocrefClearInput),
            10 => Some(COMP6OUTSEL_A::Timer16inputCapture1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoSelection`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP6OUTSEL_A::NoSelection
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer1breakInput
    }
    #[doc = "Checks if the value of the field is `Timer1breakInput2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer1breakInput2
    }
    #[doc = "Checks if the value of the field is `Timer2inputCapture2`"]
    #[inline(always)]
    pub fn is_timer2input_capture2(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer2inputCapture2
    }
    #[doc = "Checks if the value of the field is `Timer2ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer2ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer16ocrefClearInput`"]
    #[inline(always)]
    pub fn is_timer16ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer16ocrefClearInput
    }
    #[doc = "Checks if the value of the field is `Timer16inputCapture1`"]
    #[inline(always)]
    pub fn is_timer16input_capture1(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer16inputCapture1
    }
}
#[doc = "Field `COMP6OUTSEL` writer - Comparator 6 output selection"]
pub type COMP6OUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, COMP6OUTSEL_A, 4, O>;
impl<'a, const O: u8> COMP6OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::NoSelection)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer1breakInput)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer1breakInput2)
    }
    #[doc = "Timer 2 input capture 2"]
    #[inline(always)]
    pub fn timer2input_capture2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer2inputCapture2)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer2ocrefClearInput)
    }
    #[doc = "Timer 16 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer16ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer16ocrefClearInput)
    }
    #[doc = "Timer 16 input capture 1"]
    #[inline(always)]
    pub fn timer16input_capture1(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer16inputCapture1)
    }
}
#[doc = "Comparator 6 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6POL_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<COMP6POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6POL` reader - Comparator 6 output polarity"]
pub type COMP6POL_R = crate::BitReader<COMP6POL_A>;
impl COMP6POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6POL_A {
        match self.bits {
            false => COMP6POL_A::NotInverted,
            true => COMP6POL_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP6POL_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP6POL_A::Inverted
    }
}
#[doc = "Field `COMP6POL` writer - Comparator 6 output polarity"]
pub type COMP6POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, COMP6POL_A, O>;
impl<'a, const O: u8> COMP6POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::Inverted)
    }
}
#[doc = "Comparator 6 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP6_BLANKING_A {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "3: TIM2 OC4 selected as blanking source"]
    Tim2oc4 = 3,
    #[doc = "4: TIM15 OC2 selected as blanking source"]
    Tim15oc2 = 4,
}
impl From<COMP6_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6_BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `COMP6_BLANKING` reader - Comparator 6 blanking source"]
pub type COMP6_BLANKING_R = crate::FieldReader<u8, COMP6_BLANKING_A>;
impl COMP6_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6_BLANKING_A> {
        match self.bits {
            0 => Some(COMP6_BLANKING_A::NoBlanking),
            3 => Some(COMP6_BLANKING_A::Tim2oc4),
            4 => Some(COMP6_BLANKING_A::Tim15oc2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoBlanking`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP6_BLANKING_A::NoBlanking
    }
    #[doc = "Checks if the value of the field is `Tim2oc4`"]
    #[inline(always)]
    pub fn is_tim2oc4(&self) -> bool {
        *self == COMP6_BLANKING_A::Tim2oc4
    }
    #[doc = "Checks if the value of the field is `Tim15oc2`"]
    #[inline(always)]
    pub fn is_tim15oc2(&self) -> bool {
        *self == COMP6_BLANKING_A::Tim15oc2
    }
}
#[doc = "Field `COMP6_BLANKING` writer - Comparator 6 blanking source"]
pub type COMP6_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, COMP6_BLANKING_A, 3, O>;
impl<'a, const O: u8> COMP6_BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::NoBlanking)
    }
    #[doc = "TIM2 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc4(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::Tim2oc4)
    }
    #[doc = "TIM15 OC2 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc2(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::Tim15oc2)
    }
}
#[doc = "Comparator 6 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    Low = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    High = 1,
}
impl From<COMP6OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6OUT` reader - Comparator 6 output"]
pub type COMP6OUT_R = crate::BitReader<COMP6OUT_A>;
impl COMP6OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6OUT_A {
        match self.bits {
            false => COMP6OUT_A::Low,
            true => COMP6OUT_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP6OUT_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP6OUT_A::High
    }
}
#[doc = "Comparator 6 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP6LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    Unlocked = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    Locked = 1,
}
impl From<COMP6LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP6LOCK` reader - Comparator 6 lock"]
pub type COMP6LOCK_R = crate::BitReader<COMP6LOCK_A>;
impl COMP6LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6LOCK_A {
        match self.bits {
            false => COMP6LOCK_A::Unlocked,
            true => COMP6LOCK_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Unlocked`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP6LOCK_A::Unlocked
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP6LOCK_A::Locked
    }
}
#[doc = "Field `COMP6LOCK` writer - Comparator 6 lock"]
pub type COMP6LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, COMP6LOCK_A, O>;
impl<'a, const O: u8> COMP6LOCK_W<'a, O> {
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::Unlocked)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::Locked)
    }
}
#[doc = "Field `COMP6WINMODE` reader - Comparator 6 window mode"]
pub type COMP6WINMODE_R = crate::BitReader<bool>;
#[doc = "Field `COMP6WINMODE` writer - Comparator 6 window mode"]
pub type COMP6WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, bool, O>;
#[doc = "Field `COMP6MODE` reader - Comparator 6 mode"]
pub type COMP6MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP6MODE` writer - Comparator 6 mode"]
pub type COMP6MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP6INPSEL` reader - Comparator 6 non inverted input"]
pub type COMP6INPSEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP6INPSEL` writer - Comparator 6 non inverted input"]
pub type COMP6INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, bool, O>;
#[doc = "Field `COMP6HYST` reader - Comparator 6 hysteresis"]
pub type COMP6HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP6HYST` writer - Comparator 6 hysteresis"]
pub type COMP6HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP6INMSEL3` reader - Comparator 6 inverting input selection"]
pub type COMP6INMSEL3_R = crate::BitReader<bool>;
#[doc = "Field `COMP6INMSEL3` writer - Comparator 6 inverting input selection"]
pub type COMP6INMSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel(&self) -> COMP6INMSEL_R {
        COMP6INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6outsel(&self) -> COMP6OUTSEL_R {
        COMP6OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 6 output"]
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn comp6winmode(&self) -> COMP6WINMODE_R {
        COMP6WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&self) -> COMP6MODE_R {
        COMP6MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input"]
    #[inline(always)]
    pub fn comp6inpsel(&self) -> COMP6INPSEL_R {
        COMP6INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&self) -> COMP6HYST_R {
        COMP6HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel3(&self) -> COMP6INMSEL3_R {
        COMP6INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&mut self) -> COMP6EN_W<0> {
        COMP6EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel(&mut self) -> COMP6INMSEL_W<4> {
        COMP6INMSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6outsel(&mut self) -> COMP6OUTSEL_W<10> {
        COMP6OUTSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&mut self) -> COMP6POL_W<15> {
        COMP6POL_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W<18> {
        COMP6_BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W<31> {
        COMP6LOCK_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn comp6winmode(&mut self) -> COMP6WINMODE_W<9> {
        COMP6WINMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&mut self) -> COMP6MODE_W<2> {
        COMP6MODE_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input"]
    #[inline(always)]
    pub fn comp6inpsel(&mut self) -> COMP6INPSEL_W<7> {
        COMP6INPSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&mut self) -> COMP6HYST_W<16> {
        COMP6HYST_W::new(self)
    }
    #[doc = "Bit 22 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel3(&mut self) -> COMP6INMSEL3_W<22> {
        COMP6INMSEL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_csr](index.html) module"]
pub struct COMP6_CSR_SPEC;
impl crate::RegisterSpec for COMP6_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_csr::R](R) reader structure"]
impl crate::Readable for COMP6_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_csr::W](W) writer structure"]
impl crate::Writable for COMP6_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP6_CSR to value 0"]
impl crate::Resettable for COMP6_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

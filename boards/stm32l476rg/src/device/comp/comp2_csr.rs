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
#[doc = "Comparator 2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Comparator 1 disabled"]
    Disabled = 0,
    #[doc = "1: Comparator 1 enabled"]
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator 2 enable bit"]
pub type EN_R = crate::BitReader<EN_A>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
#[doc = "Field `EN` writer - Comparator 2 enable bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Comparator 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    #[doc = "Comparator 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
#[doc = "Power Mode of the comparator 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRMODE_A {
    #[doc = "0: High speed / full power"]
    HighSpeed = 0,
    #[doc = "1: Medium speed / medium power"]
    MediumSpeed = 1,
    #[doc = "3: Low speed / ultra-low power"]
    LowSpeed = 3,
}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRMODE` reader - Power Mode of the comparator 2"]
pub type PWRMODE_R = crate::FieldReader<u8, PWRMODE_A>;
impl PWRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRMODE_A> {
        match self.bits {
            0 => Some(PWRMODE_A::HighSpeed),
            1 => Some(PWRMODE_A::MediumSpeed),
            3 => Some(PWRMODE_A::LowSpeed),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HighSpeed`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == PWRMODE_A::HighSpeed
    }
    #[doc = "Checks if the value of the field is `MediumSpeed`"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == PWRMODE_A::MediumSpeed
    }
    #[doc = "Checks if the value of the field is `LowSpeed`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == PWRMODE_A::LowSpeed
    }
}
#[doc = "Field `PWRMODE` writer - Power Mode of the comparator 2"]
pub type PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, PWRMODE_A, 2, O>;
impl<'a, const O: u8> PWRMODE_W<'a, O> {
    #[doc = "High speed / full power"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::HighSpeed)
    }
    #[doc = "Medium speed / medium power"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::MediumSpeed)
    }
    #[doc = "Low speed / ultra-low power"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut W {
        self.variant(PWRMODE_A::LowSpeed)
    }
}
#[doc = "Comparator 2 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    OneQuarterVref = 0,
    #[doc = "1: 1/2 of VRefint"]
    OneHalfVref = 1,
    #[doc = "2: 3/4 of VRefint"]
    ThreeQuarterVref = 2,
    #[doc = "3: VRefint"]
    Vref = 3,
    #[doc = "4: DAC Channel 1"]
    DacCh1 = 4,
    #[doc = "5: DAC Channel 2"]
    DacCh2 = 5,
    #[doc = "6: PB3"]
    Pb3 = 6,
    #[doc = "7: PB7"]
    Pb7 = 7,
}
impl From<INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INMSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INMSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub type INMSEL_R = crate::FieldReader<u8, INMSEL_A>;
impl INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INMSEL_A {
        match self.bits {
            0 => INMSEL_A::OneQuarterVref,
            1 => INMSEL_A::OneHalfVref,
            2 => INMSEL_A::ThreeQuarterVref,
            3 => INMSEL_A::Vref,
            4 => INMSEL_A::DacCh1,
            5 => INMSEL_A::DacCh2,
            6 => INMSEL_A::Pb3,
            7 => INMSEL_A::Pb7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OneQuarterVref`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == INMSEL_A::OneQuarterVref
    }
    #[doc = "Checks if the value of the field is `OneHalfVref`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == INMSEL_A::OneHalfVref
    }
    #[doc = "Checks if the value of the field is `ThreeQuarterVref`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == INMSEL_A::ThreeQuarterVref
    }
    #[doc = "Checks if the value of the field is `Vref`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == INMSEL_A::Vref
    }
    #[doc = "Checks if the value of the field is `DacCh1`"]
    #[inline(always)]
    pub fn is_dac_ch1(&self) -> bool {
        *self == INMSEL_A::DacCh1
    }
    #[doc = "Checks if the value of the field is `DacCh2`"]
    #[inline(always)]
    pub fn is_dac_ch2(&self) -> bool {
        *self == INMSEL_A::DacCh2
    }
    #[doc = "Checks if the value of the field is `Pb3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == INMSEL_A::Pb3
    }
    #[doc = "Checks if the value of the field is `Pb7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == INMSEL_A::Pb7
    }
}
#[doc = "Field `INMSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub type INMSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP2_CSR_SPEC, u8, INMSEL_A, 3, O>;
impl<'a, const O: u8> INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::OneQuarterVref)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::OneHalfVref)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(INMSEL_A::ThreeQuarterVref)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(INMSEL_A::Vref)
    }
    #[doc = "DAC Channel 1"]
    #[inline(always)]
    pub fn dac_ch1(self) -> &'a mut W {
        self.variant(INMSEL_A::DacCh1)
    }
    #[doc = "DAC Channel 2"]
    #[inline(always)]
    pub fn dac_ch2(self) -> &'a mut W {
        self.variant(INMSEL_A::DacCh2)
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(INMSEL_A::Pb3)
    }
    #[doc = "PB7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(INMSEL_A::Pb7)
    }
}
#[doc = "Comparator 2 Input Plus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPSEL_A {
    #[doc = "0: PB4 connected to input plus"]
    Pb4 = 0,
    #[doc = "1: PB6 connected to input plus"]
    Pb6 = 1,
}
impl From<INPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub type INPSEL_R = crate::BitReader<INPSEL_A>;
impl INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPSEL_A {
        match self.bits {
            false => INPSEL_A::Pb4,
            true => INPSEL_A::Pb6,
        }
    }
    #[doc = "Checks if the value of the field is `Pb4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == INPSEL_A::Pb4
    }
    #[doc = "Checks if the value of the field is `Pb6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == INPSEL_A::Pb6
    }
}
#[doc = "Field `INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub type INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, INPSEL_A, O>;
impl<'a, const O: u8> INPSEL_W<'a, O> {
    #[doc = "PB4 connected to input plus"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(INPSEL_A::Pb4)
    }
    #[doc = "PB6 connected to input plus"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(INPSEL_A::Pb6)
    }
}
#[doc = "Windows mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINMODE_A {
    #[doc = "0: COMP2 input plus is not connected to COMP1"]
    Disabled = 0,
    #[doc = "1: COMP2 input plus is connected to COMP1 plus"]
    Enabled = 1,
}
impl From<WINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINMODE` reader - Windows mode selection bit"]
pub type WINMODE_R = crate::BitReader<WINMODE_A>;
impl WINMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINMODE_A {
        match self.bits {
            false => WINMODE_A::Disabled,
            true => WINMODE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WINMODE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WINMODE_A::Enabled
    }
}
#[doc = "Field `WINMODE` writer - Windows mode selection bit"]
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, WINMODE_A, O>;
impl<'a, const O: u8> WINMODE_W<'a, O> {
    #[doc = "COMP2 input plus is not connected to COMP1"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WINMODE_A::Disabled)
    }
    #[doc = "COMP2 input plus is connected to COMP1 plus"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WINMODE_A::Enabled)
    }
}
#[doc = "Comparator 2 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITY_A {
    #[doc = "0: Output is not inverted"]
    NotInverted = 0,
    #[doc = "1: Output is inverted"]
    Inverted = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selection bit"]
pub type POLARITY_R = crate::BitReader<POLARITY_A>;
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::NotInverted,
            true => POLARITY_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInverted`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == POLARITY_A::NotInverted
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY_A::Inverted
    }
}
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selection bit"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, POLARITY_A, O>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::NotInverted)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::Inverted)
    }
}
#[doc = "Comparator 2 hysteresis selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYST_A {
    #[doc = "0: No hysteresis"]
    NoHysteresis = 0,
    #[doc = "1: Low hysteresis"]
    LowHysteresis = 1,
    #[doc = "2: Medium hysteresis"]
    MediumHysteresis = 2,
    #[doc = "3: High hysteresis"]
    HighHysteresis = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selection bits"]
pub type HYST_R = crate::FieldReader<u8, HYST_A>;
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::NoHysteresis,
            1 => HYST_A::LowHysteresis,
            2 => HYST_A::MediumHysteresis,
            3 => HYST_A::HighHysteresis,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoHysteresis`"]
    #[inline(always)]
    pub fn is_no_hysteresis(&self) -> bool {
        *self == HYST_A::NoHysteresis
    }
    #[doc = "Checks if the value of the field is `LowHysteresis`"]
    #[inline(always)]
    pub fn is_low_hysteresis(&self) -> bool {
        *self == HYST_A::LowHysteresis
    }
    #[doc = "Checks if the value of the field is `MediumHysteresis`"]
    #[inline(always)]
    pub fn is_medium_hysteresis(&self) -> bool {
        *self == HYST_A::MediumHysteresis
    }
    #[doc = "Checks if the value of the field is `HighHysteresis`"]
    #[inline(always)]
    pub fn is_high_hysteresis(&self) -> bool {
        *self == HYST_A::HighHysteresis
    }
}
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selection bits"]
pub type HYST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP2_CSR_SPEC, u8, HYST_A, 2, O>;
impl<'a, const O: u8> HYST_W<'a, O> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn no_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::NoHysteresis)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::LowHysteresis)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::MediumHysteresis)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high_hysteresis(self) -> &'a mut W {
        self.variant(HYST_A::HighHysteresis)
    }
}
#[doc = "Comparator 2 blanking source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLANKING_A {
    #[doc = "0: No blanking"]
    NoBlanking = 0,
    #[doc = "4: TIM15 OC1 selected as blanking source"]
    Tim1oc5 = 4,
}
impl From<BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BLANKING` reader - Comparator 2 blanking source selection bits"]
pub type BLANKING_R = crate::FieldReader<u8, BLANKING_A>;
impl BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLANKING_A> {
        match self.bits {
            0 => Some(BLANKING_A::NoBlanking),
            4 => Some(BLANKING_A::Tim1oc5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoBlanking`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == BLANKING_A::NoBlanking
    }
    #[doc = "Checks if the value of the field is `Tim1oc5`"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == BLANKING_A::Tim1oc5
    }
}
#[doc = "Field `BLANKING` writer - Comparator 2 blanking source selection bits"]
pub type BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, BLANKING_A, 3, O>;
impl<'a, const O: u8> BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(BLANKING_A::NoBlanking)
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut W {
        self.variant(BLANKING_A::Tim1oc5)
    }
}
#[doc = "Scaler bridge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRGEN_A {
    #[doc = "0: Scaler resistor bridge disabled"]
    Disabled = 0,
    #[doc = "1: Scaler resistor bridge enabled"]
    Enabled = 1,
}
impl From<BRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub type BRGEN_R = crate::BitReader<BRGEN_A>;
impl BRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRGEN_A {
        match self.bits {
            false => BRGEN_A::Disabled,
            true => BRGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN_A::Enabled
    }
}
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, BRGEN_A, O>;
impl<'a, const O: u8> BRGEN_W<'a, O> {
    #[doc = "Scaler resistor bridge disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRGEN_A::Disabled)
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRGEN_A::Enabled)
    }
}
#[doc = "Voltage scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCALEN_A {
    #[doc = "0: Voltage scaler disabled"]
    Disabled = 0,
    #[doc = "1: Voltage scaler enabled"]
    Enabled = 1,
}
impl From<SCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCALEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub type SCALEN_R = crate::BitReader<SCALEN_A>;
impl SCALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALEN_A {
        match self.bits {
            false => SCALEN_A::Disabled,
            true => SCALEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN_A::Enabled
    }
}
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, SCALEN_A, O>;
impl<'a, const O: u8> SCALEN_W<'a, O> {
    #[doc = "Voltage scaler disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCALEN_A::Disabled)
    }
    #[doc = "Voltage scaler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCALEN_A::Enabled)
    }
}
#[doc = "Comparator 2 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALUE_A {
    #[doc = "0: Comparator output is low"]
    Low = 0,
    #[doc = "1: Comparator output is high"]
    High = 1,
}
impl From<VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VALUE` reader - Comparator 2 output status bit"]
pub type VALUE_R = crate::BitReader<VALUE_A>;
impl VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALUE_A {
        match self.bits {
            false => VALUE_A::Low,
            true => VALUE_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VALUE_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VALUE_A::High
    }
}
#[doc = "Field `LOCK` writer - COMP2_CSR register lock bit"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<2> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<4> {
        INMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<7> {
        INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn winmode(&mut self) -> WINMODE_W<9> {
        WINMODE_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<15> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W<18> {
        BLANKING_W::new(self)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<22> {
        BRGEN_W::new(self)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<23> {
        SCALEN_W::new(self)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
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

#[doc = "Register `OPAMP2_CSR` reader"]
pub struct R(crate::R<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP2_CSR` writer"]
pub struct W(crate::W<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_CSR_SPEC>;
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
impl From<crate::W<OPAMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operational amplifier Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPAEN_A {
    #[doc = "0: OpAmp disabled"]
    Disabled = 0,
    #[doc = "1: OpAmp enabled"]
    Enabled = 1,
}
impl From<OPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OPAEN_R = crate::BitReader<OPAEN_A>;
impl OPAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPAEN_A {
        match self.bits {
            false => OPAEN_A::Disabled,
            true => OPAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAEN_A::Enabled
    }
}
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, OPAEN_A, O>;
impl<'a, const O: u8> OPAEN_W<'a, O> {
    #[doc = "OpAmp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAEN_A::Disabled)
    }
    #[doc = "OpAmp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAEN_A::Enabled)
    }
}
#[doc = "Operational amplifier Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPALPM_A {
    #[doc = "0: OpAmp in normal mode"]
    Normal = 0,
    #[doc = "1: OpAmp in low power mode"]
    Low = 1,
}
impl From<OPALPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPALPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPALPM` reader - Operational amplifier Low Power Mode"]
pub type OPALPM_R = crate::BitReader<OPALPM_A>;
impl OPALPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPALPM_A {
        match self.bits {
            false => OPALPM_A::Normal,
            true => OPALPM_A::Low,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OPALPM_A::Normal
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OPALPM_A::Low
    }
}
#[doc = "Field `OPALPM` writer - Operational amplifier Low Power Mode"]
pub type OPALPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, OPALPM_A, O>;
impl<'a, const O: u8> OPALPM_W<'a, O> {
    #[doc = "OpAmp in normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OPALPM_A::Normal)
    }
    #[doc = "OpAmp in low power mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OPALPM_A::Low)
    }
}
#[doc = "Operational amplifier PGA mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPAMODE_A {
    #[doc = "0: internal PGA diabled"]
    PgaDisabled = 0,
    #[doc = "2: internal PGA enabled, gain programmed in PGA_GAIN"]
    PgaEnabled = 2,
    #[doc = "3: internal follower"]
    Follower = 3,
}
impl From<OPAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPAMODE` reader - Operational amplifier PGA mode"]
pub type OPAMODE_R = crate::FieldReader<u8, OPAMODE_A>;
impl OPAMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAMODE_A> {
        match self.bits {
            0 => Some(OPAMODE_A::PgaDisabled),
            2 => Some(OPAMODE_A::PgaEnabled),
            3 => Some(OPAMODE_A::Follower),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PgaDisabled`"]
    #[inline(always)]
    pub fn is_pga_disabled(&self) -> bool {
        *self == OPAMODE_A::PgaDisabled
    }
    #[doc = "Checks if the value of the field is `PgaEnabled`"]
    #[inline(always)]
    pub fn is_pga_enabled(&self) -> bool {
        *self == OPAMODE_A::PgaEnabled
    }
    #[doc = "Checks if the value of the field is `Follower`"]
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == OPAMODE_A::Follower
    }
}
#[doc = "Field `OPAMODE` writer - Operational amplifier PGA mode"]
pub type OPAMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, OPAMODE_A, 2, O>;
impl<'a, const O: u8> OPAMODE_W<'a, O> {
    #[doc = "internal PGA diabled"]
    #[inline(always)]
    pub fn pga_disabled(self) -> &'a mut W {
        self.variant(OPAMODE_A::PgaDisabled)
    }
    #[doc = "internal PGA enabled, gain programmed in PGA_GAIN"]
    #[inline(always)]
    pub fn pga_enabled(self) -> &'a mut W {
        self.variant(OPAMODE_A::PgaEnabled)
    }
    #[doc = "internal follower"]
    #[inline(always)]
    pub fn follower(self) -> &'a mut W {
        self.variant(OPAMODE_A::Follower)
    }
}
#[doc = "Operational amplifier Programmable amplifier gain value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGA_GAIN_A {
    #[doc = "0: Gain 2"]
    Gain2 = 0,
    #[doc = "1: Gain 4"]
    Gain4 = 1,
    #[doc = "2: Gain 8"]
    Gain8 = 2,
    #[doc = "3: Gain 16"]
    Gain16 = 3,
}
impl From<PGA_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_R = crate::FieldReader<u8, PGA_GAIN_A>;
impl PGA_GAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGA_GAIN_A {
        match self.bits {
            0 => PGA_GAIN_A::Gain2,
            1 => PGA_GAIN_A::Gain4,
            2 => PGA_GAIN_A::Gain8,
            3 => PGA_GAIN_A::Gain16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Gain2`"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == PGA_GAIN_A::Gain2
    }
    #[doc = "Checks if the value of the field is `Gain4`"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == PGA_GAIN_A::Gain4
    }
    #[doc = "Checks if the value of the field is `Gain8`"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == PGA_GAIN_A::Gain8
    }
    #[doc = "Checks if the value of the field is `Gain16`"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == PGA_GAIN_A::Gain16
    }
}
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PGA_GAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_CSR_SPEC, u8, PGA_GAIN_A, 2, O>;
impl<'a, const O: u8> PGA_GAIN_W<'a, O> {
    #[doc = "Gain 2"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2)
    }
    #[doc = "Gain 4"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4)
    }
    #[doc = "Gain 8"]
    #[inline(always)]
    pub fn gain8(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8)
    }
    #[doc = "Gain 16"]
    #[inline(always)]
    pub fn gain16(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16)
    }
}
#[doc = "Inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VM_SEL_A {
    #[doc = "0: GPIO connectet to VINM"]
    Gpio = 0,
    #[doc = "1: Low leakage inputs connecte (only available in certen BGA cases"]
    LowLeakage = 1,
    #[doc = "2: OPAMP in PGA mode"]
    PgaMode = 2,
}
impl From<VM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VM_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VM_SEL_R = crate::FieldReader<u8, VM_SEL_A>;
impl VM_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VM_SEL_A> {
        match self.bits {
            0 => Some(VM_SEL_A::Gpio),
            1 => Some(VM_SEL_A::LowLeakage),
            2 => Some(VM_SEL_A::PgaMode),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Gpio`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == VM_SEL_A::Gpio
    }
    #[doc = "Checks if the value of the field is `LowLeakage`"]
    #[inline(always)]
    pub fn is_low_leakage(&self) -> bool {
        *self == VM_SEL_A::LowLeakage
    }
    #[doc = "Checks if the value of the field is `PgaMode`"]
    #[inline(always)]
    pub fn is_pga_mode(&self) -> bool {
        *self == VM_SEL_A::PgaMode
    }
}
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VM_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, VM_SEL_A, 2, O>;
impl<'a, const O: u8> VM_SEL_W<'a, O> {
    #[doc = "GPIO connectet to VINM"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(VM_SEL_A::Gpio)
    }
    #[doc = "Low leakage inputs connecte (only available in certen BGA cases"]
    #[inline(always)]
    pub fn low_leakage(self) -> &'a mut W {
        self.variant(VM_SEL_A::LowLeakage)
    }
    #[doc = "OPAMP in PGA mode"]
    #[inline(always)]
    pub fn pga_mode(self) -> &'a mut W {
        self.variant(VM_SEL_A::PgaMode)
    }
}
#[doc = "Non inverted input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VP_SEL_A {
    #[doc = "0: GPIO connectet to VINP"]
    Gpio = 0,
    #[doc = "1: DAC connected to VPINP"]
    Dac = 1,
}
impl From<VP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VP_SEL` reader - Non inverted input selection"]
pub type VP_SEL_R = crate::BitReader<VP_SEL_A>;
impl VP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VP_SEL_A {
        match self.bits {
            false => VP_SEL_A::Gpio,
            true => VP_SEL_A::Dac,
        }
    }
    #[doc = "Checks if the value of the field is `Gpio`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == VP_SEL_A::Gpio
    }
    #[doc = "Checks if the value of the field is `Dac`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == VP_SEL_A::Dac
    }
}
#[doc = "Field `VP_SEL` writer - Non inverted input selection"]
pub type VP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, VP_SEL_A, O>;
impl<'a, const O: u8> VP_SEL_W<'a, O> {
    #[doc = "GPIO connectet to VINP"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(VP_SEL_A::Gpio)
    }
    #[doc = "DAC connected to VPINP"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(VP_SEL_A::Dac)
    }
}
#[doc = "Calibration mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALON_A {
    #[doc = "0: Normal mode"]
    Disabled = 0,
    #[doc = "1: Calibration mode"]
    Enabled = 1,
}
impl From<CALON_A> for bool {
    #[inline(always)]
    fn from(variant: CALON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CALON_R = crate::BitReader<CALON_A>;
impl CALON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALON_A {
        match self.bits {
            false => CALON_A::Disabled,
            true => CALON_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALON_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALON_A::Enabled
    }
}
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, CALON_A, O>;
impl<'a, const O: u8> CALON_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALON_A::Disabled)
    }
    #[doc = "Calibration mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALON_A::Enabled)
    }
}
#[doc = "Calibration selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALSEL_A {
    #[doc = "0: 0.2V applied to OPAMP inputs during calibration"]
    Nmos = 0,
    #[doc = "1: VDDA-0.2V applied to OPAMP inputs during calibration\""]
    Pmos = 1,
}
impl From<CALSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CALSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CALSEL_R = crate::BitReader<CALSEL_A>;
impl CALSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALSEL_A {
        match self.bits {
            false => CALSEL_A::Nmos,
            true => CALSEL_A::Pmos,
        }
    }
    #[doc = "Checks if the value of the field is `Nmos`"]
    #[inline(always)]
    pub fn is_nmos(&self) -> bool {
        *self == CALSEL_A::Nmos
    }
    #[doc = "Checks if the value of the field is `Pmos`"]
    #[inline(always)]
    pub fn is_pmos(&self) -> bool {
        *self == CALSEL_A::Pmos
    }
}
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, CALSEL_A, O>;
impl<'a, const O: u8> CALSEL_W<'a, O> {
    #[doc = "0.2V applied to OPAMP inputs during calibration"]
    #[inline(always)]
    pub fn nmos(self) -> &'a mut W {
        self.variant(CALSEL_A::Nmos)
    }
    #[doc = "VDDA-0.2V applied to OPAMP inputs during calibration\""]
    #[inline(always)]
    pub fn pmos(self) -> &'a mut W {
        self.variant(CALSEL_A::Pmos)
    }
}
#[doc = "allows to switch from AOP offset trimmed values to AOP offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERTRIM_A {
    #[doc = "0: Factory trim used"]
    Factory = 0,
    #[doc = "1: User trim used"]
    User = 1,
}
impl From<USERTRIM_A> for bool {
    #[inline(always)]
    fn from(variant: USERTRIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset"]
pub type USERTRIM_R = crate::BitReader<USERTRIM_A>;
impl USERTRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USERTRIM_A {
        match self.bits {
            false => USERTRIM_A::Factory,
            true => USERTRIM_A::User,
        }
    }
    #[doc = "Checks if the value of the field is `Factory`"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == USERTRIM_A::Factory
    }
    #[doc = "Checks if the value of the field is `User`"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == USERTRIM_A::User
    }
}
#[doc = "Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset"]
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, USERTRIM_A, O>;
impl<'a, const O: u8> USERTRIM_W<'a, O> {
    #[doc = "Factory trim used"]
    #[inline(always)]
    pub fn factory(self) -> &'a mut W {
        self.variant(USERTRIM_A::Factory)
    }
    #[doc = "User trim used"]
    #[inline(always)]
    pub fn user(self) -> &'a mut W {
        self.variant(USERTRIM_A::User)
    }
}
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output"]
pub type CALOUT_R = crate::BitReader<bool>;
#[doc = "Field `CALOUT` writer - Operational amplifier calibration output"]
pub type CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    #[doc = "Bit 1 - Operational amplifier Low Power Mode"]
    #[inline(always)]
    pub fn opalpm(&mut self) -> OPALPM_W<1> {
        OPALPM_W::new(self)
    }
    #[doc = "Bits 2:3 - Operational amplifier PGA mode"]
    #[inline(always)]
    pub fn opamode(&mut self) -> OPAMODE_W<2> {
        OPAMODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<4> {
        PGA_GAIN_W::new(self)
    }
    #[doc = "Bits 8:9 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<8> {
        VM_SEL_W::new(self)
    }
    #[doc = "Bit 10 - Non inverted input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<10> {
        VP_SEL_W::new(self)
    }
    #[doc = "Bit 12 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<12> {
        CALON_W::new(self)
    }
    #[doc = "Bit 13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<13> {
        CALSEL_W::new(self)
    }
    #[doc = "Bit 14 - allows to switch from AOP offset trimmed values to AOP offset"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> USERTRIM_W<14> {
        USERTRIM_W::new(self)
    }
    #[doc = "Bit 15 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W<15> {
        CALOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP2 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp2_csr](index.html) module"]
pub struct OPAMP2_CSR_SPEC;
impl crate::RegisterSpec for OPAMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp2_csr::R](R) reader structure"]
impl crate::Readable for OPAMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp2_csr::W](W) writer structure"]
impl crate::Writable for OPAMP2_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for OPAMP2_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

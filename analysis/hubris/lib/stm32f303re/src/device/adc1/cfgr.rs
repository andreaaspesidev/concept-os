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
#[doc = "Field `AWD1CH` reader - AWDCH1CH"]
pub type AWD1CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWD1CH` writer - AWDCH1CH"]
pub type AWD1CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 5, O>;
#[doc = "JAUTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAUTO_A {
    #[doc = "0: Automatic injected group conversion disabled"]
    Disabled = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    Enabled = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAUTO` reader - JAUTO"]
pub type JAUTO_R = crate::BitReader<JAUTO_A>;
impl JAUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::Disabled,
            true => JAUTO_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO_A::Enabled
    }
}
#[doc = "Field `JAUTO` writer - JAUTO"]
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JAUTO_A, O>;
impl<'a, const O: u8> JAUTO_W<'a, O> {
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::Disabled)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::Enabled)
    }
}
#[doc = "JAWD1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on injected channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog 1 enabled on injected channels"]
    Enabled = 1,
}
impl From<JAWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub type JAWD1EN_R = crate::BitReader<JAWD1EN_A>;
impl JAWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAWD1EN_A {
        match self.bits {
            false => JAWD1EN_A::Disabled,
            true => JAWD1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN_A::Enabled
    }
}
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub type JAWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JAWD1EN_A, O>;
impl<'a, const O: u8> JAWD1EN_W<'a, O> {
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::Disabled)
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::Enabled)
    }
}
#[doc = "AWD1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on regular channels"]
    Disabled = 0,
    #[doc = "1: Analog watchdog 1 enabled on regular channels"]
    Enabled = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
impl AWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::Disabled,
            true => AWD1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN_A::Enabled
    }
}
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub type AWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, AWD1EN_A, O>;
impl<'a, const O: u8> AWD1EN_W<'a, O> {
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Disabled)
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Enabled)
    }
}
#[doc = "AWD1SGL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD1SGL_A {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    All = 0,
    #[doc = "1: Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    Single = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
impl AWD1SGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::All,
            true => AWD1SGL_A::Single,
        }
    }
    #[doc = "Checks if the value of the field is `All`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWD1SGL_A::All
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWD1SGL_A::Single
    }
}
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub type AWD1SGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, AWD1SGL_A, O>;
impl<'a, const O: u8> AWD1SGL_W<'a, O> {
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(AWD1SGL_A::All)
    }
    #[doc = "Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AWD1SGL_A::Single)
    }
}
#[doc = "JQM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JQM_A {
    #[doc = "0: JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    Mode0 = 0,
    #[doc = "1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    Mode1 = 1,
}
impl From<JQM_A> for bool {
    #[inline(always)]
    fn from(variant: JQM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQM` reader - JQM"]
pub type JQM_R = crate::BitReader<JQM_A>;
impl JQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQM_A {
        match self.bits {
            false => JQM_A::Mode0,
            true => JQM_A::Mode1,
        }
    }
    #[doc = "Checks if the value of the field is `Mode0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == JQM_A::Mode0
    }
    #[doc = "Checks if the value of the field is `Mode1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == JQM_A::Mode1
    }
}
#[doc = "Field `JQM` writer - JQM"]
pub type JQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JQM_A, O>;
impl<'a, const O: u8> JQM_W<'a, O> {
    #[doc = "JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(JQM_A::Mode0)
    }
    #[doc = "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(JQM_A::Mode1)
    }
}
#[doc = "JDISCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDISCEN_A {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    Enabled = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub type JDISCEN_R = crate::BitReader<JDISCEN_A>;
impl JDISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::Disabled,
            true => JDISCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN_A::Enabled
    }
}
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JDISCEN_A, O>;
impl<'a, const O: u8> JDISCEN_W<'a, O> {
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::Disabled)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::Enabled)
    }
}
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "DISCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    Disabled = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    Enabled = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISCEN` reader - DISCEN"]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::Disabled,
            true => DISCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::Enabled
    }
}
#[doc = "Field `DISCEN` writer - DISCEN"]
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, DISCEN_A, O>;
impl<'a, const O: u8> DISCEN_W<'a, O> {
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Disabled)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Enabled)
    }
}
#[doc = "AUTDLY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTDLY_A {
    #[doc = "0: Auto delayed conversion mode off"]
    Off = 0,
    #[doc = "1: Auto delayed conversion mode on"]
    On = 1,
}
impl From<AUTDLY_A> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub type AUTDLY_R = crate::BitReader<AUTDLY_A>;
impl AUTDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTDLY_A {
        match self.bits {
            false => AUTDLY_A::Off,
            true => AUTDLY_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTDLY_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == AUTDLY_A::On
    }
}
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub type AUTDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, AUTDLY_A, O>;
impl<'a, const O: u8> AUTDLY_W<'a, O> {
    #[doc = "Auto delayed conversion mode off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AUTDLY_A::Off)
    }
    #[doc = "Auto delayed conversion mode on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(AUTDLY_A::On)
    }
}
#[doc = "CONT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    Single = 0,
    #[doc = "1: Continuous conversion mode"]
    Continuous = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - CONT"]
pub type CONT_R = crate::BitReader<CONT_A>;
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::Single,
            true => CONT_A::Continuous,
        }
    }
    #[doc = "Checks if the value of the field is `Single`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::Single
    }
    #[doc = "Checks if the value of the field is `Continuous`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
#[doc = "Field `CONT` writer - CONT"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::Single)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
#[doc = "OVRMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRMOD_A {
    #[doc = "0: Preserve DR register when an overrun is detected"]
    Preserve = 0,
    #[doc = "1: Overwrite DR register when an overrun is detected"]
    Overwrite = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::Preserve,
            true => OVRMOD_A::Overwrite,
        }
    }
    #[doc = "Checks if the value of the field is `Preserve`"]
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD_A::Preserve
    }
    #[doc = "Checks if the value of the field is `Overwrite`"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD_A::Overwrite
    }
}
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub type OVRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, OVRMOD_A, O>;
impl<'a, const O: u8> OVRMOD_W<'a, O> {
    #[doc = "Preserve DR register when an overrun is detected"]
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::Preserve)
    }
    #[doc = "Overwrite DR register when an overrun is detected"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::Overwrite)
    }
}
#[doc = "EXTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    Disabled = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RisingEdge = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FallingEdge = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BothEdges = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTEN` reader - EXTEN"]
pub type EXTEN_R = crate::FieldReader<u8, EXTEN_A>;
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::Disabled,
            1 => EXTEN_A::RisingEdge,
            2 => EXTEN_A::FallingEdge,
            3 => EXTEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN_A::FallingEdge
    }
    #[doc = "Checks if the value of the field is `BothEdges`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN_A::BothEdges
    }
}
#[doc = "Field `EXTEN` writer - EXTEN"]
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, EXTEN_A, 2, O>;
impl<'a, const O: u8> EXTEN_W<'a, O> {
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::Disabled)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RisingEdge)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FallingEdge)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BothEdges)
    }
}
#[doc = "EXTSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "0: Timer 1 CC1 event"]
    Tim1Cc1 = 0,
    #[doc = "1: Timer 1 CC2 event"]
    Tim1Cc2 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    Tim1Cc3 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    Tim2Cc2 = 3,
    #[doc = "4: Timer 3 TRGO event"]
    Tim3Trgo = 4,
    #[doc = "6: EXTI line 11"]
    Exti11 = 6,
    #[doc = "7: HRTIM_ADCTRG1 event"]
    HrtimAdctrg1 = 7,
    #[doc = "8: HRTIM_ADCTRG3 event"]
    HrtimAdctrg3 = 8,
    #[doc = "9: Timer 1 TRGO event"]
    Tim1Trgo = 9,
    #[doc = "10: Timer 1 TRGO2 event"]
    Tim1Trgo2 = 10,
    #[doc = "11: Timer 2 TRGO event"]
    Tim2Trgo = 11,
    #[doc = "13: Timer 6 TRGO event"]
    Tim6Trgo = 13,
    #[doc = "14: Timer 15 TRGO event"]
    Tim15Trgo = 14,
    #[doc = "15: Timer 3 CC4 event"]
    Tim3Cc4 = 15,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTSEL` reader - EXTSEL"]
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            0 => Some(EXTSEL_A::Tim1Cc1),
            1 => Some(EXTSEL_A::Tim1Cc2),
            2 => Some(EXTSEL_A::Tim1Cc3),
            3 => Some(EXTSEL_A::Tim2Cc2),
            4 => Some(EXTSEL_A::Tim3Trgo),
            6 => Some(EXTSEL_A::Exti11),
            7 => Some(EXTSEL_A::HrtimAdctrg1),
            8 => Some(EXTSEL_A::HrtimAdctrg3),
            9 => Some(EXTSEL_A::Tim1Trgo),
            10 => Some(EXTSEL_A::Tim1Trgo2),
            11 => Some(EXTSEL_A::Tim2Trgo),
            13 => Some(EXTSEL_A::Tim6Trgo),
            14 => Some(EXTSEL_A::Tim15Trgo),
            15 => Some(EXTSEL_A::Tim3Cc4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Tim1Cc1`"]
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc1
    }
    #[doc = "Checks if the value of the field is `Tim1Cc2`"]
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc2
    }
    #[doc = "Checks if the value of the field is `Tim1Cc3`"]
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc3
    }
    #[doc = "Checks if the value of the field is `Tim2Cc2`"]
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        *self == EXTSEL_A::Tim2Cc2
    }
    #[doc = "Checks if the value of the field is `Tim3Trgo`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim3Trgo
    }
    #[doc = "Checks if the value of the field is `Exti11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL_A::Exti11
    }
    #[doc = "Checks if the value of the field is `HrtimAdctrg1`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg1(&self) -> bool {
        *self == EXTSEL_A::HrtimAdctrg1
    }
    #[doc = "Checks if the value of the field is `HrtimAdctrg3`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg3(&self) -> bool {
        *self == EXTSEL_A::HrtimAdctrg3
    }
    #[doc = "Checks if the value of the field is `Tim1Trgo`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim1Trgo
    }
    #[doc = "Checks if the value of the field is `Tim1Trgo2`"]
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL_A::Tim1Trgo2
    }
    #[doc = "Checks if the value of the field is `Tim2Trgo`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim2Trgo
    }
    #[doc = "Checks if the value of the field is `Tim6Trgo`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim6Trgo
    }
    #[doc = "Checks if the value of the field is `Tim15Trgo`"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim15Trgo
    }
    #[doc = "Checks if the value of the field is `Tim3Cc4`"]
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == EXTSEL_A::Tim3Cc4
    }
}
#[doc = "Field `EXTSEL` writer - EXTSEL"]
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, EXTSEL_A, 4, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Cc2)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3Trgo)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::Exti11)
    }
    #[doc = "HRTIM_ADCTRG1 event"]
    #[inline(always)]
    pub fn hrtim_adctrg1(self) -> &'a mut W {
        self.variant(EXTSEL_A::HrtimAdctrg1)
    }
    #[doc = "HRTIM_ADCTRG3 event"]
    #[inline(always)]
    pub fn hrtim_adctrg3(self) -> &'a mut W {
        self.variant(EXTSEL_A::HrtimAdctrg3)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Trgo)
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Trgo2)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Trgo)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim6Trgo)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim15Trgo)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3Cc4)
    }
}
#[doc = "ALIGN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_A {
    #[doc = "0: Right alignment"]
    Right = 0,
    #[doc = "1: Left alignment"]
    Left = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - ALIGN"]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::Right,
            true => ALIGN_A::Left,
        }
    }
    #[doc = "Checks if the value of the field is `Right`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::Right
    }
    #[doc = "Checks if the value of the field is `Left`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::Left
    }
}
#[doc = "Field `ALIGN` writer - ALIGN"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::Right)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::Left)
    }
}
#[doc = "RES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit"]
    Bits12 = 0,
    #[doc = "1: 10-bit"]
    Bits10 = 1,
    #[doc = "2: 8-bit"]
    Bits8 = 2,
    #[doc = "3: 6-bit"]
    Bits6 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RES` reader - RES"]
pub type RES_R = crate::FieldReader<u8, RES_A>;
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::Bits12,
            1 => RES_A::Bits10,
            2 => RES_A::Bits8,
            3 => RES_A::Bits6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Bits12`"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == RES_A::Bits12
    }
    #[doc = "Checks if the value of the field is `Bits10`"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == RES_A::Bits10
    }
    #[doc = "Checks if the value of the field is `Bits8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == RES_A::Bits8
    }
    #[doc = "Checks if the value of the field is `Bits6`"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == RES_A::Bits6
    }
}
#[doc = "Field `RES` writer - RES"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, RES_A, 2, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(RES_A::Bits12)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(RES_A::Bits10)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(RES_A::Bits8)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut W {
        self.variant(RES_A::Bits6)
    }
}
#[doc = "DMACFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMACFG_A {
    #[doc = "0: DMA One Shot Mode selected"]
    OneShot = 0,
    #[doc = "1: DMA circular mode selected"]
    Circular = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACFG` reader - DMACFG"]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::OneShot,
            true => DMACFG_A::Circular,
        }
    }
    #[doc = "Checks if the value of the field is `OneShot`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG_A::OneShot
    }
    #[doc = "Checks if the value of the field is `Circular`"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG_A::Circular
    }
}
#[doc = "Field `DMACFG` writer - DMACFG"]
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, DMACFG_A, O>;
impl<'a, const O: u8> DMACFG_W<'a, O> {
    #[doc = "DMA One Shot Mode selected"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::OneShot)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(DMACFG_A::Circular)
    }
}
#[doc = "DMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    Disabled = 0,
    #[doc = "1: DMA enabled"]
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awd1ch(&mut self) -> AWD1CH_W<26> {
        AWD1CH_W::new(self)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<25> {
        JAUTO_W::new(self)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<24> {
        JAWD1EN_W::new(self)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<23> {
        AWD1EN_W::new(self)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<22> {
        AWD1SGL_W::new(self)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<21> {
        JQM_W::new(self)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<20> {
        JDISCEN_W::new(self)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<17> {
        DISCNUM_W::new(self)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<14> {
        AUTDLY_W::new(self)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<6> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<5> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<1> {
        DMACFG_W::new(self)
    }
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
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

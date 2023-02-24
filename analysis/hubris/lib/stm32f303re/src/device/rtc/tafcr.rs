#[doc = "Register `TAFCR` reader"]
pub struct R(crate::R<TAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFCR` writer"]
pub struct W(crate::W<TAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFCR_SPEC>;
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
impl From<crate::W<TAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Tamper 1 detection enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1E_A {
    #[doc = "0: RTC_TAMPx input detection disabled"]
    Disabled = 0,
    #[doc = "1: RTC_TAMPx input detection enabled"]
    Enabled = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub type TAMP1E_R = crate::BitReader<TAMP1E_A>;
impl TAMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::Disabled,
            true => TAMP1E_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1E_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1E_A::Enabled
    }
}
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, TAMP1E_A, O>;
impl<'a, const O: u8> TAMP1E_W<'a, O> {
    #[doc = "RTC_TAMPx input detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::Disabled)
    }
    #[doc = "RTC_TAMPx input detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMP1E_A::Enabled)
    }
}
#[doc = "Active level for tamper 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMP1TRG_A {
    #[doc = "0: If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    RisingEdge = 0,
    #[doc = "1: If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    FallingEdge = 1,
}
impl From<TAMP1TRG_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub type TAMP1TRG_R = crate::BitReader<TAMP1TRG_A>;
impl TAMP1TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMP1TRG_A {
        match self.bits {
            false => TAMP1TRG_A::RisingEdge,
            true => TAMP1TRG_A::FallingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `RisingEdge`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TAMP1TRG_A::RisingEdge
    }
    #[doc = "Checks if the value of the field is `FallingEdge`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TAMP1TRG_A::FallingEdge
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, TAMP1TRG_A, O>;
impl<'a, const O: u8> TAMP1TRG_W<'a, O> {
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input staying low triggers a tamper detection event."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::RisingEdge)
    }
    #[doc = "If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT =\u{338} 00: RTC_TAMPx input falling edge triggers a tamper detection event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TAMP1TRG_A::FallingEdge)
    }
}
#[doc = "Tamper interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPIE_A {
    #[doc = "0: Tamper interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tamper interrupt enabled"]
    Enabled = 1,
}
impl From<TAMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader<TAMPIE_A>;
impl TAMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPIE_A {
        match self.bits {
            false => TAMPIE_A::Disabled,
            true => TAMPIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPIE_A::Enabled
    }
}
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, TAMPIE_A, O>;
impl<'a, const O: u8> TAMPIE_W<'a, O> {
    #[doc = "Tamper interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPIE_A::Disabled)
    }
    #[doc = "Tamper interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPIE_A::Enabled)
    }
}
#[doc = "Tamper 2 detection enable"]
pub use TAMP1E_A as TAMP2E_A;
#[doc = "Tamper 3 detection enable"]
pub use TAMP1E_A as TAMP3E_A;
#[doc = "Field `TAMP2E` reader - Tamper 2 detection enable"]
pub use TAMP1E_R as TAMP2E_R;
#[doc = "Field `TAMP3E` reader - Tamper 3 detection enable"]
pub use TAMP1E_R as TAMP3E_R;
#[doc = "Field `TAMP2E` writer - Tamper 2 detection enable"]
pub use TAMP1E_W as TAMP2E_W;
#[doc = "Field `TAMP3E` writer - Tamper 3 detection enable"]
pub use TAMP1E_W as TAMP3E_W;
#[doc = "Active level for tamper 2"]
pub use TAMP1TRG_A as TAMP2TRG_A;
#[doc = "Active level for tamper 3"]
pub use TAMP1TRG_A as TAMP3TRG_A;
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2"]
pub use TAMP1TRG_R as TAMP2TRG_R;
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3"]
pub use TAMP1TRG_R as TAMP3TRG_R;
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2"]
pub use TAMP1TRG_W as TAMP2TRG_W;
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3"]
pub use TAMP1TRG_W as TAMP3TRG_W;
#[doc = "Activate timestamp on tamper detection event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPTS_A {
    #[doc = "0: Tamper detection event does not cause a timestamp to be saved"]
    NoSave = 0,
    #[doc = "1: Save timestamp on tamper detection event"]
    Save = 1,
}
impl From<TAMPTS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader<TAMPTS_A>;
impl TAMPTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPTS_A {
        match self.bits {
            false => TAMPTS_A::NoSave,
            true => TAMPTS_A::Save,
        }
    }
    #[doc = "Checks if the value of the field is `NoSave`"]
    #[inline(always)]
    pub fn is_no_save(&self) -> bool {
        *self == TAMPTS_A::NoSave
    }
    #[doc = "Checks if the value of the field is `Save`"]
    #[inline(always)]
    pub fn is_save(&self) -> bool {
        *self == TAMPTS_A::Save
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, TAMPTS_A, O>;
impl<'a, const O: u8> TAMPTS_W<'a, O> {
    #[doc = "Tamper detection event does not cause a timestamp to be saved"]
    #[inline(always)]
    pub fn no_save(self) -> &'a mut W {
        self.variant(TAMPTS_A::NoSave)
    }
    #[doc = "Save timestamp on tamper detection event"]
    #[inline(always)]
    pub fn save(self) -> &'a mut W {
        self.variant(TAMPTS_A::Save)
    }
}
#[doc = "Tamper sampling frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFREQ_A {
    #[doc = "0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    Div32768 = 0,
    #[doc = "1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    Div16384 = 1,
    #[doc = "2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    Div8192 = 2,
    #[doc = "3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    Div4096 = 3,
    #[doc = "4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    Div2048 = 4,
    #[doc = "5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    Div1024 = 5,
    #[doc = "6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    Div512 = 6,
    #[doc = "7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    Div256 = 7,
}
impl From<TAMPFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TAMPFREQ_R = crate::FieldReader<u8, TAMPFREQ_A>;
impl TAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPFREQ_A {
        match self.bits {
            0 => TAMPFREQ_A::Div32768,
            1 => TAMPFREQ_A::Div16384,
            2 => TAMPFREQ_A::Div8192,
            3 => TAMPFREQ_A::Div4096,
            4 => TAMPFREQ_A::Div2048,
            5 => TAMPFREQ_A::Div1024,
            6 => TAMPFREQ_A::Div512,
            7 => TAMPFREQ_A::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == TAMPFREQ_A::Div32768
    }
    #[doc = "Checks if the value of the field is `Div16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == TAMPFREQ_A::Div16384
    }
    #[doc = "Checks if the value of the field is `Div8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == TAMPFREQ_A::Div8192
    }
    #[doc = "Checks if the value of the field is `Div4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == TAMPFREQ_A::Div4096
    }
    #[doc = "Checks if the value of the field is `Div2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == TAMPFREQ_A::Div2048
    }
    #[doc = "Checks if the value of the field is `Div1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == TAMPFREQ_A::Div1024
    }
    #[doc = "Checks if the value of the field is `Div512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == TAMPFREQ_A::Div512
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TAMPFREQ_A::Div256
    }
}
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TAMPFREQ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAFCR_SPEC, u8, TAMPFREQ_A, 3, O>;
impl<'a, const O: u8> TAMPFREQ_W<'a, O> {
    #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div32768)
    }
    #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div16384)
    }
    #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div8192)
    }
    #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div4096)
    }
    #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div2048)
    }
    #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div1024)
    }
    #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div512)
    }
    #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(TAMPFREQ_A::Div256)
    }
}
#[doc = "Tamper filter count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPFLT_A {
    #[doc = "0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    Immediate = 0,
    #[doc = "1: Tamper event is activated after 2 consecutive samples at the active level"]
    Samples2 = 1,
    #[doc = "2: Tamper event is activated after 4 consecutive samples at the active level"]
    Samples4 = 2,
    #[doc = "3: Tamper event is activated after 8 consecutive samples at the active level"]
    Samples8 = 3,
}
impl From<TAMPFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAMPFLT` reader - Tamper filter count"]
pub type TAMPFLT_R = crate::FieldReader<u8, TAMPFLT_A>;
impl TAMPFLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPFLT_A {
        match self.bits {
            0 => TAMPFLT_A::Immediate,
            1 => TAMPFLT_A::Samples2,
            2 => TAMPFLT_A::Samples4,
            3 => TAMPFLT_A::Samples8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Immediate`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TAMPFLT_A::Immediate
    }
    #[doc = "Checks if the value of the field is `Samples2`"]
    #[inline(always)]
    pub fn is_samples2(&self) -> bool {
        *self == TAMPFLT_A::Samples2
    }
    #[doc = "Checks if the value of the field is `Samples4`"]
    #[inline(always)]
    pub fn is_samples4(&self) -> bool {
        *self == TAMPFLT_A::Samples4
    }
    #[doc = "Checks if the value of the field is `Samples8`"]
    #[inline(always)]
    pub fn is_samples8(&self) -> bool {
        *self == TAMPFLT_A::Samples8
    }
}
#[doc = "Field `TAMPFLT` writer - Tamper filter count"]
pub type TAMPFLT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAFCR_SPEC, u8, TAMPFLT_A, 2, O>;
impl<'a, const O: u8> TAMPFLT_W<'a, O> {
    #[doc = "Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Immediate)
    }
    #[doc = "Tamper event is activated after 2 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples2(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Samples2)
    }
    #[doc = "Tamper event is activated after 4 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples4(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Samples4)
    }
    #[doc = "Tamper event is activated after 8 consecutive samples at the active level"]
    #[inline(always)]
    pub fn samples8(self) -> &'a mut W {
        self.variant(TAMPFLT_A::Samples8)
    }
}
#[doc = "Tamper precharge duration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TAMPPRCH_A {
    #[doc = "0: 1 RTCCLK cycle"]
    Cycles1 = 0,
    #[doc = "1: 2 RTCCLK cycles"]
    Cycles2 = 1,
    #[doc = "2: 4 RTCCLK cycles"]
    Cycles4 = 2,
    #[doc = "3: 8 RTCCLK cycles"]
    Cycles8 = 3,
}
impl From<TAMPPRCH_A> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TAMPPRCH` reader - Tamper precharge duration"]
pub type TAMPPRCH_R = crate::FieldReader<u8, TAMPPRCH_A>;
impl TAMPPRCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPPRCH_A {
        match self.bits {
            0 => TAMPPRCH_A::Cycles1,
            1 => TAMPPRCH_A::Cycles2,
            2 => TAMPPRCH_A::Cycles4,
            3 => TAMPPRCH_A::Cycles8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles1`"]
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH_A::Cycles1
    }
    #[doc = "Checks if the value of the field is `Cycles2`"]
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH_A::Cycles2
    }
    #[doc = "Checks if the value of the field is `Cycles4`"]
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH_A::Cycles4
    }
    #[doc = "Checks if the value of the field is `Cycles8`"]
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH_A::Cycles8
    }
}
#[doc = "Field `TAMPPRCH` writer - Tamper precharge duration"]
pub type TAMPPRCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TAFCR_SPEC, u8, TAMPPRCH_A, 2, O>;
impl<'a, const O: u8> TAMPPRCH_W<'a, O> {
    #[doc = "1 RTCCLK cycle"]
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles1)
    }
    #[doc = "2 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles2)
    }
    #[doc = "4 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles4)
    }
    #[doc = "8 RTCCLK cycles"]
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut W {
        self.variant(TAMPPRCH_A::Cycles8)
    }
}
#[doc = "TAMPER pull-up disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPPUDIS_A {
    #[doc = "0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    Enabled = 0,
    #[doc = "1: Disable precharge of RTC_TAMPx pins"]
    Disabled = 1,
}
impl From<TAMPPUDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPPUDIS` reader - TAMPER pull-up disable"]
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS_A>;
impl TAMPPUDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAMPPUDIS_A {
        match self.bits {
            false => TAMPPUDIS_A::Enabled,
            true => TAMPPUDIS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS_A::Disabled
    }
}
#[doc = "Field `TAMPPUDIS` writer - TAMPER pull-up disable"]
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, TAMPPUDIS_A, O>;
impl<'a, const O: u8> TAMPPUDIS_W<'a, O> {
    #[doc = "Precharge RTC_TAMPx pins before sampling (enable internal pull-up)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::Enabled)
    }
    #[doc = "Disable precharge of RTC_TAMPx pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TAMPPUDIS_A::Disabled)
    }
}
#[doc = "PC13 value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC13VALUE_A {
    #[doc = "0: If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic low"]
    Low = 0,
    #[doc = "1: If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic high"]
    High = 1,
}
impl From<PC13VALUE_A> for bool {
    #[inline(always)]
    fn from(variant: PC13VALUE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC13VALUE` reader - PC13 value"]
pub type PC13VALUE_R = crate::BitReader<PC13VALUE_A>;
impl PC13VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC13VALUE_A {
        match self.bits {
            false => PC13VALUE_A::Low,
            true => PC13VALUE_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PC13VALUE_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PC13VALUE_A::High
    }
}
#[doc = "Field `PC13VALUE` writer - PC13 value"]
pub type PC13VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, PC13VALUE_A, O>;
impl<'a, const O: u8> PC13VALUE_W<'a, O> {
    #[doc = "If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PC13VALUE_A::Low)
    }
    #[doc = "If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PC13VALUE_A::High)
    }
}
#[doc = "PC13 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PC13MODE_A {
    #[doc = "0: PCx is controlled by the GPIO configuration Register. Consequently PC15 is floating in Standby mode"]
    Floating = 0,
    #[doc = "1: PCx is forced to push-pull output if LSE is disabled"]
    PushPull = 1,
}
impl From<PC13MODE_A> for bool {
    #[inline(always)]
    fn from(variant: PC13MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC13MODE` reader - PC13 mode"]
pub type PC13MODE_R = crate::BitReader<PC13MODE_A>;
impl PC13MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PC13MODE_A {
        match self.bits {
            false => PC13MODE_A::Floating,
            true => PC13MODE_A::PushPull,
        }
    }
    #[doc = "Checks if the value of the field is `Floating`"]
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PC13MODE_A::Floating
    }
    #[doc = "Checks if the value of the field is `PushPull`"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == PC13MODE_A::PushPull
    }
}
#[doc = "Field `PC13MODE` writer - PC13 mode"]
pub type PC13MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, PC13MODE_A, O>;
impl<'a, const O: u8> PC13MODE_W<'a, O> {
    #[doc = "PCx is controlled by the GPIO configuration Register. Consequently PC15 is floating in Standby mode"]
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PC13MODE_A::Floating)
    }
    #[doc = "PCx is forced to push-pull output if LSE is disabled"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(PC13MODE_A::PushPull)
    }
}
#[doc = "PC 14 mode"]
pub use PC13MODE_A as PC14MODE_A;
#[doc = "PC15 mode"]
pub use PC13MODE_A as PC15MODE_A;
#[doc = "Field `PC14MODE` reader - PC 14 mode"]
pub use PC13MODE_R as PC14MODE_R;
#[doc = "Field `PC15MODE` reader - PC15 mode"]
pub use PC13MODE_R as PC15MODE_R;
#[doc = "Field `PC14MODE` writer - PC 14 mode"]
pub use PC13MODE_W as PC14MODE_W;
#[doc = "Field `PC15MODE` writer - PC15 mode"]
pub use PC13MODE_W as PC15MODE_W;
#[doc = "PC14 value"]
pub use PC13VALUE_A as PC14VALUE_A;
#[doc = "PC15 value"]
pub use PC13VALUE_A as PC15VALUE_A;
#[doc = "Field `PC14VALUE` reader - PC14 value"]
pub use PC13VALUE_R as PC14VALUE_R;
#[doc = "Field `PC15VALUE` reader - PC15 value"]
pub use PC13VALUE_R as PC15VALUE_R;
#[doc = "Field `PC14VALUE` writer - PC14 value"]
pub use PC13VALUE_W as PC14VALUE_W;
#[doc = "Field `PC15VALUE` writer - PC15 value"]
pub use PC13VALUE_W as PC15VALUE_W;
impl R {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - PC13 value"]
    #[inline(always)]
    pub fn pc13value(&self) -> PC13VALUE_R {
        PC13VALUE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&self) -> PC13MODE_R {
        PC13MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&self) -> PC14VALUE_R {
        PC14VALUE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PC 14 mode"]
    #[inline(always)]
    pub fn pc14mode(&self) -> PC14MODE_R {
        PC14MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&self) -> PC15VALUE_R {
        PC15VALUE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&self) -> PC15MODE_R {
        PC15MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<1> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<3> {
        TAMP2E_W::new(self)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<4> {
        TAMP2TRG_W::new(self)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<5> {
        TAMP3E_W::new(self)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<6> {
        TAMP3TRG_W::new(self)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<7> {
        TAMPTS_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<8> {
        TAMPFREQ_W::new(self)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<11> {
        TAMPFLT_W::new(self)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<13> {
        TAMPPRCH_W::new(self)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<15> {
        TAMPPUDIS_W::new(self)
    }
    #[doc = "Bit 18 - PC13 value"]
    #[inline(always)]
    pub fn pc13value(&mut self) -> PC13VALUE_W<18> {
        PC13VALUE_W::new(self)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&mut self) -> PC13MODE_W<19> {
        PC13MODE_W::new(self)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&mut self) -> PC14VALUE_W<20> {
        PC14VALUE_W::new(self)
    }
    #[doc = "Bit 21 - PC 14 mode"]
    #[inline(always)]
    pub fn pc14mode(&mut self) -> PC14MODE_W<21> {
        PC14MODE_W::new(self)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&mut self) -> PC15VALUE_W<22> {
        PC15VALUE_W::new(self)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&mut self) -> PC15MODE_W<23> {
        PC15MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafcr](index.html) module"]
pub struct TAFCR_SPEC;
impl crate::RegisterSpec for TAFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafcr::R](R) reader structure"]
impl crate::Readable for TAFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafcr::W](W) writer structure"]
impl crate::Writable for TAFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TAFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

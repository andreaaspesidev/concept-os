#[doc = "Register `ISTR` reader"]
pub struct R(crate::R<ISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTR` writer"]
pub struct W(crate::W<ISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTR_SPEC>;
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
impl From<crate::W<ISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP_ID` reader - Endpoint Identifier"]
pub type EP_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Direction of transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: data transmitted by the USB peripheral to the host PC"]
    To = 0,
    #[doc = "1: data received by the USB peripheral from the host PC"]
    From = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction of transaction"]
pub type DIR_R = crate::BitReader<DIR_A>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::To,
            true => DIR_A::From,
        }
    }
    #[doc = "Checks if the value of the field is `To`"]
    #[inline(always)]
    pub fn is_to(&self) -> bool {
        *self == DIR_A::To
    }
    #[doc = "Checks if the value of the field is `From`"]
    #[inline(always)]
    pub fn is_from(&self) -> bool {
        *self == DIR_A::From
    }
}
#[doc = "Expected start frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOF_A {
    #[doc = "1: an SOF packet is expected but not received"]
    ExpectedStartOfFrame = 1,
}
impl From<ESOF_A> for bool {
    #[inline(always)]
    fn from(variant: ESOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESOF` reader - Expected start frame"]
pub type ESOF_R = crate::BitReader<ESOF_A>;
impl ESOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ESOF_A> {
        match self.bits {
            true => Some(ESOF_A::ExpectedStartOfFrame),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ExpectedStartOfFrame`"]
    #[inline(always)]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOF_A::ExpectedStartOfFrame
    }
}
#[doc = "Field `ESOF` writer - Expected start frame"]
pub type ESOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, ESOF_A, O>;
impl<'a, const O: u8> ESOF_W<'a, O> {
    #[doc = "an SOF packet is expected but not received"]
    #[inline(always)]
    pub fn expected_start_of_frame(self) -> &'a mut W {
        self.variant(ESOF_A::ExpectedStartOfFrame)
    }
}
#[doc = "start of frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "1: beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    StartOfFrame = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOF` reader - start of frame"]
pub type SOF_R = crate::BitReader<SOF_A>;
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOF_A> {
        match self.bits {
            true => Some(SOF_A::StartOfFrame),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `StartOfFrame`"]
    #[inline(always)]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOF_A::StartOfFrame
    }
}
#[doc = "Field `SOF` writer - start of frame"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, SOF_A, O>;
impl<'a, const O: u8> SOF_W<'a, O> {
    #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    #[inline(always)]
    pub fn start_of_frame(self) -> &'a mut W {
        self.variant(SOF_A::StartOfFrame)
    }
}
#[doc = "reset request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_A {
    #[doc = "1: peripheral detects an active USB RESET signal at its inputs"]
    Reset = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - reset request"]
pub type RESET_R = crate::BitReader<RESET_A>;
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESET_A> {
        match self.bits {
            true => Some(RESET_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESET_A::Reset
    }
}
#[doc = "Field `RESET` writer - reset request"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, RESET_A, O>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "peripheral detects an active USB RESET signal at its inputs"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESET_A::Reset)
    }
}
#[doc = "Suspend mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSP_A {
    #[doc = "1: no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    Suspend = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSP` reader - Suspend mode request"]
pub type SUSP_R = crate::BitReader<SUSP_A>;
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUSP_A> {
        match self.bits {
            true => Some(SUSP_A::Suspend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Suspend`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == SUSP_A::Suspend
    }
}
#[doc = "Field `SUSP` writer - Suspend mode request"]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, SUSP_A, O>;
impl<'a, const O: u8> SUSP_W<'a, O> {
    #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(SUSP_A::Suspend)
    }
}
#[doc = "Wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUP_A {
    #[doc = "1: activity is detected that wakes up the USB peripheral"]
    Wakeup = 1,
}
impl From<WKUP_A> for bool {
    #[inline(always)]
    fn from(variant: WKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUP` reader - Wakeup"]
pub type WKUP_R = crate::BitReader<WKUP_A>;
impl WKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKUP_A> {
        match self.bits {
            true => Some(WKUP_A::Wakeup),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Wakeup`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUP_A::Wakeup
    }
}
#[doc = "Field `WKUP` writer - Wakeup"]
pub type WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, WKUP_A, O>;
impl<'a, const O: u8> WKUP_W<'a, O> {
    #[doc = "activity is detected that wakes up the USB peripheral"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(WKUP_A::Wakeup)
    }
}
#[doc = "Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_A {
    #[doc = "1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    Error = 1,
}
impl From<ERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - Error"]
pub type ERR_R = crate::BitReader<ERR_A>;
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERR_A> {
        match self.bits {
            true => Some(ERR_A::Error),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERR_A::Error
    }
}
#[doc = "Field `ERR` writer - Error"]
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, ERR_A, O>;
impl<'a, const O: u8> ERR_W<'a, O> {
    #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERR_A::Error)
    }
}
#[doc = "Packet memory area over / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVR_A {
    #[doc = "1: microcontroller has not been able to respond in time to an USB memory request"]
    Overrun = 1,
}
impl From<PMAOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PMAOVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAOVR` reader - Packet memory area over / underrun"]
pub type PMAOVR_R = crate::BitReader<PMAOVR_A>;
impl PMAOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMAOVR_A> {
        match self.bits {
            true => Some(PMAOVR_A::Overrun),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Overrun`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVR_A::Overrun
    }
}
#[doc = "Field `PMAOVR` writer - Packet memory area over / underrun"]
pub type PMAOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, PMAOVR_A, O>;
impl<'a, const O: u8> PMAOVR_W<'a, O> {
    #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(PMAOVR_A::Overrun)
    }
}
#[doc = "Correct transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTR_A {
    #[doc = "1: endpoint has successfully completed a transaction"]
    Completed = 1,
}
impl From<CTR_A> for bool {
    #[inline(always)]
    fn from(variant: CTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTR` reader - Correct transfer"]
pub type CTR_R = crate::BitReader<CTR_A>;
impl CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTR_A> {
        match self.bits {
            true => Some(CTR_A::Completed),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Completed`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == CTR_A::Completed
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Expected start frame"]
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W<8> {
        ESOF_W::new(self)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<9> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - reset request"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<10> {
        RESET_W::new(self)
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<11> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W<12> {
        WKUP_W::new(self)
    }
    #[doc = "Bit 13 - Error"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<13> {
        ERR_W::new(self)
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W<14> {
        PMAOVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](index.html) module"]
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [istr::R](R) reader structure"]
impl crate::Readable for ISTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [istr::W](W) writer structure"]
impl crate::Writable for ISTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISTR to value 0"]
impl crate::Resettable for ISTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

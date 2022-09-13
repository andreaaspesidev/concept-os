#[doc = "Register `RQR` reader"]
pub struct R(crate::R<RQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RQR` writer"]
pub struct W(crate::W<RQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQR_SPEC>;
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
impl From<crate::W<RQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRQ_A {
    #[doc = "1: Set the TXE flags. This allows to discard the transmit data"]
    Discard = 1,
}
impl From<TXFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRQ` reader - Transmit data flush request"]
pub type TXFRQ_R = crate::BitReader<TXFRQ_A>;
impl TXFRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFRQ_A> {
        match self.bits {
            true => Some(TXFRQ_A::Discard),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Discard`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == TXFRQ_A::Discard
    }
}
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, TXFRQ_A, O>;
impl<'a, const O: u8> TXFRQ_W<'a, O> {
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFRQ_A::Discard)
    }
}
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQ_A {
    #[doc = "1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    Discard = 1,
}
impl From<RXFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRQ` reader - Receive data flush request"]
pub type RXFRQ_R = crate::BitReader<RXFRQ_A>;
impl RXFRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFRQ_A> {
        match self.bits {
            true => Some(RXFRQ_A::Discard),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Discard`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        *self == RXFRQ_A::Discard
    }
}
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, RXFRQ_A, O>;
impl<'a, const O: u8> RXFRQ_W<'a, O> {
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_A::Discard)
    }
}
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQ_A {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    Mute = 1,
}
impl From<MMRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMRQ` reader - Mute mode request"]
pub type MMRQ_R = crate::BitReader<MMRQ_A>;
impl MMRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMRQ_A> {
        match self.bits {
            true => Some(MMRQ_A::Mute),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Mute`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == MMRQ_A::Mute
    }
}
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MMRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, MMRQ_A, O>;
impl<'a, const O: u8> MMRQ_W<'a, O> {
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_A::Mute)
    }
}
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQ_A {
    #[doc = "1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    Break = 1,
}
impl From<SBKRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKRQ` reader - Send break request"]
pub type SBKRQ_R = crate::BitReader<SBKRQ_A>;
impl SBKRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SBKRQ_A> {
        match self.bits {
            true => Some(SBKRQ_A::Break),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Break`"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBKRQ_A::Break
    }
}
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SBKRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, SBKRQ_A, O>;
impl<'a, const O: u8> SBKRQ_W<'a, O> {
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_A::Break)
    }
}
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRRQ_A {
    #[doc = "1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    Request = 1,
}
impl From<ABRRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRRQ` reader - Auto baud rate request"]
pub type ABRRQ_R = crate::BitReader<ABRRQ_A>;
impl ABRRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABRRQ_A> {
        match self.bits {
            true => Some(ABRRQ_A::Request),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Request`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        *self == ABRRQ_A::Request
    }
}
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub type ABRRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, ABRRQ_A, O>;
impl<'a, const O: u8> ABRRQ_W<'a, O> {
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(ABRRQ_A::Request)
    }
}
impl R {
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&self) -> TXFRQ_R {
        TXFRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&self) -> RXFRQ_R {
        RXFRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&self) -> MMRQ_R {
        MMRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&self) -> SBKRQ_R {
        SBKRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&self) -> ABRRQ_R {
        ABRRQ_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W<4> {
        TXFRQ_W::new(self)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W<3> {
        RXFRQ_W::new(self)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W<2> {
        MMRQ_W::new(self)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W<1> {
        SBKRQ_W::new(self)
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&mut self) -> ABRRQ_W<0> {
        ABRRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqr](index.html) module"]
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rqr::R](R) reader structure"]
impl crate::Readable for RQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rqr::W](W) writer structure"]
impl crate::Writable for RQR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

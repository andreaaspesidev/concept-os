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
pub enum TXFRQ_AW {
    #[doc = "1: Set the TXE flags. This allows to discard the transmit data"]
    Discard = 1,
}
impl From<TXFRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, TXFRQ_AW, O>;
impl<'a, const O: u8> TXFRQ_W<'a, O> {
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFRQ_AW::Discard)
    }
}
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQ_AW {
    #[doc = "1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    Discard = 1,
}
impl From<RXFRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RXFRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, RXFRQ_AW, O>;
impl<'a, const O: u8> RXFRQ_W<'a, O> {
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_AW::Discard)
    }
}
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQ_AW {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    Mute = 1,
}
impl From<MMRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MMRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, MMRQ_AW, O>;
impl<'a, const O: u8> MMRQ_W<'a, O> {
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_AW::Mute)
    }
}
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQ_AW {
    #[doc = "1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    Break = 1,
}
impl From<SBKRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SBKRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, SBKRQ_AW, O>;
impl<'a, const O: u8> SBKRQ_W<'a, O> {
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_AW::Break)
    }
}
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRRQ_AW {
    #[doc = "1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    Request = 1,
}
impl From<ABRRQ_AW> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub type ABRRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RQR_SPEC, ABRRQ_AW, O>;
impl<'a, const O: u8> ABRRQ_W<'a, O> {
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(ABRRQ_AW::Request)
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
#[doc = "Request register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqr](index.html) module"]
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
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

#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup from Stop mode clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUCF_AW {
    #[doc = "1: Clears the WUF flag in the ISR register"]
    Clear = 1,
}
impl From<WUCF_AW> for bool {
    #[inline(always)]
    fn from(variant: WUCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUCF` writer - Wakeup from Stop mode clear flag"]
pub type WUCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, WUCF_AW, O>;
impl<'a, const O: u8> WUCF_W<'a, O> {
    #[doc = "Clears the WUF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCF_AW::Clear)
    }
}
#[doc = "Character match clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMCF_AW {
    #[doc = "1: Clears the CMF flag in the ISR register"]
    Clear = 1,
}
impl From<CMCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CMCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMCF` writer - Character match clear flag"]
pub type CMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CMCF_AW, O>;
impl<'a, const O: u8> CMCF_W<'a, O> {
    #[doc = "Clears the CMF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCF_AW::Clear)
    }
}
#[doc = "CTS clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSCF_AW {
    #[doc = "1: Clears the CTSIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTSCF_AW> for bool {
    #[inline(always)]
    fn from(variant: CTSCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSCF` writer - CTS clear flag"]
pub type CTSCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, CTSCF_AW, O>;
impl<'a, const O: u8> CTSCF_W<'a, O> {
    #[doc = "Clears the CTSIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCF_AW::Clear)
    }
}
#[doc = "Transmission complete clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCF_AW {
    #[doc = "1: Clears the TC flag in the ISR register"]
    Clear = 1,
}
impl From<TCCF_AW> for bool {
    #[inline(always)]
    fn from(variant: TCCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCCF` writer - Transmission complete clear flag"]
pub type TCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, TCCF_AW, O>;
impl<'a, const O: u8> TCCF_W<'a, O> {
    #[doc = "Clears the TC flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCF_AW::Clear)
    }
}
#[doc = "Idle line detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECF_AW {
    #[doc = "1: Clears the IDLE flag in the ISR register"]
    Clear = 1,
}
impl From<IDLECF_AW> for bool {
    #[inline(always)]
    fn from(variant: IDLECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLECF` writer - Idle line detected clear flag"]
pub type IDLECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, IDLECF_AW, O>;
impl<'a, const O: u8> IDLECF_W<'a, O> {
    #[doc = "Clears the IDLE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECF_AW::Clear)
    }
}
#[doc = "Overrun error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORECF_AW {
    #[doc = "1: Clears the ORE flag in the ISR register"]
    Clear = 1,
}
impl From<ORECF_AW> for bool {
    #[inline(always)]
    fn from(variant: ORECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORECF` writer - Overrun error clear flag"]
pub type ORECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, ORECF_AW, O>;
impl<'a, const O: u8> ORECF_W<'a, O> {
    #[doc = "Clears the ORE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECF_AW::Clear)
    }
}
#[doc = "Noise detected clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCF_AW {
    #[doc = "1: Clears the NF flag in the ISR register"]
    Clear = 1,
}
impl From<NCF_AW> for bool {
    #[inline(always)]
    fn from(variant: NCF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCF` writer - Noise detected clear flag"]
pub type NCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, NCF_AW, O>;
impl<'a, const O: u8> NCF_W<'a, O> {
    #[doc = "Clears the NF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCF_AW::Clear)
    }
}
#[doc = "Framing error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FECF_AW {
    #[doc = "1: Clears the FE flag in the ISR register"]
    Clear = 1,
}
impl From<FECF_AW> for bool {
    #[inline(always)]
    fn from(variant: FECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FECF` writer - Framing error clear flag"]
pub type FECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, FECF_AW, O>;
impl<'a, const O: u8> FECF_W<'a, O> {
    #[doc = "Clears the FE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECF_AW::Clear)
    }
}
#[doc = "Parity error clear flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECF_AW {
    #[doc = "1: Clears the PE flag in the ISR register"]
    Clear = 1,
}
impl From<PECF_AW> for bool {
    #[inline(always)]
    fn from(variant: PECF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECF` writer - Parity error clear flag"]
pub type PECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, PECF_AW, O>;
impl<'a, const O: u8> PECF_W<'a, O> {
    #[doc = "Clears the PE flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECF_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<20> {
        WUCF_W::new(self)
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<17> {
        CMCF_W::new(self)
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<9> {
        CTSCF_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<6> {
        TCCF_W::new(self)
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<4> {
        IDLECF_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<3> {
        ORECF_W::new(self)
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W<2> {
        NCF_W::new(self)
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<1> {
        FECF_W::new(self)
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<0> {
        PECF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

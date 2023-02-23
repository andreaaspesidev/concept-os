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
#[doc = "Field `CRXBFF` writer - Clear receive buffer full flag"]
pub type CRXBFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CTXBEF` writer - Clear transmit buffer empty flag"]
pub type CTXBEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CRXBERF` writer - Clear receive CRC error flag"]
pub type CRXBERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CRXOVRF` writer - Clear receive overrun error flag"]
pub type CRXOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CTXUNRF` writer - Clear transmit underrun error flag"]
pub type CTXUNRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CTCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CSRF` writer - Clear slave resume flag"]
pub type CSRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear receive buffer full flag"]
    #[inline(always)]
    pub fn crxbff(&mut self) -> CRXBFF_W<0> {
        CRXBFF_W::new(self)
    }
    #[doc = "Bit 1 - Clear transmit buffer empty flag"]
    #[inline(always)]
    pub fn ctxbef(&mut self) -> CTXBEF_W<1> {
        CTXBEF_W::new(self)
    }
    #[doc = "Bit 2 - Clear receive CRC error flag"]
    #[inline(always)]
    pub fn crxberf(&mut self) -> CRXBERF_W<2> {
        CRXBERF_W::new(self)
    }
    #[doc = "Bit 3 - Clear receive overrun error flag"]
    #[inline(always)]
    pub fn crxovrf(&mut self) -> CRXOVRF_W<3> {
        CRXOVRF_W::new(self)
    }
    #[doc = "Bit 4 - Clear transmit underrun error flag"]
    #[inline(always)]
    pub fn ctxunrf(&mut self) -> CTXUNRF_W<4> {
        CTXUNRF_W::new(self)
    }
    #[doc = "Bit 7 - Clear transfer complete flag"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W<7> {
        CTCF_W::new(self)
    }
    #[doc = "Bit 8 - Clear slave resume flag"]
    #[inline(always)]
    pub fn csrf(&mut self) -> CSRF_W<8> {
        CSRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SWPMI Interrupt Flag Clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
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

#[doc = "Register `AR` writer"]
pub struct W(crate::W<AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AR_SPEC>;
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
impl From<crate::W<AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAR` writer - Flash address"]
pub type FAR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Flash address"]
    #[inline(always)]
    pub fn far(&mut self) -> FAR_W<0> {
        FAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Flash address register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ar](index.html) module"]
pub struct AR_SPEC;
impl crate::RegisterSpec for AR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ar::W](W) writer structure"]
impl crate::Writable for AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

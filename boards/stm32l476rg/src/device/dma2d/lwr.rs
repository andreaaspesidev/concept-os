#[doc = "Register `LWR` reader"]
pub struct R(crate::R<LWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LWR` writer"]
pub struct W(crate::W<LWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LWR_SPEC>;
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
impl From<crate::W<LWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LW` reader - Line watermark"]
pub type LW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LW` writer - Line watermark"]
pub type LW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LWR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Line watermark"]
    #[inline(always)]
    pub fn lw(&self) -> LW_R {
        LW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Line watermark"]
    #[inline(always)]
    pub fn lw(&mut self) -> LW_W<0> {
        LW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "line watermark register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lwr](index.html) module"]
pub struct LWR_SPEC;
impl crate::RegisterSpec for LWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lwr::R](R) reader structure"]
impl crate::Readable for LWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lwr::W](W) writer structure"]
impl crate::Writable for LWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LWR to value 0"]
impl crate::Resettable for LWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

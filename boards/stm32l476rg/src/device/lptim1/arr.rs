#[doc = "Register `ARR` reader"]
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARR` writer"]
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - Auto reload value"]
pub type ARR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARR` writer - Auto reload value"]
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Auto reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto reload value"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Autoreload Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](index.html) module"]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arr::R](R) reader structure"]
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arr::W](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARR to value 0x01"]
impl crate::Resettable for ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

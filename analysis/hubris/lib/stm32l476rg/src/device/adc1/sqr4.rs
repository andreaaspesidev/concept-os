#[doc = "Register `SQR4` reader"]
pub struct R(crate::R<SQR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR4` writer"]
pub struct W(crate::W<SQR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR4_SPEC>;
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
impl From<crate::W<SQR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ16` reader - SQ16"]
pub type SQ16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ16` writer - SQ16"]
pub type SQ16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ15` reader - SQ15"]
pub type SQ15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ15` writer - SQ15"]
pub type SQ15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W<6> {
        SQ16_W::new(self)
    }
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W<0> {
        SQ15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr4](index.html) module"]
pub struct SQR4_SPEC;
impl crate::RegisterSpec for SQR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr4::R](R) reader structure"]
impl crate::Readable for SQR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr4::W](W) writer structure"]
impl crate::Writable for SQR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR4 to value 0"]
impl crate::Resettable for SQR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `SQR2` reader"]
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR2` writer"]
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ9` reader - SQ9"]
pub type SQ9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ9` writer - SQ9"]
pub type SQ9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ8` reader - SQ8"]
pub type SQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ8` writer - SQ8"]
pub type SQ8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ7` reader - SQ7"]
pub type SQ7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ7` writer - SQ7"]
pub type SQ7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ6` reader - SQ6"]
pub type SQ6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ6` writer - SQ6"]
pub type SQ6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ5` reader - SQ5"]
pub type SQ5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ5` writer - SQ5"]
pub type SQ5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 24:28 - SQ9"]
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ8"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ7"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ6"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - SQ5"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - SQ9"]
    #[inline(always)]
    pub fn sq9(&mut self) -> SQ9_W<24> {
        SQ9_W::new(self)
    }
    #[doc = "Bits 18:22 - SQ8"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W<18> {
        SQ8_W::new(self)
    }
    #[doc = "Bits 12:16 - SQ7"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W<12> {
        SQ7_W::new(self)
    }
    #[doc = "Bits 6:10 - SQ6"]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W<6> {
        SQ6_W::new(self)
    }
    #[doc = "Bits 0:4 - SQ5"]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W<0> {
        SQ5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](index.html) module"]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr2::R](R) reader structure"]
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr2::W](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

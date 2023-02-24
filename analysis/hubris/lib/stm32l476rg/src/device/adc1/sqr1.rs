#[doc = "Register `SQR1` reader"]
pub struct R(crate::R<SQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR1` writer"]
pub struct W(crate::W<SQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR1_SPEC>;
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
impl From<crate::W<SQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ4` reader - SQ4"]
pub type SQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ4` writer - SQ4"]
pub type SQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ3` reader - SQ3"]
pub type SQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ3` writer - SQ3"]
pub type SQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ2` reader - SQ2"]
pub type SQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ2` writer - SQ2"]
pub type SQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `SQ1` reader - SQ1"]
pub type SQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQ1` writer - SQ1"]
pub type SQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `L` reader - Regular channel sequence length"]
pub type L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L` writer - Regular channel sequence length"]
pub type L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 24:28 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - SQ4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W<24> {
        SQ4_W::new(self)
    }
    #[doc = "Bits 18:22 - SQ3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W<18> {
        SQ3_W::new(self)
    }
    #[doc = "Bits 12:16 - SQ2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W<12> {
        SQ2_W::new(self)
    }
    #[doc = "Bits 6:10 - SQ1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W<6> {
        SQ1_W::new(self)
    }
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W<0> {
        L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](index.html) module"]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr1::R](R) reader structure"]
impl crate::Readable for SQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr1::W](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

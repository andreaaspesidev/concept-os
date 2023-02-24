#[doc = "Register `JSQR` reader"]
pub struct R(crate::R<JSQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JSQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JSQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JSQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JSQR` writer"]
pub struct W(crate::W<JSQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JSQR_SPEC>;
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
impl From<crate::W<JSQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JSQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JSQ4` reader - JSQ4"]
pub type JSQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ4` writer - JSQ4"]
pub type JSQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JSQ3` reader - JSQ3"]
pub type JSQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ3` writer - JSQ3"]
pub type JSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JSQ2` reader - JSQ2"]
pub type JSQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ2` writer - JSQ2"]
pub type JSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JSQ1` reader - JSQ1"]
pub type JSQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSQ1` writer - JSQ1"]
pub type JSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JEXTEN` reader - JEXTEN"]
pub type JEXTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTEN` writer - JEXTEN"]
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 2, O>;
#[doc = "Field `JEXTSEL` reader - JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JEXTSEL` writer - JEXTSEL"]
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 4, O>;
#[doc = "Field `JL` reader - JL"]
pub type JL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JL` writer - JL"]
pub type JL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 26:30 - JSQ4"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - JSQ3"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - JSQ2"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - JSQ1"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 2:5 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 26:30 - JSQ4"]
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ4_W<26> {
        JSQ4_W::new(self)
    }
    #[doc = "Bits 20:24 - JSQ3"]
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ3_W<20> {
        JSQ3_W::new(self)
    }
    #[doc = "Bits 14:18 - JSQ2"]
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ2_W<14> {
        JSQ2_W::new(self)
    }
    #[doc = "Bits 8:12 - JSQ1"]
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ1_W<8> {
        JSQ1_W::new(self)
    }
    #[doc = "Bits 6:7 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<6> {
        JEXTEN_W::new(self)
    }
    #[doc = "Bits 2:5 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<2> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<0> {
        JL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jsqr](index.html) module"]
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jsqr::R](R) reader structure"]
impl crate::Readable for JSQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jsqr::W](W) writer structure"]
impl crate::Writable for JSQR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

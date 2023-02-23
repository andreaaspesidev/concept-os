#[doc = "Register `SHHR` reader"]
pub struct R(crate::R<SHHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHHR` writer"]
pub struct W(crate::W<SHHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHHR_SPEC>;
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
impl From<crate::W<SHHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THOLD1` reader - DAC Channel 1 hold Time"]
pub type THOLD1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THOLD1` writer - DAC Channel 1 hold Time"]
pub type THOLD1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SHHR_SPEC, u16, u16, 10, O>;
#[doc = "Field `THOLD2` reader - DAC Channel 2 hold time"]
pub type THOLD2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `THOLD2` writer - DAC Channel 2 hold time"]
pub type THOLD2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SHHR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time"]
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time"]
    #[inline(always)]
    pub fn thold2(&self) -> THOLD2_R {
        THOLD2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time"]
    #[inline(always)]
    pub fn thold1(&mut self) -> THOLD1_W<0> {
        THOLD1_W::new(self)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time"]
    #[inline(always)]
    pub fn thold2(&mut self) -> THOLD2_W<16> {
        THOLD2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample and Hold hold time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shhr](index.html) module"]
pub struct SHHR_SPEC;
impl crate::RegisterSpec for SHHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shhr::R](R) reader structure"]
impl crate::Readable for SHHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shhr::W](W) writer structure"]
impl crate::Writable for SHHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHHR to value 0x0001_0001"]
impl crate::Resettable for SHHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}

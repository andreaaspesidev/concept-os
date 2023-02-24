#[doc = "Register `SHRR` reader"]
pub struct R(crate::R<SHRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHRR` writer"]
pub struct W(crate::W<SHRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHRR_SPEC>;
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
impl From<crate::W<SHRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TREFRESH1` reader - DAC Channel 1 refresh Time"]
pub type TREFRESH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TREFRESH1` writer - DAC Channel 1 refresh Time"]
pub type TREFRESH1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SHRR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TREFRESH2` reader - DAC Channel 2 refresh Time"]
pub type TREFRESH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TREFRESH2` writer - DAC Channel 2 refresh Time"]
pub type TREFRESH2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SHRR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time"]
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time"]
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time"]
    #[inline(always)]
    pub fn trefresh1(&mut self) -> TREFRESH1_W<0> {
        TREFRESH1_W::new(self)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time"]
    #[inline(always)]
    pub fn trefresh2(&mut self) -> TREFRESH2_W<16> {
        TREFRESH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample and Hold refresh time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shrr](index.html) module"]
pub struct SHRR_SPEC;
impl crate::RegisterSpec for SHRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shrr::R](R) reader structure"]
impl crate::Readable for SHRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shrr::W](W) writer structure"]
impl crate::Writable for SHRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHRR to value 0x01"]
impl crate::Resettable for SHRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

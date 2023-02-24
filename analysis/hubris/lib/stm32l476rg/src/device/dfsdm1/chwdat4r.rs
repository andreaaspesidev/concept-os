#[doc = "Register `CHWDAT4R` reader"]
pub struct R(crate::R<CHWDAT4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHWDAT4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHWDAT4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHWDAT4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHWDAT4R` writer"]
pub struct W(crate::W<CHWDAT4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHWDAT4R_SPEC>;
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
impl From<crate::W<CHWDAT4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHWDAT4R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDATA` reader - WDATA"]
pub type WDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDATA` writer - WDATA"]
pub type WDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHWDAT4R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W<0> {
        WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHWDAT4R\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chwdat4r](index.html) module"]
pub struct CHWDAT4R_SPEC;
impl crate::RegisterSpec for CHWDAT4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chwdat4r::R](R) reader structure"]
impl crate::Readable for CHWDAT4R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chwdat4r::W](W) writer structure"]
impl crate::Writable for CHWDAT4R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHWDAT4R to value 0"]
impl crate::Resettable for CHWDAT4R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

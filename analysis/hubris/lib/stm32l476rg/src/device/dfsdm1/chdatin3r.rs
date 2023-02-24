#[doc = "Register `CHDATIN3R` reader"]
pub struct R(crate::R<CHDATIN3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDATIN3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDATIN3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDATIN3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHDATIN3R` writer"]
pub struct W(crate::W<CHDATIN3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDATIN3R_SPEC>;
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
impl From<crate::W<CHDATIN3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDATIN3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDAT1` reader - INDAT1"]
pub type INDAT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INDAT1` writer - INDAT1"]
pub type INDAT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHDATIN3R_SPEC, u16, u16, 16, O>;
#[doc = "Field `INDAT0` reader - INDAT0"]
pub type INDAT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INDAT0` writer - INDAT0"]
pub type INDAT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHDATIN3R_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - INDAT1"]
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W<16> {
        INDAT1_W::new(self)
    }
    #[doc = "Bits 0:15 - INDAT0"]
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W<0> {
        INDAT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHDATIN3R\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdatin3r](index.html) module"]
pub struct CHDATIN3R_SPEC;
impl crate::RegisterSpec for CHDATIN3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdatin3r::R](R) reader structure"]
impl crate::Readable for CHDATIN3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chdatin3r::W](W) writer structure"]
impl crate::Writable for CHDATIN3R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHDATIN3R to value 0"]
impl crate::Resettable for CHDATIN3R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

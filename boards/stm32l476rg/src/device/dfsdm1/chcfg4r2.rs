#[doc = "Register `CHCFG4R2` reader"]
pub struct R(crate::R<CHCFG4R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCFG4R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCFG4R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCFG4R2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCFG4R2` writer"]
pub struct W(crate::W<CHCFG4R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCFG4R2_SPEC>;
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
impl From<crate::W<CHCFG4R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCFG4R2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - OFFSET"]
pub type OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET` writer - OFFSET"]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCFG4R2_SPEC, u32, u32, 24, O>;
#[doc = "Field `DTRBS` reader - DTRBS"]
pub type DTRBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTRBS` writer - DTRBS"]
pub type DTRBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCFG4R2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&self) -> DTRBS_R {
        DTRBS_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - OFFSET"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W<8> {
        OFFSET_W::new(self)
    }
    #[doc = "Bits 3:7 - DTRBS"]
    #[inline(always)]
    pub fn dtrbs(&mut self) -> DTRBS_W<3> {
        DTRBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHCFG4R2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg4r2](index.html) module"]
pub struct CHCFG4R2_SPEC;
impl crate::RegisterSpec for CHCFG4R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chcfg4r2::R](R) reader structure"]
impl crate::Readable for CHCFG4R2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chcfg4r2::W](W) writer structure"]
impl crate::Writable for CHCFG4R2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCFG4R2 to value 0"]
impl crate::Resettable for CHCFG4R2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

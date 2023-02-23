#[doc = "Register `DFSDM0_JCHGR` reader"]
pub struct R(crate::R<DFSDM0_JCHGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM0_JCHGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM0_JCHGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM0_JCHGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM0_JCHGR` writer"]
pub struct W(crate::W<DFSDM0_JCHGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM0_JCHGR_SPEC>;
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
impl From<crate::W<DFSDM0_JCHGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM0_JCHGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JCHG` reader - Injected channel group selection"]
pub type JCHG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JCHG` writer - Injected channel group selection"]
pub type JCHG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM0_JCHGR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&mut self) -> JCHG_W<0> {
        JCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_jchgr](index.html) module"]
pub struct DFSDM0_JCHGR_SPEC;
impl crate::RegisterSpec for DFSDM0_JCHGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm0_jchgr::R](R) reader structure"]
impl crate::Readable for DFSDM0_JCHGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm0_jchgr::W](W) writer structure"]
impl crate::Writable for DFSDM0_JCHGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM0_JCHGR to value 0x01"]
impl crate::Resettable for DFSDM0_JCHGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

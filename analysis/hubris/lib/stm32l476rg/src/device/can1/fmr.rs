#[doc = "Register `FMR` reader"]
pub struct R(crate::R<FMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMR` writer"]
pub struct W(crate::W<FMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMR_SPEC>;
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
impl From<crate::W<FMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINIT` reader - Filter initialization mode"]
pub type FINIT_R = crate::BitReader<bool>;
#[doc = "Field `FINIT` writer - Filter initialization mode"]
pub type FINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMR_SPEC, bool, O>;
#[doc = "Field `CANSB` reader - CAN start bank"]
pub type CANSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CANSB` writer - CAN start bank"]
pub type CANSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - Filter initialization mode"]
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - CAN start bank"]
    #[inline(always)]
    pub fn cansb(&self) -> CANSB_R {
        CANSB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter initialization mode"]
    #[inline(always)]
    pub fn finit(&mut self) -> FINIT_W<0> {
        FINIT_W::new(self)
    }
    #[doc = "Bits 8:13 - CAN start bank"]
    #[inline(always)]
    pub fn cansb(&mut self) -> CANSB_W<8> {
        CANSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter master register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmr](index.html) module"]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmr::R](R) reader structure"]
impl crate::Readable for FMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmr::W](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMR to value 0x2a1c_0e01"]
impl crate::Resettable for FMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2a1c_0e01
    }
}

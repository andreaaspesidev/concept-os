#[doc = "Register `OPAMP1_OTR` reader"]
pub struct R(crate::R<OPAMP1_OTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP1_OTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP1_OTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP1_OTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP1_OTR` writer"]
pub struct W(crate::W<OPAMP1_OTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP1_OTR_SPEC>;
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
impl From<crate::W<OPAMP1_OTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP1_OTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TRIMOFFSETN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TRIMOFFSETN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP1_OTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TRIMOFFSETP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TRIMOFFSETP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP1_OTR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<0> {
        TRIMOFFSETN_W::new(self)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<8> {
        TRIMOFFSETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP1 offset trimming register in normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_otr](index.html) module"]
pub struct OPAMP1_OTR_SPEC;
impl crate::RegisterSpec for OPAMP1_OTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp1_otr::R](R) reader structure"]
impl crate::Readable for OPAMP1_OTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp1_otr::W](W) writer structure"]
impl crate::Writable for OPAMP1_OTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP1_OTR to value 0"]
impl crate::Resettable for OPAMP1_OTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

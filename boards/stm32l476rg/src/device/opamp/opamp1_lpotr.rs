#[doc = "Register `OPAMP1_LPOTR` reader"]
pub struct R(crate::R<OPAMP1_LPOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP1_LPOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP1_LPOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP1_LPOTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP1_LPOTR` writer"]
pub struct W(crate::W<OPAMP1_LPOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP1_LPOTR_SPEC>;
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
impl From<crate::W<OPAMP1_LPOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP1_LPOTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIMLPOFFSETN` reader - Trim for NMOS differential pairs"]
pub type TRIMLPOFFSETN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMLPOFFSETN` writer - Trim for NMOS differential pairs"]
pub type TRIMLPOFFSETN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP1_LPOTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIMLPOFFSETP` reader - Trim for PMOS differential pairs"]
pub type TRIMLPOFFSETP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMLPOFFSETP` writer - Trim for PMOS differential pairs"]
pub type TRIMLPOFFSETP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP1_LPOTR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TRIMLPOFFSETN_R {
        TRIMLPOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TRIMLPOFFSETP_R {
        TRIMLPOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for NMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W<0> {
        TRIMLPOFFSETN_W::new(self)
    }
    #[doc = "Bits 8:12 - Trim for PMOS differential pairs"]
    #[inline(always)]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W<8> {
        TRIMLPOFFSETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP1 offset trimming register in low-power mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp1_lpotr](index.html) module"]
pub struct OPAMP1_LPOTR_SPEC;
impl crate::RegisterSpec for OPAMP1_LPOTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp1_lpotr::R](R) reader structure"]
impl crate::Readable for OPAMP1_LPOTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp1_lpotr::W](W) writer structure"]
impl crate::Writable for OPAMP1_LPOTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP1_LPOTR to value 0"]
impl crate::Resettable for OPAMP1_LPOTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

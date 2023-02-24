#[doc = "Register `VDSL` reader"]
pub struct R(crate::R<VDSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDSL` writer"]
pub struct W(crate::W<VDSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDSL_SPEC>;
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
impl From<crate::W<VDSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENG` reader - Non-volatile data segment length"]
pub type LENG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LENG` writer - Non-volatile data segment length"]
pub type LENG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, VDSL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&mut self) -> LENG_W<6> {
        LENG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Volatile data segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdsl](index.html) module"]
pub struct VDSL_SPEC;
impl crate::RegisterSpec for VDSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdsl::R](R) reader structure"]
impl crate::Readable for VDSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdsl::W](W) writer structure"]
impl crate::Writable for VDSL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDSL to value 0"]
impl crate::Resettable for VDSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

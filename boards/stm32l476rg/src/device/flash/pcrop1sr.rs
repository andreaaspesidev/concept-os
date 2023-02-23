#[doc = "Register `PCROP1SR` reader"]
pub struct R(crate::R<PCROP1SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1SR` writer"]
pub struct W(crate::W<PCROP1SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1SR_SPEC>;
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
impl From<crate::W<PCROP1SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1_STRT` reader - Bank 1 PCROP area start offset"]
pub type PCROP1_STRT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCROP1_STRT` writer - Bank 1 PCROP area start offset"]
pub type PCROP1_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP1SR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&self) -> PCROP1_STRT_R {
        PCROP1_STRT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&mut self) -> PCROP1_STRT_W<0> {
        PCROP1_STRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 PCROP Start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1sr](index.html) module"]
pub struct PCROP1SR_SPEC;
impl crate::RegisterSpec for PCROP1SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1sr::R](R) reader structure"]
impl crate::Readable for PCROP1SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1sr::W](W) writer structure"]
impl crate::Writable for PCROP1SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP1SR to value 0xffff_0000"]
impl crate::Resettable for PCROP1SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}

#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCEIC` reader - Max count error interrupt clear"]
pub type MCEIC_R = crate::BitReader<bool>;
#[doc = "Field `MCEIC` writer - Max count error interrupt clear"]
pub type MCEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `EOAIC` reader - End of acquisition interrupt clear"]
pub type EOAIC_R = crate::BitReader<bool>;
#[doc = "Field `EOAIC` writer - End of acquisition interrupt clear"]
pub type EOAIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Max count error interrupt clear"]
    #[inline(always)]
    pub fn mceic(&self) -> MCEIC_R {
        MCEIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - End of acquisition interrupt clear"]
    #[inline(always)]
    pub fn eoaic(&self) -> EOAIC_R {
        EOAIC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Max count error interrupt clear"]
    #[inline(always)]
    pub fn mceic(&mut self) -> MCEIC_W<1> {
        MCEIC_W::new(self)
    }
    #[doc = "Bit 0 - End of acquisition interrupt clear"]
    #[inline(always)]
    pub fn eoaic(&mut self) -> EOAIC_W<0> {
        EOAIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

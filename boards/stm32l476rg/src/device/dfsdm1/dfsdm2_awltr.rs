#[doc = "Register `DFSDM2_AWLTR` reader"]
pub struct R(crate::R<DFSDM2_AWLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM2_AWLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM2_AWLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM2_AWLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM2_AWLTR` writer"]
pub struct W(crate::W<DFSDM2_AWLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM2_AWLTR_SPEC>;
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
impl From<crate::W<DFSDM2_AWLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM2_AWLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWLT` reader - Analog watchdog low threshold"]
pub type AWLT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWLT` writer - Analog watchdog low threshold"]
pub type AWLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM2_AWLTR_SPEC, u32, u32, 24, O>;
#[doc = "Field `BKAWL` reader - Break signal assignment to analog watchdog low threshold event"]
pub type BKAWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKAWL` writer - Break signal assignment to analog watchdog low threshold event"]
pub type BKAWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM2_AWLTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W<8> {
        AWLT_W::new(self)
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W<0> {
        BKAWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm2_awltr](index.html) module"]
pub struct DFSDM2_AWLTR_SPEC;
impl crate::RegisterSpec for DFSDM2_AWLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm2_awltr::R](R) reader structure"]
impl crate::Readable for DFSDM2_AWLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm2_awltr::W](W) writer structure"]
impl crate::Writable for DFSDM2_AWLTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM2_AWLTR to value 0"]
impl crate::Resettable for DFSDM2_AWLTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

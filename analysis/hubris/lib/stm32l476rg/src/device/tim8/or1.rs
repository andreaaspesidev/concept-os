#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETR_ADC2_RMP` reader - External trigger remap on ADC2 analog watchdog"]
pub type ETR_ADC2_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_ADC2_RMP` writer - External trigger remap on ADC2 analog watchdog"]
pub type ETR_ADC2_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ETR_ADC3_RMP` reader - External trigger remap on ADC3 analog watchdog"]
pub type ETR_ADC3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETR_ADC3_RMP` writer - External trigger remap on ADC3 analog watchdog"]
pub type ETR_ADC3_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TI1_RMP` reader - Input Capture 1 remap"]
pub type TI1_RMP_R = crate::BitReader<bool>;
#[doc = "Field `TI1_RMP` writer - Input Capture 1 remap"]
pub type TI1_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - External trigger remap on ADC2 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc2_rmp(&self) -> ETR_ADC2_RMP_R {
        ETR_ADC2_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc3_rmp(&self) -> ETR_ADC3_RMP_R {
        ETR_ADC3_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External trigger remap on ADC2 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc2_rmp(&mut self) -> ETR_ADC2_RMP_W<0> {
        ETR_ADC2_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc3_rmp(&mut self) -> ETR_ADC3_RMP_W<2> {
        ETR_ADC3_RMP_W::new(self)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<4> {
        TI1_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

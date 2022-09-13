#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability"]
pub type TIM1_ETR_ADC1_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability"]
pub type TIM1_ETR_ADC1_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIM1_ETR_ADC4_RMP` reader - TIM1_ETR_ADC4 remapping capability"]
pub type TIM1_ETR_ADC4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_ETR_ADC4_RMP` writer - TIM1_ETR_ADC4 remapping capability"]
pub type TIM1_ETR_ADC4_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TIM1_ETR_ADC4 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc4_rmp(&self) -> TIM1_ETR_ADC4_RMP_R {
        TIM1_ETR_ADC4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W<0> {
        TIM1_ETR_ADC1_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - TIM1_ETR_ADC4 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc4_rmp(&mut self) -> TIM1_ETR_ADC4_RMP_W<2> {
        TIM1_ETR_ADC4_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

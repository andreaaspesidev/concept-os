#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JQOVF` reader - JQOVF"]
pub type JQOVF_R = crate::BitReader<bool>;
#[doc = "Field `JQOVF` writer - JQOVF"]
pub type JQOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `AWD3` reader - AWD3"]
pub type AWD3_R = crate::BitReader<bool>;
#[doc = "Field `AWD3` writer - AWD3"]
pub type AWD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `AWD2` reader - AWD2"]
pub type AWD2_R = crate::BitReader<bool>;
#[doc = "Field `AWD2` writer - AWD2"]
pub type AWD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `AWD1` reader - AWD1"]
pub type AWD1_R = crate::BitReader<bool>;
#[doc = "Field `AWD1` writer - AWD1"]
pub type AWD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `JEOS` reader - JEOS"]
pub type JEOS_R = crate::BitReader<bool>;
#[doc = "Field `JEOS` writer - JEOS"]
pub type JEOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `JEOC` reader - JEOC"]
pub type JEOC_R = crate::BitReader<bool>;
#[doc = "Field `JEOC` writer - JEOC"]
pub type JEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - OVR"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EOS` reader - EOS"]
pub type EOS_R = crate::BitReader<bool>;
#[doc = "Field `EOS` writer - EOS"]
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EOC` reader - EOC"]
pub type EOC_R = crate::BitReader<bool>;
#[doc = "Field `EOC` writer - EOC"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EOSMP_R = crate::BitReader<bool>;
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EOSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `ADRDY` reader - ADRDY"]
pub type ADRDY_R = crate::BitReader<bool>;
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type ADRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - JQOVF"]
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - JEOS"]
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - JEOC"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - JQOVF"]
    #[inline(always)]
    pub fn jqovf(&mut self) -> JQOVF_W<10> {
        JQOVF_W::new(self)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD3_W<9> {
        AWD3_W::new(self)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD2_W<8> {
        AWD2_W::new(self)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD1_W<7> {
        AWD1_W::new(self)
    }
    #[doc = "Bit 6 - JEOS"]
    #[inline(always)]
    pub fn jeos(&mut self) -> JEOS_W<6> {
        JEOS_W::new(self)
    }
    #[doc = "Bit 5 - JEOC"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<5> {
        JEOC_W::new(self)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<3> {
        EOS_W::new(self)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<2> {
        EOC_W::new(self)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<1> {
        EOSMP_W::new(self)
    }
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<0> {
        ADRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

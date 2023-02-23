#[doc = "Register `OFR1` reader"]
pub struct R(crate::R<OFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR1` writer"]
pub struct W(crate::W<OFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR1_SPEC>;
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
impl From<crate::W<OFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET1_EN` reader - OFFSET1_EN"]
pub type OFFSET1_EN_R = crate::BitReader<bool>;
#[doc = "Field `OFFSET1_EN` writer - OFFSET1_EN"]
pub type OFFSET1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR1_SPEC, bool, O>;
#[doc = "Field `OFFSET1_CH` reader - OFFSET1_CH"]
pub type OFFSET1_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET1_CH` writer - OFFSET1_CH"]
pub type OFFSET1_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET1` reader - OFFSET1"]
pub type OFFSET1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET1` writer - OFFSET1"]
pub type OFFSET1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - OFFSET1_EN"]
    #[inline(always)]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<31> {
        OFFSET1_EN_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET1_CH"]
    #[inline(always)]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<26> {
        OFFSET1_CH_W::new(self)
    }
    #[doc = "Bits 0:11 - OFFSET1"]
    #[inline(always)]
    pub fn offset1(&mut self) -> OFFSET1_W<0> {
        OFFSET1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr1](index.html) module"]
pub struct OFR1_SPEC;
impl crate::RegisterSpec for OFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr1::R](R) reader structure"]
impl crate::Readable for OFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr1::W](W) writer structure"]
impl crate::Writable for OFR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR1 to value 0"]
impl crate::Resettable for OFR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

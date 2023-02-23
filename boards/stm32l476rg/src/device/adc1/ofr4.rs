#[doc = "Register `OFR4` reader"]
pub struct R(crate::R<OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR4` writer"]
pub struct W(crate::W<OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR4_SPEC>;
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
impl From<crate::W<OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET4_EN` reader - OFFSET4_EN"]
pub type OFFSET4_EN_R = crate::BitReader<bool>;
#[doc = "Field `OFFSET4_EN` writer - OFFSET4_EN"]
pub type OFFSET4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR4_SPEC, bool, O>;
#[doc = "Field `OFFSET4_CH` reader - OFFSET4_CH"]
pub type OFFSET4_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET4_CH` writer - OFFSET4_CH"]
pub type OFFSET4_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET4` reader - OFFSET4"]
pub type OFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET4` writer - OFFSET4"]
pub type OFFSET4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR4_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 31 - OFFSET4_EN"]
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:11 - OFFSET4"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - OFFSET4_EN"]
    #[inline(always)]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W<31> {
        OFFSET4_EN_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<26> {
        OFFSET4_CH_W::new(self)
    }
    #[doc = "Bits 0:11 - OFFSET4"]
    #[inline(always)]
    pub fn offset4(&mut self) -> OFFSET4_W<0> {
        OFFSET4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr4](index.html) module"]
pub struct OFR4_SPEC;
impl crate::RegisterSpec for OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr4::R](R) reader structure"]
impl crate::Readable for OFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr4::W](W) writer structure"]
impl crate::Writable for OFR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for OFR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

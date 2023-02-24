#[doc = "Register `OFR3` reader"]
pub struct R(crate::R<OFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR3` writer"]
pub struct W(crate::W<OFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR3_SPEC>;
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
impl From<crate::W<OFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET3_EN` reader - OFFSET3_EN"]
pub type OFFSET3_EN_R = crate::BitReader<bool>;
#[doc = "Field `OFFSET3_EN` writer - OFFSET3_EN"]
pub type OFFSET3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR3_SPEC, bool, O>;
#[doc = "Field `OFFSET3_CH` reader - OFFSET3_CH"]
pub type OFFSET3_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET3_CH` writer - OFFSET3_CH"]
pub type OFFSET3_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET3` reader - OFFSET3"]
pub type OFFSET3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET3` writer - OFFSET3"]
pub type OFFSET3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR3_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 31 - OFFSET3_EN"]
    #[inline(always)]
    pub fn offset3_en(&self) -> OFFSET3_EN_R {
        OFFSET3_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET3_CH"]
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:11 - OFFSET3"]
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - OFFSET3_EN"]
    #[inline(always)]
    pub fn offset3_en(&mut self) -> OFFSET3_EN_W<31> {
        OFFSET3_EN_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET3_CH"]
    #[inline(always)]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<26> {
        OFFSET3_CH_W::new(self)
    }
    #[doc = "Bits 0:11 - OFFSET3"]
    #[inline(always)]
    pub fn offset3(&mut self) -> OFFSET3_W<0> {
        OFFSET3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr3](index.html) module"]
pub struct OFR3_SPEC;
impl crate::RegisterSpec for OFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr3::R](R) reader structure"]
impl crate::Readable for OFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr3::W](W) writer structure"]
impl crate::Writable for OFR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR3 to value 0"]
impl crate::Resettable for OFR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

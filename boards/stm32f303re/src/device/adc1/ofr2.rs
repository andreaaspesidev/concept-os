#[doc = "Register `OFR2` reader"]
pub struct R(crate::R<OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR2` writer"]
pub struct W(crate::W<OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR2_SPEC>;
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
impl From<crate::W<OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "OFFSET2_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFSET2_EN_A {
    #[doc = "0: Offset disabled"]
    Disabled = 0,
    #[doc = "1: Offset enabled"]
    Enabled = 1,
}
impl From<OFFSET2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET2_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSET2_EN` reader - OFFSET2_EN"]
pub type OFFSET2_EN_R = crate::BitReader<OFFSET2_EN_A>;
impl OFFSET2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSET2_EN_A {
        match self.bits {
            false => OFFSET2_EN_A::Disabled,
            true => OFFSET2_EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET2_EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET2_EN_A::Enabled
    }
}
#[doc = "Field `OFFSET2_EN` writer - OFFSET2_EN"]
pub type OFFSET2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR2_SPEC, OFFSET2_EN_A, O>;
impl<'a, const O: u8> OFFSET2_EN_W<'a, O> {
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OFFSET2_EN_A::Disabled)
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OFFSET2_EN_A::Enabled)
    }
}
#[doc = "Field `OFFSET2_CH` reader - OFFSET2_CH"]
pub type OFFSET2_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET2_CH` writer - OFFSET2_CH"]
pub type OFFSET2_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR2_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET2` reader - OFFSET2"]
pub type OFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET2` writer - OFFSET2"]
pub type OFFSET2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 31 - OFFSET2_EN"]
    #[inline(always)]
    pub fn offset2_en(&self) -> OFFSET2_EN_R {
        OFFSET2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 26:30 - OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2_ch(&self) -> OFFSET2_CH_R {
        OFFSET2_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 0:11 - OFFSET2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - OFFSET2_EN"]
    #[inline(always)]
    pub fn offset2_en(&mut self) -> OFFSET2_EN_W<31> {
        OFFSET2_EN_W::new(self)
    }
    #[doc = "Bits 26:30 - OFFSET2_CH"]
    #[inline(always)]
    pub fn offset2_ch(&mut self) -> OFFSET2_CH_W<26> {
        OFFSET2_CH_W::new(self)
    }
    #[doc = "Bits 0:11 - OFFSET2"]
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W<0> {
        OFFSET2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr2](index.html) module"]
pub struct OFR2_SPEC;
impl crate::RegisterSpec for OFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr2::R](R) reader structure"]
impl crate::Readable for OFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr2::W](W) writer structure"]
impl crate::Writable for OFR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFR2 to value 0"]
impl crate::Resettable for OFR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

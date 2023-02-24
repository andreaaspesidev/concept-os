#[doc = "Register `TR3` reader"]
pub struct R(crate::R<TR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR3` writer"]
pub struct W(crate::W<TR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR3_SPEC>;
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
impl From<crate::W<TR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT3` reader - HT3"]
pub type HT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HT3` writer - HT3"]
pub type HT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR3_SPEC, u8, u8, 8, O>;
#[doc = "Field `LT3` reader - LT3"]
pub type LT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LT3` writer - LT3"]
pub type LT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - HT3"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - LT3"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - HT3"]
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W<16> {
        HT3_W::new(self)
    }
    #[doc = "Bits 0:7 - LT3"]
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W<0> {
        LT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr3](index.html) module"]
pub struct TR3_SPEC;
impl crate::RegisterSpec for TR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr3::R](R) reader structure"]
impl crate::Readable for TR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr3::W](W) writer structure"]
impl crate::Writable for TR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR3 to value 0x0fff_0000"]
impl crate::Resettable for TR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

#[doc = "Register `TR2` reader"]
pub struct R(crate::R<TR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR2` writer"]
pub struct W(crate::W<TR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR2_SPEC>;
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
impl From<crate::W<TR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT2` reader - HT2"]
pub type HT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HT2` writer - HT2"]
pub type HT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `LT2` reader - LT2"]
pub type LT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LT2` writer - LT2"]
pub type LT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - HT2"]
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - LT2"]
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - HT2"]
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W<16> {
        HT2_W::new(self)
    }
    #[doc = "Bits 0:7 - LT2"]
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W<0> {
        LT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr2](index.html) module"]
pub struct TR2_SPEC;
impl crate::RegisterSpec for TR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr2::R](R) reader structure"]
impl crate::Readable for TR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr2::W](W) writer structure"]
impl crate::Writable for TR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR2 to value 0x0fff_0000"]
impl crate::Resettable for TR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

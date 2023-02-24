#[doc = "Register `SWIER2` reader"]
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER2` writer"]
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Software interrupt on line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIER32_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWIER32_A> for bool {
    #[inline(always)]
    fn from(variant: SWIER32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWIER32` reader - Software interrupt on line 32"]
pub type SWIER32_R = crate::BitReader<SWIER32_A>;
impl SWIER32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIER32_A> {
        match self.bits {
            true => Some(SWIER32_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWIER32_A::Pend
    }
}
#[doc = "Field `SWIER32` writer - Software interrupt on line 32"]
pub type SWIER32_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, SWIER32_A, O>;
impl<'a, const O: u8> SWIER32_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWIER32_A::Pend)
    }
}
#[doc = "Software interrupt on line 33"]
pub use SWIER32_A as SWIER33_A;
#[doc = "Field `SWIER33` reader - Software interrupt on line 33"]
pub use SWIER32_R as SWIER33_R;
#[doc = "Field `SWIER33` writer - Software interrupt on line 33"]
pub use SWIER32_W as SWIER33_W;
impl R {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&self) -> SWIER32_R {
        SWIER32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&self) -> SWIER33_R {
        SWIER33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 32"]
    #[inline(always)]
    pub fn swier32(&mut self) -> SWIER32_W<0> {
        SWIER32_W::new(self)
    }
    #[doc = "Bit 1 - Software interrupt on line 33"]
    #[inline(always)]
    pub fn swier33(&mut self) -> SWIER33_W<1> {
        SWIER33_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier2::R](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier2::W](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

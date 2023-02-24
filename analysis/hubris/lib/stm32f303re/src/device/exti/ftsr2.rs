#[doc = "Register `FTSR2` reader"]
pub struct R(crate::R<FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTSR2` writer"]
pub struct W(crate::W<FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR2_SPEC>;
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
impl From<crate::W<FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Falling trigger event configuration bit of line 32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR32_A {
    #[doc = "0: Falling edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Falling edge trigger is enabled"]
    Enabled = 1,
}
impl From<TR32_A> for bool {
    #[inline(always)]
    fn from(variant: TR32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR32` reader - Falling trigger event configuration bit of line 32"]
pub type TR32_R = crate::BitReader<TR32_A>;
impl TR32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR32_A {
        match self.bits {
            false => TR32_A::Disabled,
            true => TR32_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR32_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR32_A::Enabled
    }
}
#[doc = "Field `TR32` writer - Falling trigger event configuration bit of line 32"]
pub type TR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR2_SPEC, TR32_A, O>;
impl<'a, const O: u8> TR32_W<'a, O> {
    #[doc = "Falling edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR32_A::Disabled)
    }
    #[doc = "Falling edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR32_A::Enabled)
    }
}
#[doc = "Falling trigger event configuration bit of line 33"]
pub use TR32_A as TR33_A;
#[doc = "Field `TR33` reader - Falling trigger event configuration bit of line 33"]
pub use TR32_R as TR33_R;
#[doc = "Field `TR33` writer - Falling trigger event configuration bit of line 33"]
pub use TR32_W as TR33_W;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&self) -> TR32_R {
        TR32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&self) -> TR33_R {
        TR33_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration bit of line 32"]
    #[inline(always)]
    pub fn tr32(&mut self) -> TR32_W<0> {
        TR32_W::new(self)
    }
    #[doc = "Bit 1 - Falling trigger event configuration bit of line 33"]
    #[inline(always)]
    pub fn tr33(&mut self) -> TR33_W<1> {
        TR33_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Falling Trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr2](index.html) module"]
pub struct FTSR2_SPEC;
impl crate::RegisterSpec for FTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftsr2::R](R) reader structure"]
impl crate::Readable for FTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftsr2::W](W) writer structure"]
impl crate::Writable for FTSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for FTSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `RTSR2` reader"]
pub struct R(crate::R<RTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTSR2` writer"]
pub struct W(crate::W<RTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR2_SPEC>;
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
impl From<crate::W<RTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Rising trigger event configuration bit of line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT35_A {
    #[doc = "0: Rising edge trigger is disabled"]
    Disabled = 0,
    #[doc = "1: Rising edge trigger is enabled"]
    Enabled = 1,
}
impl From<RT35_A> for bool {
    #[inline(always)]
    fn from(variant: RT35_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT35` reader - Rising trigger event configuration bit of line 35"]
pub type RT35_R = crate::BitReader<RT35_A>;
impl RT35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT35_A {
        match self.bits {
            false => RT35_A::Disabled,
            true => RT35_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT35_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT35_A::Enabled
    }
}
#[doc = "Field `RT35` writer - Rising trigger event configuration bit of line 35"]
pub type RT35_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, RT35_A, O>;
impl<'a, const O: u8> RT35_W<'a, O> {
    #[doc = "Rising edge trigger is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT35_A::Disabled)
    }
    #[doc = "Rising edge trigger is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT35_A::Enabled)
    }
}
#[doc = "Rising trigger event configuration bit of line 36"]
pub use RT35_A as RT36_A;
#[doc = "Rising trigger event configuration bit of line 37"]
pub use RT35_A as RT37_A;
#[doc = "Rising trigger event configuration bit of line 38"]
pub use RT35_A as RT38_A;
#[doc = "Field `RT36` reader - Rising trigger event configuration bit of line 36"]
pub use RT35_R as RT36_R;
#[doc = "Field `RT37` reader - Rising trigger event configuration bit of line 37"]
pub use RT35_R as RT37_R;
#[doc = "Field `RT38` reader - Rising trigger event configuration bit of line 38"]
pub use RT35_R as RT38_R;
#[doc = "Field `RT36` writer - Rising trigger event configuration bit of line 36"]
pub use RT35_W as RT36_W;
#[doc = "Field `RT37` writer - Rising trigger event configuration bit of line 37"]
pub use RT35_W as RT37_W;
#[doc = "Field `RT38` writer - Rising trigger event configuration bit of line 38"]
pub use RT35_W as RT38_W;
impl R {
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn rt35(&self) -> RT35_R {
        RT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn rt36(&self) -> RT36_R {
        RT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn rt37(&self) -> RT37_R {
        RT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn rt38(&self) -> RT38_R {
        RT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 35"]
    #[inline(always)]
    pub fn rt35(&mut self) -> RT35_W<3> {
        RT35_W::new(self)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 36"]
    #[inline(always)]
    pub fn rt36(&mut self) -> RT36_W<4> {
        RT36_W::new(self)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 37"]
    #[inline(always)]
    pub fn rt37(&mut self) -> RT37_W<5> {
        RT37_W::new(self)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 38"]
    #[inline(always)]
    pub fn rt38(&mut self) -> RT38_W<6> {
        RT38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rising Trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr2](index.html) module"]
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtsr2::R](R) reader structure"]
impl crate::Readable for RTSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtsr2::W](W) writer structure"]
impl crate::Writable for RTSR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for RTSR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

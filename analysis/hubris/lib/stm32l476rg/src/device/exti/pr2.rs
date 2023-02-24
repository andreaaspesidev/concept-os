#[doc = "Register `PR2` reader"]
pub struct R(crate::R<PR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR2` writer"]
pub struct W(crate::W<PR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR2_SPEC>;
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
impl From<crate::W<PR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pending interrupt flag on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF35_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<PIF35_A> for bool {
    #[inline(always)]
    fn from(variant: PIF35_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF35` reader - Pending interrupt flag on line 35"]
pub type PIF35_R = crate::BitReader<PIF35_A>;
impl PIF35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIF35_A {
        match self.bits {
            false => PIF35_A::NotPending,
            true => PIF35_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF35_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF35_A::Pending
    }
}
#[doc = "Pending interrupt flag on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIF35_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<PIF35_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF35_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIF35` writer - Pending interrupt flag on line 35"]
pub type PIF35_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR2_SPEC, PIF35_AW, O>;
impl<'a, const O: u8> PIF35_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF35_AW::Clear)
    }
}
#[doc = "Pending interrupt flag on line 36"]
pub use PIF35_A as PIF36_A;
#[doc = "Pending interrupt flag on line 37"]
pub use PIF35_A as PIF37_A;
#[doc = "Pending interrupt flag on line 38"]
pub use PIF35_A as PIF38_A;
#[doc = "Pending interrupt flag on line 36"]
pub use PIF35_AW as PIF36_AW;
#[doc = "Pending interrupt flag on line 37"]
pub use PIF35_AW as PIF37_AW;
#[doc = "Pending interrupt flag on line 38"]
pub use PIF35_AW as PIF38_AW;
#[doc = "Field `PIF36` reader - Pending interrupt flag on line 36"]
pub use PIF35_R as PIF36_R;
#[doc = "Field `PIF37` reader - Pending interrupt flag on line 37"]
pub use PIF35_R as PIF37_R;
#[doc = "Field `PIF38` reader - Pending interrupt flag on line 38"]
pub use PIF35_R as PIF38_R;
#[doc = "Field `PIF36` writer - Pending interrupt flag on line 36"]
pub use PIF35_W as PIF36_W;
#[doc = "Field `PIF37` writer - Pending interrupt flag on line 37"]
pub use PIF35_W as PIF37_W;
#[doc = "Field `PIF38` writer - Pending interrupt flag on line 38"]
pub use PIF35_W as PIF38_W;
impl R {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn pif35(&self) -> PIF35_R {
        PIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn pif36(&self) -> PIF36_R {
        PIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn pif37(&self) -> PIF37_R {
        PIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn pif38(&self) -> PIF38_R {
        PIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Pending interrupt flag on line 35"]
    #[inline(always)]
    pub fn pif35(&mut self) -> PIF35_W<3> {
        PIF35_W::new(self)
    }
    #[doc = "Bit 4 - Pending interrupt flag on line 36"]
    #[inline(always)]
    pub fn pif36(&mut self) -> PIF36_W<4> {
        PIF36_W::new(self)
    }
    #[doc = "Bit 5 - Pending interrupt flag on line 37"]
    #[inline(always)]
    pub fn pif37(&mut self) -> PIF37_W<5> {
        PIF37_W::new(self)
    }
    #[doc = "Bit 6 - Pending interrupt flag on line 38"]
    #[inline(always)]
    pub fn pif38(&mut self) -> PIF38_W<6> {
        PIF38_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](index.html) module"]
pub struct PR2_SPEC;
impl crate::RegisterSpec for PR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr2::R](R) reader structure"]
impl crate::Readable for PR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr2::W](W) writer structure"]
impl crate::Writable for PR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR2 to value 0"]
impl crate::Resettable for PR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

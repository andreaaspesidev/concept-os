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
#[doc = "Software interrupt on line 35\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI35_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI35_A> for bool {
    #[inline(always)]
    fn from(variant: SWI35_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWI35` reader - Software interrupt on line 35"]
pub type SWI35_R = crate::BitReader<SWI35_A>;
impl SWI35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI35_A> {
        match self.bits {
            true => Some(SWI35_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI35_A::Pend
    }
}
#[doc = "Field `SWI35` writer - Software interrupt on line 35"]
pub type SWI35_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, SWI35_A, O>;
impl<'a, const O: u8> SWI35_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI35_A::Pend)
    }
}
#[doc = "Software interrupt on line 36"]
pub use SWI35_A as SWI36_A;
#[doc = "Software interrupt on line 37"]
pub use SWI35_A as SWI37_A;
#[doc = "Software interrupt on line 38"]
pub use SWI35_A as SWI38_A;
#[doc = "Field `SWI36` reader - Software interrupt on line 36"]
pub use SWI35_R as SWI36_R;
#[doc = "Field `SWI37` reader - Software interrupt on line 37"]
pub use SWI35_R as SWI37_R;
#[doc = "Field `SWI38` reader - Software interrupt on line 38"]
pub use SWI35_R as SWI38_R;
#[doc = "Field `SWI36` writer - Software interrupt on line 36"]
pub use SWI35_W as SWI36_W;
#[doc = "Field `SWI37` writer - Software interrupt on line 37"]
pub use SWI35_W as SWI37_W;
#[doc = "Field `SWI38` writer - Software interrupt on line 38"]
pub use SWI35_W as SWI38_W;
impl R {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    pub fn swi35(&self) -> SWI35_R {
        SWI35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    pub fn swi36(&self) -> SWI36_R {
        SWI36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    pub fn swi37(&self) -> SWI37_R {
        SWI37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    pub fn swi38(&self) -> SWI38_R {
        SWI38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Software interrupt on line 35"]
    #[inline(always)]
    pub fn swi35(&mut self) -> SWI35_W<3> {
        SWI35_W::new(self)
    }
    #[doc = "Bit 4 - Software interrupt on line 36"]
    #[inline(always)]
    pub fn swi36(&mut self) -> SWI36_W<4> {
        SWI36_W::new(self)
    }
    #[doc = "Bit 5 - Software interrupt on line 37"]
    #[inline(always)]
    pub fn swi37(&mut self) -> SWI37_W<5> {
        SWI37_W::new(self)
    }
    #[doc = "Bit 6 - Software interrupt on line 38"]
    #[inline(always)]
    pub fn swi38(&mut self) -> SWI38_W<6> {
        SWI38_W::new(self)
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

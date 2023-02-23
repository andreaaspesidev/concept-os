#[doc = "Register `PUCRH` reader"]
pub struct R(crate::R<PUCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRH` writer"]
pub struct W(crate::W<PUCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRH_SPEC>;
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
impl From<crate::W<PUCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU1` reader - Port H pull-up bit y (y=0..1)"]
pub type PU1_R = crate::BitReader<bool>;
#[doc = "Field `PU1` writer - Port H pull-up bit y (y=0..1)"]
pub type PU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRH_SPEC, bool, O>;
#[doc = "Field `PU0` reader - Port H pull-up bit y (y=0..1)"]
pub type PU0_R = crate::BitReader<bool>;
#[doc = "Field `PU0` writer - Port H pull-up bit y (y=0..1)"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 0 - Port H pull-up bit y (y=0..1)"]
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port H pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrh](index.html) module"]
pub struct PUCRH_SPEC;
impl crate::RegisterSpec for PUCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrh::R](R) reader structure"]
impl crate::Readable for PUCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrh::W](W) writer structure"]
impl crate::Writable for PUCRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCRH to value 0"]
impl crate::Resettable for PUCRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

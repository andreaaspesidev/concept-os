#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP18` reader - SMP18"]
pub type SMP18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP18` writer - SMP18"]
pub type SMP18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP17` reader - SMP17"]
pub type SMP17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP17` writer - SMP17"]
pub type SMP17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP16` reader - SMP16"]
pub type SMP16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP16` writer - SMP16"]
pub type SMP16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP15` reader - SMP15"]
pub type SMP15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP15` writer - SMP15"]
pub type SMP15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP14` reader - SMP14"]
pub type SMP14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP14` writer - SMP14"]
pub type SMP14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP13` reader - SMP13"]
pub type SMP13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP13` writer - SMP13"]
pub type SMP13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP12` reader - SMP12"]
pub type SMP12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP12` writer - SMP12"]
pub type SMP12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP11` reader - SMP11"]
pub type SMP11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP11` writer - SMP11"]
pub type SMP11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `SMP10` reader - SMP10"]
pub type SMP10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMP10` writer - SMP10"]
pub type SMP10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&mut self) -> SMP18_W<24> {
        SMP18_W::new(self)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

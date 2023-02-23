#[doc = "Register `PUCRE` reader"]
pub struct R(crate::R<PUCRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRE` writer"]
pub struct W(crate::W<PUCRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRE_SPEC>;
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
impl From<crate::W<PUCRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU15` reader - Port E pull-up bit y (y=0..15)"]
pub type PU15_R = crate::BitReader<bool>;
#[doc = "Field `PU15` writer - Port E pull-up bit y (y=0..15)"]
pub type PU15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU14` reader - Port E pull-up bit y (y=0..15)"]
pub type PU14_R = crate::BitReader<bool>;
#[doc = "Field `PU14` writer - Port E pull-up bit y (y=0..15)"]
pub type PU14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU13` reader - Port E pull-up bit y (y=0..15)"]
pub type PU13_R = crate::BitReader<bool>;
#[doc = "Field `PU13` writer - Port E pull-up bit y (y=0..15)"]
pub type PU13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU12` reader - Port E pull-up bit y (y=0..15)"]
pub type PU12_R = crate::BitReader<bool>;
#[doc = "Field `PU12` writer - Port E pull-up bit y (y=0..15)"]
pub type PU12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU11` reader - Port E pull-up bit y (y=0..15)"]
pub type PU11_R = crate::BitReader<bool>;
#[doc = "Field `PU11` writer - Port E pull-up bit y (y=0..15)"]
pub type PU11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU10` reader - Port E pull-up bit y (y=0..15)"]
pub type PU10_R = crate::BitReader<bool>;
#[doc = "Field `PU10` writer - Port E pull-up bit y (y=0..15)"]
pub type PU10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU9` reader - Port E pull-up bit y (y=0..15)"]
pub type PU9_R = crate::BitReader<bool>;
#[doc = "Field `PU9` writer - Port E pull-up bit y (y=0..15)"]
pub type PU9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU8` reader - Port E pull-up bit y (y=0..15)"]
pub type PU8_R = crate::BitReader<bool>;
#[doc = "Field `PU8` writer - Port E pull-up bit y (y=0..15)"]
pub type PU8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU7` reader - Port E pull-up bit y (y=0..15)"]
pub type PU7_R = crate::BitReader<bool>;
#[doc = "Field `PU7` writer - Port E pull-up bit y (y=0..15)"]
pub type PU7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU6` reader - Port E pull-up bit y (y=0..15)"]
pub type PU6_R = crate::BitReader<bool>;
#[doc = "Field `PU6` writer - Port E pull-up bit y (y=0..15)"]
pub type PU6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU5` reader - Port E pull-up bit y (y=0..15)"]
pub type PU5_R = crate::BitReader<bool>;
#[doc = "Field `PU5` writer - Port E pull-up bit y (y=0..15)"]
pub type PU5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU4` reader - Port E pull-up bit y (y=0..15)"]
pub type PU4_R = crate::BitReader<bool>;
#[doc = "Field `PU4` writer - Port E pull-up bit y (y=0..15)"]
pub type PU4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU3` reader - Port E pull-up bit y (y=0..15)"]
pub type PU3_R = crate::BitReader<bool>;
#[doc = "Field `PU3` writer - Port E pull-up bit y (y=0..15)"]
pub type PU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU2` reader - Port E pull-up bit y (y=0..15)"]
pub type PU2_R = crate::BitReader<bool>;
#[doc = "Field `PU2` writer - Port E pull-up bit y (y=0..15)"]
pub type PU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU1` reader - Port E pull-up bit y (y=0..15)"]
pub type PU1_R = crate::BitReader<bool>;
#[doc = "Field `PU1` writer - Port E pull-up bit y (y=0..15)"]
pub type PU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
#[doc = "Field `PU0` reader - Port E pull-up bit y (y=0..15)"]
pub type PU0_R = crate::BitReader<bool>;
#[doc = "Field `PU0` writer - Port E pull-up bit y (y=0..15)"]
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 15 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W<15> {
        PU15_W::new(self)
    }
    #[doc = "Bit 14 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W<14> {
        PU14_W::new(self)
    }
    #[doc = "Bit 13 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W<13> {
        PU13_W::new(self)
    }
    #[doc = "Bit 12 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&mut self) -> PU12_W<12> {
        PU12_W::new(self)
    }
    #[doc = "Bit 11 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&mut self) -> PU11_W<11> {
        PU11_W::new(self)
    }
    #[doc = "Bit 10 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&mut self) -> PU10_W<10> {
        PU10_W::new(self)
    }
    #[doc = "Bit 9 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W<9> {
        PU9_W::new(self)
    }
    #[doc = "Bit 8 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W<8> {
        PU8_W::new(self)
    }
    #[doc = "Bit 7 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<7> {
        PU7_W::new(self)
    }
    #[doc = "Bit 6 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    #[doc = "Bit 5 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W<5> {
        PU5_W::new(self)
    }
    #[doc = "Bit 4 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<4> {
        PU4_W::new(self)
    }
    #[doc = "Bit 3 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    #[doc = "Bit 2 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    #[doc = "Bit 1 - Port E pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    #[doc = "Bit 0 - Port E pull-up bit y (y=0..15)"]
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
#[doc = "Power Port E pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucre](index.html) module"]
pub struct PUCRE_SPEC;
impl crate::RegisterSpec for PUCRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucre::R](R) reader structure"]
impl crate::Readable for PUCRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucre::W](W) writer structure"]
impl crate::Writable for PUCRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCRE to value 0"]
impl crate::Resettable for PUCRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

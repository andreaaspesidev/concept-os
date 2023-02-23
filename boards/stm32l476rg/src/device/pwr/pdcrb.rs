#[doc = "Register `PDCRB` reader"]
pub struct R(crate::R<PDCRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRB` writer"]
pub struct W(crate::W<PDCRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRB_SPEC>;
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
impl From<crate::W<PDCRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD15` reader - Port B pull-down bit y (y=0..15)"]
pub type PD15_R = crate::BitReader<bool>;
#[doc = "Field `PD15` writer - Port B pull-down bit y (y=0..15)"]
pub type PD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD14` reader - Port B pull-down bit y (y=0..15)"]
pub type PD14_R = crate::BitReader<bool>;
#[doc = "Field `PD14` writer - Port B pull-down bit y (y=0..15)"]
pub type PD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD13` reader - Port B pull-down bit y (y=0..15)"]
pub type PD13_R = crate::BitReader<bool>;
#[doc = "Field `PD13` writer - Port B pull-down bit y (y=0..15)"]
pub type PD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD12` reader - Port B pull-down bit y (y=0..15)"]
pub type PD12_R = crate::BitReader<bool>;
#[doc = "Field `PD12` writer - Port B pull-down bit y (y=0..15)"]
pub type PD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD11` reader - Port B pull-down bit y (y=0..15)"]
pub type PD11_R = crate::BitReader<bool>;
#[doc = "Field `PD11` writer - Port B pull-down bit y (y=0..15)"]
pub type PD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD10` reader - Port B pull-down bit y (y=0..15)"]
pub type PD10_R = crate::BitReader<bool>;
#[doc = "Field `PD10` writer - Port B pull-down bit y (y=0..15)"]
pub type PD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD9` reader - Port B pull-down bit y (y=0..15)"]
pub type PD9_R = crate::BitReader<bool>;
#[doc = "Field `PD9` writer - Port B pull-down bit y (y=0..15)"]
pub type PD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD8` reader - Port B pull-down bit y (y=0..15)"]
pub type PD8_R = crate::BitReader<bool>;
#[doc = "Field `PD8` writer - Port B pull-down bit y (y=0..15)"]
pub type PD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD7` reader - Port B pull-down bit y (y=0..15)"]
pub type PD7_R = crate::BitReader<bool>;
#[doc = "Field `PD7` writer - Port B pull-down bit y (y=0..15)"]
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD6` reader - Port B pull-down bit y (y=0..15)"]
pub type PD6_R = crate::BitReader<bool>;
#[doc = "Field `PD6` writer - Port B pull-down bit y (y=0..15)"]
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD5` reader - Port B pull-down bit y (y=0..15)"]
pub type PD5_R = crate::BitReader<bool>;
#[doc = "Field `PD5` writer - Port B pull-down bit y (y=0..15)"]
pub type PD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD4` reader - Port B pull-down bit y (y=0..15)"]
pub type PD4_R = crate::BitReader<bool>;
#[doc = "Field `PD4` writer - Port B pull-down bit y (y=0..15)"]
pub type PD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD3` reader - Port B pull-down bit y (y=0..15)"]
pub type PD3_R = crate::BitReader<bool>;
#[doc = "Field `PD3` writer - Port B pull-down bit y (y=0..15)"]
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD2` reader - Port B pull-down bit y (y=0..15)"]
pub type PD2_R = crate::BitReader<bool>;
#[doc = "Field `PD2` writer - Port B pull-down bit y (y=0..15)"]
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD1` reader - Port B pull-down bit y (y=0..15)"]
pub type PD1_R = crate::BitReader<bool>;
#[doc = "Field `PD1` writer - Port B pull-down bit y (y=0..15)"]
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
#[doc = "Field `PD0` reader - Port B pull-down bit y (y=0..15)"]
pub type PD0_R = crate::BitReader<bool>;
#[doc = "Field `PD0` writer - Port B pull-down bit y (y=0..15)"]
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 15 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<15> {
        PD15_W::new(self)
    }
    #[doc = "Bit 14 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    #[doc = "Bit 13 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    #[doc = "Bit 12 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd12(&mut self) -> PD12_W<12> {
        PD12_W::new(self)
    }
    #[doc = "Bit 11 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd11(&mut self) -> PD11_W<11> {
        PD11_W::new(self)
    }
    #[doc = "Bit 10 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    #[doc = "Bit 9 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd9(&mut self) -> PD9_W<9> {
        PD9_W::new(self)
    }
    #[doc = "Bit 8 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd8(&mut self) -> PD8_W<8> {
        PD8_W::new(self)
    }
    #[doc = "Bit 7 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    #[doc = "Bit 6 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 5 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    #[doc = "Bit 4 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    #[doc = "Bit 3 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 2 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 1 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 0 - Port B pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port B pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrb](index.html) module"]
pub struct PDCRB_SPEC;
impl crate::RegisterSpec for PDCRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrb::R](R) reader structure"]
impl crate::Readable for PDCRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrb::W](W) writer structure"]
impl crate::Writable for PDCRB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDCRB to value 0"]
impl crate::Resettable for PDCRB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

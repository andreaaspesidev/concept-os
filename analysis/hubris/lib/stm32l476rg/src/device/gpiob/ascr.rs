#[doc = "Register `ASCR` reader"]
pub struct R(crate::R<ASCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASCR` writer"]
pub struct W(crate::W<ASCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASCR_SPEC>;
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
impl From<crate::W<ASCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASC0` reader - Port analog switch control"]
pub type ASC0_R = crate::BitReader<bool>;
#[doc = "Field `ASC0` writer - Port analog switch control"]
pub type ASC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC1` reader - Port analog switch control"]
pub type ASC1_R = crate::BitReader<bool>;
#[doc = "Field `ASC1` writer - Port analog switch control"]
pub type ASC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC2` reader - Port analog switch control"]
pub type ASC2_R = crate::BitReader<bool>;
#[doc = "Field `ASC2` writer - Port analog switch control"]
pub type ASC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC3` reader - Port analog switch control"]
pub type ASC3_R = crate::BitReader<bool>;
#[doc = "Field `ASC3` writer - Port analog switch control"]
pub type ASC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC4` reader - Port analog switch control"]
pub type ASC4_R = crate::BitReader<bool>;
#[doc = "Field `ASC4` writer - Port analog switch control"]
pub type ASC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC5` reader - Port analog switch control"]
pub type ASC5_R = crate::BitReader<bool>;
#[doc = "Field `ASC5` writer - Port analog switch control"]
pub type ASC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC6` reader - Port analog switch control"]
pub type ASC6_R = crate::BitReader<bool>;
#[doc = "Field `ASC6` writer - Port analog switch control"]
pub type ASC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC7` reader - Port analog switch control"]
pub type ASC7_R = crate::BitReader<bool>;
#[doc = "Field `ASC7` writer - Port analog switch control"]
pub type ASC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC8` reader - Port analog switch control"]
pub type ASC8_R = crate::BitReader<bool>;
#[doc = "Field `ASC8` writer - Port analog switch control"]
pub type ASC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC9` reader - Port analog switch control"]
pub type ASC9_R = crate::BitReader<bool>;
#[doc = "Field `ASC9` writer - Port analog switch control"]
pub type ASC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC10` reader - Port analog switch control"]
pub type ASC10_R = crate::BitReader<bool>;
#[doc = "Field `ASC10` writer - Port analog switch control"]
pub type ASC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC11` reader - Port analog switch control"]
pub type ASC11_R = crate::BitReader<bool>;
#[doc = "Field `ASC11` writer - Port analog switch control"]
pub type ASC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC12` reader - Port analog switch control"]
pub type ASC12_R = crate::BitReader<bool>;
#[doc = "Field `ASC12` writer - Port analog switch control"]
pub type ASC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC13` reader - Port analog switch control"]
pub type ASC13_R = crate::BitReader<bool>;
#[doc = "Field `ASC13` writer - Port analog switch control"]
pub type ASC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC14` reader - Port analog switch control"]
pub type ASC14_R = crate::BitReader<bool>;
#[doc = "Field `ASC14` writer - Port analog switch control"]
pub type ASC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
#[doc = "Field `ASC15` reader - Port analog switch control"]
pub type ASC15_R = crate::BitReader<bool>;
#[doc = "Field `ASC15` writer - Port analog switch control"]
pub type ASC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ASCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&self) -> ASC0_R {
        ASC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&self) -> ASC1_R {
        ASC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&self) -> ASC2_R {
        ASC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&self) -> ASC3_R {
        ASC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&self) -> ASC4_R {
        ASC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&self) -> ASC5_R {
        ASC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&self) -> ASC6_R {
        ASC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&self) -> ASC7_R {
        ASC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&self) -> ASC8_R {
        ASC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&self) -> ASC9_R {
        ASC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&self) -> ASC10_R {
        ASC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&self) -> ASC11_R {
        ASC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&self) -> ASC12_R {
        ASC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&self) -> ASC13_R {
        ASC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&self) -> ASC14_R {
        ASC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&self) -> ASC15_R {
        ASC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port analog switch control"]
    #[inline(always)]
    pub fn asc0(&mut self) -> ASC0_W<0> {
        ASC0_W::new(self)
    }
    #[doc = "Bit 1 - Port analog switch control"]
    #[inline(always)]
    pub fn asc1(&mut self) -> ASC1_W<1> {
        ASC1_W::new(self)
    }
    #[doc = "Bit 2 - Port analog switch control"]
    #[inline(always)]
    pub fn asc2(&mut self) -> ASC2_W<2> {
        ASC2_W::new(self)
    }
    #[doc = "Bit 3 - Port analog switch control"]
    #[inline(always)]
    pub fn asc3(&mut self) -> ASC3_W<3> {
        ASC3_W::new(self)
    }
    #[doc = "Bit 4 - Port analog switch control"]
    #[inline(always)]
    pub fn asc4(&mut self) -> ASC4_W<4> {
        ASC4_W::new(self)
    }
    #[doc = "Bit 5 - Port analog switch control"]
    #[inline(always)]
    pub fn asc5(&mut self) -> ASC5_W<5> {
        ASC5_W::new(self)
    }
    #[doc = "Bit 6 - Port analog switch control"]
    #[inline(always)]
    pub fn asc6(&mut self) -> ASC6_W<6> {
        ASC6_W::new(self)
    }
    #[doc = "Bit 7 - Port analog switch control"]
    #[inline(always)]
    pub fn asc7(&mut self) -> ASC7_W<7> {
        ASC7_W::new(self)
    }
    #[doc = "Bit 8 - Port analog switch control"]
    #[inline(always)]
    pub fn asc8(&mut self) -> ASC8_W<8> {
        ASC8_W::new(self)
    }
    #[doc = "Bit 9 - Port analog switch control"]
    #[inline(always)]
    pub fn asc9(&mut self) -> ASC9_W<9> {
        ASC9_W::new(self)
    }
    #[doc = "Bit 10 - Port analog switch control"]
    #[inline(always)]
    pub fn asc10(&mut self) -> ASC10_W<10> {
        ASC10_W::new(self)
    }
    #[doc = "Bit 11 - Port analog switch control"]
    #[inline(always)]
    pub fn asc11(&mut self) -> ASC11_W<11> {
        ASC11_W::new(self)
    }
    #[doc = "Bit 12 - Port analog switch control"]
    #[inline(always)]
    pub fn asc12(&mut self) -> ASC12_W<12> {
        ASC12_W::new(self)
    }
    #[doc = "Bit 13 - Port analog switch control"]
    #[inline(always)]
    pub fn asc13(&mut self) -> ASC13_W<13> {
        ASC13_W::new(self)
    }
    #[doc = "Bit 14 - Port analog switch control"]
    #[inline(always)]
    pub fn asc14(&mut self) -> ASC14_W<14> {
        ASC14_W::new(self)
    }
    #[doc = "Bit 15 - Port analog switch control"]
    #[inline(always)]
    pub fn asc15(&mut self) -> ASC15_W<15> {
        ASC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port analog switch control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ascr](index.html) module"]
pub struct ASCR_SPEC;
impl crate::RegisterSpec for ASCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ascr::R](R) reader structure"]
impl crate::Readable for ASCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ascr::W](W) writer structure"]
impl crate::Writable for ASCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASCR to value 0"]
impl crate::Resettable for ASCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `IOSCR` reader"]
pub struct R(crate::R<IOSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOSCR` writer"]
pub struct W(crate::W<IOSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOSCR_SPEC>;
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
impl From<crate::W<IOSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `G1_IO1` reader - G1_IO1 sampling mode"]
pub type G1_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G1_IO1` writer - G1_IO1 sampling mode"]
pub type G1_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G1_IO2` reader - G1_IO2 sampling mode"]
pub type G1_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G1_IO2` writer - G1_IO2 sampling mode"]
pub type G1_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G1_IO3` reader - G1_IO3 sampling mode"]
pub type G1_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G1_IO3` writer - G1_IO3 sampling mode"]
pub type G1_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G1_IO4` reader - G1_IO4 sampling mode"]
pub type G1_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G1_IO4` writer - G1_IO4 sampling mode"]
pub type G1_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G2_IO1` reader - G2_IO1 sampling mode"]
pub type G2_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G2_IO1` writer - G2_IO1 sampling mode"]
pub type G2_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G2_IO2` reader - G2_IO2 sampling mode"]
pub type G2_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G2_IO2` writer - G2_IO2 sampling mode"]
pub type G2_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G2_IO3` reader - G2_IO3 sampling mode"]
pub type G2_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G2_IO3` writer - G2_IO3 sampling mode"]
pub type G2_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G2_IO4` reader - G2_IO4 sampling mode"]
pub type G2_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G2_IO4` writer - G2_IO4 sampling mode"]
pub type G2_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G3_IO1` reader - G3_IO1 sampling mode"]
pub type G3_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G3_IO1` writer - G3_IO1 sampling mode"]
pub type G3_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G3_IO2` reader - G3_IO2 sampling mode"]
pub type G3_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G3_IO2` writer - G3_IO2 sampling mode"]
pub type G3_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G3_IO3` reader - G3_IO3 sampling mode"]
pub type G3_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G3_IO3` writer - G3_IO3 sampling mode"]
pub type G3_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G3_IO4` reader - G3_IO4 sampling mode"]
pub type G3_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G3_IO4` writer - G3_IO4 sampling mode"]
pub type G3_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G4_IO1` reader - G4_IO1 sampling mode"]
pub type G4_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G4_IO1` writer - G4_IO1 sampling mode"]
pub type G4_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G4_IO2` reader - G4_IO2 sampling mode"]
pub type G4_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G4_IO2` writer - G4_IO2 sampling mode"]
pub type G4_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G4_IO3` reader - G4_IO3 sampling mode"]
pub type G4_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G4_IO3` writer - G4_IO3 sampling mode"]
pub type G4_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G4_IO4` reader - G4_IO4 sampling mode"]
pub type G4_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G4_IO4` writer - G4_IO4 sampling mode"]
pub type G4_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G5_IO1` reader - G5_IO1 sampling mode"]
pub type G5_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G5_IO1` writer - G5_IO1 sampling mode"]
pub type G5_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G5_IO2` reader - G5_IO2 sampling mode"]
pub type G5_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G5_IO2` writer - G5_IO2 sampling mode"]
pub type G5_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G5_IO3` reader - G5_IO3 sampling mode"]
pub type G5_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G5_IO3` writer - G5_IO3 sampling mode"]
pub type G5_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G5_IO4` reader - G5_IO4 sampling mode"]
pub type G5_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G5_IO4` writer - G5_IO4 sampling mode"]
pub type G5_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G6_IO1` reader - G6_IO1 sampling mode"]
pub type G6_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G6_IO1` writer - G6_IO1 sampling mode"]
pub type G6_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G6_IO2` reader - G6_IO2 sampling mode"]
pub type G6_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G6_IO2` writer - G6_IO2 sampling mode"]
pub type G6_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G6_IO3` reader - G6_IO3 sampling mode"]
pub type G6_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G6_IO3` writer - G6_IO3 sampling mode"]
pub type G6_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G6_IO4` reader - G6_IO4 sampling mode"]
pub type G6_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G6_IO4` writer - G6_IO4 sampling mode"]
pub type G6_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G7_IO1` reader - G7_IO1 sampling mode"]
pub type G7_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G7_IO1` writer - G7_IO1 sampling mode"]
pub type G7_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G7_IO2` reader - G7_IO2 sampling mode"]
pub type G7_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G7_IO2` writer - G7_IO2 sampling mode"]
pub type G7_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G7_IO3` reader - G7_IO3 sampling mode"]
pub type G7_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G7_IO3` writer - G7_IO3 sampling mode"]
pub type G7_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G7_IO4` reader - G7_IO4 sampling mode"]
pub type G7_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G7_IO4` writer - G7_IO4 sampling mode"]
pub type G7_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G8_IO1` reader - G8_IO1 sampling mode"]
pub type G8_IO1_R = crate::BitReader<bool>;
#[doc = "Field `G8_IO1` writer - G8_IO1 sampling mode"]
pub type G8_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G8_IO2` reader - G8_IO2 sampling mode"]
pub type G8_IO2_R = crate::BitReader<bool>;
#[doc = "Field `G8_IO2` writer - G8_IO2 sampling mode"]
pub type G8_IO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G8_IO3` reader - G8_IO3 sampling mode"]
pub type G8_IO3_R = crate::BitReader<bool>;
#[doc = "Field `G8_IO3` writer - G8_IO3 sampling mode"]
pub type G8_IO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
#[doc = "Field `G8_IO4` reader - G8_IO4 sampling mode"]
pub type G8_IO4_R = crate::BitReader<bool>;
#[doc = "Field `G8_IO4` writer - G8_IO4 sampling mode"]
pub type G8_IO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOSCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - G1_IO1 sampling mode"]
    #[inline(always)]
    pub fn g1_io1(&self) -> G1_IO1_R {
        G1_IO1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - G1_IO2 sampling mode"]
    #[inline(always)]
    pub fn g1_io2(&self) -> G1_IO2_R {
        G1_IO2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - G1_IO3 sampling mode"]
    #[inline(always)]
    pub fn g1_io3(&self) -> G1_IO3_R {
        G1_IO3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - G1_IO4 sampling mode"]
    #[inline(always)]
    pub fn g1_io4(&self) -> G1_IO4_R {
        G1_IO4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - G2_IO1 sampling mode"]
    #[inline(always)]
    pub fn g2_io1(&self) -> G2_IO1_R {
        G2_IO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - G2_IO2 sampling mode"]
    #[inline(always)]
    pub fn g2_io2(&self) -> G2_IO2_R {
        G2_IO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - G2_IO3 sampling mode"]
    #[inline(always)]
    pub fn g2_io3(&self) -> G2_IO3_R {
        G2_IO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - G2_IO4 sampling mode"]
    #[inline(always)]
    pub fn g2_io4(&self) -> G2_IO4_R {
        G2_IO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - G3_IO1 sampling mode"]
    #[inline(always)]
    pub fn g3_io1(&self) -> G3_IO1_R {
        G3_IO1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - G3_IO2 sampling mode"]
    #[inline(always)]
    pub fn g3_io2(&self) -> G3_IO2_R {
        G3_IO2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - G3_IO3 sampling mode"]
    #[inline(always)]
    pub fn g3_io3(&self) -> G3_IO3_R {
        G3_IO3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - G3_IO4 sampling mode"]
    #[inline(always)]
    pub fn g3_io4(&self) -> G3_IO4_R {
        G3_IO4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - G4_IO1 sampling mode"]
    #[inline(always)]
    pub fn g4_io1(&self) -> G4_IO1_R {
        G4_IO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - G4_IO2 sampling mode"]
    #[inline(always)]
    pub fn g4_io2(&self) -> G4_IO2_R {
        G4_IO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - G4_IO3 sampling mode"]
    #[inline(always)]
    pub fn g4_io3(&self) -> G4_IO3_R {
        G4_IO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - G4_IO4 sampling mode"]
    #[inline(always)]
    pub fn g4_io4(&self) -> G4_IO4_R {
        G4_IO4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - G5_IO1 sampling mode"]
    #[inline(always)]
    pub fn g5_io1(&self) -> G5_IO1_R {
        G5_IO1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - G5_IO2 sampling mode"]
    #[inline(always)]
    pub fn g5_io2(&self) -> G5_IO2_R {
        G5_IO2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - G5_IO3 sampling mode"]
    #[inline(always)]
    pub fn g5_io3(&self) -> G5_IO3_R {
        G5_IO3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - G5_IO4 sampling mode"]
    #[inline(always)]
    pub fn g5_io4(&self) -> G5_IO4_R {
        G5_IO4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - G6_IO1 sampling mode"]
    #[inline(always)]
    pub fn g6_io1(&self) -> G6_IO1_R {
        G6_IO1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - G6_IO2 sampling mode"]
    #[inline(always)]
    pub fn g6_io2(&self) -> G6_IO2_R {
        G6_IO2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - G6_IO3 sampling mode"]
    #[inline(always)]
    pub fn g6_io3(&self) -> G6_IO3_R {
        G6_IO3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - G6_IO4 sampling mode"]
    #[inline(always)]
    pub fn g6_io4(&self) -> G6_IO4_R {
        G6_IO4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - G7_IO1 sampling mode"]
    #[inline(always)]
    pub fn g7_io1(&self) -> G7_IO1_R {
        G7_IO1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - G7_IO2 sampling mode"]
    #[inline(always)]
    pub fn g7_io2(&self) -> G7_IO2_R {
        G7_IO2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - G7_IO3 sampling mode"]
    #[inline(always)]
    pub fn g7_io3(&self) -> G7_IO3_R {
        G7_IO3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - G7_IO4 sampling mode"]
    #[inline(always)]
    pub fn g7_io4(&self) -> G7_IO4_R {
        G7_IO4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - G8_IO1 sampling mode"]
    #[inline(always)]
    pub fn g8_io1(&self) -> G8_IO1_R {
        G8_IO1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - G8_IO2 sampling mode"]
    #[inline(always)]
    pub fn g8_io2(&self) -> G8_IO2_R {
        G8_IO2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - G8_IO3 sampling mode"]
    #[inline(always)]
    pub fn g8_io3(&self) -> G8_IO3_R {
        G8_IO3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - G8_IO4 sampling mode"]
    #[inline(always)]
    pub fn g8_io4(&self) -> G8_IO4_R {
        G8_IO4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - G1_IO1 sampling mode"]
    #[inline(always)]
    pub fn g1_io1(&mut self) -> G1_IO1_W<0> {
        G1_IO1_W::new(self)
    }
    #[doc = "Bit 1 - G1_IO2 sampling mode"]
    #[inline(always)]
    pub fn g1_io2(&mut self) -> G1_IO2_W<1> {
        G1_IO2_W::new(self)
    }
    #[doc = "Bit 2 - G1_IO3 sampling mode"]
    #[inline(always)]
    pub fn g1_io3(&mut self) -> G1_IO3_W<2> {
        G1_IO3_W::new(self)
    }
    #[doc = "Bit 3 - G1_IO4 sampling mode"]
    #[inline(always)]
    pub fn g1_io4(&mut self) -> G1_IO4_W<3> {
        G1_IO4_W::new(self)
    }
    #[doc = "Bit 4 - G2_IO1 sampling mode"]
    #[inline(always)]
    pub fn g2_io1(&mut self) -> G2_IO1_W<4> {
        G2_IO1_W::new(self)
    }
    #[doc = "Bit 5 - G2_IO2 sampling mode"]
    #[inline(always)]
    pub fn g2_io2(&mut self) -> G2_IO2_W<5> {
        G2_IO2_W::new(self)
    }
    #[doc = "Bit 6 - G2_IO3 sampling mode"]
    #[inline(always)]
    pub fn g2_io3(&mut self) -> G2_IO3_W<6> {
        G2_IO3_W::new(self)
    }
    #[doc = "Bit 7 - G2_IO4 sampling mode"]
    #[inline(always)]
    pub fn g2_io4(&mut self) -> G2_IO4_W<7> {
        G2_IO4_W::new(self)
    }
    #[doc = "Bit 8 - G3_IO1 sampling mode"]
    #[inline(always)]
    pub fn g3_io1(&mut self) -> G3_IO1_W<8> {
        G3_IO1_W::new(self)
    }
    #[doc = "Bit 9 - G3_IO2 sampling mode"]
    #[inline(always)]
    pub fn g3_io2(&mut self) -> G3_IO2_W<9> {
        G3_IO2_W::new(self)
    }
    #[doc = "Bit 10 - G3_IO3 sampling mode"]
    #[inline(always)]
    pub fn g3_io3(&mut self) -> G3_IO3_W<10> {
        G3_IO3_W::new(self)
    }
    #[doc = "Bit 11 - G3_IO4 sampling mode"]
    #[inline(always)]
    pub fn g3_io4(&mut self) -> G3_IO4_W<11> {
        G3_IO4_W::new(self)
    }
    #[doc = "Bit 12 - G4_IO1 sampling mode"]
    #[inline(always)]
    pub fn g4_io1(&mut self) -> G4_IO1_W<12> {
        G4_IO1_W::new(self)
    }
    #[doc = "Bit 13 - G4_IO2 sampling mode"]
    #[inline(always)]
    pub fn g4_io2(&mut self) -> G4_IO2_W<13> {
        G4_IO2_W::new(self)
    }
    #[doc = "Bit 14 - G4_IO3 sampling mode"]
    #[inline(always)]
    pub fn g4_io3(&mut self) -> G4_IO3_W<14> {
        G4_IO3_W::new(self)
    }
    #[doc = "Bit 15 - G4_IO4 sampling mode"]
    #[inline(always)]
    pub fn g4_io4(&mut self) -> G4_IO4_W<15> {
        G4_IO4_W::new(self)
    }
    #[doc = "Bit 16 - G5_IO1 sampling mode"]
    #[inline(always)]
    pub fn g5_io1(&mut self) -> G5_IO1_W<16> {
        G5_IO1_W::new(self)
    }
    #[doc = "Bit 17 - G5_IO2 sampling mode"]
    #[inline(always)]
    pub fn g5_io2(&mut self) -> G5_IO2_W<17> {
        G5_IO2_W::new(self)
    }
    #[doc = "Bit 18 - G5_IO3 sampling mode"]
    #[inline(always)]
    pub fn g5_io3(&mut self) -> G5_IO3_W<18> {
        G5_IO3_W::new(self)
    }
    #[doc = "Bit 19 - G5_IO4 sampling mode"]
    #[inline(always)]
    pub fn g5_io4(&mut self) -> G5_IO4_W<19> {
        G5_IO4_W::new(self)
    }
    #[doc = "Bit 20 - G6_IO1 sampling mode"]
    #[inline(always)]
    pub fn g6_io1(&mut self) -> G6_IO1_W<20> {
        G6_IO1_W::new(self)
    }
    #[doc = "Bit 21 - G6_IO2 sampling mode"]
    #[inline(always)]
    pub fn g6_io2(&mut self) -> G6_IO2_W<21> {
        G6_IO2_W::new(self)
    }
    #[doc = "Bit 22 - G6_IO3 sampling mode"]
    #[inline(always)]
    pub fn g6_io3(&mut self) -> G6_IO3_W<22> {
        G6_IO3_W::new(self)
    }
    #[doc = "Bit 23 - G6_IO4 sampling mode"]
    #[inline(always)]
    pub fn g6_io4(&mut self) -> G6_IO4_W<23> {
        G6_IO4_W::new(self)
    }
    #[doc = "Bit 24 - G7_IO1 sampling mode"]
    #[inline(always)]
    pub fn g7_io1(&mut self) -> G7_IO1_W<24> {
        G7_IO1_W::new(self)
    }
    #[doc = "Bit 25 - G7_IO2 sampling mode"]
    #[inline(always)]
    pub fn g7_io2(&mut self) -> G7_IO2_W<25> {
        G7_IO2_W::new(self)
    }
    #[doc = "Bit 26 - G7_IO3 sampling mode"]
    #[inline(always)]
    pub fn g7_io3(&mut self) -> G7_IO3_W<26> {
        G7_IO3_W::new(self)
    }
    #[doc = "Bit 27 - G7_IO4 sampling mode"]
    #[inline(always)]
    pub fn g7_io4(&mut self) -> G7_IO4_W<27> {
        G7_IO4_W::new(self)
    }
    #[doc = "Bit 28 - G8_IO1 sampling mode"]
    #[inline(always)]
    pub fn g8_io1(&mut self) -> G8_IO1_W<28> {
        G8_IO1_W::new(self)
    }
    #[doc = "Bit 29 - G8_IO2 sampling mode"]
    #[inline(always)]
    pub fn g8_io2(&mut self) -> G8_IO2_W<29> {
        G8_IO2_W::new(self)
    }
    #[doc = "Bit 30 - G8_IO3 sampling mode"]
    #[inline(always)]
    pub fn g8_io3(&mut self) -> G8_IO3_W<30> {
        G8_IO3_W::new(self)
    }
    #[doc = "Bit 31 - G8_IO4 sampling mode"]
    #[inline(always)]
    pub fn g8_io4(&mut self) -> G8_IO4_W<31> {
        G8_IO4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O sampling control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioscr](index.html) module"]
pub struct IOSCR_SPEC;
impl crate::RegisterSpec for IOSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioscr::R](R) reader structure"]
impl crate::Readable for IOSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioscr::W](W) writer structure"]
impl crate::Writable for IOSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOSCR to value 0"]
impl crate::Resettable for IOSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

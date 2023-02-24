#[doc = "Register `BRR` writer"]
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BR0_AW {
    #[doc = "0: No action on the corresponding ODx bit"]
    NoAction = 0,
    #[doc = "1: Reset the ODx bit"]
    Reset = 1,
}
impl From<BR0_AW> for bool {
    #[inline(always)]
    fn from(variant: BR0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR0` writer - Port Reset bit"]
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, BR0_AW, O>;
impl<'a, const O: u8> BR0_W<'a, O> {
    #[doc = "No action on the corresponding ODx bit"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0_AW::NoAction)
    }
    #[doc = "Reset the ODx bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0_AW::Reset)
    }
}
#[doc = "Port Reset bit"]
pub use BR0_AW as BR1_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR2_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR3_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR4_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR5_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR6_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR7_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR8_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR9_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR10_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR11_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR12_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR13_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR14_AW;
#[doc = "Port Reset bit"]
pub use BR0_AW as BR15_AW;
#[doc = "Field `BR1` writer - Port Reset bit"]
pub use BR0_W as BR1_W;
#[doc = "Field `BR2` writer - Port Reset bit"]
pub use BR0_W as BR2_W;
#[doc = "Field `BR3` writer - Port Reset bit"]
pub use BR0_W as BR3_W;
#[doc = "Field `BR4` writer - Port Reset bit"]
pub use BR0_W as BR4_W;
#[doc = "Field `BR5` writer - Port Reset bit"]
pub use BR0_W as BR5_W;
#[doc = "Field `BR6` writer - Port Reset bit"]
pub use BR0_W as BR6_W;
#[doc = "Field `BR7` writer - Port Reset bit"]
pub use BR0_W as BR7_W;
#[doc = "Field `BR8` writer - Port Reset bit"]
pub use BR0_W as BR8_W;
#[doc = "Field `BR9` writer - Port Reset bit"]
pub use BR0_W as BR9_W;
#[doc = "Field `BR10` writer - Port Reset bit"]
pub use BR0_W as BR10_W;
#[doc = "Field `BR11` writer - Port Reset bit"]
pub use BR0_W as BR11_W;
#[doc = "Field `BR12` writer - Port Reset bit"]
pub use BR0_W as BR12_W;
#[doc = "Field `BR13` writer - Port Reset bit"]
pub use BR0_W as BR13_W;
#[doc = "Field `BR14` writer - Port Reset bit"]
pub use BR0_W as BR14_W;
#[doc = "Field `BR15` writer - Port Reset bit"]
pub use BR0_W as BR15_W;
impl W {
    #[doc = "Bit 0 - Port Reset bit"]
    #[inline(always)]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    #[doc = "Bit 1 - Port Reset bit"]
    #[inline(always)]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    #[doc = "Bit 2 - Port Reset bit"]
    #[inline(always)]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline(always)]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    #[doc = "Bit 4 - Port Reset bit"]
    #[inline(always)]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    #[doc = "Bit 5 - Port Reset bit"]
    #[inline(always)]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    #[doc = "Bit 6 - Port Reset bit"]
    #[inline(always)]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    #[doc = "Bit 7 - Port Reset bit"]
    #[inline(always)]
    pub fn br7(&mut self) -> BR7_W<7> {
        BR7_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset bit"]
    #[inline(always)]
    pub fn br8(&mut self) -> BR8_W<8> {
        BR8_W::new(self)
    }
    #[doc = "Bit 9 - Port Reset bit"]
    #[inline(always)]
    pub fn br9(&mut self) -> BR9_W<9> {
        BR9_W::new(self)
    }
    #[doc = "Bit 10 - Port Reset bit"]
    #[inline(always)]
    pub fn br10(&mut self) -> BR10_W<10> {
        BR10_W::new(self)
    }
    #[doc = "Bit 11 - Port Reset bit"]
    #[inline(always)]
    pub fn br11(&mut self) -> BR11_W<11> {
        BR11_W::new(self)
    }
    #[doc = "Bit 12 - Port Reset bit"]
    #[inline(always)]
    pub fn br12(&mut self) -> BR12_W<12> {
        BR12_W::new(self)
    }
    #[doc = "Bit 13 - Port Reset bit"]
    #[inline(always)]
    pub fn br13(&mut self) -> BR13_W<13> {
        BR13_W::new(self)
    }
    #[doc = "Bit 14 - Port Reset bit"]
    #[inline(always)]
    pub fn br14(&mut self) -> BR14_W<14> {
        BR14_W::new(self)
    }
    #[doc = "Bit 15 - Port Reset bit"]
    #[inline(always)]
    pub fn br15(&mut self) -> BR15_W<15> {
        BR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO port bit reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](index.html) module"]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [brr::W](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

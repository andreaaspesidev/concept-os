#[doc = "Register `AWD2CR` reader"]
pub struct R(crate::R<AWD2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD2CR` writer"]
pub struct W(crate::W<AWD2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD2CR_SPEC>;
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
impl From<crate::W<AWD2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AWD2CH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD2CH0_A {
    #[doc = "0: Input channel not monitored by AWDx"]
    NotMonitored = 0,
    #[doc = "1: Input channel monitored by AWDx"]
    Monitored = 1,
}
impl From<AWD2CH0_A> for bool {
    #[inline(always)]
    fn from(variant: AWD2CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD2CH0` reader - AWD2CH"]
pub type AWD2CH0_R = crate::BitReader<AWD2CH0_A>;
impl AWD2CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD2CH0_A {
        match self.bits {
            false => AWD2CH0_A::NotMonitored,
            true => AWD2CH0_A::Monitored,
        }
    }
    #[doc = "Checks if the value of the field is `NotMonitored`"]
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD2CH0_A::NotMonitored
    }
    #[doc = "Checks if the value of the field is `Monitored`"]
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD2CH0_A::Monitored
    }
}
#[doc = "Field `AWD2CH0` writer - AWD2CH"]
pub type AWD2CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AWD2CR_SPEC, AWD2CH0_A, O>;
impl<'a, const O: u8> AWD2CH0_W<'a, O> {
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::NotMonitored)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD2CH0_A::Monitored)
    }
}
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH1_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH2_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH3_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH4_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH5_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH6_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH7_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH8_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH9_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH10_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH11_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH12_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH13_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH14_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH15_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH16_A;
#[doc = "AWD2CH"]
pub use AWD2CH0_A as AWD2CH17_A;
#[doc = "Field `AWD2CH1` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH1_R;
#[doc = "Field `AWD2CH2` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH2_R;
#[doc = "Field `AWD2CH3` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH3_R;
#[doc = "Field `AWD2CH4` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH4_R;
#[doc = "Field `AWD2CH5` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH5_R;
#[doc = "Field `AWD2CH6` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH6_R;
#[doc = "Field `AWD2CH7` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH7_R;
#[doc = "Field `AWD2CH8` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH8_R;
#[doc = "Field `AWD2CH9` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH9_R;
#[doc = "Field `AWD2CH10` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH10_R;
#[doc = "Field `AWD2CH11` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH11_R;
#[doc = "Field `AWD2CH12` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH12_R;
#[doc = "Field `AWD2CH13` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH13_R;
#[doc = "Field `AWD2CH14` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH14_R;
#[doc = "Field `AWD2CH15` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH15_R;
#[doc = "Field `AWD2CH16` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH16_R;
#[doc = "Field `AWD2CH17` reader - AWD2CH"]
pub use AWD2CH0_R as AWD2CH17_R;
#[doc = "Field `AWD2CH1` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH1_W;
#[doc = "Field `AWD2CH2` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH2_W;
#[doc = "Field `AWD2CH3` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH3_W;
#[doc = "Field `AWD2CH4` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH4_W;
#[doc = "Field `AWD2CH5` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH5_W;
#[doc = "Field `AWD2CH6` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH6_W;
#[doc = "Field `AWD2CH7` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH7_W;
#[doc = "Field `AWD2CH8` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH8_W;
#[doc = "Field `AWD2CH9` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH9_W;
#[doc = "Field `AWD2CH10` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH10_W;
#[doc = "Field `AWD2CH11` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH11_W;
#[doc = "Field `AWD2CH12` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH12_W;
#[doc = "Field `AWD2CH13` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH13_W;
#[doc = "Field `AWD2CH14` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH14_W;
#[doc = "Field `AWD2CH15` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH15_W;
#[doc = "Field `AWD2CH16` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH16_W;
#[doc = "Field `AWD2CH17` writer - AWD2CH"]
pub use AWD2CH0_W as AWD2CH17_W;
impl R {
    #[doc = "Bit 1 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch0(&self) -> AWD2CH0_R {
        AWD2CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch1(&self) -> AWD2CH1_R {
        AWD2CH1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch2(&self) -> AWD2CH2_R {
        AWD2CH2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch3(&self) -> AWD2CH3_R {
        AWD2CH3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch4(&self) -> AWD2CH4_R {
        AWD2CH4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch5(&self) -> AWD2CH5_R {
        AWD2CH5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch6(&self) -> AWD2CH6_R {
        AWD2CH6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch7(&self) -> AWD2CH7_R {
        AWD2CH7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch8(&self) -> AWD2CH8_R {
        AWD2CH8_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch9(&self) -> AWD2CH9_R {
        AWD2CH9_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch10(&self) -> AWD2CH10_R {
        AWD2CH10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch11(&self) -> AWD2CH11_R {
        AWD2CH11_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch12(&self) -> AWD2CH12_R {
        AWD2CH12_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch13(&self) -> AWD2CH13_R {
        AWD2CH13_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch14(&self) -> AWD2CH14_R {
        AWD2CH14_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch15(&self) -> AWD2CH15_R {
        AWD2CH15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch16(&self) -> AWD2CH16_R {
        AWD2CH16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch17(&self) -> AWD2CH17_R {
        AWD2CH17_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch0(&mut self) -> AWD2CH0_W<1> {
        AWD2CH0_W::new(self)
    }
    #[doc = "Bit 2 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch1(&mut self) -> AWD2CH1_W<2> {
        AWD2CH1_W::new(self)
    }
    #[doc = "Bit 3 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch2(&mut self) -> AWD2CH2_W<3> {
        AWD2CH2_W::new(self)
    }
    #[doc = "Bit 4 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch3(&mut self) -> AWD2CH3_W<4> {
        AWD2CH3_W::new(self)
    }
    #[doc = "Bit 5 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch4(&mut self) -> AWD2CH4_W<5> {
        AWD2CH4_W::new(self)
    }
    #[doc = "Bit 6 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch5(&mut self) -> AWD2CH5_W<6> {
        AWD2CH5_W::new(self)
    }
    #[doc = "Bit 7 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch6(&mut self) -> AWD2CH6_W<7> {
        AWD2CH6_W::new(self)
    }
    #[doc = "Bit 8 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch7(&mut self) -> AWD2CH7_W<8> {
        AWD2CH7_W::new(self)
    }
    #[doc = "Bit 9 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch8(&mut self) -> AWD2CH8_W<9> {
        AWD2CH8_W::new(self)
    }
    #[doc = "Bit 10 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch9(&mut self) -> AWD2CH9_W<10> {
        AWD2CH9_W::new(self)
    }
    #[doc = "Bit 11 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch10(&mut self) -> AWD2CH10_W<11> {
        AWD2CH10_W::new(self)
    }
    #[doc = "Bit 12 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch11(&mut self) -> AWD2CH11_W<12> {
        AWD2CH11_W::new(self)
    }
    #[doc = "Bit 13 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch12(&mut self) -> AWD2CH12_W<13> {
        AWD2CH12_W::new(self)
    }
    #[doc = "Bit 14 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch13(&mut self) -> AWD2CH13_W<14> {
        AWD2CH13_W::new(self)
    }
    #[doc = "Bit 15 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch14(&mut self) -> AWD2CH14_W<15> {
        AWD2CH14_W::new(self)
    }
    #[doc = "Bit 16 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch15(&mut self) -> AWD2CH15_W<16> {
        AWD2CH15_W::new(self)
    }
    #[doc = "Bit 17 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch16(&mut self) -> AWD2CH16_W<17> {
        AWD2CH16_W::new(self)
    }
    #[doc = "Bit 18 - AWD2CH"]
    #[inline(always)]
    pub fn awd2ch17(&mut self) -> AWD2CH17_W<18> {
        AWD2CH17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Watchdog 2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd2cr](index.html) module"]
pub struct AWD2CR_SPEC;
impl crate::RegisterSpec for AWD2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd2cr::R](R) reader structure"]
impl crate::Readable for AWD2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd2cr::W](W) writer structure"]
impl crate::Writable for AWD2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWD2CR to value 0"]
impl crate::Resettable for AWD2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

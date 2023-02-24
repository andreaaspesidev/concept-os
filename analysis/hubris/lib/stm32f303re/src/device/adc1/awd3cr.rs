#[doc = "Register `AWD3CR` reader"]
pub struct R(crate::R<AWD3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD3CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD3CR` writer"]
pub struct W(crate::W<AWD3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD3CR_SPEC>;
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
impl From<crate::W<AWD3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD3CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AWD3CH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD3CH0_A {
    #[doc = "0: Input channel not monitored by AWDx"]
    NotMonitored = 0,
    #[doc = "1: Input channel monitored by AWDx"]
    Monitored = 1,
}
impl From<AWD3CH0_A> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD3CH0` reader - AWD3CH"]
pub type AWD3CH0_R = crate::BitReader<AWD3CH0_A>;
impl AWD3CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD3CH0_A {
        match self.bits {
            false => AWD3CH0_A::NotMonitored,
            true => AWD3CH0_A::Monitored,
        }
    }
    #[doc = "Checks if the value of the field is `NotMonitored`"]
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD3CH0_A::NotMonitored
    }
    #[doc = "Checks if the value of the field is `Monitored`"]
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD3CH0_A::Monitored
    }
}
#[doc = "Field `AWD3CH0` writer - AWD3CH"]
pub type AWD3CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AWD3CR_SPEC, AWD3CH0_A, O>;
impl<'a, const O: u8> AWD3CH0_W<'a, O> {
    #[doc = "Input channel not monitored by AWDx"]
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::NotMonitored)
    }
    #[doc = "Input channel monitored by AWDx"]
    #[inline(always)]
    pub fn monitored(self) -> &'a mut W {
        self.variant(AWD3CH0_A::Monitored)
    }
}
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH1_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH2_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH3_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH4_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH5_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH6_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH7_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH8_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH9_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH10_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH11_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH12_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH13_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH14_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH15_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH16_A;
#[doc = "AWD3CH"]
pub use AWD3CH0_A as AWD3CH17_A;
#[doc = "Field `AWD3CH1` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH1_R;
#[doc = "Field `AWD3CH2` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH2_R;
#[doc = "Field `AWD3CH3` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH3_R;
#[doc = "Field `AWD3CH4` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH4_R;
#[doc = "Field `AWD3CH5` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH5_R;
#[doc = "Field `AWD3CH6` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH6_R;
#[doc = "Field `AWD3CH7` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH7_R;
#[doc = "Field `AWD3CH8` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH8_R;
#[doc = "Field `AWD3CH9` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH9_R;
#[doc = "Field `AWD3CH10` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH10_R;
#[doc = "Field `AWD3CH11` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH11_R;
#[doc = "Field `AWD3CH12` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH12_R;
#[doc = "Field `AWD3CH13` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH13_R;
#[doc = "Field `AWD3CH14` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH14_R;
#[doc = "Field `AWD3CH15` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH15_R;
#[doc = "Field `AWD3CH16` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH16_R;
#[doc = "Field `AWD3CH17` reader - AWD3CH"]
pub use AWD3CH0_R as AWD3CH17_R;
#[doc = "Field `AWD3CH1` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH1_W;
#[doc = "Field `AWD3CH2` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH2_W;
#[doc = "Field `AWD3CH3` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH3_W;
#[doc = "Field `AWD3CH4` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH4_W;
#[doc = "Field `AWD3CH5` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH5_W;
#[doc = "Field `AWD3CH6` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH6_W;
#[doc = "Field `AWD3CH7` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH7_W;
#[doc = "Field `AWD3CH8` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH8_W;
#[doc = "Field `AWD3CH9` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH9_W;
#[doc = "Field `AWD3CH10` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH10_W;
#[doc = "Field `AWD3CH11` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH11_W;
#[doc = "Field `AWD3CH12` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH12_W;
#[doc = "Field `AWD3CH13` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH13_W;
#[doc = "Field `AWD3CH14` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH14_W;
#[doc = "Field `AWD3CH15` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH15_W;
#[doc = "Field `AWD3CH16` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH16_W;
#[doc = "Field `AWD3CH17` writer - AWD3CH"]
pub use AWD3CH0_W as AWD3CH17_W;
impl R {
    #[doc = "Bit 1 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH0_R {
        AWD3CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH1_R {
        AWD3CH1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH2_R {
        AWD3CH2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH3_R {
        AWD3CH3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH4_R {
        AWD3CH4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH5_R {
        AWD3CH5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH6_R {
        AWD3CH6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH7_R {
        AWD3CH7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH8_R {
        AWD3CH8_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH9_R {
        AWD3CH9_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH10_R {
        AWD3CH10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH11_R {
        AWD3CH11_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH12_R {
        AWD3CH12_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH13_R {
        AWD3CH13_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH14_R {
        AWD3CH14_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH15_R {
        AWD3CH15_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH16_R {
        AWD3CH16_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH17_R {
        AWD3CH17_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch0(&mut self) -> AWD3CH0_W<1> {
        AWD3CH0_W::new(self)
    }
    #[doc = "Bit 2 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch1(&mut self) -> AWD3CH1_W<2> {
        AWD3CH1_W::new(self)
    }
    #[doc = "Bit 3 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch2(&mut self) -> AWD3CH2_W<3> {
        AWD3CH2_W::new(self)
    }
    #[doc = "Bit 4 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch3(&mut self) -> AWD3CH3_W<4> {
        AWD3CH3_W::new(self)
    }
    #[doc = "Bit 5 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch4(&mut self) -> AWD3CH4_W<5> {
        AWD3CH4_W::new(self)
    }
    #[doc = "Bit 6 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch5(&mut self) -> AWD3CH5_W<6> {
        AWD3CH5_W::new(self)
    }
    #[doc = "Bit 7 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch6(&mut self) -> AWD3CH6_W<7> {
        AWD3CH6_W::new(self)
    }
    #[doc = "Bit 8 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch7(&mut self) -> AWD3CH7_W<8> {
        AWD3CH7_W::new(self)
    }
    #[doc = "Bit 9 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch8(&mut self) -> AWD3CH8_W<9> {
        AWD3CH8_W::new(self)
    }
    #[doc = "Bit 10 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch9(&mut self) -> AWD3CH9_W<10> {
        AWD3CH9_W::new(self)
    }
    #[doc = "Bit 11 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch10(&mut self) -> AWD3CH10_W<11> {
        AWD3CH10_W::new(self)
    }
    #[doc = "Bit 12 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch11(&mut self) -> AWD3CH11_W<12> {
        AWD3CH11_W::new(self)
    }
    #[doc = "Bit 13 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch12(&mut self) -> AWD3CH12_W<13> {
        AWD3CH12_W::new(self)
    }
    #[doc = "Bit 14 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch13(&mut self) -> AWD3CH13_W<14> {
        AWD3CH13_W::new(self)
    }
    #[doc = "Bit 15 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch14(&mut self) -> AWD3CH14_W<15> {
        AWD3CH14_W::new(self)
    }
    #[doc = "Bit 16 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch15(&mut self) -> AWD3CH15_W<16> {
        AWD3CH15_W::new(self)
    }
    #[doc = "Bit 17 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch16(&mut self) -> AWD3CH16_W<17> {
        AWD3CH16_W::new(self)
    }
    #[doc = "Bit 18 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch17(&mut self) -> AWD3CH17_W<18> {
        AWD3CH17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Watchdog 3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd3cr](index.html) module"]
pub struct AWD3CR_SPEC;
impl crate::RegisterSpec for AWD3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd3cr::R](R) reader structure"]
impl crate::Readable for AWD3CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd3cr::W](W) writer structure"]
impl crate::Writable for AWD3CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for AWD3CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

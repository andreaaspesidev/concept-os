#[doc = "Register `APB1_FZ` reader"]
pub struct R(crate::R<APB1_FZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1_FZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1_FZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1_FZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1_FZ` writer"]
pub struct W(crate::W<APB1_FZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1_FZ_SPEC>;
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
impl From<crate::W<APB1_FZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1_FZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIM2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM3_STOP` reader - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM3_STOP` writer - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM4_STOP` reader - Debug Timer 4 stopped when Core is halted"]
pub type DBG_TIM4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM4_STOP` writer - Debug Timer 4 stopped when Core is halted"]
pub type DBG_TIM4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM5_STOP` reader - Debug Timer 5 stopped when Core is halted"]
pub type DBG_TIM5_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM5_STOP` writer - Debug Timer 5 stopped when Core is halted"]
pub type DBG_TIM5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIM6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIM6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM7_STOP` reader - Debug Timer 7 stopped when Core is halted"]
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM7_STOP` writer - Debug Timer 7 stopped when Core is halted"]
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM12_STOP` reader - Debug Timer 12 stopped when Core is halted"]
pub type DBG_TIM12_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM12_STOP` writer - Debug Timer 12 stopped when Core is halted"]
pub type DBG_TIM12_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM13_STOP` reader - Debug Timer 13 stopped when Core is halted"]
pub type DBG_TIM13_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM13_STOP` writer - Debug Timer 13 stopped when Core is halted"]
pub type DBG_TIM13_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIMER14_STOP` reader - Debug Timer 14 stopped when Core is halted"]
pub type DBG_TIMER14_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER14_STOP` writer - Debug Timer 14 stopped when Core is halted"]
pub type DBG_TIMER14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_TIM18_STOP` reader - Debug Timer 18 stopped when Core is halted"]
pub type DBG_TIM18_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM18_STOP` writer - Debug Timer 18 stopped when Core is halted"]
pub type DBG_TIM18_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub type I2C1_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub type I2C1_SMBUS_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub type I2C2_SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub type I2C2_SMBUS_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
#[doc = "Field `DBG_CAN_STOP` reader - Debug CAN stopped when core is halted"]
pub type DBG_CAN_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_CAN_STOP` writer - Debug CAN stopped when core is halted"]
pub type DBG_CAN_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1_FZ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Timer 4 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 5 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Timer 12 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Timer 13 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&self) -> DBG_TIMER14_STOP_R {
        DBG_TIMER14_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Timer 18 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&self) -> DBG_TIM18_STOP_R {
        DBG_TIM18_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2C1_SMBUS_TIMEOUT_R {
        I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2C2_SMBUS_TIMEOUT_R {
        I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Debug CAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DBG_CAN_STOP_R {
        DBG_CAN_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<0> {
        DBG_TIM2_STOP_W::new(self)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Debug Timer 4 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<2> {
        DBG_TIM4_STOP_W::new(self)
    }
    #[doc = "Bit 3 - Debug Timer 5 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<3> {
        DBG_TIM5_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<4> {
        DBG_TIM6_STOP_W::new(self)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
    }
    #[doc = "Bit 6 - Debug Timer 12 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<6> {
        DBG_TIM12_STOP_W::new(self)
    }
    #[doc = "Bit 7 - Debug Timer 13 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<7> {
        DBG_TIM13_STOP_W::new(self)
    }
    #[doc = "Bit 8 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&mut self) -> DBG_TIMER14_STOP_W<8> {
        DBG_TIMER14_STOP_W::new(self)
    }
    #[doc = "Bit 9 - Debug Timer 18 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&mut self) -> DBG_TIM18_STOP_W<9> {
        DBG_TIM18_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<11> {
        DBG_WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&mut self) -> I2C1_SMBUS_TIMEOUT_W<21> {
        I2C1_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&mut self) -> I2C2_SMBUS_TIMEOUT_W<22> {
        I2C2_SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 25 - Debug CAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DBG_CAN_STOP_W<25> {
        DBG_CAN_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Low Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1_fz](index.html) module"]
pub struct APB1_FZ_SPEC;
impl crate::RegisterSpec for APB1_FZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1_fz::R](R) reader structure"]
impl crate::Readable for APB1_FZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1_fz::W](W) writer structure"]
impl crate::Writable for APB1_FZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1_FZ to value 0"]
impl crate::Resettable for APB1_FZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

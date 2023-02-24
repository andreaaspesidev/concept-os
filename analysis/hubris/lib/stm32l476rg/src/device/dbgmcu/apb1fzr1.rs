#[doc = "Register `APB1FZR1` reader"]
pub struct R(crate::R<APB1FZR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1FZR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1FZR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1FZR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1FZR1` writer"]
pub struct W(crate::W<APB1FZR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1FZR1_SPEC>;
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
impl From<crate::W<APB1FZR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1FZR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted"]
pub type DBG_TIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted"]
pub type DBG_TIM3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM4_STOP` reader - TIM4 counter stopped when core is halted"]
pub type DBG_TIM4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM4_STOP` writer - TIM4 counter stopped when core is halted"]
pub type DBG_TIM4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM5_STOP` reader - TIM5 counter stopped when core is halted"]
pub type DBG_TIM5_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM5_STOP` writer - TIM5 counter stopped when core is halted"]
pub type DBG_TIM5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIMER6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIMER6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted"]
pub type DBG_TIM7_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted"]
pub type DBG_TIM7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DBG_I2C1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted"]
pub type DBG_I2C1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout mode stopped when core is halted"]
pub type DBG_I2C2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout mode stopped when core is halted"]
pub type DBG_I2C2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_CAN_STOP` reader - bxCAN stopped when core is halted"]
pub type DBG_CAN_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_CAN_STOP` writer - bxCAN stopped when core is halted"]
pub type DBG_CAN_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
#[doc = "Field `DBG_LPTIMER_STOP` reader - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIMER_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_LPTIMER_STOP` writer - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIMER_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1FZR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - bxCAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DBG_CAN_STOP_R {
        DBG_CAN_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptimer_stop(&self) -> DBG_LPTIMER_STOP_R {
        DBG_LPTIMER_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<0> {
        DBG_TIMER2_STOP_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<1> {
        DBG_TIM3_STOP_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<2> {
        DBG_TIM4_STOP_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<3> {
        DBG_TIM5_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W<4> {
        DBG_TIMER6_STOP_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<5> {
        DBG_TIM7_STOP_W::new(self)
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
    #[doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<21> {
        DBG_I2C1_STOP_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<22> {
        DBG_I2C2_STOP_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<23> {
        DBG_I2C3_STOP_W::new(self)
    }
    #[doc = "Bit 25 - bxCAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&mut self) -> DBG_CAN_STOP_W<25> {
        DBG_CAN_STOP_W::new(self)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptimer_stop(&mut self) -> DBG_LPTIMER_STOP_W<31> {
        DBG_LPTIMER_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB Low Freeze Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr1](index.html) module"]
pub struct APB1FZR1_SPEC;
impl crate::RegisterSpec for APB1FZR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1fzr1::R](R) reader structure"]
impl crate::Readable for APB1FZR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1fzr1::W](W) writer structure"]
impl crate::Writable for APB1FZR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1FZR1 to value 0"]
impl crate::Resettable for APB1FZR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

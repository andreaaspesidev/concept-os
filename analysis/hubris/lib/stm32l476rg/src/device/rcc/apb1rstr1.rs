#[doc = "Register `APB1RSTR1` reader"]
pub struct R(crate::R<APB1RSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR1` writer"]
pub struct W(crate::W<APB1RSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR1_SPEC>;
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
impl From<crate::W<APB1RSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTIM1RST` reader - Low Power Timer 1 reset"]
pub type LPTIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1RST` writer - Low Power Timer 1 reset"]
pub type LPTIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `OPAMPRST` reader - OPAMP interface reset"]
pub type OPAMPRST_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPRST` writer - OPAMP interface reset"]
pub type OPAMPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `DAC1RST` reader - DAC1 interface reset"]
pub type DAC1RST_R = crate::BitReader<bool>;
#[doc = "Field `DAC1RST` writer - DAC1 interface reset"]
pub type DAC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader<bool>;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub type CAN1RST_R = crate::BitReader<bool>;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub type CAN1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub type I2C3RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub type I2C3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `I2C2RST` reader - I2C2 reset"]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - I2C2 reset"]
pub type I2C2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `I2C1RST` reader - I2C1 reset"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1 reset"]
pub type I2C1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `UART5RST` reader - UART5 reset"]
pub type UART5RST_R = crate::BitReader<bool>;
#[doc = "Field `UART5RST` writer - UART5 reset"]
pub type UART5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `UART4RST` reader - UART4 reset"]
pub type UART4RST_R = crate::BitReader<bool>;
#[doc = "Field `UART4RST` writer - UART4 reset"]
pub type UART4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `USART3RST` reader - USART3 reset"]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - USART3 reset"]
pub type USART3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `USART2RST` reader - USART2 reset"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART2 reset"]
pub type USART2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `SPI3RST` reader - SPI3 reset"]
pub type SPI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3RST` writer - SPI3 reset"]
pub type SPI3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `SPI2RST` reader - SPI2 reset"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI2 reset"]
pub type SPI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `LCDRST` reader - LCD interface reset"]
pub type LCDRST_R = crate::BitReader<bool>;
#[doc = "Field `LCDRST` writer - LCD interface reset"]
pub type LCDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `TIM7RST` reader - TIM7 timer reset"]
pub type TIM7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM7RST` writer - TIM7 timer reset"]
pub type TIM7RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `TIM6RST` reader - TIM6 timer reset"]
pub type TIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM6RST` writer - TIM6 timer reset"]
pub type TIM6RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `TIM5RST` reader - TIM5 timer reset"]
pub type TIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM5RST` writer - TIM5 timer reset"]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `TIM4RST` reader - TIM3 timer reset"]
pub type TIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM4RST` writer - TIM3 timer reset"]
pub type TIM4RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `TIM3RST` reader - TIM3 timer reset"]
pub type TIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RST` writer - TIM3 timer reset"]
pub type TIM3RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `TIM2RST` reader - TIM2 timer reset"]
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - TIM2 timer reset"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `CRSRST` reader - CRS reset"]
pub type CRSRST_R = crate::BitReader<bool>;
#[doc = "Field `CRSRST` writer - CRS reset"]
pub type CRSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
#[doc = "Field `CAN2RST` reader - CAN2 reset"]
pub type CAN2RST_R = crate::BitReader<bool>;
#[doc = "Field `CAN2RST` writer - CAN2 reset"]
pub type CAN2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP interface reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 interface reset"]
    #[inline(always)]
    pub fn dac1rst(&self) -> DAC1RST_R {
        DAC1RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD interface reset"]
    #[inline(always)]
    pub fn lcdrst(&self) -> LCDRST_R {
        LCDRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 timer reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 24 - CRS reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&self) -> CAN2RST_R {
        CAN2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Low Power Timer 1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<31> {
        LPTIM1RST_W::new(self)
    }
    #[doc = "Bit 30 - OPAMP interface reset"]
    #[inline(always)]
    pub fn opamprst(&mut self) -> OPAMPRST_W<30> {
        OPAMPRST_W::new(self)
    }
    #[doc = "Bit 29 - DAC1 interface reset"]
    #[inline(always)]
    pub fn dac1rst(&mut self) -> DAC1RST_W<29> {
        DAC1RST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&mut self) -> CAN1RST_W<25> {
        CAN1RST_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<23> {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 20 - UART5 reset"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 19 - UART4 reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 18 - USART3 reset"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 9 - LCD interface reset"]
    #[inline(always)]
    pub fn lcdrst(&mut self) -> LCDRST_W<9> {
        LCDRST_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 timer reset"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 timer reset"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 timer reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 timer reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 0 - TIM2 timer reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 24 - CRS reset"]
    #[inline(always)]
    pub fn crsrst(&mut self) -> CRSRST_W<24> {
        CRSRST_W::new(self)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&mut self) -> CAN2RST_W<26> {
        CAN2RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr1](index.html) module"]
pub struct APB1RSTR1_SPEC;
impl crate::RegisterSpec for APB1RSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr1::R](R) reader structure"]
impl crate::Readable for APB1RSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr1::W](W) writer structure"]
impl crate::Writable for APB1RSTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1RSTR1 to value 0"]
impl crate::Resettable for APB1RSTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

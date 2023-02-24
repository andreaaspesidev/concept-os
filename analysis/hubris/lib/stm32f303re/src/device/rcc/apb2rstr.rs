#[doc = "Register `APB2RSTR` reader"]
pub struct R(crate::R<APB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2RSTR` writer"]
pub struct W(crate::W<APB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2RSTR_SPEC>;
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
impl From<crate::W<APB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SYSCFG and COMP reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG and COMP reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST_A>;
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRST_A> {
        match self.bits {
            true => Some(SYSCFGRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST_A::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG and COMP reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, SYSCFGRST_A, O>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::Reset)
    }
}
#[doc = "TIM1 timer reset"]
pub use SYSCFGRST_A as TIM1RST_A;
#[doc = "SPI 1 reset"]
pub use SYSCFGRST_A as SPI1RST_A;
#[doc = "TIM8 timer reset"]
pub use SYSCFGRST_A as TIM8RST_A;
#[doc = "USART1 reset"]
pub use SYSCFGRST_A as USART1RST_A;
#[doc = "TIM15 timer reset"]
pub use SYSCFGRST_A as TIM15RST_A;
#[doc = "TIM16 timer reset"]
pub use SYSCFGRST_A as TIM16RST_A;
#[doc = "TIM17 timer reset"]
pub use SYSCFGRST_A as TIM17RST_A;
#[doc = "SPI4 reset"]
pub use SYSCFGRST_A as SPI4RST_A;
#[doc = "TIM20 timer reset"]
pub use SYSCFGRST_A as TIM20RST_A;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub use SYSCFGRST_R as TIM1RST_R;
#[doc = "Field `SPI1RST` reader - SPI 1 reset"]
pub use SYSCFGRST_R as SPI1RST_R;
#[doc = "Field `TIM8RST` reader - TIM8 timer reset"]
pub use SYSCFGRST_R as TIM8RST_R;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub use SYSCFGRST_R as USART1RST_R;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub use SYSCFGRST_R as TIM15RST_R;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub use SYSCFGRST_R as TIM16RST_R;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub use SYSCFGRST_R as TIM17RST_R;
#[doc = "Field `SPI4RST` reader - SPI4 reset"]
pub use SYSCFGRST_R as SPI4RST_R;
#[doc = "Field `TIM20RST` reader - TIM20 timer reset"]
pub use SYSCFGRST_R as TIM20RST_R;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub use SYSCFGRST_W as TIM1RST_W;
#[doc = "Field `SPI1RST` writer - SPI 1 reset"]
pub use SYSCFGRST_W as SPI1RST_W;
#[doc = "Field `TIM8RST` writer - TIM8 timer reset"]
pub use SYSCFGRST_W as TIM8RST_W;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub use SYSCFGRST_W as USART1RST_W;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub use SYSCFGRST_W as TIM15RST_W;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub use SYSCFGRST_W as TIM16RST_W;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub use SYSCFGRST_W as TIM17RST_W;
#[doc = "Field `SPI4RST` writer - SPI4 reset"]
pub use SYSCFGRST_W as SPI4RST_W;
#[doc = "Field `TIM20RST` writer - TIM20 timer reset"]
pub use SYSCFGRST_W as TIM20RST_W;
impl R {
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM20 timer reset"]
    #[inline(always)]
    pub fn tim20rst(&self) -> TIM20RST_R {
        TIM20RST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG and COMP reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI4 reset"]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W<15> {
        SPI4RST_W::new(self)
    }
    #[doc = "Bit 20 - TIM20 timer reset"]
    #[inline(always)]
    pub fn tim20rst(&mut self) -> TIM20RST_W<20> {
        TIM20RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register (RCC_APB2RSTR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
pub struct APB2RSTR_SPEC;
impl crate::RegisterSpec for APB2RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2rstr::R](R) reader structure"]
impl crate::Readable for APB2RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2rstr::W](W) writer structure"]
impl crate::Writable for APB2RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for APB2RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `APB1ENR2` reader"]
pub struct R(crate::R<APB1ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR2` writer"]
pub struct W(crate::W<APB1ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR2_SPEC>;
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
impl From<crate::W<APB1ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LPTIM2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM2EN_A {
    #[doc = "0: LPTIM2 clock disabled"]
    Disabled = 0,
    #[doc = "1: LPTIM2 clock enabled"]
    Enabled = 1,
}
impl From<LPTIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2EN"]
pub type LPTIM2EN_R = crate::BitReader<LPTIM2EN_A>;
impl LPTIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM2EN_A {
        match self.bits {
            false => LPTIM2EN_A::Disabled,
            true => LPTIM2EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPTIM2EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPTIM2EN_A::Enabled
    }
}
#[doc = "Field `LPTIM2EN` writer - LPTIM2EN"]
pub type LPTIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, LPTIM2EN_A, O>;
impl<'a, const O: u8> LPTIM2EN_W<'a, O> {
    #[doc = "LPTIM2 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::Disabled)
    }
    #[doc = "LPTIM2 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::Enabled)
    }
}
#[doc = "Field `SWPMI1EN` reader - Single wire protocol clock enable"]
pub type SWPMI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SWPMI1EN` writer - Single wire protocol clock enable"]
pub type SWPMI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, bool, O>;
#[doc = "Low power UART 1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPUART1EN_A {
    #[doc = "0: LPUART1 clock disabled"]
    Disabled = 0,
    #[doc = "1: LPUART1 clock enabled"]
    Enabled = 1,
}
impl From<LPUART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPUART1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1EN` reader - Low power UART 1 clock enable"]
pub type LPUART1EN_R = crate::BitReader<LPUART1EN_A>;
impl LPUART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPUART1EN_A {
        match self.bits {
            false => LPUART1EN_A::Disabled,
            true => LPUART1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPUART1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPUART1EN_A::Enabled
    }
}
#[doc = "Field `LPUART1EN` writer - Low power UART 1 clock enable"]
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, LPUART1EN_A, O>;
impl<'a, const O: u8> LPUART1EN_W<'a, O> {
    #[doc = "LPUART1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::Disabled)
    }
    #[doc = "LPUART1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPUART1EN_A::Enabled)
    }
}
#[doc = "Field `I2C4EN` reader - I2C4 clock enable"]
pub type I2C4EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C4EN` writer - I2C4 clock enable"]
pub type I2C4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol clock enable"]
    #[inline(always)]
    pub fn swpmi1en(&self) -> SWPMI1EN_R {
        SWPMI1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<5> {
        LPTIM2EN_W::new(self)
    }
    #[doc = "Bit 2 - Single wire protocol clock enable"]
    #[inline(always)]
    pub fn swpmi1en(&mut self) -> SWPMI1EN_W<2> {
        SWPMI1EN_W::new(self)
    }
    #[doc = "Bit 0 - Low power UART 1 clock enable"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<0> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 1 - I2C4 clock enable"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W<1> {
        I2C4EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr2](index.html) module"]
pub struct APB1ENR2_SPEC;
impl crate::RegisterSpec for APB1ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr2::R](R) reader structure"]
impl crate::Readable for APB1ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr2::W](W) writer structure"]
impl crate::Writable for APB1ENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1ENR2 to value 0"]
impl crate::Resettable for APB1ENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

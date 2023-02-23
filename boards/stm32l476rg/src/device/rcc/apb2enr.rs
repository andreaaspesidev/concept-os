#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFSDMEN` reader - DFSDM timer clock enable"]
pub type DFSDMEN_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMEN` writer - DFSDM timer clock enable"]
pub type DFSDMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SAI2EN` reader - SAI2 clock enable"]
pub type SAI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI2EN` writer - SAI2 clock enable"]
pub type SAI2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SAI1EN` reader - SAI1 clock enable"]
pub type SAI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1EN` writer - SAI1 clock enable"]
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM17EN` reader - TIM17 timer clock enable"]
pub type TIM17EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17EN` writer - TIM17 timer clock enable"]
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM16EN` reader - TIM16 timer clock enable"]
pub type TIM16EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16EN` writer - TIM16 timer clock enable"]
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM15EN` reader - TIM15 timer clock enable"]
pub type TIM15EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM15EN` writer - TIM15 timer clock enable"]
pub type TIM15EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "USART1clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1EN_A {
    #[doc = "0: USART1 clock disabled"]
    Disabled = 0,
    #[doc = "1: USART1 clock enabled"]
    Enabled = 1,
}
impl From<USART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART1EN` reader - USART1clock enable"]
pub type USART1EN_R = crate::BitReader<USART1EN_A>;
impl USART1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1EN_A {
        match self.bits {
            false => USART1EN_A::Disabled,
            true => USART1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USART1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USART1EN_A::Enabled
    }
}
#[doc = "Field `USART1EN` writer - USART1clock enable"]
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, USART1EN_A, O>;
impl<'a, const O: u8> USART1EN_W<'a, O> {
    #[doc = "USART1 clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1EN_A::Disabled)
    }
    #[doc = "USART1 clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1EN_A::Enabled)
    }
}
#[doc = "Field `TIM8EN` reader - TIM8 timer clock enable"]
pub type TIM8EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8EN` writer - TIM8 timer clock enable"]
pub type TIM8EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM1EN` reader - TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - TIM1 timer clock enable"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SDMMCEN` reader - SDMMC clock enable"]
pub type SDMMCEN_R = crate::BitReader<bool>;
#[doc = "Field `SDMMCEN` writer - SDMMC clock enable"]
pub type SDMMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `FIREWALLEN` reader - Firewall clock enable"]
pub type FIREWALLEN_R = crate::BitReader<bool>;
#[doc = "Field `FIREWALLEN` writer - Firewall clock enable"]
pub type FIREWALLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG clock enable"]
pub type SYSCFGEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGEN` writer - SYSCFG clock enable"]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - DFSDM timer clock enable"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - SDMMC clock enable"]
    #[inline(always)]
    pub fn sdmmcen(&self) -> SDMMCEN_R {
        SDMMCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 7 - Firewall clock enable"]
    #[inline(always)]
    pub fn firewallen(&self) -> FIREWALLEN_R {
        FIREWALLEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DFSDM timer clock enable"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<24> {
        DFSDMEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI2 clock enable"]
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W<22> {
        SAI2EN_W::new(self)
    }
    #[doc = "Bit 21 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<21> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<16> {
        TIM15EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 10 - SDMMC clock enable"]
    #[inline(always)]
    pub fn sdmmcen(&mut self) -> SDMMCEN_W<10> {
        SDMMCEN_W::new(self)
    }
    #[doc = "Bit 7 - Firewall clock enable"]
    #[inline(always)]
    pub fn firewallen(&mut self) -> FIREWALLEN_W<7> {
        FIREWALLEN_W::new(self)
    }
    #[doc = "Bit 0 - SYSCFG clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

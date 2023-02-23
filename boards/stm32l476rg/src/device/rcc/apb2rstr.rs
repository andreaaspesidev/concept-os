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
#[doc = "Field `DFSDMRST` reader - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DFSDMRST_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMRST` writer - Digital filters for sigma-delata modulators (DFSDM) reset"]
pub type DFSDMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SAI2RST` reader - Serial audio interface 2 (SAI2) reset"]
pub type SAI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI2RST` writer - Serial audio interface 2 (SAI2) reset"]
pub type SAI2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SAI1RST` reader - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SAI1RST` writer - Serial audio interface 1 (SAI1) reset"]
pub type SAI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub type TIM17RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub type TIM17RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type TIM16RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type TIM16RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub type TIM15RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub type TIM15RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM8RST` reader - TIM8 timer reset"]
pub type TIM8RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM8RST` writer - TIM8 timer reset"]
pub type TIM8RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SDMMCRST` reader - SDMMC reset"]
pub type SDMMCRST_R = crate::BitReader<bool>;
#[doc = "Field `SDMMCRST` writer - SDMMC reset"]
pub type SDMMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
#[doc = "Field `SYSCFGRST` reader - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGRST` writer - System configuration (SYSCFG) reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2RSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - SDMMC reset"]
    #[inline(always)]
    pub fn sdmmcrst(&self) -> SDMMCRST_R {
        SDMMCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset"]
    #[inline(always)]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W<24> {
        DFSDMRST_W::new(self)
    }
    #[doc = "Bit 22 - Serial audio interface 2 (SAI2) reset"]
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<22> {
        SAI2RST_W::new(self)
    }
    #[doc = "Bit 21 - Serial audio interface 1 (SAI1) reset"]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<21> {
        SAI1RST_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<18> {
        TIM17RST_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<17> {
        TIM16RST_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<16> {
        TIM15RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<14> {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 timer reset"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<13> {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<12> {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<11> {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 10 - SDMMC reset"]
    #[inline(always)]
    pub fn sdmmcrst(&mut self) -> SDMMCRST_W<10> {
        SDMMCRST_W::new(self)
    }
    #[doc = "Bit 0 - System configuration (SYSCFG) reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<0> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rstr](index.html) module"]
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

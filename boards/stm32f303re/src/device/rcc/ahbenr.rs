#[doc = "Register `AHBENR` reader"]
pub struct R(crate::R<AHBENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBENR` writer"]
pub struct W(crate::W<AHBENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBENR_SPEC>;
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
impl From<crate::W<AHBENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1EN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
impl DMA1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::Disabled,
            true => DMA1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::Enabled
    }
}
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBENR_SPEC, DMA1EN_A, O>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Enabled)
    }
}
#[doc = "DMA2 clock enable"]
pub use DMA1EN_A as DMA2EN_A;
#[doc = "SRAM interface clock enable"]
pub use DMA1EN_A as SRAMEN_A;
#[doc = "FLITF clock enable"]
pub use DMA1EN_A as FLITFEN_A;
#[doc = "FMC clock enable"]
pub use DMA1EN_A as FMCEN_A;
#[doc = "CRC clock enable"]
pub use DMA1EN_A as CRCEN_A;
#[doc = "IO port H clock enable"]
pub use DMA1EN_A as IOPHEN_A;
#[doc = "I/O port A clock enable"]
pub use DMA1EN_A as IOPAEN_A;
#[doc = "I/O port B clock enable"]
pub use DMA1EN_A as IOPBEN_A;
#[doc = "I/O port C clock enable"]
pub use DMA1EN_A as IOPCEN_A;
#[doc = "I/O port D clock enable"]
pub use DMA1EN_A as IOPDEN_A;
#[doc = "I/O port E clock enable"]
pub use DMA1EN_A as IOPEEN_A;
#[doc = "I/O port F clock enable"]
pub use DMA1EN_A as IOPFEN_A;
#[doc = "I/O port G clock enable"]
pub use DMA1EN_A as IOPGEN_A;
#[doc = "Touch sensing controller clock enable"]
pub use DMA1EN_A as TSCEN_A;
#[doc = "ADC1 and ADC2 clock enable"]
pub use DMA1EN_A as ADC12EN_A;
#[doc = "ADC3 and ADC4 clock enable"]
pub use DMA1EN_A as ADC34EN_A;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub use DMA1EN_R as DMA2EN_R;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub use DMA1EN_R as SRAMEN_R;
#[doc = "Field `FLITFEN` reader - FLITF clock enable"]
pub use DMA1EN_R as FLITFEN_R;
#[doc = "Field `FMCEN` reader - FMC clock enable"]
pub use DMA1EN_R as FMCEN_R;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub use DMA1EN_R as CRCEN_R;
#[doc = "Field `IOPHEN` reader - IO port H clock enable"]
pub use DMA1EN_R as IOPHEN_R;
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub use DMA1EN_R as IOPAEN_R;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub use DMA1EN_R as IOPBEN_R;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub use DMA1EN_R as IOPCEN_R;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub use DMA1EN_R as IOPDEN_R;
#[doc = "Field `IOPEEN` reader - I/O port E clock enable"]
pub use DMA1EN_R as IOPEEN_R;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub use DMA1EN_R as IOPFEN_R;
#[doc = "Field `IOPGEN` reader - I/O port G clock enable"]
pub use DMA1EN_R as IOPGEN_R;
#[doc = "Field `TSCEN` reader - Touch sensing controller clock enable"]
pub use DMA1EN_R as TSCEN_R;
#[doc = "Field `ADC12EN` reader - ADC1 and ADC2 clock enable"]
pub use DMA1EN_R as ADC12EN_R;
#[doc = "Field `ADC34EN` reader - ADC3 and ADC4 clock enable"]
pub use DMA1EN_R as ADC34EN_R;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub use DMA1EN_W as DMA2EN_W;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub use DMA1EN_W as SRAMEN_W;
#[doc = "Field `FLITFEN` writer - FLITF clock enable"]
pub use DMA1EN_W as FLITFEN_W;
#[doc = "Field `FMCEN` writer - FMC clock enable"]
pub use DMA1EN_W as FMCEN_W;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub use DMA1EN_W as CRCEN_W;
#[doc = "Field `IOPHEN` writer - IO port H clock enable"]
pub use DMA1EN_W as IOPHEN_W;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub use DMA1EN_W as IOPAEN_W;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub use DMA1EN_W as IOPBEN_W;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub use DMA1EN_W as IOPCEN_W;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub use DMA1EN_W as IOPDEN_W;
#[doc = "Field `IOPEEN` writer - I/O port E clock enable"]
pub use DMA1EN_W as IOPEEN_W;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub use DMA1EN_W as IOPFEN_W;
#[doc = "Field `IOPGEN` writer - I/O port G clock enable"]
pub use DMA1EN_W as IOPGEN_W;
#[doc = "Field `TSCEN` writer - Touch sensing controller clock enable"]
pub use DMA1EN_W as TSCEN_W;
#[doc = "Field `ADC12EN` writer - ADC1 and ADC2 clock enable"]
pub use DMA1EN_W as ADC12EN_W;
#[doc = "Field `ADC34EN` writer - ADC3 and ADC4 clock enable"]
pub use DMA1EN_W as ADC34EN_W;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - IO port H clock enable"]
    #[inline(always)]
    pub fn iophen(&self) -> IOPHEN_R {
        IOPHEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&self) -> IOPGEN_R {
        IOPGEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 clock enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 clock enable"]
    #[inline(always)]
    pub fn adc34en(&self) -> ADC34EN_R {
        ADC34EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W<2> {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FLITF clock enable"]
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W<4> {
        FLITFEN_W::new(self)
    }
    #[doc = "Bit 5 - FMC clock enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<5> {
        FMCEN_W::new(self)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<6> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 16 - IO port H clock enable"]
    #[inline(always)]
    pub fn iophen(&mut self) -> IOPHEN_W<16> {
        IOPHEN_W::new(self)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<17> {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<18> {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 19 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<19> {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 20 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<20> {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 21 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W<21> {
        IOPEEN_W::new(self)
    }
    #[doc = "Bit 22 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&mut self) -> IOPFEN_W<22> {
        IOPFEN_W::new(self)
    }
    #[doc = "Bit 23 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&mut self) -> IOPGEN_W<23> {
        IOPGEN_W::new(self)
    }
    #[doc = "Bit 24 - Touch sensing controller clock enable"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<24> {
        TSCEN_W::new(self)
    }
    #[doc = "Bit 28 - ADC1 and ADC2 clock enable"]
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W<28> {
        ADC12EN_W::new(self)
    }
    #[doc = "Bit 29 - ADC3 and ADC4 clock enable"]
    #[inline(always)]
    pub fn adc34en(&mut self) -> ADC34EN_W<29> {
        ADC34EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbenr](index.html) module"]
pub struct AHBENR_SPEC;
impl crate::RegisterSpec for AHBENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbenr::R](R) reader structure"]
impl crate::Readable for AHBENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbenr::W](W) writer structure"]
impl crate::Writable for AHBENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AHBENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}

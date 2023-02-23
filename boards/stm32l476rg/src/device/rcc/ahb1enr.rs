#[doc = "Register `AHB1ENR` reader"]
pub struct R(crate::R<AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1ENR` writer"]
pub struct W(crate::W<AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1ENR_SPEC>;
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
impl From<crate::W<AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSCEN` reader - Touch Sensing Controller clock enable"]
pub type TSCEN_R = crate::BitReader<bool>;
#[doc = "Field `TSCEN` writer - Touch Sensing Controller clock enable"]
pub type TSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `FLASHEN` reader - Flash memory interface clock enable"]
pub type FLASHEN_R = crate::BitReader<bool>;
#[doc = "Field `FLASHEN` writer - Flash memory interface clock enable"]
pub type FLASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type DMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type DMA2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
#[doc = "Field `DMA2DEN` reader - DMA2D clock enable"]
pub type DMA2DEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2DEN` writer - DMA2D clock enable"]
pub type DMA2DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Touch Sensing Controller clock enable"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 17 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Touch Sensing Controller clock enable"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<16> {
        TSCEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 8 - Flash memory interface clock enable"]
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<8> {
        FLASHEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 17 - DMA2D clock enable"]
    #[inline(always)]
    pub fn dma2den(&mut self) -> DMA2DEN_W<17> {
        DMA2DEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1enr](index.html) module"]
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1enr::R](R) reader structure"]
impl crate::Readable for AHB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1enr::W](W) writer structure"]
impl crate::Writable for AHB1ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB1ENR to value 0x0100"]
impl crate::Resettable for AHB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}

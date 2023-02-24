#[doc = "Register `PLLSAI2CFGR` reader"]
pub struct R(crate::R<PLLSAI2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAI2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAI2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAI2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSAI2CFGR` writer"]
pub struct W(crate::W<PLLSAI2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAI2CFGR_SPEC>;
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
impl From<crate::W<PLLSAI2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAI2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSAI2R` reader - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub type PLLSAI2R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI2R` writer - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub type PLLSAI2R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI2CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLSAI2REN` reader - PLLSAI2 PLLADC2CLK output enable"]
pub type PLLSAI2REN_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2REN` writer - PLLSAI2 PLLADC2CLK output enable"]
pub type PLLSAI2REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI2P_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable"]
pub type PLLSAI2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI2CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO"]
pub type PLLSAI2N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO"]
pub type PLLSAI2N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI2CFGR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK"]
pub type PLLSAI2PDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK"]
pub type PLLSAI2PDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLLSAI2CFGR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W<25> {
        PLLSAI2R_W::new(self)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W<24> {
        PLLSAI2REN_W::new(self)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W<17> {
        PLLSAI2P_W::new(self)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W<16> {
        PLLSAI2PEN_W::new(self)
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W<8> {
        PLLSAI2N_W::new(self)
    }
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W<27> {
        PLLSAI2PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLSAI2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsai2cfgr](index.html) module"]
pub struct PLLSAI2CFGR_SPEC;
impl crate::RegisterSpec for PLLSAI2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllsai2cfgr::R](R) reader structure"]
impl crate::Readable for PLLSAI2CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllsai2cfgr::W](W) writer structure"]
impl crate::Writable for PLLSAI2CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSAI2CFGR to value 0x1000"]
impl crate::Resettable for PLLSAI2CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}

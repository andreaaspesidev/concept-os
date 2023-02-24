#[doc = "Register `PLLSAI1CFGR` reader"]
pub struct R(crate::R<PLLSAI1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAI1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAI1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAI1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSAI1CFGR` writer"]
pub struct W(crate::W<PLLSAI1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAI1CFGR_SPEC>;
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
impl From<crate::W<PLLSAI1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAI1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSAI1R` reader - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
pub type PLLSAI1R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI1R` writer - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
pub type PLLSAI1R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLSAI1REN` reader - PLLSAI1 PLLADC1CLK output enable"]
pub type PLLSAI1REN_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1REN` writer - PLLSAI1 PLLADC1CLK output enable"]
pub type PLLSAI1REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1Q` reader - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
pub type PLLSAI1Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI1Q` writer - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
pub type PLLSAI1Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLSAI1QEN` reader - SAI1PLL PLLUSB2CLK output enable"]
pub type PLLSAI1QEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1QEN` writer - SAI1PLL PLLUSB2CLK output enable"]
pub type PLLSAI1QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1P` reader - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI1P_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1P` writer - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
pub type PLLSAI1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1PEN` reader - SAI1PLL PLLSAI1CLK output enable"]
pub type PLLSAI1PEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1PEN` writer - SAI1PLL PLLSAI1CLK output enable"]
pub type PLLSAI1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLSAI1CFGR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1N` reader - SAI1PLL multiplication factor for VCO"]
pub type PLLSAI1N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI1N` writer - SAI1PLL multiplication factor for VCO"]
pub type PLLSAI1N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLLSAI1PDIV` reader - PLLSAI1 division factor for PLLSAI1CLK"]
pub type PLLSAI1PDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSAI1PDIV` writer - PLLSAI1 division factor for PLLSAI1CLK"]
pub type PLLSAI1PDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLLSAI1CFGR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&self) -> PLLSAI1R_R {
        PLLSAI1R_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&self) -> PLLSAI1REN_R {
        PLLSAI1REN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&self) -> PLLSAI1Q_R {
        PLLSAI1Q_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&self) -> PLLSAI1QEN_R {
        PLLSAI1QEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&self) -> PLLSAI1P_R {
        PLLSAI1P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&self) -> PLLSAI1PEN_R {
        PLLSAI1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&self) -> PLLSAI1N_R {
        PLLSAI1N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK"]
    #[inline(always)]
    pub fn pllsai1pdiv(&self) -> PLLSAI1PDIV_R {
        PLLSAI1PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai1r(&mut self) -> PLLSAI1R_W<25> {
        PLLSAI1R_W::new(self)
    }
    #[doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1ren(&mut self) -> PLLSAI1REN_W<24> {
        PLLSAI1REN_W::new(self)
    }
    #[doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)"]
    #[inline(always)]
    pub fn pllsai1q(&mut self) -> PLLSAI1Q_W<21> {
        PLLSAI1Q_W::new(self)
    }
    #[doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable"]
    #[inline(always)]
    pub fn pllsai1qen(&mut self) -> PLLSAI1QEN_W<20> {
        PLLSAI1QEN_W::new(self)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai1p(&mut self) -> PLLSAI1P_W<17> {
        PLLSAI1P_W::new(self)
    }
    #[doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable"]
    #[inline(always)]
    pub fn pllsai1pen(&mut self) -> PLLSAI1PEN_W<16> {
        PLLSAI1PEN_W::new(self)
    }
    #[doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai1n(&mut self) -> PLLSAI1N_W<8> {
        PLLSAI1N_W::new(self)
    }
    #[doc = "Bits 27:31 - PLLSAI1 division factor for PLLSAI1CLK"]
    #[inline(always)]
    pub fn pllsai1pdiv(&mut self) -> PLLSAI1PDIV_W<27> {
        PLLSAI1PDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLSAI1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsai1cfgr](index.html) module"]
pub struct PLLSAI1CFGR_SPEC;
impl crate::RegisterSpec for PLLSAI1CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllsai1cfgr::R](R) reader structure"]
impl crate::Readable for PLLSAI1CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllsai1cfgr::W](W) writer structure"]
impl crate::Writable for PLLSAI1CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSAI1CFGR to value 0x1000"]
impl crate::Resettable for PLLSAI1CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}

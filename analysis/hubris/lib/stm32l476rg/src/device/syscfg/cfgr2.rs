#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPF` reader - SRAM2 parity error flag"]
pub type SPF_R = crate::BitReader<bool>;
#[doc = "Field `SPF` writer - SRAM2 parity error flag"]
pub type SPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `ECCL` writer - ECC Lock"]
pub type ECCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `PVDL` writer - PVD lock enable bit"]
pub type PVDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `SPL` writer - SRAM2 parity lock bit"]
pub type SPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `CLL` writer - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
pub type CLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SRAM2 parity error flag"]
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W<8> {
        SPF_W::new(self)
    }
    #[doc = "Bit 3 - ECC Lock"]
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W<3> {
        ECCL_W::new(self)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<2> {
        PVDL_W::new(self)
    }
    #[doc = "Bit 1 - SRAM2 parity lock bit"]
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W<1> {
        SPL_W::new(self)
    }
    #[doc = "Bit 0 - Cortex-M4 LOCKUP (Hardfault) output enable bit"]
    #[inline(always)]
    pub fn cll(&mut self) -> CLL_W<0> {
        CLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

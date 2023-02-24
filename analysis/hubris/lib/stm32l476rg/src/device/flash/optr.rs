#[doc = "Register `OPTR` reader"]
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTR` writer"]
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDP` reader - Read protection level"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BOR_LEV` reader - BOR reset Level"]
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOR_LEV` writer - BOR reset Level"]
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type NRST_STOP_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type NRST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type NRST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type NRST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IDWG_SW_R = crate::BitReader<bool>;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IDWG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WWDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `BFB2` reader - Dual-bank boot"]
pub type BFB2_R = crate::BitReader<bool>;
#[doc = "Field `BFB2` writer - Dual-bank boot"]
pub type BFB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `DUALBANK` reader - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
pub type DUALBANK_R = crate::BitReader<bool>;
#[doc = "Field `DUALBANK` writer - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
pub type DUALBANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type NBOOT1_R = crate::BitReader<bool>;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type NBOOT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `SRAM2_PE` reader - SRAM2 parity check enable"]
pub type SRAM2_PE_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2_PE` writer - SRAM2 parity check enable"]
pub type SRAM2_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `SRAM2_RST` reader - SRAM2 Erase when system reset"]
pub type SRAM2_RST_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2_RST` writer - SRAM2 Erase when system reset"]
pub type SRAM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nSWBOOT0` reader - Software BOOT0"]
pub type NSWBOOT0_R = crate::BitReader<bool>;
#[doc = "Field `nSWBOOT0` writer - Software BOOT0"]
pub type NSWBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type NBOOT0_R = crate::BitReader<bool>;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type NBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Dual-bank boot"]
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline(always)]
    pub fn dualbank(&self) -> DUALBANK_R {
        DUALBANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    pub fn n_swboot0(&self) -> NSWBOOT0_R {
        NSWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&self) -> NBOOT0_R {
        NBOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    #[doc = "Bits 8:10 - BOR reset Level"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<8> {
        BOR_LEV_W::new(self)
    }
    #[doc = "Bit 12 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&mut self) -> NRST_STOP_W<12> {
        NRST_STOP_W::new(self)
    }
    #[doc = "Bit 13 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRST_STDBY_W<13> {
        NRST_STDBY_W::new(self)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<16> {
        IDWG_SW_W::new(self)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<17> {
        IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<18> {
        IWDG_STDBY_W::new(self)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<19> {
        WWDG_SW_W::new(self)
    }
    #[doc = "Bit 20 - Dual-bank boot"]
    #[inline(always)]
    pub fn bfb2(&mut self) -> BFB2_W<20> {
        BFB2_W::new(self)
    }
    #[doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices"]
    #[inline(always)]
    pub fn dualbank(&mut self) -> DUALBANK_W<21> {
        DUALBANK_W::new(self)
    }
    #[doc = "Bit 23 - Boot configuration"]
    #[inline(always)]
    pub fn n_boot1(&mut self) -> NBOOT1_W<23> {
        NBOOT1_W::new(self)
    }
    #[doc = "Bit 24 - SRAM2 parity check enable"]
    #[inline(always)]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<24> {
        SRAM2_PE_W::new(self)
    }
    #[doc = "Bit 25 - SRAM2 Erase when system reset"]
    #[inline(always)]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<25> {
        SRAM2_RST_W::new(self)
    }
    #[doc = "Bit 26 - Software BOOT0"]
    #[inline(always)]
    pub fn n_swboot0(&mut self) -> NSWBOOT0_W<26> {
        NSWBOOT0_W::new(self)
    }
    #[doc = "Bit 27 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn n_boot0(&mut self) -> NBOOT0_W<27> {
        NBOOT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [optr](index.html) module"]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [optr::R](R) reader structure"]
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [optr::W](W) writer structure"]
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPTR to value 0xf000_0000"]
impl crate::Resettable for OPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf000_0000
    }
}

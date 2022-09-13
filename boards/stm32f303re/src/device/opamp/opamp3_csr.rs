#[doc = "Register `OPAMP3_CSR` reader"]
pub struct R(crate::R<OPAMP3_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP3_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP3_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP3_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP3_CSR` writer"]
pub struct W(crate::W<OPAMP3_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP3_CSR_SPEC>;
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
impl From<crate::W<OPAMP3_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP3_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAMP3EN` reader - OPAMP3 enable"]
pub type OPAMP3EN_R = crate::BitReader<bool>;
#[doc = "Field `OPAMP3EN` writer - OPAMP3 enable"]
pub type OPAMP3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `FORCE_VP` reader - FORCE_VP"]
pub type FORCE_VP_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_VP` writer - FORCE_VP"]
pub type FORCE_VP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `VP_SEL` reader - OPAMP Non inverting input selection"]
pub type VP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VP_SEL` writer - OPAMP Non inverting input selection"]
pub type VP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `VM_SEL` reader - OPAMP inverting input selection"]
pub type VM_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VM_SEL` writer - OPAMP inverting input selection"]
pub type VM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TCM_EN` reader - Timer controlled Mux mode enable"]
pub type TCM_EN_R = crate::BitReader<bool>;
#[doc = "Field `TCM_EN` writer - Timer controlled Mux mode enable"]
pub type TCM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `VMS_SEL` reader - OPAMP inverting input secondary selection"]
pub type VMS_SEL_R = crate::BitReader<bool>;
#[doc = "Field `VMS_SEL` writer - OPAMP inverting input secondary selection"]
pub type VMS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection"]
pub type VPS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection"]
pub type VPS_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CALON` reader - Calibration mode enable"]
pub type CALON_R = crate::BitReader<bool>;
#[doc = "Field `CALON` writer - Calibration mode enable"]
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CALSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CALSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PGA_GAIN` reader - Gain in PGA mode"]
pub type PGA_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGA_GAIN` writer - Gain in PGA mode"]
pub type PGA_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `USER_TRIM` reader - User trimming enable"]
pub type USER_TRIM_R = crate::BitReader<bool>;
#[doc = "Field `USER_TRIM` writer - User trimming enable"]
pub type USER_TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)"]
pub type TRIMOFFSETP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)"]
pub type TRIMOFFSETP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)"]
pub type TRIMOFFSETN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)"]
pub type TRIMOFFSETN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP3_CSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `TSTREF` reader - TSTREF"]
pub type TSTREF_R = crate::BitReader<bool>;
#[doc = "Field `TSTREF` writer - TSTREF"]
pub type TSTREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
#[doc = "Field `OUTCAL` reader - OPAMP ouput status flag"]
pub type OUTCAL_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` reader - OPAMP lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - OPAMP lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP3_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OPAMP3 enable"]
    #[inline(always)]
    pub fn opamp3en(&self) -> OPAMP3EN_R {
        OPAMP3EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPAMP ouput status flag"]
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP3 enable"]
    #[inline(always)]
    pub fn opamp3en(&mut self) -> OPAMP3EN_W<0> {
        OPAMP3EN_W::new(self)
    }
    #[doc = "Bit 1 - FORCE_VP"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> FORCE_VP_W<1> {
        FORCE_VP_W::new(self)
    }
    #[doc = "Bits 2:3 - OPAMP Non inverting input selection"]
    #[inline(always)]
    pub fn vp_sel(&mut self) -> VP_SEL_W<2> {
        VP_SEL_W::new(self)
    }
    #[doc = "Bits 5:6 - OPAMP inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VM_SEL_W<5> {
        VM_SEL_W::new(self)
    }
    #[doc = "Bit 7 - Timer controlled Mux mode enable"]
    #[inline(always)]
    pub fn tcm_en(&mut self) -> TCM_EN_W<7> {
        TCM_EN_W::new(self)
    }
    #[doc = "Bit 8 - OPAMP inverting input secondary selection"]
    #[inline(always)]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<8> {
        VMS_SEL_W::new(self)
    }
    #[doc = "Bits 9:10 - OPAMP Non inverting input secondary selection"]
    #[inline(always)]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<9> {
        VPS_SEL_W::new(self)
    }
    #[doc = "Bit 11 - Calibration mode enable"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W<11> {
        CALON_W::new(self)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CALSEL_W<12> {
        CALSEL_W::new(self)
    }
    #[doc = "Bits 14:17 - Gain in PGA mode"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<14> {
        PGA_GAIN_W::new(self)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn user_trim(&mut self) -> USER_TRIM_W<18> {
        USER_TRIM_W::new(self)
    }
    #[doc = "Bits 19:23 - Offset trimming value (PMOS)"]
    #[inline(always)]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<19> {
        TRIMOFFSETP_W::new(self)
    }
    #[doc = "Bits 24:28 - Offset trimming value (NMOS)"]
    #[inline(always)]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<24> {
        TRIMOFFSETN_W::new(self)
    }
    #[doc = "Bit 29 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W<29> {
        TSTREF_W::new(self)
    }
    #[doc = "Bit 31 - OPAMP lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPAMP3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp3_csr](index.html) module"]
pub struct OPAMP3_CSR_SPEC;
impl crate::RegisterSpec for OPAMP3_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp3_csr::R](R) reader structure"]
impl crate::Readable for OPAMP3_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp3_csr::W](W) writer structure"]
impl crate::Writable for OPAMP3_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPAMP3_CSR to value 0"]
impl crate::Resettable for OPAMP3_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

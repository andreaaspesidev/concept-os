#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PVMO4` reader - Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
pub type PVMO4_R = crate::BitReader<bool>;
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
pub type PVMO3_R = crate::BitReader<bool>;
#[doc = "Field `PVMO2` reader - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
pub type PVMO2_R = crate::BitReader<bool>;
#[doc = "Field `PVMO1` reader - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
pub type PVMO1_R = crate::BitReader<bool>;
#[doc = "Field `PVDO` reader - Power voltage detector output"]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VOSF_R = crate::BitReader<bool>;
#[doc = "Field `REGLPF` reader - Low-power regulator flag"]
pub type REGLPF_R = crate::BitReader<bool>;
#[doc = "Field `REGLPS` reader - Low-power regulator started"]
pub type REGLPS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 15 - Peripheral voltage monitoring output: VDDA vs. 2.2 V"]
    #[inline(always)]
    pub fn pvmo4(&self) -> PVMO4_R {
        PVMO4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V"]
    #[inline(always)]
    pub fn pvmo2(&self) -> PVMO2_R {
        PVMO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
    #[inline(always)]
    pub fn pvmo1(&self) -> PVMO1_R {
        PVMO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Low-power regulator flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Low-power regulator started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

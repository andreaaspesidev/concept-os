#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Option byte error\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTERR_A {
    #[doc = "1: The loaded option byte and its complement do not match"]
    OptionByteError = 1,
}
impl From<OPTERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OPTERR_R = crate::BitReader<OPTERR_A>;
impl OPTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTERR_A> {
        match self.bits {
            true => Some(OPTERR_A::OptionByteError),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OptionByteError`"]
    #[inline(always)]
    pub fn is_option_byte_error(&self) -> bool {
        *self == OPTERR_A::OptionByteError
    }
}
#[doc = "WDG_SW\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDG_SW_A {
    #[doc = "0: Hardware watchdog"]
    Hardware = 0,
    #[doc = "1: Software watchdog"]
    Software = 1,
}
impl From<WDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<WDG_SW_A>;
impl WDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDG_SW_A {
        match self.bits {
            false => WDG_SW_A::Hardware,
            true => WDG_SW_A::Software,
        }
    }
    #[doc = "Checks if the value of the field is `Hardware`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW_A::Hardware
    }
    #[doc = "Checks if the value of the field is `Software`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW_A::Software
    }
}
#[doc = "nRST_STOP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STOP_A {
    #[doc = "0: Reset generated when entering Stop mode"]
    Reset = 0,
    #[doc = "1: No reset generated"]
    NoReset = 1,
}
impl From<NRST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type NRST_STOP_R = crate::BitReader<NRST_STOP_A>;
impl NRST_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRST_STOP_A {
        match self.bits {
            false => NRST_STOP_A::Reset,
            true => NRST_STOP_A::NoReset,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == NRST_STOP_A::Reset
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == NRST_STOP_A::NoReset
    }
}
#[doc = "nRST_STDBY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRST_STDBY_A {
    #[doc = "0: Reset generated when entering Standby mode"]
    Reset = 0,
    #[doc = "1: No reset generated"]
    NoReset = 1,
}
impl From<NRST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: NRST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type NRST_STDBY_R = crate::BitReader<NRST_STDBY_A>;
impl NRST_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRST_STDBY_A {
        match self.bits {
            false => NRST_STDBY_A::Reset,
            true => NRST_STDBY_A::NoReset,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == NRST_STDBY_A::Reset
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == NRST_STDBY_A::NoReset
    }
}
#[doc = "BOOT1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBOOT1_A {
    #[doc = "0: Together with BOOT0, select the device boot mode"]
    Disabled = 0,
    #[doc = "1: Together with BOOT0, select the device boot mode"]
    Enabled = 1,
}
impl From<NBOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: NBOOT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `nBOOT1` reader - BOOT1"]
pub type NBOOT1_R = crate::BitReader<NBOOT1_A>;
impl NBOOT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBOOT1_A {
        match self.bits {
            false => NBOOT1_A::Disabled,
            true => NBOOT1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NBOOT1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NBOOT1_A::Enabled
    }
}
#[doc = "VDDA_MONITOR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDA_MONITOR_A {
    #[doc = "0: VDDA power supply supervisor disabled"]
    Disabled = 0,
    #[doc = "1: VDDA power supply supervisor enabled"]
    Enabled = 1,
}
impl From<VDDA_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: VDDA_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDA_MONITOR` reader - VDDA_MONITOR"]
pub type VDDA_MONITOR_R = crate::BitReader<VDDA_MONITOR_A>;
impl VDDA_MONITOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDA_MONITOR_A {
        match self.bits {
            false => VDDA_MONITOR_A::Disabled,
            true => VDDA_MONITOR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDA_MONITOR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDA_MONITOR_A::Enabled
    }
}
#[doc = "SRAM_PARITY_CHECK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_PARITY_CHECK_A {
    #[doc = "0: RAM parity check disabled"]
    Disabled = 0,
    #[doc = "1: RAM parity check enabled"]
    Enabled = 1,
}
impl From<SRAM_PARITY_CHECK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PARITY_CHECK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_PARITY_CHECK` reader - SRAM_PARITY_CHECK"]
pub type SRAM_PARITY_CHECK_R = crate::BitReader<SRAM_PARITY_CHECK_A>;
impl SRAM_PARITY_CHECK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PARITY_CHECK_A {
        match self.bits {
            false => SRAM_PARITY_CHECK_A::Disabled,
            true => SRAM_PARITY_CHECK_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM_PARITY_CHECK_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM_PARITY_CHECK_A::Enabled
    }
}
#[doc = "Field `Data0` reader - Data0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data1` reader - Data1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Read protection Level status\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RDPRT_A {
    #[doc = "0: Level 0"]
    Level0 = 0,
    #[doc = "1: Level 1"]
    Level1 = 1,
    #[doc = "3: Level 2"]
    Level2 = 3,
}
impl From<RDPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: RDPRT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RDPRT` reader - Read protection Level status"]
pub type RDPRT_R = crate::FieldReader<u8, RDPRT_A>;
impl RDPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDPRT_A> {
        match self.bits {
            0 => Some(RDPRT_A::Level0),
            1 => Some(RDPRT_A::Level1),
            3 => Some(RDPRT_A::Level2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Level0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPRT_A::Level0
    }
    #[doc = "Checks if the value of the field is `Level1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPRT_A::Level1
    }
    #[doc = "Checks if the value of the field is `Level2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPRT_A::Level2
    }
}
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRST_STOP_R {
        NRST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBOOT1_R {
        NBOOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SRAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn sram_parity_check(&self) -> SRAM_PARITY_CHECK_R {
        SRAM_PARITY_CHECK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 1:2 - Read protection Level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0xffff_ff0f"]
impl crate::Resettable for OBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ff0f
    }
}

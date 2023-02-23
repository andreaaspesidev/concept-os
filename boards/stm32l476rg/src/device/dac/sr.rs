#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC channel1 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAUDR1_A {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel x"]
    NoError = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    Error = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag"]
pub type DMAUDR1_R = crate::BitReader<DMAUDR1_A>;
impl DMAUDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::NoError,
            true => DMAUDR1_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMAUDR1_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMAUDR1_A::Error
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag"]
pub type DMAUDR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, DMAUDR1_A, O>;
impl<'a, const O: u8> DMAUDR1_W<'a, O> {
    #[doc = "No DMA underrun error condition occurred for DAC channel x"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(DMAUDR1_A::NoError)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(DMAUDR1_A::Error)
    }
}
#[doc = "DAC Channel 1 calibration offset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAL_FLAG1_A {
    #[doc = "0: Calibration trimming value is lower than the offset correction value"]
    Lower = 0,
    #[doc = "1: Calibration trimming value is equal or greater than the offset correction value"]
    EqualHigher = 1,
}
impl From<CAL_FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status"]
pub type CAL_FLAG1_R = crate::BitReader<CAL_FLAG1_A>;
impl CAL_FLAG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAL_FLAG1_A {
        match self.bits {
            false => CAL_FLAG1_A::Lower,
            true => CAL_FLAG1_A::EqualHigher,
        }
    }
    #[doc = "Checks if the value of the field is `Lower`"]
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == CAL_FLAG1_A::Lower
    }
    #[doc = "Checks if the value of the field is `EqualHigher`"]
    #[inline(always)]
    pub fn is_equal_higher(&self) -> bool {
        *self == CAL_FLAG1_A::EqualHigher
    }
}
#[doc = "DAC Channel 1 busy writing sample time flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWST1_A {
    #[doc = "0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    Idle = 0,
    #[doc = "1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    Busy = 1,
}
impl From<BWST1_A> for bool {
    #[inline(always)]
    fn from(variant: BWST1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWST1` reader - DAC Channel 1 busy writing sample time flag"]
pub type BWST1_R = crate::BitReader<BWST1_A>;
impl BWST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWST1_A {
        match self.bits {
            false => BWST1_A::Idle,
            true => BWST1_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BWST1_A::Idle
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BWST1_A::Busy
    }
}
#[doc = "DAC Channel 2 busy writing sample time flag"]
pub use BWST1_A as BWST2_A;
#[doc = "Field `BWST2` reader - DAC Channel 2 busy writing sample time flag"]
pub use BWST1_R as BWST2_R;
#[doc = "DAC Channel 2 calibration offset status"]
pub use CAL_FLAG1_A as CAL_FLAG2_A;
#[doc = "Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status"]
pub use CAL_FLAG1_R as CAL_FLAG2_R;
#[doc = "DAC channel2 DMA underrun flag"]
pub use DMAUDR1_A as DMAUDR2_A;
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag"]
pub use DMAUDR1_R as DMAUDR2_R;
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag"]
pub use DMAUDR1_W as DMAUDR2_W;
impl R {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration offset status"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC Channel 1 busy writing sample time flag"]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration offset status"]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC Channel 2 busy writing sample time flag"]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<13> {
        DMAUDR1_W::new(self)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<29> {
        DMAUDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

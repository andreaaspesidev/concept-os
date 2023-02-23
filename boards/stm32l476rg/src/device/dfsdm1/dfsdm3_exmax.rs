#[doc = "Register `DFSDM3_EXMAX` reader"]
pub struct R(crate::R<DFSDM3_EXMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM3_EXMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM3_EXMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM3_EXMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXMAX` reader - Extremes detector maximum value"]
pub type EXMAX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXMAXCH` reader - Extremes detector maximum data channel"]
pub type EXMAXCH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:31 - Extremes detector maximum value"]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - Extremes detector maximum data channel"]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_exmax](index.html) module"]
pub struct DFSDM3_EXMAX_SPEC;
impl crate::RegisterSpec for DFSDM3_EXMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm3_exmax::R](R) reader structure"]
impl crate::Readable for DFSDM3_EXMAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM3_EXMAX to value 0x8000_0000"]
impl crate::Resettable for DFSDM3_EXMAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}

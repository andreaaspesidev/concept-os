#[doc = "Register `DFSDM3_RDATAR` reader"]
pub struct R(crate::R<DFSDM3_RDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM3_RDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM3_RDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM3_RDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - Regular channel conversion data"]
pub type RDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RPEND` reader - Regular channel pending data"]
pub type RPEND_R = crate::BitReader<bool>;
#[doc = "Field `RDATACH` reader - Regular channel most recently converted"]
pub type RDATACH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:31 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 4 - Regular channel pending data"]
    #[inline(always)]
    pub fn rpend(&self) -> RPEND_R {
        RPEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Regular channel most recently converted"]
    #[inline(always)]
    pub fn rdatach(&self) -> RDATACH_R {
        RDATACH_R::new((self.bits & 7) as u8)
    }
}
#[doc = "data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm3_rdatar](index.html) module"]
pub struct DFSDM3_RDATAR_SPEC;
impl crate::RegisterSpec for DFSDM3_RDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm3_rdatar::R](R) reader structure"]
impl crate::Readable for DFSDM3_RDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM3_RDATAR to value 0"]
impl crate::Resettable for DFSDM3_RDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

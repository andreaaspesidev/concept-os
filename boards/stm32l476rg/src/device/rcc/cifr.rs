#[doc = "Register `CIFR` reader"]
pub struct R(crate::R<CIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LSECSSF` reader - LSE Clock security system interrupt flag"]
pub type LSECSSF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2RDYF` reader - PLLSAI2 ready interrupt flag"]
pub type PLLSAI2RDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag"]
pub type PLLSAI1RDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYF` reader - PLL ready interrupt flag"]
pub type PLLRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `MSIRDYF` reader - MSI ready interrupt flag"]
pub type MSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSI48RDYF` reader - HSI48 ready interrupt flag"]
pub type HSI48RDYF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 9 - LSE Clock security system interrupt flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai2rdyf(&self) -> PLLSAI2RDYF_R {
        PLLSAI2RDYF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt flag"]
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> PLLSAI1RDYF_R {
        PLLSAI1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt flag"]
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt flag"]
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Clock interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cifr](index.html) module"]
pub struct CIFR_SPEC;
impl crate::RegisterSpec for CIFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cifr::R](R) reader structure"]
impl crate::Readable for CIFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CIFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

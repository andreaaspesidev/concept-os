#[doc = "Register `CIER` reader"]
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIER` writer"]
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSECSSIE` reader - LSE clock security system interrupt enable"]
pub type LSECSSIE_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSIE` writer - LSE clock security system interrupt enable"]
pub type LSECSSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `PLLSAI2RDYIE` reader - PLLSAI2 ready interrupt enable"]
pub type PLLSAI2RDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2RDYIE` writer - PLLSAI2 ready interrupt enable"]
pub type PLLSAI2RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable"]
pub type PLLSAI1RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `PLLRDYIE` reader - PLL ready interrupt enable"]
pub type PLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYIE` writer - PLL ready interrupt enable"]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `MSIRDYIE` reader - MSI ready interrupt enable"]
pub type MSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `MSIRDYIE` writer - MSI ready interrupt enable"]
pub type MSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
#[doc = "Field `HSI48RDYIE` reader - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSI48RDYIE` writer - HSI48 ready interrupt enable"]
pub type HSI48RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&self) -> PLLSAI2RDYIE_R {
        PLLSAI2RDYIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - LSE clock security system interrupt enable"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    #[doc = "Bit 7 - PLLSAI2 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai2rdyie(&mut self) -> PLLSAI2RDYIE_W<7> {
        PLLSAI2RDYIE_W::new(self)
    }
    #[doc = "Bit 6 - PLLSAI1 ready interrupt enable"]
    #[inline(always)]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<6> {
        PLLSAI1RDYIE_W::new(self)
    }
    #[doc = "Bit 5 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<5> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 4 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<4> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 3 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 2 - MSI ready interrupt enable"]
    #[inline(always)]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<2> {
        MSIRDYIE_W::new(self)
    }
    #[doc = "Bit 1 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 0 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<0> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI48 ready interrupt enable"]
    #[inline(always)]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<10> {
        HSI48RDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cier](index.html) module"]
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cier::R](R) reader structure"]
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cier::W](W) writer structure"]
impl crate::Writable for CIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

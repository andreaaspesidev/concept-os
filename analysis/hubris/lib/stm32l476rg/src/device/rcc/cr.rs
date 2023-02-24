#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag"]
pub type PLLSAI2RDY_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2ON` reader - SAI2 PLL enable"]
pub type PLLSAI2ON_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI2ON` writer - SAI2 PLL enable"]
pub type PLLSAI2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag"]
pub type PLLSAI1RDY_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1ON` reader - SAI1 PLL enable"]
pub type PLLSAI1ON_R = crate::BitReader<bool>;
#[doc = "Field `PLLSAI1ON` writer - SAI1 PLL enable"]
pub type PLLSAI1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub type PLLON_R = crate::BitReader<bool>;
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = crate::BitReader<bool>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = crate::BitReader<bool>;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSIASFS` reader - HSI automatic start from Stop"]
pub type HSIASFS_R = crate::BitReader<bool>;
#[doc = "Field `HSIASFS` writer - HSI automatic start from Stop"]
pub type HSIASFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `HSIKERON` reader - HSI always enable for peripheral kernels"]
pub type HSIKERON_R = crate::BitReader<bool>;
#[doc = "Field `HSIKERON` writer - HSI always enable for peripheral kernels"]
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HSION` reader - HSI clock enable"]
pub type HSION_R = crate::BitReader<bool>;
#[doc = "Field `HSION` writer - HSI clock enable"]
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "MSI clock ranges\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    #[doc = "0: range 0 around 100 kHz"]
    Range100k = 0,
    #[doc = "1: range 1 around 200 kHz"]
    Range200k = 1,
    #[doc = "2: range 2 around 400 kHz"]
    Range400k = 2,
    #[doc = "3: range 3 around 800 kHz"]
    Range800k = 3,
    #[doc = "4: range 4 around 1 MHz"]
    Range1m = 4,
    #[doc = "5: range 5 around 2 MHz"]
    Range2m = 5,
    #[doc = "6: range 6 around 4 MHz"]
    Range4m = 6,
    #[doc = "7: range 7 around 8 MHz"]
    Range8m = 7,
    #[doc = "8: range 8 around 16 MHz"]
    Range16m = 8,
    #[doc = "9: range 9 around 24 MHz"]
    Range24m = 9,
    #[doc = "10: range 10 around 32 MHz"]
    Range32m = 10,
    #[doc = "11: range 11 around 48 MHz"]
    Range48m = 11,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSIRANGE` reader - MSI clock ranges"]
pub type MSIRANGE_R = crate::FieldReader<u8, MSIRANGE_A>;
impl MSIRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MSIRANGE_A> {
        match self.bits {
            0 => Some(MSIRANGE_A::Range100k),
            1 => Some(MSIRANGE_A::Range200k),
            2 => Some(MSIRANGE_A::Range400k),
            3 => Some(MSIRANGE_A::Range800k),
            4 => Some(MSIRANGE_A::Range1m),
            5 => Some(MSIRANGE_A::Range2m),
            6 => Some(MSIRANGE_A::Range4m),
            7 => Some(MSIRANGE_A::Range8m),
            8 => Some(MSIRANGE_A::Range16m),
            9 => Some(MSIRANGE_A::Range24m),
            10 => Some(MSIRANGE_A::Range32m),
            11 => Some(MSIRANGE_A::Range48m),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Range100k`"]
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE_A::Range100k
    }
    #[doc = "Checks if the value of the field is `Range200k`"]
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE_A::Range200k
    }
    #[doc = "Checks if the value of the field is `Range400k`"]
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE_A::Range400k
    }
    #[doc = "Checks if the value of the field is `Range800k`"]
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE_A::Range800k
    }
    #[doc = "Checks if the value of the field is `Range1m`"]
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE_A::Range1m
    }
    #[doc = "Checks if the value of the field is `Range2m`"]
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE_A::Range2m
    }
    #[doc = "Checks if the value of the field is `Range4m`"]
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE_A::Range4m
    }
    #[doc = "Checks if the value of the field is `Range8m`"]
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE_A::Range8m
    }
    #[doc = "Checks if the value of the field is `Range16m`"]
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE_A::Range16m
    }
    #[doc = "Checks if the value of the field is `Range24m`"]
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE_A::Range24m
    }
    #[doc = "Checks if the value of the field is `Range32m`"]
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE_A::Range32m
    }
    #[doc = "Checks if the value of the field is `Range48m`"]
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE_A::Range48m
    }
}
#[doc = "Field `MSIRANGE` writer - MSI clock ranges"]
pub type MSIRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, MSIRANGE_A, 4, O>;
impl<'a, const O: u8> MSIRANGE_W<'a, O> {
    #[doc = "range 0 around 100 kHz"]
    #[inline(always)]
    pub fn range100k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range100k)
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline(always)]
    pub fn range200k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range200k)
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline(always)]
    pub fn range400k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range400k)
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline(always)]
    pub fn range800k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range800k)
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline(always)]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range1m)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline(always)]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range2m)
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline(always)]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range4m)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline(always)]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range8m)
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline(always)]
    pub fn range16m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range16m)
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline(always)]
    pub fn range24m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range24m)
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline(always)]
    pub fn range32m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range32m)
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline(always)]
    pub fn range48m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range48m)
    }
}
#[doc = "Field `MSIRGSEL` writer - MSI clock range selection"]
pub type MSIRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MSIPLLEN` reader - MSI clock PLL enable"]
pub type MSIPLLEN_R = crate::BitReader<bool>;
#[doc = "Field `MSIPLLEN` writer - MSI clock PLL enable"]
pub type MSIPLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub type MSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `MSION` reader - MSI clock enable"]
pub type MSION_R = crate::BitReader<bool>;
#[doc = "Field `MSION` writer - MSI clock enable"]
pub type MSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 29 - SAI2 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDY_R {
        PLLSAI2RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    pub fn pllsai2on(&self) -> PLLSAI2ON_R {
        PLLSAI2ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - SAI1 PLL clock ready flag"]
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline(always)]
    pub fn pllsai2on(&mut self) -> PLLSAI2ON_W<28> {
        PLLSAI2ON_W::new(self)
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline(always)]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<26> {
        PLLSAI1ON_W::new(self)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline(always)]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<11> {
        HSIASFS_W::new(self)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<9> {
        HSIKERON_W::new(self)
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<8> {
        HSION_W::new(self)
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline(always)]
    pub fn msirange(&mut self) -> MSIRANGE_W<4> {
        MSIRANGE_W::new(self)
    }
    #[doc = "Bit 3 - MSI clock range selection"]
    #[inline(always)]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<3> {
        MSIRGSEL_W::new(self)
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline(always)]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<2> {
        MSIPLLEN_W::new(self)
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&mut self) -> MSION_W<0> {
        MSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x63
    }
}

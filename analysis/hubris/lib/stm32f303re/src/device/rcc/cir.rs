#[doc = "Register `CIR` reader"]
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR` writer"]
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LSI Ready Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYF_A {
    #[doc = "0: No clock ready interrupt"]
    NotInterrupted = 0,
    #[doc = "1: Clock ready interrupt"]
    Interrupted = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI Ready Interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<LSIRDYF_A>;
impl LSIRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::NotInterrupted,
            true => LSIRDYF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYF_A::Interrupted
    }
}
#[doc = "LSE Ready Interrupt flag"]
pub use LSIRDYF_A as LSERDYF_A;
#[doc = "HSI Ready Interrupt flag"]
pub use LSIRDYF_A as HSIRDYF_A;
#[doc = "HSE Ready Interrupt flag"]
pub use LSIRDYF_A as HSERDYF_A;
#[doc = "PLL Ready Interrupt flag"]
pub use LSIRDYF_A as PLLRDYF_A;
#[doc = "Field `LSERDYF` reader - LSE Ready Interrupt flag"]
pub use LSIRDYF_R as LSERDYF_R;
#[doc = "Field `HSIRDYF` reader - HSI Ready Interrupt flag"]
pub use LSIRDYF_R as HSIRDYF_R;
#[doc = "Field `HSERDYF` reader - HSE Ready Interrupt flag"]
pub use LSIRDYF_R as HSERDYF_R;
#[doc = "Field `PLLRDYF` reader - PLL Ready Interrupt flag"]
pub use LSIRDYF_R as PLLRDYF_R;
#[doc = "Clock Security System Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSF_A {
    #[doc = "0: No clock security interrupt caused by HSE clock failure"]
    NotInterrupted = 0,
    #[doc = "1: Clock security interrupt caused by HSE clock failure"]
    Interrupted = 1,
}
impl From<CSSF_A> for bool {
    #[inline(always)]
    fn from(variant: CSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSF` reader - Clock Security System Interrupt flag"]
pub type CSSF_R = crate::BitReader<CSSF_A>;
impl CSSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSF_A {
        match self.bits {
            false => CSSF_A::NotInterrupted,
            true => CSSF_A::Interrupted,
        }
    }
    #[doc = "Checks if the value of the field is `NotInterrupted`"]
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == CSSF_A::NotInterrupted
    }
    #[doc = "Checks if the value of the field is `Interrupted`"]
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == CSSF_A::Interrupted
    }
}
#[doc = "LSI Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI Ready Interrupt Enable"]
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE_A>;
impl LSIRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::Disabled,
            true => LSIRDYIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE_A::Enabled
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI Ready Interrupt Enable"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSIRDYIE_A, O>;
impl<'a, const O: u8> LSIRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::Enabled)
    }
}
#[doc = "LSE Ready Interrupt Enable"]
pub use LSIRDYIE_A as LSERDYIE_A;
#[doc = "HSI Ready Interrupt Enable"]
pub use LSIRDYIE_A as HSIRDYIE_A;
#[doc = "HSE Ready Interrupt Enable"]
pub use LSIRDYIE_A as HSERDYIE_A;
#[doc = "PLL Ready Interrupt Enable"]
pub use LSIRDYIE_A as PLLRDYIE_A;
#[doc = "Field `LSERDYIE` reader - LSE Ready Interrupt Enable"]
pub use LSIRDYIE_R as LSERDYIE_R;
#[doc = "Field `HSIRDYIE` reader - HSI Ready Interrupt Enable"]
pub use LSIRDYIE_R as HSIRDYIE_R;
#[doc = "Field `HSERDYIE` reader - HSE Ready Interrupt Enable"]
pub use LSIRDYIE_R as HSERDYIE_R;
#[doc = "Field `PLLRDYIE` reader - PLL Ready Interrupt Enable"]
pub use LSIRDYIE_R as PLLRDYIE_R;
#[doc = "Field `LSERDYIE` writer - LSE Ready Interrupt Enable"]
pub use LSIRDYIE_W as LSERDYIE_W;
#[doc = "Field `HSIRDYIE` writer - HSI Ready Interrupt Enable"]
pub use LSIRDYIE_W as HSIRDYIE_W;
#[doc = "Field `HSERDYIE` writer - HSE Ready Interrupt Enable"]
pub use LSIRDYIE_W as HSERDYIE_W;
#[doc = "Field `PLLRDYIE` writer - PLL Ready Interrupt Enable"]
pub use LSIRDYIE_W as PLLRDYIE_W;
#[doc = "LSI Ready Interrupt Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYC_AW {
    #[doc = "1: Clear interrupt flag"]
    Clear = 1,
}
impl From<LSIRDYC_AW> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDYC` writer - LSI Ready Interrupt Clear"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, LSIRDYC_AW, O>;
impl<'a, const O: u8> LSIRDYC_W<'a, O> {
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LSIRDYC_AW::Clear)
    }
}
#[doc = "LSE Ready Interrupt Clear"]
pub use LSIRDYC_AW as LSERDYC_AW;
#[doc = "HSI Ready Interrupt Clear"]
pub use LSIRDYC_AW as HSIRDYC_AW;
#[doc = "HSE Ready Interrupt Clear"]
pub use LSIRDYC_AW as HSERDYC_AW;
#[doc = "PLL Ready Interrupt Clear"]
pub use LSIRDYC_AW as PLLRDYC_AW;
#[doc = "Field `LSERDYC` writer - LSE Ready Interrupt Clear"]
pub use LSIRDYC_W as LSERDYC_W;
#[doc = "Field `HSIRDYC` writer - HSI Ready Interrupt Clear"]
pub use LSIRDYC_W as HSIRDYC_W;
#[doc = "Field `HSERDYC` writer - HSE Ready Interrupt Clear"]
pub use LSIRDYC_W as HSERDYC_W;
#[doc = "Field `PLLRDYC` writer - PLL Ready Interrupt Clear"]
pub use LSIRDYC_W as PLLRDYC_W;
#[doc = "Clock security system interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSC_AW {
    #[doc = "1: Clear CSSF flag"]
    Clear = 1,
}
impl From<CSSC_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, CSSC_AW, O>;
impl<'a, const O: u8> CSSC_W<'a, O> {
    #[doc = "Clear CSSF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSSC_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - LSI Ready Interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE Ready Interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI Ready Interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE Ready Interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL Ready Interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Security System Interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - LSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<8> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<9> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<10> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 11 - HSE Ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<11> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 12 - PLL Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<12> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 16 - LSI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<16> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 17 - LSE Ready Interrupt Clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W<17> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 18 - HSI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<18> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 19 - HSE Ready Interrupt Clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W<19> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 20 - PLL Ready Interrupt Clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<20> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W<23> {
        CSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock interrupt register (RCC_CIR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](index.html) module"]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir::R](R) reader structure"]
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir::W](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

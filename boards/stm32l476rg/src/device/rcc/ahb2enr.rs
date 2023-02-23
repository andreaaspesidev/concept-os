#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub type RNGEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `AESEN` reader - AES accelerator clock enable"]
pub type AESEN_R = crate::BitReader<bool>;
#[doc = "Field `AESEN` writer - AES accelerator clock enable"]
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "ADC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    #[doc = "0: ADC clock disabled"]
    Disabled = 0,
    #[doc = "1: ADC clock enabled"]
    Enabled = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub type ADCEN_R = crate::BitReader<ADCEN_A>;
impl ADCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::Disabled,
            true => ADCEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN_A::Enabled
    }
}
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, ADCEN_A, O>;
impl<'a, const O: u8> ADCEN_W<'a, O> {
    #[doc = "ADC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Disabled)
    }
    #[doc = "ADC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Enabled)
    }
}
#[doc = "Field `OTGFSEN` reader - OTG full speed clock enable"]
pub type OTGFSEN_R = crate::BitReader<bool>;
#[doc = "Field `OTGFSEN` writer - OTG full speed clock enable"]
pub type OTGFSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub type GPIOHEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub type GPIOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub type GPIOGEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub type GPIOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub type GPIOFEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub type GPIOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub type GPIOEEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub type GPIOEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub type GPIODEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub type GPIOCEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub type GPIOBEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `GPIOIEN` reader - IO port I clock enable"]
pub type GPIOIEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOIEN` writer - IO port I clock enable"]
pub type GPIOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `DCMIEN` reader - DCMI clock enable"]
pub type DCMIEN_R = crate::BitReader<bool>;
#[doc = "Field `DCMIEN` writer - DCMI clock enable"]
pub type DCMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
#[doc = "Field `HASH1EN` reader - HASH clock enable"]
pub type HASH1EN_R = crate::BitReader<bool>;
#[doc = "Field `HASH1EN` writer - HASH clock enable"]
pub type HASH1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<16> {
        AESEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<13> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 12 - OTG full speed clock enable"]
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<12> {
        OTGFSEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    #[doc = "Bit 14 - DCMI clock enable"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W<14> {
        DCMIEN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hash1en(&mut self) -> HASH1EN_W<17> {
        HASH1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal low speed oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSION_A {
    #[doc = "0: LSI oscillator Off"]
    Off = 0,
    #[doc = "1: LSI oscillator On"]
    On = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - Internal low speed oscillator enable"]
pub type LSION_R = crate::BitReader<LSION_A>;
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::Off,
            true => LSION_A::On,
        }
    }
    #[doc = "Checks if the value of the field is `Off`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION_A::Off
    }
    #[doc = "Checks if the value of the field is `On`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION_A::On
    }
}
#[doc = "Field `LSION` writer - Internal low speed oscillator enable"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSION_A, O>;
impl<'a, const O: u8> LSION_W<'a, O> {
    #[doc = "LSI oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::Off)
    }
    #[doc = "LSI oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::On)
    }
}
#[doc = "Internal low speed oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDY_A {
    #[doc = "0: LSI oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSI oscillator ready"]
    Ready = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - Internal low speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader<LSIRDY_A>;
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::NotReady,
            true => LSIRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY_A::Ready
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMVF_A {
    #[doc = "1: Clears the reset flag"]
    Clear = 1,
}
impl From<RMVF_A> for bool {
    #[inline(always)]
    fn from(variant: RMVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVF_A>;
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RMVF_A> {
        match self.bits {
            true => Some(RMVF_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVF_A::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RMVF_A, O>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    #[doc = "Clears the reset flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVF_A::Clear)
    }
}
#[doc = "Option byte loader reset flag"]
pub use V18PWRRSTF_A as OBLRSTF_A;
#[doc = "PIN reset flag"]
pub use V18PWRRSTF_A as PINRSTF_A;
#[doc = "POR/PDR reset flag"]
pub use V18PWRRSTF_A as PORRSTF_A;
#[doc = "Software reset flag"]
pub use V18PWRRSTF_A as SFTRSTF_A;
#[doc = "Independent watchdog reset flag"]
pub use V18PWRRSTF_A as IWDGRSTF_A;
#[doc = "Window watchdog reset flag"]
pub use V18PWRRSTF_A as WWDGRSTF_A;
#[doc = "Low-power reset flag"]
pub use V18PWRRSTF_A as LPWRRSTF_A;
#[doc = "Field `OBLRSTF` reader - Option byte loader reset flag"]
pub use V18PWRRSTF_R as OBLRSTF_R;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub use V18PWRRSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub use V18PWRRSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub use V18PWRRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub use V18PWRRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub use V18PWRRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub use V18PWRRSTF_R as LPWRRSTF_R;
#[doc = "Field `OBLRSTF` writer - Option byte loader reset flag"]
pub use V18PWRRSTF_W as OBLRSTF_W;
#[doc = "Field `PINRSTF` writer - PIN reset flag"]
pub use V18PWRRSTF_W as PINRSTF_W;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub use V18PWRRSTF_W as PORRSTF_W;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub use V18PWRRSTF_W as SFTRSTF_W;
#[doc = "Field `IWDGRSTF` writer - Independent watchdog reset flag"]
pub use V18PWRRSTF_W as IWDGRSTF_W;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub use V18PWRRSTF_W as WWDGRSTF_W;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag"]
pub use V18PWRRSTF_W as LPWRRSTF_W;
#[doc = "Reset flag of the 1.8 V domain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V18PWRRSTF_A {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<V18PWRRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: V18PWRRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V18PWRRSTF` reader - Reset flag of the 1.8 V domain"]
pub type V18PWRRSTF_R = crate::BitReader<V18PWRRSTF_A>;
impl V18PWRRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V18PWRRSTF_A {
        match self.bits {
            false => V18PWRRSTF_A::NoReset,
            true => V18PWRRSTF_A::Reset,
        }
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == V18PWRRSTF_A::NoReset
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == V18PWRRSTF_A::Reset
    }
}
#[doc = "Field `V18PWRRSTF` writer - Reset flag of the 1.8 V domain"]
pub type V18PWRRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, V18PWRRSTF_A, O>;
impl<'a, const O: u8> V18PWRRSTF_W<'a, O> {
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(V18PWRRSTF_A::NoReset)
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(V18PWRRSTF_A::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset flag of the 1.8 V domain"]
    #[inline(always)]
    pub fn v18pwrrstf(&self) -> V18PWRRSTF_R {
        V18PWRRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<24> {
        RMVF_W::new(self)
    }
    #[doc = "Bit 25 - Option byte loader reset flag"]
    #[inline(always)]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<25> {
        OBLRSTF_W::new(self)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W<26> {
        PINRSTF_W::new(self)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<27> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<28> {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<29> {
        IWDGRSTF_W::new(self)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<30> {
        WWDGRSTF_W::new(self)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<31> {
        LPWRRSTF_W::new(self)
    }
    #[doc = "Bit 23 - Reset flag of the 1.8 V domain"]
    #[inline(always)]
    pub fn v18pwrrstf(&mut self) -> V18PWRRSTF_W<23> {
        V18PWRRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/status register (RCC_CSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}

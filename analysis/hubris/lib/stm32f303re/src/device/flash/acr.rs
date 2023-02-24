#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LATENCY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states, if 0 < HCLK <= 24 MHz"]
    Ws0 = 0,
    #[doc = "1: 1 wait state, if 24 < HCLK <= 48 MHz"]
    Ws1 = 1,
    #[doc = "2: 2 wait states, if 48 < HCLK <= 72 MHz"]
    Ws2 = 2,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LATENCY` reader - LATENCY"]
pub type LATENCY_R = crate::FieldReader<u8, LATENCY_A>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LATENCY_A> {
        match self.bits {
            0 => Some(LATENCY_A::Ws0),
            1 => Some(LATENCY_A::Ws1),
            2 => Some(LATENCY_A::Ws2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Ws0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::Ws0
    }
    #[doc = "Checks if the value of the field is `Ws1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::Ws1
    }
    #[doc = "Checks if the value of the field is `Ws2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::Ws2
    }
}
#[doc = "Field `LATENCY` writer - LATENCY"]
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, LATENCY_A, 3, O>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "0 wait states, if 0 < HCLK <= 24 MHz"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws0)
    }
    #[doc = "1 wait state, if 24 < HCLK <= 48 MHz"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws1)
    }
    #[doc = "2 wait states, if 48 < HCLK <= 72 MHz"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::Ws2)
    }
}
#[doc = "PRFTBE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTBE_A {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTBE_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTBE` reader - PRFTBE"]
pub type PRFTBE_R = crate::BitReader<PRFTBE_A>;
impl PRFTBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTBE_A {
        match self.bits {
            false => PRFTBE_A::Disabled,
            true => PRFTBE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBE_A::Enabled
    }
}
#[doc = "Field `PRFTBE` writer - PRFTBE"]
pub type PRFTBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, PRFTBE_A, O>;
impl<'a, const O: u8> PRFTBE_W<'a, O> {
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTBE_A::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTBE_A::Enabled)
    }
}
#[doc = "PRFTBS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTBS_A {
    #[doc = "0: Prefetch buffer is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch buffer is enabled"]
    Enabled = 1,
}
impl From<PRFTBS_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTBS` reader - PRFTBS"]
pub type PRFTBS_R = crate::BitReader<PRFTBS_A>;
impl PRFTBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTBS_A {
        match self.bits {
            false => PRFTBS_A::Disabled,
            true => PRFTBS_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBS_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBS_A::Enabled
    }
}
#[doc = "Flash half cycle access enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HLFCYA_A {
    #[doc = "0: Half cycle is disabled"]
    Disabled = 0,
    #[doc = "1: Half cycle is enabled"]
    Enabled = 1,
}
impl From<HLFCYA_A> for bool {
    #[inline(always)]
    fn from(variant: HLFCYA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HLFCYA` reader - Flash half cycle access enable"]
pub type HLFCYA_R = crate::BitReader<HLFCYA_A>;
impl HLFCYA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HLFCYA_A {
        match self.bits {
            false => HLFCYA_A::Disabled,
            true => HLFCYA_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HLFCYA_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HLFCYA_A::Enabled
    }
}
#[doc = "Field `HLFCYA` writer - Flash half cycle access enable"]
pub type HLFCYA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, HLFCYA_A, O>;
impl<'a, const O: u8> HLFCYA_W<'a, O> {
    #[doc = "Half cycle is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HLFCYA_A::Disabled)
    }
    #[doc = "Half cycle is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HLFCYA_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&self) -> PRFTBE_R {
        PRFTBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRFTBS"]
    #[inline(always)]
    pub fn prftbs(&self) -> PRFTBS_R {
        PRFTBS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline(always)]
    pub fn hlfcya(&self) -> HLFCYA_R {
        HLFCYA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&mut self) -> PRFTBE_W<4> {
        PRFTBE_W::new(self)
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline(always)]
    pub fn hlfcya(&mut self) -> HLFCYA_W<3> {
        HLFCYA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0x30"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}

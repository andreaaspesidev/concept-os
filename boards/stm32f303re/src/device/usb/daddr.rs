#[doc = "Register `DADDR` reader"]
pub struct R(crate::R<DADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADDR` writer"]
pub struct W(crate::W<DADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR_SPEC>;
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
impl From<crate::W<DADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD` reader - Device address"]
pub type ADD_R = crate::BitReader<bool>;
#[doc = "Field `ADD` writer - Device address"]
pub type ADD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Field `ADD1` reader - Device address"]
pub type ADD1_R = crate::BitReader<bool>;
#[doc = "Field `ADD1` writer - Device address"]
pub type ADD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Field `ADD2` reader - Device address"]
pub type ADD2_R = crate::BitReader<bool>;
#[doc = "Field `ADD2` writer - Device address"]
pub type ADD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Field `ADD3` reader - Device address"]
pub type ADD3_R = crate::BitReader<bool>;
#[doc = "Field `ADD3` writer - Device address"]
pub type ADD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Field `ADD4` reader - Device address"]
pub type ADD4_R = crate::BitReader<bool>;
#[doc = "Field `ADD4` writer - Device address"]
pub type ADD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Field `ADD5` reader - Device address"]
pub type ADD5_R = crate::BitReader<bool>;
#[doc = "Field `ADD5` writer - Device address"]
pub type ADD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Field `ADD6` reader - Device address"]
pub type ADD6_R = crate::BitReader<bool>;
#[doc = "Field `ADD6` writer - Device address"]
pub type ADD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, bool, O>;
#[doc = "Enable function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EF_A {
    #[doc = "0: USB device disabled"]
    Disabled = 0,
    #[doc = "1: USB device enabled"]
    Enabled = 1,
}
impl From<EF_A> for bool {
    #[inline(always)]
    fn from(variant: EF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EF` reader - Enable function"]
pub type EF_R = crate::BitReader<EF_A>;
impl EF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EF_A {
        match self.bits {
            false => EF_A::Disabled,
            true => EF_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EF_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EF_A::Enabled
    }
}
#[doc = "Field `EF` writer - Enable function"]
pub type EF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DADDR_SPEC, EF_A, O>;
impl<'a, const O: u8> EF_W<'a, O> {
    #[doc = "USB device disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EF_A::Disabled)
    }
    #[doc = "USB device enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EF_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&self) -> ADD1_R {
        ADD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&self) -> ADD3_R {
        ADD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&self) -> ADD4_R {
        ADD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&self) -> ADD5_R {
        ADD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&self) -> ADD6_R {
        ADD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    #[doc = "Bit 1 - Device address"]
    #[inline(always)]
    pub fn add1(&mut self) -> ADD1_W<1> {
        ADD1_W::new(self)
    }
    #[doc = "Bit 2 - Device address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W<2> {
        ADD2_W::new(self)
    }
    #[doc = "Bit 3 - Device address"]
    #[inline(always)]
    pub fn add3(&mut self) -> ADD3_W<3> {
        ADD3_W::new(self)
    }
    #[doc = "Bit 4 - Device address"]
    #[inline(always)]
    pub fn add4(&mut self) -> ADD4_W<4> {
        ADD4_W::new(self)
    }
    #[doc = "Bit 5 - Device address"]
    #[inline(always)]
    pub fn add5(&mut self) -> ADD5_W<5> {
        ADD5_W::new(self)
    }
    #[doc = "Bit 6 - Device address"]
    #[inline(always)]
    pub fn add6(&mut self) -> ADD6_W<6> {
        ADD6_W::new(self)
    }
    #[doc = "Bit 7 - Enable function"]
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W<7> {
        EF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](index.html) module"]
pub struct DADDR_SPEC;
impl crate::RegisterSpec for DADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daddr::R](R) reader structure"]
impl crate::Readable for DADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daddr::W](W) writer structure"]
impl crate::Writable for DADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

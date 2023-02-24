#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type MCOPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type MCOSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STOPWUCK` reader - Wakeup from Stop and CSS backup clock selection"]
pub type STOPWUCK_R = crate::BitReader<bool>;
#[doc = "Field `STOPWUCK` writer - Wakeup from Stop and CSS backup clock selection"]
pub type STOPWUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub type PPRE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PPRE1` reader - PB low-speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE1` writer - PB low-speed prescaler (APB1)"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 28:30 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 8:10 - PB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<24> {
        MCOSEL_W::new(self)
    }
    #[doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection"]
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<15> {
        STOPWUCK_W::new(self)
    }
    #[doc = "Bits 11:13 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 8:10 - PB low-speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

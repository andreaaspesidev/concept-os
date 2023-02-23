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
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection"]
pub type SYNCPOL_R = crate::BitReader<bool>;
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection"]
pub type SYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection"]
pub type SYNCSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection"]
pub type SYNCSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYNCDIV` reader - SYNC divider"]
pub type SYNCDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCDIV` writer - SYNC divider"]
pub type SYNCDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FELIM` reader - Frequency error limit"]
pub type FELIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FELIM` writer - Frequency error limit"]
pub type FELIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RELOAD` reader - Counter reload value"]
pub type RELOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELOAD` writer - Counter reload value"]
pub type RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider"]
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - SYNC polarity selection"]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<31> {
        SYNCPOL_W::new(self)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection"]
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<28> {
        SYNCSRC_W::new(self)
    }
    #[doc = "Bits 24:26 - SYNC divider"]
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<24> {
        SYNCDIV_W::new(self)
    }
    #[doc = "Bits 16:23 - Frequency error limit"]
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W<16> {
        FELIM_W::new(self)
    }
    #[doc = "Bits 0:15 - Counter reload value"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<0> {
        RELOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
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
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2022_bb7f
    }
}

#[doc = "Register `EGR` writer"]
pub struct W(crate::W<EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EGR_SPEC>;
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
impl From<crate::W<EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BG` writer - Break generation"]
pub type BG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `COMG` writer - Capture/Compare control update generation"]
pub type COMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub type CC2G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, bool, O>;
#[doc = "Update generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UG_AW {
    #[doc = "1: Re-initializes the timer counter and generates an update of the registers."]
    Update = 1,
}
impl From<UG_AW> for bool {
    #[inline(always)]
    fn from(variant: UG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EGR_SPEC, UG_AW, O>;
impl<'a, const O: u8> UG_W<'a, O> {
    #[doc = "Re-initializes the timer counter and generates an update of the registers."]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(UG_AW::Update)
    }
}
impl W {
    #[doc = "Bit 7 - Break generation"]
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<7> {
        BG_W::new(self)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<6> {
        TG_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare control update generation"]
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<5> {
        COMG_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<2> {
        CC2G_W::new(self)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [egr](index.html) module"]
pub struct EGR_SPEC;
impl crate::RegisterSpec for EGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [egr::W](W) writer structure"]
impl crate::Writable for EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

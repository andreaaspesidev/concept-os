#[doc = "Register `FS_DCTL` reader"]
pub struct R(crate::R<FS_DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_DCTL` writer"]
pub struct W(crate::W<FS_DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_DCTL_SPEC>;
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
impl From<crate::W<FS_DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub type RWUSIG_R = crate::BitReader<bool>;
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub type RWUSIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub type SDIS_R = crate::BitReader<bool>;
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub type SDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub type GINSTS_R = crate::BitReader<bool>;
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub type GONSTS_R = crate::BitReader<bool>;
#[doc = "Field `TCTL` reader - Test control"]
pub type TCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCTL` writer - Test control"]
pub type TCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_DCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SGINAK` reader - Set global IN NAK"]
pub type SGINAK_R = crate::BitReader<bool>;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
#[doc = "Field `CGINAK` reader - Clear global IN NAK"]
pub type CGINAK_R = crate::BitReader<bool>;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
#[doc = "Field `SGONAK` reader - Set global OUT NAK"]
pub type SGONAK_R = crate::BitReader<bool>;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
#[doc = "Field `CGONAK` reader - Clear global OUT NAK"]
pub type CGONAK_R = crate::BitReader<bool>;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub type POPRGDNE_R = crate::BitReader<bool>;
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub type POPRGDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&self) -> SGINAK_R {
        SGINAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&self) -> CGINAK_R {
        CGINAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&self) -> SGONAK_R {
        SGONAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&self) -> CGONAK_R {
        CGONAK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W<0> {
        RWUSIG_W::new(self)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W<1> {
        SDIS_W::new(self)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W<4> {
        TCTL_W::new(self)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W<7> {
        SGINAK_W::new(self)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W<8> {
        CGINAK_W::new(self)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W<9> {
        SGONAK_W::new(self)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W<10> {
        CGONAK_W::new(self)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<11> {
        POPRGDNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_dctl](index.html) module"]
pub struct FS_DCTL_SPEC;
impl crate::RegisterSpec for FS_DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_dctl::R](R) reader structure"]
impl crate::Readable for FS_DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_dctl::W](W) writer structure"]
impl crate::Writable for FS_DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FS_DCTL to value 0"]
impl crate::Resettable for FS_DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

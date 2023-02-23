#[doc = "Register `FS_PCGCCTL` reader"]
pub struct R(crate::R<FS_PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_PCGCCTL` writer"]
pub struct W(crate::W<FS_PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_PCGCCTL_SPEC>;
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
impl From<crate::W<FS_PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type STPPCLK_R = crate::BitReader<bool>;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type STPPCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_PCGCCTL_SPEC, bool, O>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader<bool>;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_PCGCCTL_SPEC, bool, O>;
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub type PHYSUSP_R = crate::BitReader<bool>;
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub type PHYSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_PCGCCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W<0> {
        STPPCLK_W::new(self)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<1> {
        GATEHCLK_W::new(self)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&mut self) -> PHYSUSP_W<4> {
        PHYSUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_pcgcctl](index.html) module"]
pub struct FS_PCGCCTL_SPEC;
impl crate::RegisterSpec for FS_PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_pcgcctl::R](R) reader structure"]
impl crate::Readable for FS_PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_pcgcctl::W](W) writer structure"]
impl crate::Writable for FS_PCGCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FS_PCGCCTL to value 0"]
impl crate::Resettable for FS_PCGCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

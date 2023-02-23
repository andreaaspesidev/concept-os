#[doc = "Register `MEMRMP` reader"]
pub struct R(crate::R<MEMRMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMRMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMRMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMRMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMRMP` writer"]
pub struct W(crate::W<MEMRMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMRMP_SPEC>;
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
impl From<crate::W<MEMRMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMRMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB_MODE` reader - Flash Bank mode selection"]
pub type FB_MODE_R = crate::BitReader<bool>;
#[doc = "Field `FB_MODE` writer - Flash Bank mode selection"]
pub type FB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMRMP_SPEC, bool, O>;
#[doc = "Field `QFS` reader - QUADSPI memory mapping swap"]
pub type QFS_R = crate::BitReader<bool>;
#[doc = "Field `QFS` writer - QUADSPI memory mapping swap"]
pub type QFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMRMP_SPEC, bool, O>;
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub type MEM_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MEMRMP_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - QUADSPI memory mapping swap"]
    #[inline(always)]
    pub fn qfs(&self) -> QFS_R {
        QFS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W<8> {
        FB_MODE_W::new(self)
    }
    #[doc = "Bit 3 - QUADSPI memory mapping swap"]
    #[inline(always)]
    pub fn qfs(&mut self) -> QFS_W<3> {
        QFS_W::new(self)
    }
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memrmp](index.html) module"]
pub struct MEMRMP_SPEC;
impl crate::RegisterSpec for MEMRMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memrmp::R](R) reader structure"]
impl crate::Readable for MEMRMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memrmp::W](W) writer structure"]
impl crate::Writable for MEMRMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MEMRMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

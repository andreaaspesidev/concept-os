#[doc = "Register `PCROP1ER` reader"]
pub struct R(crate::R<PCROP1ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1ER` writer"]
pub struct W(crate::W<PCROP1ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1ER_SPEC>;
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
impl From<crate::W<PCROP1ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1_END` reader - Bank 1 PCROP area end offset"]
pub type PCROP1_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCROP1_END` writer - Bank 1 PCROP area end offset"]
pub type PCROP1_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP1ER_SPEC, u16, u16, 16, O>;
#[doc = "Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_R = crate::BitReader<bool>;
#[doc = "Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased"]
pub type PCROP_RDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCROP1ER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 1 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W<0> {
        PCROP1_END_W::new(self)
    }
    #[doc = "Bit 31 - PCROP area preserved when RDP level decreased"]
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<31> {
        PCROP_RDP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 PCROP End address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1er](index.html) module"]
pub struct PCROP1ER_SPEC;
impl crate::RegisterSpec for PCROP1ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1er::R](R) reader structure"]
impl crate::Readable for PCROP1ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1er::W](W) writer structure"]
impl crate::Writable for PCROP1ER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP1ER to value 0x0fff_0000"]
impl crate::Resettable for PCROP1ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}

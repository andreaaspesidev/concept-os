#[doc = "Register `FS_DIEPTXF1` reader"]
pub struct R(crate::R<FS_DIEPTXF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_DIEPTXF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_DIEPTXF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_DIEPTXF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_DIEPTXF1` writer"]
pub struct W(crate::W<FS_DIEPTXF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_DIEPTXF1_SPEC>;
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
impl From<crate::W<FS_DIEPTXF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_DIEPTXF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFO2 transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFO2 transmit RAM start address"]
pub type INEPTXSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FS_DIEPTXF1_SPEC, u16, u16, 16, O>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FS_DIEPTXF1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<0> {
        INEPTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<16> {
        INEPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_dieptxf1](index.html) module"]
pub struct FS_DIEPTXF1_SPEC;
impl crate::RegisterSpec for FS_DIEPTXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_dieptxf1::R](R) reader structure"]
impl crate::Readable for FS_DIEPTXF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_dieptxf1::W](W) writer structure"]
impl crate::Writable for FS_DIEPTXF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FS_DIEPTXF1 to value 0x0200_0400"]
impl crate::Resettable for FS_DIEPTXF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0400
    }
}

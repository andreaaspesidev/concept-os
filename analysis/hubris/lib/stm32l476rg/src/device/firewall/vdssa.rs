#[doc = "Register `VDSSA` reader"]
pub struct R(crate::R<VDSSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDSSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDSSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDSSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDSSA` writer"]
pub struct W(crate::W<VDSSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDSSA_SPEC>;
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
impl From<crate::W<VDSSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDSSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD` reader - Volatile data segment start address"]
pub type ADD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADD` writer - Volatile data segment start address"]
pub type ADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, VDSSA_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Volatile data segment start address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<6> {
        ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Volatile data segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdssa](index.html) module"]
pub struct VDSSA_SPEC;
impl crate::RegisterSpec for VDSSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdssa::R](R) reader structure"]
impl crate::Readable for VDSSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdssa::W](W) writer structure"]
impl crate::Writable for VDSSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDSSA to value 0"]
impl crate::Resettable for VDSSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

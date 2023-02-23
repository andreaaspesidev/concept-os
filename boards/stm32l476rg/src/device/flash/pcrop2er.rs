#[doc = "Register `PCROP2ER` reader"]
pub struct R(crate::R<PCROP2ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP2ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP2ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP2ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP2ER` writer"]
pub struct W(crate::W<PCROP2ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP2ER_SPEC>;
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
impl From<crate::W<PCROP2ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP2ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP2_END` reader - Bank 2 PCROP area end offset"]
pub type PCROP2_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCROP2_END` writer - Bank 2 PCROP area end offset"]
pub type PCROP2_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP2ER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&self) -> PCROP2_END_R {
        PCROP2_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&mut self) -> PCROP2_END_W<0> {
        PCROP2_END_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 2 PCROP End address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop2er](index.html) module"]
pub struct PCROP2ER_SPEC;
impl crate::RegisterSpec for PCROP2ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop2er::R](R) reader structure"]
impl crate::Readable for PCROP2ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop2er::W](W) writer structure"]
impl crate::Writable for PCROP2ER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP2ER to value 0xffff_0000"]
impl crate::Resettable for PCROP2ER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}

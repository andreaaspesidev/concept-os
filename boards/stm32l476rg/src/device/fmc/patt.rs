#[doc = "Register `PATT` reader"]
pub struct R(crate::R<PATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATT` writer"]
pub struct W(crate::W<PATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATT_SPEC>;
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
impl From<crate::W<PATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTHIZx` reader - ATTHIZx"]
pub type ATTHIZX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHIZx` writer - ATTHIZx"]
pub type ATTHIZX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTHOLDx` reader - ATTHOLDx"]
pub type ATTHOLDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTHOLDx` writer - ATTHOLDx"]
pub type ATTHOLDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTWAITx` reader - ATTWAITx"]
pub type ATTWAITX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTWAITx` writer - ATTWAITx"]
pub type ATTWAITX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTSETx` reader - ATTSETx"]
pub type ATTSETX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTSETx` writer - ATTSETx"]
pub type ATTSETX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W<24> {
        ATTHIZX_W::new(self)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<16> {
        ATTHOLDX_W::new(self)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<8> {
        ATTWAITX_W::new(self)
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&mut self) -> ATTSETX_W<0> {
        ATTSETX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Attribute memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patt](index.html) module"]
pub struct PATT_SPEC;
impl crate::RegisterSpec for PATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patt::R](R) reader structure"]
impl crate::Readable for PATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patt::W](W) writer structure"]
impl crate::Writable for PATT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATT to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}

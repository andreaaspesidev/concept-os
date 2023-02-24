#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPR` reader - Low-power run"]
pub type LPR_R = crate::BitReader<bool>;
#[doc = "Field `LPR` writer - Low-power run"]
pub type LPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<bool>;
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `LPMS` reader - Low-power mode selection"]
pub type LPMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMS` writer - Low-power mode selection"]
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - Low-power run"]
    #[inline(always)]
    pub fn lpr(&mut self) -> LPR_W<14> {
        LPR_W::new(self)
    }
    #[doc = "Bits 9:10 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<9> {
        VOS_W::new(self)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bits 0:2 - Low-power mode selection"]
    #[inline(always)]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0x0200"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}

#[doc = "Register `SCSR` reader"]
pub struct R(crate::R<SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCSR` writer"]
pub struct W(crate::W<SCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCSR_SPEC>;
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
impl From<crate::W<SCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM2BSY` reader - SRAM2 busy by erase operation"]
pub type SRAM2BSY_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2ER` reader - SRAM2 Erase"]
pub type SRAM2ER_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2ER` writer - SRAM2 Erase"]
pub type SRAM2ER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SRAM2 busy by erase operation"]
    #[inline(always)]
    pub fn sram2bsy(&self) -> SRAM2BSY_R {
        SRAM2BSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W<0> {
        SRAM2ER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](index.html) module"]
pub struct SCSR_SPEC;
impl crate::RegisterSpec for SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scsr::R](R) reader structure"]
impl crate::Readable for SCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scsr::W](W) writer structure"]
impl crate::Writable for SCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

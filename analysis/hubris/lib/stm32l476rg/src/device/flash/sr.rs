#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOP` reader - End of operation"]
pub type EOP_R = crate::BitReader<bool>;
#[doc = "Field `EOP` writer - End of operation"]
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OPERR` reader - Operation error"]
pub type OPERR_R = crate::BitReader<bool>;
#[doc = "Field `OPERR` writer - Operation error"]
pub type OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `PROGERR` reader - Programming error"]
pub type PROGERR_R = crate::BitReader<bool>;
#[doc = "Field `PROGERR` writer - Programming error"]
pub type PROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `WRPERR` reader - Write protected error"]
pub type WRPERR_R = crate::BitReader<bool>;
#[doc = "Field `WRPERR` writer - Write protected error"]
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `PGAERR` reader - Programming alignment error"]
pub type PGAERR_R = crate::BitReader<bool>;
#[doc = "Field `PGAERR` writer - Programming alignment error"]
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `SIZERR` reader - Size error"]
pub type SIZERR_R = crate::BitReader<bool>;
#[doc = "Field `SIZERR` writer - Size error"]
pub type SIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `PGSERR` reader - Programming sequence error"]
pub type PGSERR_R = crate::BitReader<bool>;
#[doc = "Field `PGSERR` writer - Programming sequence error"]
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `MISERR` reader - Fast programming data miss error"]
pub type MISERR_R = crate::BitReader<bool>;
#[doc = "Field `MISERR` writer - Fast programming data miss error"]
pub type MISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `FASTERR` reader - Fast programming error"]
pub type FASTERR_R = crate::BitReader<bool>;
#[doc = "Field `FASTERR` writer - Fast programming error"]
pub type FASTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RDERR` reader - PCROP read error"]
pub type RDERR_R = crate::BitReader<bool>;
#[doc = "Field `RDERR` writer - PCROP read error"]
pub type RDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OPTVERR` reader - Option validity error"]
pub type OPTVERR_R = crate::BitReader<bool>;
#[doc = "Field `OPTVERR` writer - Option validity error"]
pub type OPTVERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `BSY` reader - Busy"]
pub type BSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W<0> {
        EOP_W::new(self)
    }
    #[doc = "Bit 1 - Operation error"]
    #[inline(always)]
    pub fn operr(&mut self) -> OPERR_W<1> {
        OPERR_W::new(self)
    }
    #[doc = "Bit 3 - Programming error"]
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W<3> {
        PROGERR_W::new(self)
    }
    #[doc = "Bit 4 - Write protected error"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W<4> {
        WRPERR_W::new(self)
    }
    #[doc = "Bit 5 - Programming alignment error"]
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PGAERR_W<5> {
        PGAERR_W::new(self)
    }
    #[doc = "Bit 6 - Size error"]
    #[inline(always)]
    pub fn sizerr(&mut self) -> SIZERR_W<6> {
        SIZERR_W::new(self)
    }
    #[doc = "Bit 7 - Programming sequence error"]
    #[inline(always)]
    pub fn pgserr(&mut self) -> PGSERR_W<7> {
        PGSERR_W::new(self)
    }
    #[doc = "Bit 8 - Fast programming data miss error"]
    #[inline(always)]
    pub fn miserr(&mut self) -> MISERR_W<8> {
        MISERR_W::new(self)
    }
    #[doc = "Bit 9 - Fast programming error"]
    #[inline(always)]
    pub fn fasterr(&mut self) -> FASTERR_W<9> {
        FASTERR_W::new(self)
    }
    #[doc = "Bit 14 - PCROP read error"]
    #[inline(always)]
    pub fn rderr(&mut self) -> RDERR_W<14> {
        RDERR_W::new(self)
    }
    #[doc = "Bit 15 - Option validity error"]
    #[inline(always)]
    pub fn optverr(&mut self) -> OPTVERR_W<15> {
        OPTVERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

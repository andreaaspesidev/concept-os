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
#[doc = "Field `FEMPT` reader - FEMPT"]
pub type FEMPT_R = crate::BitReader<bool>;
#[doc = "Field `IFEN` reader - IFEN"]
pub type IFEN_R = crate::BitReader<bool>;
#[doc = "Field `IFEN` writer - IFEN"]
pub type IFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ILEN` reader - ILEN"]
pub type ILEN_R = crate::BitReader<bool>;
#[doc = "Field `ILEN` writer - ILEN"]
pub type ILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IREN` reader - IREN"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - IREN"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IFS` reader - IFS"]
pub type IFS_R = crate::BitReader<bool>;
#[doc = "Field `IFS` writer - IFS"]
pub type IFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ILS` reader - ILS"]
pub type ILS_R = crate::BitReader<bool>;
#[doc = "Field `ILS` writer - ILS"]
pub type ILS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IRS` reader - IRS"]
pub type IRS_R = crate::BitReader<bool>;
#[doc = "Field `IRS` writer - IRS"]
pub type IRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - FEMPT"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IFEN"]
    #[inline(always)]
    pub fn ifen(&mut self) -> IFEN_W<5> {
        IFEN_W::new(self)
    }
    #[doc = "Bit 4 - ILEN"]
    #[inline(always)]
    pub fn ilen(&mut self) -> ILEN_W<4> {
        ILEN_W::new(self)
    }
    #[doc = "Bit 3 - IREN"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<3> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IFS"]
    #[inline(always)]
    pub fn ifs(&mut self) -> IFS_W<2> {
        IFS_W::new(self)
    }
    #[doc = "Bit 1 - ILS"]
    #[inline(always)]
    pub fn ils(&mut self) -> ILS_W<1> {
        ILS_W::new(self)
    }
    #[doc = "Bit 0 - IRS"]
    #[inline(always)]
    pub fn irs(&mut self) -> IRS_W<0> {
        IRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO status and interrupt register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
#[doc = "`reset()` method sets SR to value 0x40"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}

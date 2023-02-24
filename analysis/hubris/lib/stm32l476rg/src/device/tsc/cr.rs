#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTPH` reader - Charge transfer pulse high"]
pub type CTPH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTPH` writer - Charge transfer pulse high"]
pub type CTPH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CTPL` reader - Charge transfer pulse low"]
pub type CTPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTPL` writer - Charge transfer pulse low"]
pub type CTPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SSD` reader - Spread spectrum deviation"]
pub type SSD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSD` writer - Spread spectrum deviation"]
pub type SSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `SSE` reader - Spread spectrum enable"]
pub type SSE_R = crate::BitReader<bool>;
#[doc = "Field `SSE` writer - Spread spectrum enable"]
pub type SSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SSPSC` reader - Spread spectrum prescaler"]
pub type SSPSC_R = crate::BitReader<bool>;
#[doc = "Field `SSPSC` writer - Spread spectrum prescaler"]
pub type SSPSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PGPSC` reader - pulse generator prescaler"]
pub type PGPSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGPSC` writer - pulse generator prescaler"]
pub type PGPSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCV` reader - Max count value"]
pub type MCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCV` writer - Max count value"]
pub type MCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IODEF` reader - I/O Default mode"]
pub type IODEF_R = crate::BitReader<bool>;
#[doc = "Field `IODEF` writer - I/O Default mode"]
pub type IODEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SYNCPOL` reader - Synchronization pin polarity"]
pub type SYNCPOL_R = crate::BitReader<bool>;
#[doc = "Field `SYNCPOL` writer - Synchronization pin polarity"]
pub type SYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `AM` reader - Acquisition mode"]
pub type AM_R = crate::BitReader<bool>;
#[doc = "Field `AM` writer - Acquisition mode"]
pub type AM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `START` reader - Start a new acquisition"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start a new acquisition"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSCE` reader - Touch sensing controller enable"]
pub type TSCE_R = crate::BitReader<bool>;
#[doc = "Field `TSCE` writer - Touch sensing controller enable"]
pub type TSCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 28:31 - Charge transfer pulse high"]
    #[inline(always)]
    pub fn ctph(&self) -> CTPH_R {
        CTPH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low"]
    #[inline(always)]
    pub fn ctpl(&self) -> CTPL_R {
        CTPL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation"]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Spread spectrum enable"]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler"]
    #[inline(always)]
    pub fn sspsc(&self) -> SSPSC_R {
        SSPSC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - pulse generator prescaler"]
    #[inline(always)]
    pub fn pgpsc(&self) -> PGPSC_R {
        PGPSC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Max count value"]
    #[inline(always)]
    pub fn mcv(&self) -> MCV_R {
        MCV_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 4 - I/O Default mode"]
    #[inline(always)]
    pub fn iodef(&self) -> IODEF_R {
        IODEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization pin polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Acquisition mode"]
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Start a new acquisition"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Touch sensing controller enable"]
    #[inline(always)]
    pub fn tsce(&self) -> TSCE_R {
        TSCE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31 - Charge transfer pulse high"]
    #[inline(always)]
    pub fn ctph(&mut self) -> CTPH_W<28> {
        CTPH_W::new(self)
    }
    #[doc = "Bits 24:27 - Charge transfer pulse low"]
    #[inline(always)]
    pub fn ctpl(&mut self) -> CTPL_W<24> {
        CTPL_W::new(self)
    }
    #[doc = "Bits 17:23 - Spread spectrum deviation"]
    #[inline(always)]
    pub fn ssd(&mut self) -> SSD_W<17> {
        SSD_W::new(self)
    }
    #[doc = "Bit 16 - Spread spectrum enable"]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W<16> {
        SSE_W::new(self)
    }
    #[doc = "Bit 15 - Spread spectrum prescaler"]
    #[inline(always)]
    pub fn sspsc(&mut self) -> SSPSC_W<15> {
        SSPSC_W::new(self)
    }
    #[doc = "Bits 12:14 - pulse generator prescaler"]
    #[inline(always)]
    pub fn pgpsc(&mut self) -> PGPSC_W<12> {
        PGPSC_W::new(self)
    }
    #[doc = "Bits 5:7 - Max count value"]
    #[inline(always)]
    pub fn mcv(&mut self) -> MCV_W<5> {
        MCV_W::new(self)
    }
    #[doc = "Bit 4 - I/O Default mode"]
    #[inline(always)]
    pub fn iodef(&mut self) -> IODEF_W<4> {
        IODEF_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization pin polarity"]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<3> {
        SYNCPOL_W::new(self)
    }
    #[doc = "Bit 2 - Acquisition mode"]
    #[inline(always)]
    pub fn am(&mut self) -> AM_W<2> {
        AM_W::new(self)
    }
    #[doc = "Bit 1 - Start a new acquisition"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<1> {
        START_W::new(self)
    }
    #[doc = "Bit 0 - Touch sensing controller enable"]
    #[inline(always)]
    pub fn tsce(&mut self) -> TSCE_W<0> {
        TSCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `TAMPCR` reader"]
pub struct R(crate::R<TAMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPCR` writer"]
pub struct W(crate::W<TAMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPCR_SPEC>;
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
impl From<crate::W<TAMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub type TAMP1E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub type TAMP1TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP2E` reader - Tamper 2 detection enable"]
pub type TAMP2E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2E` writer - Tamper 2 detection enable"]
pub type TAMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2"]
pub type TAMP2TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2"]
pub type TAMP2TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP3E` reader - Tamper 3 detection enable"]
pub type TAMP3E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3E` writer - Tamper 3 detection enable"]
pub type TAMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3"]
pub type TAMP3TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3"]
pub type TAMP3TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TAMPFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TAMPFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMPCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TAMPFLT` reader - Tamper filter count"]
pub type TAMPFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPFLT` writer - Tamper filter count"]
pub type TAMPFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAMPPRCH` reader - Tamper precharge duration"]
pub type TAMPPRCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPPRCH` writer - Tamper precharge duration"]
pub type TAMPPRCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMPCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAMPPUDIS` reader - TAMPER pull-up disable"]
pub type TAMPPUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPPUDIS` writer - TAMPER pull-up disable"]
pub type TAMPPUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type TAMP1IE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type TAMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP1NOERASE` reader - Tamper 1 no erase"]
pub type TAMP1NOERASE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1NOERASE` writer - Tamper 1 no erase"]
pub type TAMP1NOERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP1MF` reader - Tamper 1 mask flag"]
pub type TAMP1MF_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1MF` writer - Tamper 1 mask flag"]
pub type TAMP1MF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type TAMP2IE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub type TAMP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP2NOERASE` reader - Tamper 2 no erase"]
pub type TAMP2NOERASE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2NOERASE` writer - Tamper 2 no erase"]
pub type TAMP2NOERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP2MF` reader - Tamper 2 mask flag"]
pub type TAMP2MF_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2MF` writer - Tamper 2 mask flag"]
pub type TAMP2MF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub type TAMP3IE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub type TAMP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP3NOERASE` reader - Tamper 3 no erase"]
pub type TAMP3NOERASE_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3NOERASE` writer - Tamper 3 no erase"]
pub type TAMP3NOERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
#[doc = "Field `TAMP3MF` reader - Tamper 3 mask flag"]
pub type TAMP3MF_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3MF` writer - Tamper 3 mask flag"]
pub type TAMP3MF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&self) -> TAMP2NOERASE_R {
        TAMP2NOERASE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&self) -> TAMP3NOERASE_R {
        TAMP3NOERASE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<1> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<3> {
        TAMP2E_W::new(self)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<4> {
        TAMP2TRG_W::new(self)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<5> {
        TAMP3E_W::new(self)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<6> {
        TAMP3TRG_W::new(self)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<7> {
        TAMPTS_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<8> {
        TAMPFREQ_W::new(self)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<11> {
        TAMPFLT_W::new(self)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<13> {
        TAMPPRCH_W::new(self)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<15> {
        TAMPPUDIS_W::new(self)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W<16> {
        TAMP1IE_W::new(self)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W<17> {
        TAMP1NOERASE_W::new(self)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W<18> {
        TAMP1MF_W::new(self)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W<19> {
        TAMP2IE_W::new(self)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&mut self) -> TAMP2NOERASE_W<20> {
        TAMP2NOERASE_W::new(self)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&mut self) -> TAMP2MF_W<21> {
        TAMP2MF_W::new(self)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W<22> {
        TAMP3IE_W::new(self)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&mut self) -> TAMP3NOERASE_W<23> {
        TAMP3NOERASE_W::new(self)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&mut self) -> TAMP3MF_W<24> {
        TAMP3MF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](index.html) module"]
pub struct TAMPCR_SPEC;
impl crate::RegisterSpec for TAMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampcr::R](R) reader structure"]
impl crate::Readable for TAMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampcr::W](W) writer structure"]
impl crate::Writable for TAMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMPCR to value 0"]
impl crate::Resettable for TAMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

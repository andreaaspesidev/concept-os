#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JQDIS` reader - Injected Queue disable"]
pub type JQDIS_R = crate::BitReader<bool>;
#[doc = "Field `JQDIS` writer - Injected Queue disable"]
pub type JQDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `AWDCH1CH` reader - AWDCH1CH"]
pub type AWDCH1CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDCH1CH` writer - AWDCH1CH"]
pub type AWDCH1CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `JAUTO` reader - JAUTO"]
pub type JAUTO_R = crate::BitReader<bool>;
#[doc = "Field `JAUTO` writer - JAUTO"]
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub type JAWD1EN_R = crate::BitReader<bool>;
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub type JAWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub type AWD1EN_R = crate::BitReader<bool>;
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub type AWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub type AWD1SGL_R = crate::BitReader<bool>;
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub type AWD1SGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `JQM` reader - JQM"]
pub type JQM_R = crate::BitReader<bool>;
#[doc = "Field `JQM` writer - JQM"]
pub type JQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub type JDISCEN_R = crate::BitReader<bool>;
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DISCEN` reader - DISCEN"]
pub type DISCEN_R = crate::BitReader<bool>;
#[doc = "Field `DISCEN` writer - DISCEN"]
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub type AUTDLY_R = crate::BitReader<bool>;
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub type AUTDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `CONT` reader - CONT"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - CONT"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub type OVRMOD_R = crate::BitReader<bool>;
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub type OVRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `EXTEN` reader - EXTEN"]
pub type EXTEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTEN` writer - EXTEN"]
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EXTSEL` reader - EXTSEL"]
pub type EXTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTSEL` writer - EXTSEL"]
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ALIGN` reader - ALIGN"]
pub type ALIGN_R = crate::BitReader<bool>;
#[doc = "Field `ALIGN` writer - ALIGN"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `RES` reader - RES"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RES` writer - RES"]
pub type RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DFSDMCFG` reader - DFSDMCFG"]
pub type DFSDMCFG_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMCFG` writer - DFSDMCFG"]
pub type DFSDMCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DMACFG` reader - DMACFG"]
pub type DMACFG_R = crate::BitReader<bool>;
#[doc = "Field `DMACFG` writer - DMACFG"]
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Injected Queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awdch1ch(&self) -> AWDCH1CH_R {
        AWDCH1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 2 - DFSDMCFG"]
    #[inline(always)]
    pub fn dfsdmcfg(&self) -> DFSDMCFG_R {
        DFSDMCFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Injected Queue disable"]
    #[inline(always)]
    pub fn jqdis(&mut self) -> JQDIS_W<31> {
        JQDIS_W::new(self)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awdch1ch(&mut self) -> AWDCH1CH_W<26> {
        AWDCH1CH_W::new(self)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JAUTO_W<25> {
        JAUTO_W::new(self)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<24> {
        JAWD1EN_W::new(self)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> AWD1EN_W<23> {
        AWD1EN_W::new(self)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<22> {
        AWD1SGL_W::new(self)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JQM_W<21> {
        JQM_W::new(self)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JDISCEN_W<20> {
        JDISCEN_W::new(self)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DISCNUM_W<17> {
        DISCNUM_W::new(self)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AUTDLY_W<14> {
        AUTDLY_W::new(self)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&mut self) -> EXTSEL_W<6> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&mut self) -> ALIGN_W<5> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Bit 2 - DFSDMCFG"]
    #[inline(always)]
    pub fn dfsdmcfg(&mut self) -> DFSDMCFG_W<2> {
        DFSDMCFG_W::new(self)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&mut self) -> DMACFG_W<1> {
        DMACFG_W::new(self)
    }
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JQOVFIE` reader - JQOVFIE"]
pub type JQOVFIE_R = crate::BitReader<bool>;
#[doc = "Field `JQOVFIE` writer - JQOVFIE"]
pub type JQOVFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `AWD3IE` reader - AWD3IE"]
pub type AWD3IE_R = crate::BitReader<bool>;
#[doc = "Field `AWD3IE` writer - AWD3IE"]
pub type AWD3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `AWD2IE` reader - AWD2IE"]
pub type AWD2IE_R = crate::BitReader<bool>;
#[doc = "Field `AWD2IE` writer - AWD2IE"]
pub type AWD2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `AWD1IE` reader - AWD1IE"]
pub type AWD1IE_R = crate::BitReader<bool>;
#[doc = "Field `AWD1IE` writer - AWD1IE"]
pub type AWD1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `JEOSIE` reader - JEOSIE"]
pub type JEOSIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOSIE` writer - JEOSIE"]
pub type JEOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `JEOCIE` reader - JEOCIE"]
pub type JEOCIE_R = crate::BitReader<bool>;
#[doc = "Field `JEOCIE` writer - JEOCIE"]
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OVRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EOSIE` reader - EOSIE"]
pub type EOSIE_R = crate::BitReader<bool>;
#[doc = "Field `EOSIE` writer - EOSIE"]
pub type EOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EOCIE` reader - EOCIE"]
pub type EOCIE_R = crate::BitReader<bool>;
#[doc = "Field `EOCIE` writer - EOCIE"]
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `EOSMPIE` reader - EOSMPIE"]
pub type EOSMPIE_R = crate::BitReader<bool>;
#[doc = "Field `EOSMPIE` writer - EOSMPIE"]
pub type EOSMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `ADRDYIE` reader - ADRDYIE"]
pub type ADRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `ADRDYIE` writer - ADRDYIE"]
pub type ADRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - JQOVFIE"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3IE"]
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2IE"]
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1IE"]
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - JEOSIE"]
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - EOSIE"]
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - EOCIE"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMPIE"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - ADRDYIE"]
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - JQOVFIE"]
    #[inline(always)]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<10> {
        JQOVFIE_W::new(self)
    }
    #[doc = "Bit 9 - AWD3IE"]
    #[inline(always)]
    pub fn awd3ie(&mut self) -> AWD3IE_W<9> {
        AWD3IE_W::new(self)
    }
    #[doc = "Bit 8 - AWD2IE"]
    #[inline(always)]
    pub fn awd2ie(&mut self) -> AWD2IE_W<8> {
        AWD2IE_W::new(self)
    }
    #[doc = "Bit 7 - AWD1IE"]
    #[inline(always)]
    pub fn awd1ie(&mut self) -> AWD1IE_W<7> {
        AWD1IE_W::new(self)
    }
    #[doc = "Bit 6 - JEOSIE"]
    #[inline(always)]
    pub fn jeosie(&mut self) -> JEOSIE_W<6> {
        JEOSIE_W::new(self)
    }
    #[doc = "Bit 5 - JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W<5> {
        JEOCIE_W::new(self)
    }
    #[doc = "Bit 4 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<4> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 3 - EOSIE"]
    #[inline(always)]
    pub fn eosie(&mut self) -> EOSIE_W<3> {
        EOSIE_W::new(self)
    }
    #[doc = "Bit 2 - EOCIE"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W<2> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 1 - EOSMPIE"]
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<1> {
        EOSMPIE_W::new(self)
    }
    #[doc = "Bit 0 - ADRDYIE"]
    #[inline(always)]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<0> {
        ADRDYIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

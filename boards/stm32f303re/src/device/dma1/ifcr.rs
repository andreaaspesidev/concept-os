#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 1 Global interrupt clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGIF1_AW {
    #[doc = "1: Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    Clear = 1,
}
impl From<CGIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CGIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CGIF1` writer - Channel 1 Global interrupt clear"]
pub type CGIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CGIF1_AW, O>;
impl<'a, const O: u8> CGIF1_W<'a, O> {
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1_AW::Clear)
    }
}
#[doc = "Channel 1 Transfer Complete clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF1_AW {
    #[doc = "1: Clears the TCIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTCIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF1` writer - Channel 1 Transfer Complete clear"]
pub type CTCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTCIF1_AW, O>;
impl<'a, const O: u8> CTCIF1_W<'a, O> {
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1_AW::Clear)
    }
}
#[doc = "Channel 1 Half Transfer clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHTIF1_AW {
    #[doc = "1: Clears the HTIF flag in the ISR register"]
    Clear = 1,
}
impl From<CHTIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF1` writer - Channel 1 Half Transfer clear"]
pub type CHTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CHTIF1_AW, O>;
impl<'a, const O: u8> CHTIF1_W<'a, O> {
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1_AW::Clear)
    }
}
#[doc = "Channel 1 Transfer Error clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEIF1_AW {
    #[doc = "1: Clears the TEIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTEIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF1` writer - Channel 1 Transfer Error clear"]
pub type CTEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTEIF1_AW, O>;
impl<'a, const O: u8> CTEIF1_W<'a, O> {
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1_AW::Clear)
    }
}
#[doc = "Channel 2 Global interrupt clear"]
pub use CGIF1_AW as CGIF2_AW;
#[doc = "Channel 3 Global interrupt clear"]
pub use CGIF1_AW as CGIF3_AW;
#[doc = "Channel 4 Global interrupt clear"]
pub use CGIF1_AW as CGIF4_AW;
#[doc = "Channel 5 Global interrupt clear"]
pub use CGIF1_AW as CGIF5_AW;
#[doc = "Channel 6 Global interrupt clear"]
pub use CGIF1_AW as CGIF6_AW;
#[doc = "Channel 7 Global interrupt clear"]
pub use CGIF1_AW as CGIF7_AW;
#[doc = "Field `CGIF2` writer - Channel 2 Global interrupt clear"]
pub use CGIF1_W as CGIF2_W;
#[doc = "Field `CGIF3` writer - Channel 3 Global interrupt clear"]
pub use CGIF1_W as CGIF3_W;
#[doc = "Field `CGIF4` writer - Channel 4 Global interrupt clear"]
pub use CGIF1_W as CGIF4_W;
#[doc = "Field `CGIF5` writer - Channel 5 Global interrupt clear"]
pub use CGIF1_W as CGIF5_W;
#[doc = "Field `CGIF6` writer - Channel 6 Global interrupt clear"]
pub use CGIF1_W as CGIF6_W;
#[doc = "Field `CGIF7` writer - Channel 7 Global interrupt clear"]
pub use CGIF1_W as CGIF7_W;
#[doc = "Channel 2 Half Transfer clear"]
pub use CHTIF1_AW as CHTIF2_AW;
#[doc = "Channel 3 Half Transfer clear"]
pub use CHTIF1_AW as CHTIF3_AW;
#[doc = "Channel 4 Half Transfer clear"]
pub use CHTIF1_AW as CHTIF4_AW;
#[doc = "Channel 5 Half Transfer clear"]
pub use CHTIF1_AW as CHTIF5_AW;
#[doc = "Channel 6 Half Transfer clear"]
pub use CHTIF1_AW as CHTIF6_AW;
#[doc = "Channel 7 Half Transfer clear"]
pub use CHTIF1_AW as CHTIF7_AW;
#[doc = "Field `CHTIF2` writer - Channel 2 Half Transfer clear"]
pub use CHTIF1_W as CHTIF2_W;
#[doc = "Field `CHTIF3` writer - Channel 3 Half Transfer clear"]
pub use CHTIF1_W as CHTIF3_W;
#[doc = "Field `CHTIF4` writer - Channel 4 Half Transfer clear"]
pub use CHTIF1_W as CHTIF4_W;
#[doc = "Field `CHTIF5` writer - Channel 5 Half Transfer clear"]
pub use CHTIF1_W as CHTIF5_W;
#[doc = "Field `CHTIF6` writer - Channel 6 Half Transfer clear"]
pub use CHTIF1_W as CHTIF6_W;
#[doc = "Field `CHTIF7` writer - Channel 7 Half Transfer clear"]
pub use CHTIF1_W as CHTIF7_W;
#[doc = "Channel 2 Transfer Complete clear"]
pub use CTCIF1_AW as CTCIF2_AW;
#[doc = "Channel 3 Transfer Complete clear"]
pub use CTCIF1_AW as CTCIF3_AW;
#[doc = "Channel 4 Transfer Complete clear"]
pub use CTCIF1_AW as CTCIF4_AW;
#[doc = "Channel 5 Transfer Complete clear"]
pub use CTCIF1_AW as CTCIF5_AW;
#[doc = "Channel 6 Transfer Complete clear"]
pub use CTCIF1_AW as CTCIF6_AW;
#[doc = "Channel 7 Transfer Complete clear"]
pub use CTCIF1_AW as CTCIF7_AW;
#[doc = "Field `CTCIF2` writer - Channel 2 Transfer Complete clear"]
pub use CTCIF1_W as CTCIF2_W;
#[doc = "Field `CTCIF3` writer - Channel 3 Transfer Complete clear"]
pub use CTCIF1_W as CTCIF3_W;
#[doc = "Field `CTCIF4` writer - Channel 4 Transfer Complete clear"]
pub use CTCIF1_W as CTCIF4_W;
#[doc = "Field `CTCIF5` writer - Channel 5 Transfer Complete clear"]
pub use CTCIF1_W as CTCIF5_W;
#[doc = "Field `CTCIF6` writer - Channel 6 Transfer Complete clear"]
pub use CTCIF1_W as CTCIF6_W;
#[doc = "Field `CTCIF7` writer - Channel 7 Transfer Complete clear"]
pub use CTCIF1_W as CTCIF7_W;
#[doc = "Channel 2 Transfer Error clear"]
pub use CTEIF1_AW as CTEIF2_AW;
#[doc = "Channel 3 Transfer Error clear"]
pub use CTEIF1_AW as CTEIF3_AW;
#[doc = "Channel 4 Transfer Error clear"]
pub use CTEIF1_AW as CTEIF4_AW;
#[doc = "Channel 5 Transfer Error clear"]
pub use CTEIF1_AW as CTEIF5_AW;
#[doc = "Channel 6 Transfer Error clear"]
pub use CTEIF1_AW as CTEIF6_AW;
#[doc = "Channel 7 Transfer Error clear"]
pub use CTEIF1_AW as CTEIF7_AW;
#[doc = "Field `CTEIF2` writer - Channel 2 Transfer Error clear"]
pub use CTEIF1_W as CTEIF2_W;
#[doc = "Field `CTEIF3` writer - Channel 3 Transfer Error clear"]
pub use CTEIF1_W as CTEIF3_W;
#[doc = "Field `CTEIF4` writer - Channel 4 Transfer Error clear"]
pub use CTEIF1_W as CTEIF4_W;
#[doc = "Field `CTEIF5` writer - Channel 5 Transfer Error clear"]
pub use CTEIF1_W as CTEIF5_W;
#[doc = "Field `CTEIF6` writer - Channel 6 Transfer Error clear"]
pub use CTEIF1_W as CTEIF6_W;
#[doc = "Field `CTEIF7` writer - Channel 7 Transfer Error clear"]
pub use CTEIF1_W as CTEIF7_W;
impl W {
    #[doc = "Bit 0 - Channel 1 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF1_W<0> {
        CGIF1_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<1> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<2> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<3> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF2_W<4> {
        CGIF2_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<5> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<6> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<7> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 8 - Channel 3 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF3_W<8> {
        CGIF3_W::new(self)
    }
    #[doc = "Bit 9 - Channel 3 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<9> {
        CTCIF3_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<10> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<11> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF4_W<12> {
        CGIF4_W::new(self)
    }
    #[doc = "Bit 13 - Channel 4 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<13> {
        CTCIF4_W::new(self)
    }
    #[doc = "Bit 14 - Channel 4 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<14> {
        CHTIF4_W::new(self)
    }
    #[doc = "Bit 15 - Channel 4 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<15> {
        CTEIF4_W::new(self)
    }
    #[doc = "Bit 16 - Channel 5 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF5_W<16> {
        CGIF5_W::new(self)
    }
    #[doc = "Bit 17 - Channel 5 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<17> {
        CTCIF5_W::new(self)
    }
    #[doc = "Bit 18 - Channel 5 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<18> {
        CHTIF5_W::new(self)
    }
    #[doc = "Bit 19 - Channel 5 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<19> {
        CTEIF5_W::new(self)
    }
    #[doc = "Bit 20 - Channel 6 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF6_W<20> {
        CGIF6_W::new(self)
    }
    #[doc = "Bit 21 - Channel 6 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<21> {
        CTCIF6_W::new(self)
    }
    #[doc = "Bit 22 - Channel 6 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<22> {
        CHTIF6_W::new(self)
    }
    #[doc = "Bit 23 - Channel 6 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<23> {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 24 - Channel 7 Global interrupt clear"]
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF7_W<24> {
        CGIF7_W::new(self)
    }
    #[doc = "Bit 25 - Channel 7 Transfer Complete clear"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<25> {
        CTCIF7_W::new(self)
    }
    #[doc = "Bit 26 - Channel 7 Half Transfer clear"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<26> {
        CHTIF7_W::new(self)
    }
    #[doc = "Bit 27 - Channel 7 Transfer Error clear"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<27> {
        CTEIF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA interrupt flag clear register (DMA_IFCR)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

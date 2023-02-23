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
#[doc = "Field `RXBFIE` reader - Receive buffer full interrupt enable"]
pub type RXBFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXBFIE` writer - Receive buffer full interrupt enable"]
pub type RXBFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TXBEIE` reader - Transmit buffer empty interrupt enable"]
pub type TXBEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXBEIE` writer - Transmit buffer empty interrupt enable"]
pub type TXBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RXBERIE` reader - Receive CRC error interrupt enable"]
pub type RXBERIE_R = crate::BitReader<bool>;
#[doc = "Field `RXBERIE` writer - Receive CRC error interrupt enable"]
pub type RXBERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RXOVRIE` reader - Receive overrun error interrupt enable"]
pub type RXOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVRIE` writer - Receive overrun error interrupt enable"]
pub type RXOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TXUNRIE` reader - Transmit underrun error interrupt enable"]
pub type TXUNRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUNRIE` writer - Transmit underrun error interrupt enable"]
pub type TXUNRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `TCIE` reader - Transmit complete interrupt enable"]
pub type TCIE_R = crate::BitReader<bool>;
#[doc = "Field `TCIE` writer - Transmit complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
#[doc = "Field `SRIE` reader - Slave resume interrupt enable"]
pub type SRIE_R = crate::BitReader<bool>;
#[doc = "Field `SRIE` writer - Slave resume interrupt enable"]
pub type SRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeie(&self) -> TXBEIE_R {
        TXBEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    pub fn rxberie(&self) -> RXBERIE_R {
        RXBERIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunrie(&self) -> TXUNRIE_R {
        TXUNRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    pub fn srie(&self) -> SRIE_R {
        SRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbfie(&mut self) -> RXBFIE_W<0> {
        RXBFIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeie(&mut self) -> TXBEIE_W<1> {
        TXBEIE_W::new(self)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    pub fn rxberie(&mut self) -> RXBERIE_W<2> {
        RXBERIE_W::new(self)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<3> {
        RXOVRIE_W::new(self)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunrie(&mut self) -> TXUNRIE_W<4> {
        TXUNRIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<5> {
        RIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<7> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    pub fn srie(&mut self) -> SRIE_W<8> {
        SRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SWPMI Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
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

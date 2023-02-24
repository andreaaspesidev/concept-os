#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEATAENDIE` reader - CE-ATA command completion signal received interrupt enable"]
pub type CEATAENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CEATAENDIE` writer - CE-ATA command completion signal received interrupt enable"]
pub type CEATAENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable"]
pub type SDIOITIE_R = crate::BitReader<bool>;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable"]
pub type SDIOITIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `RXDAVLIE` reader - Data available in Rx FIFO interrupt enable"]
pub type RXDAVLIE_R = crate::BitReader<bool>;
#[doc = "Field `RXDAVLIE` writer - Data available in Rx FIFO interrupt enable"]
pub type RXDAVLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `TXDAVLIE` reader - Data available in Tx FIFO interrupt enable"]
pub type TXDAVLIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDAVLIE` writer - Data available in Tx FIFO interrupt enable"]
pub type TXDAVLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `RXFIFOEIE` reader - Rx FIFO empty interrupt enable"]
pub type RXFIFOEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOEIE` writer - Rx FIFO empty interrupt enable"]
pub type RXFIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable"]
pub type TXFIFOEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable"]
pub type TXFIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable"]
pub type RXFIFOFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable"]
pub type RXFIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `TXFIFOFIE` reader - Tx FIFO full interrupt enable"]
pub type TXFIFOFIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOFIE` writer - Tx FIFO full interrupt enable"]
pub type TXFIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable"]
pub type RXFIFOHFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable"]
pub type RXFIFOHFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable"]
pub type TXFIFOHEIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable"]
pub type TXFIFOHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `RXACTIE` reader - Data receive acting interrupt enable"]
pub type RXACTIE_R = crate::BitReader<bool>;
#[doc = "Field `RXACTIE` writer - Data receive acting interrupt enable"]
pub type RXACTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `TXACTIE` reader - Data transmit acting interrupt enable"]
pub type TXACTIE_R = crate::BitReader<bool>;
#[doc = "Field `TXACTIE` writer - Data transmit acting interrupt enable"]
pub type TXACTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `CMDACTIE` reader - Command acting interrupt enable"]
pub type CMDACTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDACTIE` writer - Command acting interrupt enable"]
pub type CMDACTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable"]
pub type DBCKENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable"]
pub type DBCKENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `STBITERRIE` reader - Start bit error interrupt enable"]
pub type STBITERRIE_R = crate::BitReader<bool>;
#[doc = "Field `STBITERRIE` writer - Start bit error interrupt enable"]
pub type STBITERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable"]
pub type DATAENDIE_R = crate::BitReader<bool>;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable"]
pub type DATAENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable"]
pub type CMDSENTIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable"]
pub type CMDSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable"]
pub type CMDRENDIE_R = crate::BitReader<bool>;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable"]
pub type CMDRENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable"]
pub type RXOVERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable"]
pub type RXOVERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable"]
pub type TXUNDERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable"]
pub type TXUNDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable"]
pub type DTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable"]
pub type DTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable"]
pub type CTIMEOUTIE_R = crate::BitReader<bool>;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable"]
pub type CTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable"]
pub type DCRCFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable"]
pub type DCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable"]
pub type CCRCFAILIE_R = crate::BitReader<bool>;
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable"]
pub type CCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ceataendie(&self) -> CEATAENDIE_R {
        CEATAENDIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in Rx FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ceataendie(&mut self) -> CEATAENDIE_W<23> {
        CEATAENDIE_W::new(self)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<22> {
        SDIOITIE_W::new(self)
    }
    #[doc = "Bit 21 - Data available in Rx FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<21> {
        RXDAVLIE_W::new(self)
    }
    #[doc = "Bit 20 - Data available in Tx FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<20> {
        TXDAVLIE_W::new(self)
    }
    #[doc = "Bit 19 - Rx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<19> {
        RXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<18> {
        TXFIFOEIE_W::new(self)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<17> {
        RXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 16 - Tx FIFO full interrupt enable"]
    #[inline(always)]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<16> {
        TXFIFOFIE_W::new(self)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<15> {
        RXFIFOHFIE_W::new(self)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<14> {
        TXFIFOHEIE_W::new(self)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn rxactie(&mut self) -> RXACTIE_W<13> {
        RXACTIE_W::new(self)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn txactie(&mut self) -> TXACTIE_W<12> {
        TXACTIE_W::new(self)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<11> {
        CMDACTIE_W::new(self)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<10> {
        DBCKENDIE_W::new(self)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W<9> {
        STBITERRIE_W::new(self)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W<8> {
        DATAENDIE_W::new(self)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<7> {
        CMDSENTIE_W::new(self)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<6> {
        CMDRENDIE_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<5> {
        RXOVERRIE_W::new(self)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<4> {
        TXUNDERRIE_W::new(self)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<3> {
        DTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<2> {
        CTIMEOUTIE_W::new(self)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<1> {
        DCRCFAILIE_W::new(self)
    }
    #[doc = "Bit 0 - Command CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<0> {
        CCRCFAILIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

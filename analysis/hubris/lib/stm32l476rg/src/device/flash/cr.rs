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
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader<bool>;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PER` reader - Page erase"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MER1` reader - Bank 1 Mass erase"]
pub type MER1_R = crate::BitReader<bool>;
#[doc = "Field `MER1` writer - Bank 1 Mass erase"]
pub type MER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PNB` reader - Page number"]
pub type PNB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PNB` writer - Page number"]
pub type PNB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BKER` reader - Bank erase"]
pub type BKER_R = crate::BitReader<bool>;
#[doc = "Field `BKER` writer - Bank erase"]
pub type BKER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `MER2` reader - Bank 2 Mass erase"]
pub type MER2_R = crate::BitReader<bool>;
#[doc = "Field `MER2` writer - Bank 2 Mass erase"]
pub type MER2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `START` reader - Start"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OPTSTRT` reader - Options modification start"]
pub type OPTSTRT_R = crate::BitReader<bool>;
#[doc = "Field `OPTSTRT` writer - Options modification start"]
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FSTPG` reader - Fast programming"]
pub type FSTPG_R = crate::BitReader<bool>;
#[doc = "Field `FSTPG` writer - Fast programming"]
pub type FSTPG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader<bool>;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RDERRIE` reader - PCROP read error interrupt enable"]
pub type RDERRIE_R = crate::BitReader<bool>;
#[doc = "Field `RDERRIE` writer - PCROP read error interrupt enable"]
pub type RDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading"]
pub type OBL_LAUNCH_R = crate::BitReader<bool>;
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading"]
pub type OBL_LAUNCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OPTLOCK` reader - Options Lock"]
pub type OPTLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OPTLOCK` writer - Options Lock"]
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - FLASH_CR Lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - FLASH_CR Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 Mass erase"]
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - Page number"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Bank erase"]
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Bank 2 Mass erase"]
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<0> {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<1> {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Bank 1 Mass erase"]
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W<2> {
        MER1_W::new(self)
    }
    #[doc = "Bits 3:10 - Page number"]
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<3> {
        PNB_W::new(self)
    }
    #[doc = "Bit 11 - Bank erase"]
    #[inline(always)]
    pub fn bker(&mut self) -> BKER_W<11> {
        BKER_W::new(self)
    }
    #[doc = "Bit 15 - Bank 2 Mass erase"]
    #[inline(always)]
    pub fn mer2(&mut self) -> MER2_W<15> {
        MER2_W::new(self)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<16> {
        START_W::new(self)
    }
    #[doc = "Bit 17 - Options modification start"]
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<17> {
        OPTSTRT_W::new(self)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W<18> {
        FSTPG_W::new(self)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<24> {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<25> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 26 - PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<26> {
        RDERRIE_W::new(self)
    }
    #[doc = "Bit 27 - Force the option byte loading"]
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<27> {
        OBL_LAUNCH_W::new(self)
    }
    #[doc = "Bit 30 - Options Lock"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<30> {
        OPTLOCK_W::new(self)
    }
    #[doc = "Bit 31 - FLASH_CR Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0xc000_0000"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc000_0000
    }
}

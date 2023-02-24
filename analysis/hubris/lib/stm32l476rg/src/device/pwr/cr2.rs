#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USV` reader - VDDUSB USB supply valid"]
pub type USV_R = crate::BitReader<bool>;
#[doc = "Field `USV` writer - VDDUSB USB supply valid"]
pub type USV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `IOSV` reader - VDDIO2 Independent I/Os supply valid"]
pub type IOSV_R = crate::BitReader<bool>;
#[doc = "Field `IOSV` writer - VDDIO2 Independent I/Os supply valid"]
pub type IOSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVME4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type PVME4_R = crate::BitReader<bool>;
#[doc = "Field `PVME4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type PVME4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_R = crate::BitReader<bool>;
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVME2` reader - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type PVME2_R = crate::BitReader<bool>;
#[doc = "Field `PVME2` writer - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type PVME2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVME1` reader - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_R = crate::BitReader<bool>;
#[doc = "Field `PVME1` writer - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    pub fn pvme4(&self) -> PVME4_R {
        PVME4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    pub fn pvme2(&self) -> PVME2_R {
        PVME2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&self) -> PVME1_R {
        PVME1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&mut self) -> USV_W<10> {
        USV_W::new(self)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    pub fn iosv(&mut self) -> IOSV_W<9> {
        IOSV_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    pub fn pvme4(&mut self) -> PVME4_W<7> {
        PVME4_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&mut self) -> PVME3_W<6> {
        PVME3_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    pub fn pvme2(&mut self) -> PVME2_W<5> {
        PVME2_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&mut self) -> PVME1_W<4> {
        PVME1_W::new(self)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W<1> {
        PLS_W::new(self)
    }
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

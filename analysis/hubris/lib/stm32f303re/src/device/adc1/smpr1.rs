#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SMP9"]
pub use SMP1_A as SMP9_A;
#[doc = "SMP8"]
pub use SMP1_A as SMP8_A;
#[doc = "SMP7"]
pub use SMP1_A as SMP7_A;
#[doc = "SMP6"]
pub use SMP1_A as SMP6_A;
#[doc = "SMP5"]
pub use SMP1_A as SMP5_A;
#[doc = "SMP4"]
pub use SMP1_A as SMP4_A;
#[doc = "SMP3"]
pub use SMP1_A as SMP3_A;
#[doc = "SMP2"]
pub use SMP1_A as SMP2_A;
#[doc = "Field `SMP9` reader - SMP9"]
pub use SMP1_R as SMP9_R;
#[doc = "Field `SMP8` reader - SMP8"]
pub use SMP1_R as SMP8_R;
#[doc = "Field `SMP7` reader - SMP7"]
pub use SMP1_R as SMP7_R;
#[doc = "Field `SMP6` reader - SMP6"]
pub use SMP1_R as SMP6_R;
#[doc = "Field `SMP5` reader - SMP5"]
pub use SMP1_R as SMP5_R;
#[doc = "Field `SMP4` reader - SMP4"]
pub use SMP1_R as SMP4_R;
#[doc = "Field `SMP3` reader - SMP3"]
pub use SMP1_R as SMP3_R;
#[doc = "Field `SMP2` reader - SMP2"]
pub use SMP1_R as SMP2_R;
#[doc = "Field `SMP9` writer - SMP9"]
pub use SMP1_W as SMP9_W;
#[doc = "Field `SMP8` writer - SMP8"]
pub use SMP1_W as SMP8_W;
#[doc = "Field `SMP7` writer - SMP7"]
pub use SMP1_W as SMP7_W;
#[doc = "Field `SMP6` writer - SMP6"]
pub use SMP1_W as SMP6_W;
#[doc = "Field `SMP5` writer - SMP5"]
pub use SMP1_W as SMP5_W;
#[doc = "Field `SMP4` writer - SMP4"]
pub use SMP1_W as SMP4_W;
#[doc = "Field `SMP3` writer - SMP3"]
pub use SMP1_W as SMP3_W;
#[doc = "Field `SMP2` writer - SMP2"]
pub use SMP1_W as SMP2_W;
#[doc = "SMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMP1_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles15 = 0,
    #[doc = "1: 2.5 ADC clock cycles"]
    Cycles25 = 1,
    #[doc = "2: 4.5 ADC clock cycles"]
    Cycles45 = 2,
    #[doc = "3: 7.5 ADC clock cycles"]
    Cycles75 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    Cycles195 = 4,
    #[doc = "5: 61.5 ADC clock cycles"]
    Cycles615 = 5,
    #[doc = "6: 181.5 ADC clock cycles"]
    Cycles1815 = 6,
    #[doc = "7: 601.5 ADC clock cycles"]
    Cycles6015 = 7,
}
impl From<SMP1_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMP1` reader - SMP1"]
pub type SMP1_R = crate::FieldReader<u8, SMP1_A>;
impl SMP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP1_A {
        match self.bits {
            0 => SMP1_A::Cycles15,
            1 => SMP1_A::Cycles25,
            2 => SMP1_A::Cycles45,
            3 => SMP1_A::Cycles75,
            4 => SMP1_A::Cycles195,
            5 => SMP1_A::Cycles615,
            6 => SMP1_A::Cycles1815,
            7 => SMP1_A::Cycles6015,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP1_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles25`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP1_A::Cycles25
    }
    #[doc = "Checks if the value of the field is `Cycles45`"]
    #[inline(always)]
    pub fn is_cycles4_5(&self) -> bool {
        *self == SMP1_A::Cycles45
    }
    #[doc = "Checks if the value of the field is `Cycles75`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP1_A::Cycles75
    }
    #[doc = "Checks if the value of the field is `Cycles195`"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP1_A::Cycles195
    }
    #[doc = "Checks if the value of the field is `Cycles615`"]
    #[inline(always)]
    pub fn is_cycles61_5(&self) -> bool {
        *self == SMP1_A::Cycles615
    }
    #[doc = "Checks if the value of the field is `Cycles1815`"]
    #[inline(always)]
    pub fn is_cycles181_5(&self) -> bool {
        *self == SMP1_A::Cycles1815
    }
    #[doc = "Checks if the value of the field is `Cycles6015`"]
    #[inline(always)]
    pub fn is_cycles601_5(&self) -> bool {
        *self == SMP1_A::Cycles6015
    }
}
#[doc = "Field `SMP1` writer - SMP1"]
pub type SMP1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR1_SPEC, u8, SMP1_A, 3, O>;
impl<'a, const O: u8> SMP1_W<'a, O> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles15)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles25)
    }
    #[doc = "4.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles4_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles45)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles75)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles195)
    }
    #[doc = "61.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles61_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles615)
    }
    #[doc = "181.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles181_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles1815)
    }
    #[doc = "601.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles601_5(self) -> &'a mut W {
        self.variant(SMP1_A::Cycles6015)
    }
}
impl R {
    #[doc = "Bits 27:29 - SMP9"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SMP8"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SMP7"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SMP6"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SMP5"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SMP4"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SMP3"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SMP2"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 3:5 - SMP1"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 27:29 - SMP9"]
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W<27> {
        SMP9_W::new(self)
    }
    #[doc = "Bits 24:26 - SMP8"]
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W<24> {
        SMP8_W::new(self)
    }
    #[doc = "Bits 21:23 - SMP7"]
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W<21> {
        SMP7_W::new(self)
    }
    #[doc = "Bits 18:20 - SMP6"]
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W<18> {
        SMP6_W::new(self)
    }
    #[doc = "Bits 15:17 - SMP5"]
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W<15> {
        SMP5_W::new(self)
    }
    #[doc = "Bits 12:14 - SMP4"]
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W<12> {
        SMP4_W::new(self)
    }
    #[doc = "Bits 9:11 - SMP3"]
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W<9> {
        SMP3_W::new(self)
    }
    #[doc = "Bits 6:8 - SMP2"]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W<6> {
        SMP2_W::new(self)
    }
    #[doc = "Bits 3:5 - SMP1"]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W<3> {
        SMP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

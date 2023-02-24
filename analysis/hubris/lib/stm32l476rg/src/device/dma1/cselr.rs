#[doc = "Register `CSELR` reader"]
pub struct R(crate::R<CSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSELR` writer"]
pub struct W(crate::W<CSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSELR_SPEC>;
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
impl From<crate::W<CSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DMA channel 7 selection"]
pub use C1S_A as C7S_A;
#[doc = "DMA channel 6 selection"]
pub use C1S_A as C6S_A;
#[doc = "DMA channel 5 selection"]
pub use C1S_A as C5S_A;
#[doc = "DMA channel 4 selection"]
pub use C1S_A as C4S_A;
#[doc = "DMA channel 3 selection"]
pub use C1S_A as C3S_A;
#[doc = "DMA channel 2 selection"]
pub use C1S_A as C2S_A;
#[doc = "Field `C7S` reader - DMA channel 7 selection"]
pub use C1S_R as C7S_R;
#[doc = "Field `C6S` reader - DMA channel 6 selection"]
pub use C1S_R as C6S_R;
#[doc = "Field `C5S` reader - DMA channel 5 selection"]
pub use C1S_R as C5S_R;
#[doc = "Field `C4S` reader - DMA channel 4 selection"]
pub use C1S_R as C4S_R;
#[doc = "Field `C3S` reader - DMA channel 3 selection"]
pub use C1S_R as C3S_R;
#[doc = "Field `C2S` reader - DMA channel 2 selection"]
pub use C1S_R as C2S_R;
#[doc = "Field `C7S` writer - DMA channel 7 selection"]
pub use C1S_W as C7S_W;
#[doc = "Field `C6S` writer - DMA channel 6 selection"]
pub use C1S_W as C6S_W;
#[doc = "Field `C5S` writer - DMA channel 5 selection"]
pub use C1S_W as C5S_W;
#[doc = "Field `C4S` writer - DMA channel 4 selection"]
pub use C1S_W as C4S_W;
#[doc = "Field `C3S` writer - DMA channel 3 selection"]
pub use C1S_W as C3S_W;
#[doc = "Field `C2S` writer - DMA channel 2 selection"]
pub use C1S_W as C2S_W;
#[doc = "DMA channel 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C1S_A {
    #[doc = "0: Default mapping"]
    NoMapping = 0,
    #[doc = "1: Mapping 1"]
    Map1 = 1,
    #[doc = "2: Mapping 2"]
    Map2 = 2,
    #[doc = "3: Mapping 3"]
    Map3 = 3,
    #[doc = "4: Mapping 4"]
    Map4 = 4,
    #[doc = "5: Mapping 5"]
    Map5 = 5,
    #[doc = "6: Mapping 6"]
    Map6 = 6,
    #[doc = "7: Mapping 7"]
    Map7 = 7,
    #[doc = "8: Mapping 8"]
    Map8 = 8,
    #[doc = "9: Mapping 9"]
    Map9 = 9,
    #[doc = "10: Mapping 10"]
    Map10 = 10,
    #[doc = "11: Mapping 11"]
    Map11 = 11,
    #[doc = "12: Mapping 12"]
    Map12 = 12,
    #[doc = "13: Mapping 13"]
    Map13 = 13,
    #[doc = "14: Mapping 14"]
    Map14 = 14,
    #[doc = "15: Mapping 15"]
    Map15 = 15,
}
impl From<C1S_A> for u8 {
    #[inline(always)]
    fn from(variant: C1S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `C1S` reader - DMA channel 1 selection"]
pub type C1S_R = crate::FieldReader<u8, C1S_A>;
impl C1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1S_A {
        match self.bits {
            0 => C1S_A::NoMapping,
            1 => C1S_A::Map1,
            2 => C1S_A::Map2,
            3 => C1S_A::Map3,
            4 => C1S_A::Map4,
            5 => C1S_A::Map5,
            6 => C1S_A::Map6,
            7 => C1S_A::Map7,
            8 => C1S_A::Map8,
            9 => C1S_A::Map9,
            10 => C1S_A::Map10,
            11 => C1S_A::Map11,
            12 => C1S_A::Map12,
            13 => C1S_A::Map13,
            14 => C1S_A::Map14,
            15 => C1S_A::Map15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoMapping`"]
    #[inline(always)]
    pub fn is_no_mapping(&self) -> bool {
        *self == C1S_A::NoMapping
    }
    #[doc = "Checks if the value of the field is `Map1`"]
    #[inline(always)]
    pub fn is_map1(&self) -> bool {
        *self == C1S_A::Map1
    }
    #[doc = "Checks if the value of the field is `Map2`"]
    #[inline(always)]
    pub fn is_map2(&self) -> bool {
        *self == C1S_A::Map2
    }
    #[doc = "Checks if the value of the field is `Map3`"]
    #[inline(always)]
    pub fn is_map3(&self) -> bool {
        *self == C1S_A::Map3
    }
    #[doc = "Checks if the value of the field is `Map4`"]
    #[inline(always)]
    pub fn is_map4(&self) -> bool {
        *self == C1S_A::Map4
    }
    #[doc = "Checks if the value of the field is `Map5`"]
    #[inline(always)]
    pub fn is_map5(&self) -> bool {
        *self == C1S_A::Map5
    }
    #[doc = "Checks if the value of the field is `Map6`"]
    #[inline(always)]
    pub fn is_map6(&self) -> bool {
        *self == C1S_A::Map6
    }
    #[doc = "Checks if the value of the field is `Map7`"]
    #[inline(always)]
    pub fn is_map7(&self) -> bool {
        *self == C1S_A::Map7
    }
    #[doc = "Checks if the value of the field is `Map8`"]
    #[inline(always)]
    pub fn is_map8(&self) -> bool {
        *self == C1S_A::Map8
    }
    #[doc = "Checks if the value of the field is `Map9`"]
    #[inline(always)]
    pub fn is_map9(&self) -> bool {
        *self == C1S_A::Map9
    }
    #[doc = "Checks if the value of the field is `Map10`"]
    #[inline(always)]
    pub fn is_map10(&self) -> bool {
        *self == C1S_A::Map10
    }
    #[doc = "Checks if the value of the field is `Map11`"]
    #[inline(always)]
    pub fn is_map11(&self) -> bool {
        *self == C1S_A::Map11
    }
    #[doc = "Checks if the value of the field is `Map12`"]
    #[inline(always)]
    pub fn is_map12(&self) -> bool {
        *self == C1S_A::Map12
    }
    #[doc = "Checks if the value of the field is `Map13`"]
    #[inline(always)]
    pub fn is_map13(&self) -> bool {
        *self == C1S_A::Map13
    }
    #[doc = "Checks if the value of the field is `Map14`"]
    #[inline(always)]
    pub fn is_map14(&self) -> bool {
        *self == C1S_A::Map14
    }
    #[doc = "Checks if the value of the field is `Map15`"]
    #[inline(always)]
    pub fn is_map15(&self) -> bool {
        *self == C1S_A::Map15
    }
}
#[doc = "Field `C1S` writer - DMA channel 1 selection"]
pub type C1S_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSELR_SPEC, u8, C1S_A, 4, O>;
impl<'a, const O: u8> C1S_W<'a, O> {
    #[doc = "Default mapping"]
    #[inline(always)]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C1S_A::NoMapping)
    }
    #[doc = "Mapping 1"]
    #[inline(always)]
    pub fn map1(self) -> &'a mut W {
        self.variant(C1S_A::Map1)
    }
    #[doc = "Mapping 2"]
    #[inline(always)]
    pub fn map2(self) -> &'a mut W {
        self.variant(C1S_A::Map2)
    }
    #[doc = "Mapping 3"]
    #[inline(always)]
    pub fn map3(self) -> &'a mut W {
        self.variant(C1S_A::Map3)
    }
    #[doc = "Mapping 4"]
    #[inline(always)]
    pub fn map4(self) -> &'a mut W {
        self.variant(C1S_A::Map4)
    }
    #[doc = "Mapping 5"]
    #[inline(always)]
    pub fn map5(self) -> &'a mut W {
        self.variant(C1S_A::Map5)
    }
    #[doc = "Mapping 6"]
    #[inline(always)]
    pub fn map6(self) -> &'a mut W {
        self.variant(C1S_A::Map6)
    }
    #[doc = "Mapping 7"]
    #[inline(always)]
    pub fn map7(self) -> &'a mut W {
        self.variant(C1S_A::Map7)
    }
    #[doc = "Mapping 8"]
    #[inline(always)]
    pub fn map8(self) -> &'a mut W {
        self.variant(C1S_A::Map8)
    }
    #[doc = "Mapping 9"]
    #[inline(always)]
    pub fn map9(self) -> &'a mut W {
        self.variant(C1S_A::Map9)
    }
    #[doc = "Mapping 10"]
    #[inline(always)]
    pub fn map10(self) -> &'a mut W {
        self.variant(C1S_A::Map10)
    }
    #[doc = "Mapping 11"]
    #[inline(always)]
    pub fn map11(self) -> &'a mut W {
        self.variant(C1S_A::Map11)
    }
    #[doc = "Mapping 12"]
    #[inline(always)]
    pub fn map12(self) -> &'a mut W {
        self.variant(C1S_A::Map12)
    }
    #[doc = "Mapping 13"]
    #[inline(always)]
    pub fn map13(self) -> &'a mut W {
        self.variant(C1S_A::Map13)
    }
    #[doc = "Mapping 14"]
    #[inline(always)]
    pub fn map14(self) -> &'a mut W {
        self.variant(C1S_A::Map14)
    }
    #[doc = "Mapping 15"]
    #[inline(always)]
    pub fn map15(self) -> &'a mut W {
        self.variant(C1S_A::Map15)
    }
}
impl R {
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline(always)]
    pub fn c7s(&mut self) -> C7S_W<24> {
        C7S_W::new(self)
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline(always)]
    pub fn c6s(&mut self) -> C6S_W<20> {
        C6S_W::new(self)
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline(always)]
    pub fn c5s(&mut self) -> C5S_W<16> {
        C5S_W::new(self)
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline(always)]
    pub fn c4s(&mut self) -> C4S_W<12> {
        C4S_W::new(self)
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline(always)]
    pub fn c3s(&mut self) -> C3S_W<8> {
        C3S_W::new(self)
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline(always)]
    pub fn c2s(&mut self) -> C2S_W<4> {
        C2S_W::new(self)
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline(always)]
    pub fn c1s(&mut self) -> C1S_W<0> {
        C1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cselr](index.html) module"]
pub struct CSELR_SPEC;
impl crate::RegisterSpec for CSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cselr::R](R) reader structure"]
impl crate::Readable for CSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cselr::W](W) writer structure"]
impl crate::Writable for CSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSELR to value 0"]
impl crate::Resettable for CSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PREDIV division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREDIV_A {
    #[doc = "0: PREDIV input clock not divided"]
    Div1 = 0,
    #[doc = "1: PREDIV input clock divided by 2"]
    Div2 = 1,
    #[doc = "2: PREDIV input clock divided by 3"]
    Div3 = 2,
    #[doc = "3: PREDIV input clock divided by 4"]
    Div4 = 3,
    #[doc = "4: PREDIV input clock divided by 5"]
    Div5 = 4,
    #[doc = "5: PREDIV input clock divided by 6"]
    Div6 = 5,
    #[doc = "6: PREDIV input clock divided by 7"]
    Div7 = 6,
    #[doc = "7: PREDIV input clock divided by 8"]
    Div8 = 7,
    #[doc = "8: PREDIV input clock divided by 9"]
    Div9 = 8,
    #[doc = "9: PREDIV input clock divided by 10"]
    Div10 = 9,
    #[doc = "10: PREDIV input clock divided by 11"]
    Div11 = 10,
    #[doc = "11: PREDIV input clock divided by 12"]
    Div12 = 11,
    #[doc = "12: PREDIV input clock divided by 13"]
    Div13 = 12,
    #[doc = "13: PREDIV input clock divided by 14"]
    Div14 = 13,
    #[doc = "14: PREDIV input clock divided by 15"]
    Div15 = 14,
    #[doc = "15: PREDIV input clock divided by 16"]
    Div16 = 15,
}
impl From<PREDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PREDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PREDIV` reader - PREDIV division factor"]
pub type PREDIV_R = crate::FieldReader<u8, PREDIV_A>;
impl PREDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREDIV_A {
        match self.bits {
            0 => PREDIV_A::Div1,
            1 => PREDIV_A::Div2,
            2 => PREDIV_A::Div3,
            3 => PREDIV_A::Div4,
            4 => PREDIV_A::Div5,
            5 => PREDIV_A::Div6,
            6 => PREDIV_A::Div7,
            7 => PREDIV_A::Div8,
            8 => PREDIV_A::Div9,
            9 => PREDIV_A::Div10,
            10 => PREDIV_A::Div11,
            11 => PREDIV_A::Div12,
            12 => PREDIV_A::Div13,
            13 => PREDIV_A::Div14,
            14 => PREDIV_A::Div15,
            15 => PREDIV_A::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PREDIV_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PREDIV_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PREDIV_A::Div3
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PREDIV_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div5`"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PREDIV_A::Div5
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PREDIV_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div7`"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PREDIV_A::Div7
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PREDIV_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div9`"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PREDIV_A::Div9
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PREDIV_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div11`"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PREDIV_A::Div11
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PREDIV_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div13`"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PREDIV_A::Div13
    }
    #[doc = "Checks if the value of the field is `Div14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PREDIV_A::Div14
    }
    #[doc = "Checks if the value of the field is `Div15`"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PREDIV_A::Div15
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PREDIV_A::Div16
    }
}
#[doc = "Field `PREDIV` writer - PREDIV division factor"]
pub type PREDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, PREDIV_A, 4, O>;
impl<'a, const O: u8> PREDIV_W<'a, O> {
    #[doc = "PREDIV input clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PREDIV_A::Div1)
    }
    #[doc = "PREDIV input clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PREDIV_A::Div2)
    }
    #[doc = "PREDIV input clock divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(PREDIV_A::Div3)
    }
    #[doc = "PREDIV input clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PREDIV_A::Div4)
    }
    #[doc = "PREDIV input clock divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(PREDIV_A::Div5)
    }
    #[doc = "PREDIV input clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PREDIV_A::Div6)
    }
    #[doc = "PREDIV input clock divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut W {
        self.variant(PREDIV_A::Div7)
    }
    #[doc = "PREDIV input clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PREDIV_A::Div8)
    }
    #[doc = "PREDIV input clock divided by 9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut W {
        self.variant(PREDIV_A::Div9)
    }
    #[doc = "PREDIV input clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PREDIV_A::Div10)
    }
    #[doc = "PREDIV input clock divided by 11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut W {
        self.variant(PREDIV_A::Div11)
    }
    #[doc = "PREDIV input clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PREDIV_A::Div12)
    }
    #[doc = "PREDIV input clock divided by 13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut W {
        self.variant(PREDIV_A::Div13)
    }
    #[doc = "PREDIV input clock divided by 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(PREDIV_A::Div14)
    }
    #[doc = "PREDIV input clock divided by 15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut W {
        self.variant(PREDIV_A::Div15)
    }
    #[doc = "PREDIV input clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PREDIV_A::Div16)
    }
}
#[doc = "ADC1 and ADC2 prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12PRES_A {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "16: PLL clock not divided"]
    Div1 = 16,
    #[doc = "17: PLL clock divided by 2"]
    Div2 = 17,
    #[doc = "18: PLL clock divided by 4"]
    Div4 = 18,
    #[doc = "19: PLL clock divided by 6"]
    Div6 = 19,
    #[doc = "20: PLL clock divided by 8"]
    Div8 = 20,
    #[doc = "21: PLL clock divided by 10"]
    Div10 = 21,
    #[doc = "22: PLL clock divided by 12"]
    Div12 = 22,
    #[doc = "23: PLL clock divided by 16"]
    Div16 = 23,
    #[doc = "24: PLL clock divided by 32"]
    Div32 = 24,
    #[doc = "25: PLL clock divided by 64"]
    Div64 = 25,
    #[doc = "26: PLL clock divided by 128"]
    Div128 = 26,
    #[doc = "27: PLL clock divided by 256"]
    Div256 = 27,
}
impl From<ADC12PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12PRES` reader - ADC1 and ADC2 prescaler"]
pub type ADC12PRES_R = crate::FieldReader<u8, ADC12PRES_A>;
impl ADC12PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC12PRES_A> {
        match self.bits {
            0 => Some(ADC12PRES_A::NoClock),
            16 => Some(ADC12PRES_A::Div1),
            17 => Some(ADC12PRES_A::Div2),
            18 => Some(ADC12PRES_A::Div4),
            19 => Some(ADC12PRES_A::Div6),
            20 => Some(ADC12PRES_A::Div8),
            21 => Some(ADC12PRES_A::Div10),
            22 => Some(ADC12PRES_A::Div12),
            23 => Some(ADC12PRES_A::Div16),
            24 => Some(ADC12PRES_A::Div32),
            25 => Some(ADC12PRES_A::Div64),
            26 => Some(ADC12PRES_A::Div128),
            27 => Some(ADC12PRES_A::Div256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NoClock`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == ADC12PRES_A::NoClock
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ADC12PRES_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADC12PRES_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADC12PRES_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADC12PRES_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADC12PRES_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == ADC12PRES_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == ADC12PRES_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == ADC12PRES_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == ADC12PRES_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == ADC12PRES_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == ADC12PRES_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == ADC12PRES_A::Div256
    }
}
#[doc = "Field `ADC12PRES` writer - ADC1 and ADC2 prescaler"]
pub type ADC12PRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, ADC12PRES_A, 5, O>;
impl<'a, const O: u8> ADC12PRES_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(ADC12PRES_A::NoClock)
    }
    #[doc = "PLL clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div1)
    }
    #[doc = "PLL clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div2)
    }
    #[doc = "PLL clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div4)
    }
    #[doc = "PLL clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div6)
    }
    #[doc = "PLL clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div8)
    }
    #[doc = "PLL clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div10)
    }
    #[doc = "PLL clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div12)
    }
    #[doc = "PLL clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div16)
    }
    #[doc = "PLL clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div32)
    }
    #[doc = "PLL clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div64)
    }
    #[doc = "PLL clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div128)
    }
    #[doc = "PLL clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(ADC12PRES_A::Div256)
    }
}
#[doc = "ADC3 and ADC4 prescaler"]
pub use ADC12PRES_A as ADC34PRES_A;
#[doc = "Field `ADC34PRES` reader - ADC3 and ADC4 prescaler"]
pub use ADC12PRES_R as ADC34PRES_R;
#[doc = "Field `ADC34PRES` writer - ADC3 and ADC4 prescaler"]
pub use ADC12PRES_W as ADC34PRES_W;
impl R {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    pub fn adc12pres(&self) -> ADC12PRES_R {
        ADC12PRES_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - ADC3 and ADC4 prescaler"]
    #[inline(always)]
    pub fn adc34pres(&self) -> ADC34PRES_R {
        ADC34PRES_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PREDIV division factor"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PREDIV_W<0> {
        PREDIV_W::new(self)
    }
    #[doc = "Bits 4:8 - ADC1 and ADC2 prescaler"]
    #[inline(always)]
    pub fn adc12pres(&mut self) -> ADC12PRES_W<4> {
        ADC12PRES_W::new(self)
    }
    #[doc = "Bits 9:13 - ADC3 and ADC4 prescaler"]
    #[inline(always)]
    pub fn adc34pres(&mut self) -> ADC34PRES_W<9> {
        ADC34PRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

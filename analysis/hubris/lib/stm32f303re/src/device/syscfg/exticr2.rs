#[doc = "Register `EXTICR2` reader"]
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR2` writer"]
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EXTI 7 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI7_A {
    #[doc = "0: Select PA7 as the source input for the EXTI7 external interrupt"]
    Pa7 = 0,
    #[doc = "1: Select PB7 as the source input for the EXTI7 external interrupt"]
    Pb7 = 1,
    #[doc = "2: Select PC7 as the source input for the EXTI7 external interrupt"]
    Pc7 = 2,
    #[doc = "3: Select PD7 as the source input for the EXTI7 external interrupt"]
    Pd7 = 3,
    #[doc = "4: Select PE7 as the source input for the EXTI7 external interrupt"]
    Pe7 = 4,
    #[doc = "5: Select PF7 as the source input for the EXTI7 external interrupt"]
    Pf7 = 5,
    #[doc = "6: Select PG7 as the source input for the EXTI7 external interrupt"]
    Pg7 = 6,
}
impl From<EXTI7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub type EXTI7_R = crate::FieldReader<u8, EXTI7_A>;
impl EXTI7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI7_A> {
        match self.bits {
            0 => Some(EXTI7_A::Pa7),
            1 => Some(EXTI7_A::Pb7),
            2 => Some(EXTI7_A::Pc7),
            3 => Some(EXTI7_A::Pd7),
            4 => Some(EXTI7_A::Pe7),
            5 => Some(EXTI7_A::Pf7),
            6 => Some(EXTI7_A::Pg7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa7`"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == EXTI7_A::Pa7
    }
    #[doc = "Checks if the value of the field is `Pb7`"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == EXTI7_A::Pb7
    }
    #[doc = "Checks if the value of the field is `Pc7`"]
    #[inline(always)]
    pub fn is_pc7(&self) -> bool {
        *self == EXTI7_A::Pc7
    }
    #[doc = "Checks if the value of the field is `Pd7`"]
    #[inline(always)]
    pub fn is_pd7(&self) -> bool {
        *self == EXTI7_A::Pd7
    }
    #[doc = "Checks if the value of the field is `Pe7`"]
    #[inline(always)]
    pub fn is_pe7(&self) -> bool {
        *self == EXTI7_A::Pe7
    }
    #[doc = "Checks if the value of the field is `Pf7`"]
    #[inline(always)]
    pub fn is_pf7(&self) -> bool {
        *self == EXTI7_A::Pf7
    }
    #[doc = "Checks if the value of the field is `Pg7`"]
    #[inline(always)]
    pub fn is_pg7(&self) -> bool {
        *self == EXTI7_A::Pg7
    }
}
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub type EXTI7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI7_A, 4, O>;
impl<'a, const O: u8> EXTI7_W<'a, O> {
    #[doc = "Select PA7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pa7)
    }
    #[doc = "Select PB7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pb7)
    }
    #[doc = "Select PC7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pc7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pc7)
    }
    #[doc = "Select PD7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pd7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pd7)
    }
    #[doc = "Select PE7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pe7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pe7)
    }
    #[doc = "Select PF7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pf7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pf7)
    }
    #[doc = "Select PG7 as the source input for the EXTI7 external interrupt"]
    #[inline(always)]
    pub fn pg7(self) -> &'a mut W {
        self.variant(EXTI7_A::Pg7)
    }
}
#[doc = "EXTI 6 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI6_A {
    #[doc = "0: Select PA6 as the source input for the EXTI6 external interrupt"]
    Pa6 = 0,
    #[doc = "1: Select PB6 as the source input for the EXTI6 external interrupt"]
    Pb6 = 1,
    #[doc = "2: Select PC6 as the source input for the EXTI6 external interrupt"]
    Pc6 = 2,
    #[doc = "3: Select PD6 as the source input for the EXTI6 external interrupt"]
    Pd6 = 3,
    #[doc = "4: Select PE6 as the source input for the EXTI6 external interrupt"]
    Pe6 = 4,
    #[doc = "5: Select PF6 as the source input for the EXTI6 external interrupt"]
    Pf6 = 5,
    #[doc = "6: Select PG6 as the source input for the EXTI6 external interrupt"]
    Pg6 = 6,
}
impl From<EXTI6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub type EXTI6_R = crate::FieldReader<u8, EXTI6_A>;
impl EXTI6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI6_A> {
        match self.bits {
            0 => Some(EXTI6_A::Pa6),
            1 => Some(EXTI6_A::Pb6),
            2 => Some(EXTI6_A::Pc6),
            3 => Some(EXTI6_A::Pd6),
            4 => Some(EXTI6_A::Pe6),
            5 => Some(EXTI6_A::Pf6),
            6 => Some(EXTI6_A::Pg6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa6`"]
    #[inline(always)]
    pub fn is_pa6(&self) -> bool {
        *self == EXTI6_A::Pa6
    }
    #[doc = "Checks if the value of the field is `Pb6`"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == EXTI6_A::Pb6
    }
    #[doc = "Checks if the value of the field is `Pc6`"]
    #[inline(always)]
    pub fn is_pc6(&self) -> bool {
        *self == EXTI6_A::Pc6
    }
    #[doc = "Checks if the value of the field is `Pd6`"]
    #[inline(always)]
    pub fn is_pd6(&self) -> bool {
        *self == EXTI6_A::Pd6
    }
    #[doc = "Checks if the value of the field is `Pe6`"]
    #[inline(always)]
    pub fn is_pe6(&self) -> bool {
        *self == EXTI6_A::Pe6
    }
    #[doc = "Checks if the value of the field is `Pf6`"]
    #[inline(always)]
    pub fn is_pf6(&self) -> bool {
        *self == EXTI6_A::Pf6
    }
    #[doc = "Checks if the value of the field is `Pg6`"]
    #[inline(always)]
    pub fn is_pg6(&self) -> bool {
        *self == EXTI6_A::Pg6
    }
}
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub type EXTI6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI6_A, 4, O>;
impl<'a, const O: u8> EXTI6_W<'a, O> {
    #[doc = "Select PA6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pa6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pa6)
    }
    #[doc = "Select PB6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pb6)
    }
    #[doc = "Select PC6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pc6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pc6)
    }
    #[doc = "Select PD6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pd6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pd6)
    }
    #[doc = "Select PE6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pe6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pe6)
    }
    #[doc = "Select PF6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pf6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pf6)
    }
    #[doc = "Select PG6 as the source input for the EXTI6 external interrupt"]
    #[inline(always)]
    pub fn pg6(self) -> &'a mut W {
        self.variant(EXTI6_A::Pg6)
    }
}
#[doc = "EXTI 5 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI5_A {
    #[doc = "0: Select PA5 as the source input for the EXTI5 external interrupt"]
    Pa5 = 0,
    #[doc = "1: Select PB5 as the source input for the EXTI5 external interrupt"]
    Pb5 = 1,
    #[doc = "2: Select PC5 as the source input for the EXTI5 external interrupt"]
    Pc5 = 2,
    #[doc = "3: Select PD5 as the source input for the EXTI5 external interrupt"]
    Pd5 = 3,
    #[doc = "4: Select PE5 as the source input for the EXTI5 external interrupt"]
    Pe5 = 4,
    #[doc = "5: Select PF5 as the source input for the EXTI5 external interrupt"]
    Pf5 = 5,
    #[doc = "6: Select PG5 as the source input for the EXTI5 external interrupt"]
    Pg5 = 6,
}
impl From<EXTI5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub type EXTI5_R = crate::FieldReader<u8, EXTI5_A>;
impl EXTI5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI5_A> {
        match self.bits {
            0 => Some(EXTI5_A::Pa5),
            1 => Some(EXTI5_A::Pb5),
            2 => Some(EXTI5_A::Pc5),
            3 => Some(EXTI5_A::Pd5),
            4 => Some(EXTI5_A::Pe5),
            5 => Some(EXTI5_A::Pf5),
            6 => Some(EXTI5_A::Pg5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa5`"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == EXTI5_A::Pa5
    }
    #[doc = "Checks if the value of the field is `Pb5`"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == EXTI5_A::Pb5
    }
    #[doc = "Checks if the value of the field is `Pc5`"]
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == EXTI5_A::Pc5
    }
    #[doc = "Checks if the value of the field is `Pd5`"]
    #[inline(always)]
    pub fn is_pd5(&self) -> bool {
        *self == EXTI5_A::Pd5
    }
    #[doc = "Checks if the value of the field is `Pe5`"]
    #[inline(always)]
    pub fn is_pe5(&self) -> bool {
        *self == EXTI5_A::Pe5
    }
    #[doc = "Checks if the value of the field is `Pf5`"]
    #[inline(always)]
    pub fn is_pf5(&self) -> bool {
        *self == EXTI5_A::Pf5
    }
    #[doc = "Checks if the value of the field is `Pg5`"]
    #[inline(always)]
    pub fn is_pg5(&self) -> bool {
        *self == EXTI5_A::Pg5
    }
}
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub type EXTI5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI5_A, 4, O>;
impl<'a, const O: u8> EXTI5_W<'a, O> {
    #[doc = "Select PA5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pa5)
    }
    #[doc = "Select PB5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pb5)
    }
    #[doc = "Select PC5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pc5)
    }
    #[doc = "Select PD5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pd5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pd5)
    }
    #[doc = "Select PE5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pe5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pe5)
    }
    #[doc = "Select PF5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pf5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pf5)
    }
    #[doc = "Select PG5 as the source input for the EXTI5 external interrupt"]
    #[inline(always)]
    pub fn pg5(self) -> &'a mut W {
        self.variant(EXTI5_A::Pg5)
    }
}
#[doc = "EXTI 4 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI4_A {
    #[doc = "0: Select PA4 as the source input for the EXTI4 external interrupt"]
    Pa4 = 0,
    #[doc = "1: Select PB4 as the source input for the EXTI4 external interrupt"]
    Pb4 = 1,
    #[doc = "2: Select PC4 as the source input for the EXTI4 external interrupt"]
    Pc4 = 2,
    #[doc = "3: Select PD4 as the source input for the EXTI4 external interrupt"]
    Pd4 = 3,
    #[doc = "4: Select PE4 as the source input for the EXTI4 external interrupt"]
    Pe4 = 4,
    #[doc = "5: Select PF4 as the source input for the EXTI4 external interrupt"]
    Pf4 = 5,
    #[doc = "6: Select PG4 as the source input for the EXTI4 external interrupt"]
    Pg4 = 6,
}
impl From<EXTI4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub type EXTI4_R = crate::FieldReader<u8, EXTI4_A>;
impl EXTI4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI4_A> {
        match self.bits {
            0 => Some(EXTI4_A::Pa4),
            1 => Some(EXTI4_A::Pb4),
            2 => Some(EXTI4_A::Pc4),
            3 => Some(EXTI4_A::Pd4),
            4 => Some(EXTI4_A::Pe4),
            5 => Some(EXTI4_A::Pf4),
            6 => Some(EXTI4_A::Pg4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa4`"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == EXTI4_A::Pa4
    }
    #[doc = "Checks if the value of the field is `Pb4`"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == EXTI4_A::Pb4
    }
    #[doc = "Checks if the value of the field is `Pc4`"]
    #[inline(always)]
    pub fn is_pc4(&self) -> bool {
        *self == EXTI4_A::Pc4
    }
    #[doc = "Checks if the value of the field is `Pd4`"]
    #[inline(always)]
    pub fn is_pd4(&self) -> bool {
        *self == EXTI4_A::Pd4
    }
    #[doc = "Checks if the value of the field is `Pe4`"]
    #[inline(always)]
    pub fn is_pe4(&self) -> bool {
        *self == EXTI4_A::Pe4
    }
    #[doc = "Checks if the value of the field is `Pf4`"]
    #[inline(always)]
    pub fn is_pf4(&self) -> bool {
        *self == EXTI4_A::Pf4
    }
    #[doc = "Checks if the value of the field is `Pg4`"]
    #[inline(always)]
    pub fn is_pg4(&self) -> bool {
        *self == EXTI4_A::Pg4
    }
}
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub type EXTI4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, EXTI4_A, 4, O>;
impl<'a, const O: u8> EXTI4_W<'a, O> {
    #[doc = "Select PA4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pa4)
    }
    #[doc = "Select PB4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pb4)
    }
    #[doc = "Select PC4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pc4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pc4)
    }
    #[doc = "Select PD4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pd4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pd4)
    }
    #[doc = "Select PE4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pe4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pe4)
    }
    #[doc = "Select PF4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pf4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pf4)
    }
    #[doc = "Select PG4 as the source input for the EXTI4 external interrupt"]
    #[inline(always)]
    pub fn pg4(self) -> &'a mut W {
        self.variant(EXTI4_A::Pg4)
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W<12> {
        EXTI7_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W<8> {
        EXTI6_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W<4> {
        EXTI5_W::new(self)
    }
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W<0> {
        EXTI4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](index.html) module"]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr2::R](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr2::W](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

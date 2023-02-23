#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DAC Channel 1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Normal mode - DAC channelx is connected to external pin with Buffer enabled"]
    NormalPinBuffer = 0,
    #[doc = "1: Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    NormalPinChipBuffer = 1,
    #[doc = "2: Normal mode - DAC channelx is connected to external pin with Buffer disabled"]
    NormalPinNoBuffer = 2,
    #[doc = "3: Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    NormalChipNoBuffer = 3,
    #[doc = "4: S&H mode - DAC channelx is connected to external pin with Buffer enabled"]
    ShpinBuffer = 4,
    #[doc = "5: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    ShpinChipBuffer = 5,
    #[doc = "6: S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled"]
    ShpinNoBuffer = 6,
    #[doc = "7: S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    ShchipNoBuffer = 7,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE1` reader - DAC Channel 1 mode"]
pub type MODE1_R = crate::FieldReader<u8, MODE1_A>;
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::NormalPinBuffer,
            1 => MODE1_A::NormalPinChipBuffer,
            2 => MODE1_A::NormalPinNoBuffer,
            3 => MODE1_A::NormalChipNoBuffer,
            4 => MODE1_A::ShpinBuffer,
            5 => MODE1_A::ShpinChipBuffer,
            6 => MODE1_A::ShpinNoBuffer,
            7 => MODE1_A::ShchipNoBuffer,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NormalPinBuffer`"]
    #[inline(always)]
    pub fn is_normal_pin_buffer(&self) -> bool {
        *self == MODE1_A::NormalPinBuffer
    }
    #[doc = "Checks if the value of the field is `NormalPinChipBuffer`"]
    #[inline(always)]
    pub fn is_normal_pin_chip_buffer(&self) -> bool {
        *self == MODE1_A::NormalPinChipBuffer
    }
    #[doc = "Checks if the value of the field is `NormalPinNoBuffer`"]
    #[inline(always)]
    pub fn is_normal_pin_no_buffer(&self) -> bool {
        *self == MODE1_A::NormalPinNoBuffer
    }
    #[doc = "Checks if the value of the field is `NormalChipNoBuffer`"]
    #[inline(always)]
    pub fn is_normal_chip_no_buffer(&self) -> bool {
        *self == MODE1_A::NormalChipNoBuffer
    }
    #[doc = "Checks if the value of the field is `ShpinBuffer`"]
    #[inline(always)]
    pub fn is_shpin_buffer(&self) -> bool {
        *self == MODE1_A::ShpinBuffer
    }
    #[doc = "Checks if the value of the field is `ShpinChipBuffer`"]
    #[inline(always)]
    pub fn is_shpin_chip_buffer(&self) -> bool {
        *self == MODE1_A::ShpinChipBuffer
    }
    #[doc = "Checks if the value of the field is `ShpinNoBuffer`"]
    #[inline(always)]
    pub fn is_shpin_no_buffer(&self) -> bool {
        *self == MODE1_A::ShpinNoBuffer
    }
    #[doc = "Checks if the value of the field is `ShchipNoBuffer`"]
    #[inline(always)]
    pub fn is_shchip_no_buffer(&self) -> bool {
        *self == MODE1_A::ShchipNoBuffer
    }
}
#[doc = "Field `MODE1` writer - DAC Channel 1 mode"]
pub type MODE1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, MODE1_A, 3, O>;
impl<'a, const O: u8> MODE1_W<'a, O> {
    #[doc = "Normal mode - DAC channelx is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn normal_pin_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalPinBuffer)
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn normal_pin_chip_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalPinChipBuffer)
    }
    #[doc = "Normal mode - DAC channelx is connected to external pin with Buffer disabled"]
    #[inline(always)]
    pub fn normal_pin_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalPinNoBuffer)
    }
    #[doc = "Normal mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn normal_chip_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::NormalChipNoBuffer)
    }
    #[doc = "S&H mode - DAC channelx is connected to external pin with Buffer enabled"]
    #[inline(always)]
    pub fn shpin_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShpinBuffer)
    }
    #[doc = "S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer enabled"]
    #[inline(always)]
    pub fn shpin_chip_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShpinChipBuffer)
    }
    #[doc = "S&H mode - DAC channelx is connected to external pin and to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn shpin_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShpinNoBuffer)
    }
    #[doc = "S&H mode - DAC channelx is connected to on chip peripherals with Buffer disabled"]
    #[inline(always)]
    pub fn shchip_no_buffer(self) -> &'a mut W {
        self.variant(MODE1_A::ShchipNoBuffer)
    }
}
#[doc = "DAC Channel 2 mode"]
pub use MODE1_A as MODE2_A;
#[doc = "Field `MODE2` reader - DAC Channel 2 mode"]
pub use MODE1_R as MODE2_R;
#[doc = "Field `MODE2` writer - DAC Channel 2 mode"]
pub use MODE1_W as MODE2_W;
impl R {
    #[doc = "Bits 0:2 - DAC Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DAC Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W<0> {
        MODE1_W::new(self)
    }
    #[doc = "Bits 16:18 - DAC Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W<16> {
        MODE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

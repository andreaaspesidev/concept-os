#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MainFlash = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SystemFlash = 1,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    Sram = 3,
    #[doc = "4: FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)"]
    Fmc = 4,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader<u8, MEM_MODE_A>;
impl MEM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEM_MODE_A> {
        match self.bits {
            0 => Some(MEM_MODE_A::MainFlash),
            1 => Some(MEM_MODE_A::SystemFlash),
            3 => Some(MEM_MODE_A::Sram),
            4 => Some(MEM_MODE_A::Fmc),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MainFlash`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE_A::MainFlash
    }
    #[doc = "Checks if the value of the field is `SystemFlash`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE_A::SystemFlash
    }
    #[doc = "Checks if the value of the field is `Sram`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE_A::Sram
    }
    #[doc = "Checks if the value of the field is `Fmc`"]
    #[inline(always)]
    pub fn is_fmc(&self) -> bool {
        *self == MEM_MODE_A::Fmc
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, MEM_MODE_A, 3, O>;
impl<'a, const O: u8> MEM_MODE_W<'a, O> {
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SystemFlash)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Sram)
    }
    #[doc = "FMC Bank (Only the first two banks) (Available on STM32F303xD/E only)"]
    #[inline(always)]
    pub fn fmc(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Fmc)
    }
}
#[doc = "USB interrupt remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_IT_RMP_A {
    #[doc = "0: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    NotRemapped = 0,
    #[doc = "1: USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    Remapped = 1,
}
impl From<USB_IT_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: USB_IT_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_IT_RMP` reader - USB interrupt remap"]
pub type USB_IT_RMP_R = crate::BitReader<USB_IT_RMP_A>;
impl USB_IT_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_IT_RMP_A {
        match self.bits {
            false => USB_IT_RMP_A::NotRemapped,
            true => USB_IT_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == USB_IT_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == USB_IT_RMP_A::Remapped
    }
}
#[doc = "Field `USB_IT_RMP` writer - USB interrupt remap"]
pub type USB_IT_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, USB_IT_RMP_A, O>;
impl<'a, const O: u8> USB_IT_RMP_W<'a, O> {
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 19, 20 and 42 respectively"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(USB_IT_RMP_A::NotRemapped)
    }
    #[doc = "USB_HP, USB_LP and USB_WAKEUP interrupts are mapped on interrupt lines 74, 75 and 76 respectively"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(USB_IT_RMP_A::Remapped)
    }
}
#[doc = "Timer 1 ITR3 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1_ITR3_RMP_A {
    #[doc = "0: TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    NotRemapped = 0,
    #[doc = "1: TIM1_ITR3 = TIM17_OC"]
    Remapped = 1,
}
impl From<TIM1_ITR3_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1_ITR3_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1_ITR3_RMP` reader - Timer 1 ITR3 selection"]
pub type TIM1_ITR3_RMP_R = crate::BitReader<TIM1_ITR3_RMP_A>;
impl TIM1_ITR3_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1_ITR3_RMP_A {
        match self.bits {
            false => TIM1_ITR3_RMP_A::NotRemapped,
            true => TIM1_ITR3_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM1_ITR3_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM1_ITR3_RMP_A::Remapped
    }
}
#[doc = "Field `TIM1_ITR3_RMP` writer - Timer 1 ITR3 selection"]
pub type TIM1_ITR3_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM1_ITR3_RMP_A, O>;
impl<'a, const O: u8> TIM1_ITR3_RMP_W<'a, O> {
    #[doc = "TIM1_ITR3 = TIM4_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM1_ITR3_RMP_A::NotRemapped)
    }
    #[doc = "TIM1_ITR3 = TIM17_OC"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM1_ITR3_RMP_A::Remapped)
    }
}
#[doc = "DAC trigger remap (when TSEL = 001)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_TRIG_RMP_A {
    #[doc = "0: DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    NotRemapped = 0,
    #[doc = "1: DAC trigger is TIM3_TRGO"]
    Remapped = 1,
}
impl From<DAC1_TRIG_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_TRIG_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1_TRIG_RMP` reader - DAC trigger remap (when TSEL = 001)"]
pub type DAC1_TRIG_RMP_R = crate::BitReader<DAC1_TRIG_RMP_A>;
impl DAC1_TRIG_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_TRIG_RMP_A {
        match self.bits {
            false => DAC1_TRIG_RMP_A::NotRemapped,
            true => DAC1_TRIG_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == DAC1_TRIG_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == DAC1_TRIG_RMP_A::Remapped
    }
}
#[doc = "Field `DAC1_TRIG_RMP` writer - DAC trigger remap (when TSEL = 001)"]
pub type DAC1_TRIG_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, DAC1_TRIG_RMP_A, O>;
impl<'a, const O: u8> DAC1_TRIG_RMP_W<'a, O> {
    #[doc = "DAC trigger is TIM8_TRGO in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(DAC1_TRIG_RMP_A::NotRemapped)
    }
    #[doc = "DAC trigger is TIM3_TRGO"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(DAC1_TRIG_RMP_A::Remapped)
    }
}
#[doc = "ADC24 DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC2_DMA_RMP_A {
    #[doc = "0: ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    NotRemapped = 0,
    #[doc = "1: ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    Remapped = 1,
}
impl From<ADC2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC2_DMA_RMP` reader - ADC24 DMA remapping bit"]
pub type ADC2_DMA_RMP_R = crate::BitReader<ADC2_DMA_RMP_A>;
impl ADC2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC2_DMA_RMP_A {
        match self.bits {
            false => ADC2_DMA_RMP_A::NotRemapped,
            true => ADC2_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == ADC2_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == ADC2_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `ADC2_DMA_RMP` writer - ADC24 DMA remapping bit"]
pub type ADC2_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, ADC2_DMA_RMP_A, O>;
impl<'a, const O: u8> ADC2_DMA_RMP_W<'a, O> {
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 1 and 2"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::NotRemapped)
    }
    #[doc = "ADC24 DMA requests mapped on DMA2 channels 3 and 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::Remapped)
    }
}
#[doc = "TIM16 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16_DMA_RMP_A {
    #[doc = "0: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    NotRemapped = 0,
    #[doc = "1: TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    Remapped = 1,
}
impl From<TIM16_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_R = crate::BitReader<TIM16_DMA_RMP_A>;
impl TIM16_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16_DMA_RMP_A {
        match self.bits {
            false => TIM16_DMA_RMP_A::NotRemapped,
            true => TIM16_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM16_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM16_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM16_DMA_RMP_W<'a, O> {
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 3"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM16_CH1 and TIM16_UP DMA requests mapped on DMA channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM16_DMA_RMP_A::Remapped)
    }
}
#[doc = "TIM17 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17_DMA_RMP_A {
    #[doc = "0: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    NotRemapped = 0,
    #[doc = "1: TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    Remapped = 1,
}
impl From<TIM17_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_R = crate::BitReader<TIM17_DMA_RMP_A>;
impl TIM17_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17_DMA_RMP_A {
        match self.bits {
            false => TIM17_DMA_RMP_A::NotRemapped,
            true => TIM17_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM17_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM17_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM17_DMA_RMP_W<'a, O> {
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 1"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM17_CH1 and TIM17_UP DMA requests mapped on DMA channel 2"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM17_DMA_RMP_A::Remapped)
    }
}
#[doc = "TIM6 and DAC1 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM6_DAC1_CH1_DMA_RMP_A {
    #[doc = "0: TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    NotRemapped = 0,
    #[doc = "1: TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    Remapped = 1,
}
impl From<TIM6_DAC1_CH1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM6_DAC1_CH1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM6_DAC1_CH1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_CH1_DMA_RMP_R = crate::BitReader<TIM6_DAC1_CH1_DMA_RMP_A>;
impl TIM6_DAC1_CH1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM6_DAC1_CH1_DMA_RMP_A {
        match self.bits {
            false => TIM6_DAC1_CH1_DMA_RMP_A::NotRemapped,
            true => TIM6_DAC1_CH1_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM6_DAC1_CH1_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM6_DAC1_CH1_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM6_DAC1_CH1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_CH1_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM6_DAC1_CH1_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM6_DAC1_CH1_DMA_RMP_W<'a, O> {
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA2 channel 3 in STM32F303xB/C and STM32F358xC"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM6_DAC1_CH1_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM6_UP and DAC_CH1 DMA requests mapped on DMA1 channel 3"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM6_DAC1_CH1_DMA_RMP_A::Remapped)
    }
}
#[doc = "TIM7 and DAC2 DMA request remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM7_DAC1_CH2_DMA_RMP_A {
    #[doc = "0: TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    NotRemapped = 0,
    #[doc = "1: TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    Remapped = 1,
}
impl From<TIM7_DAC1_CH2_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: TIM7_DAC1_CH2_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM7_DAC1_CH2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC1_CH2_DMA_RMP_R = crate::BitReader<TIM7_DAC1_CH2_DMA_RMP_A>;
impl TIM7_DAC1_CH2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM7_DAC1_CH2_DMA_RMP_A {
        match self.bits {
            false => TIM7_DAC1_CH2_DMA_RMP_A::NotRemapped,
            true => TIM7_DAC1_CH2_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == TIM7_DAC1_CH2_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == TIM7_DAC1_CH2_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `TIM7_DAC1_CH2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC1_CH2_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, TIM7_DAC1_CH2_DMA_RMP_A, O>;
impl<'a, const O: u8> TIM7_DAC1_CH2_DMA_RMP_W<'a, O> {
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA2 channel 4 in STM32F303xB/C and STM32F358xC devices"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(TIM7_DAC1_CH2_DMA_RMP_A::NotRemapped)
    }
    #[doc = "TIM7_UP and DAC_CH2 DMA requests mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(TIM7_DAC1_CH2_DMA_RMP_A::Remapped)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB6_FMP_A {
    #[doc = "0: PB6 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB6_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP_A>;
impl I2C_PB6_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB6_FMP_A {
        match self.bits {
            false => I2C_PB6_FMP_A::Standard,
            true => I2C_PB6_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB6_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB6_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB6_FMP_A, O>;
impl<'a, const O: u8> I2C_PB6_FMP_W<'a, O> {
    #[doc = "PB6 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB6 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB6_FMP_A::Fmp)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB7_FMP_A {
    #[doc = "0: PB7 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB7_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP_A>;
impl I2C_PB7_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB7_FMP_A {
        match self.bits {
            false => I2C_PB7_FMP_A::Standard,
            true => I2C_PB7_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB7_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB7_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB7_FMP_A, O>;
impl<'a, const O: u8> I2C_PB7_FMP_W<'a, O> {
    #[doc = "PB7 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB7 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB7_FMP_A::Fmp)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB8_FMP_A {
    #[doc = "0: PB8 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB8_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP_A>;
impl I2C_PB8_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB8_FMP_A {
        match self.bits {
            false => I2C_PB8_FMP_A::Standard,
            true => I2C_PB8_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB8_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB8_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB8_FMP_A, O>;
impl<'a, const O: u8> I2C_PB8_FMP_W<'a, O> {
    #[doc = "PB8 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB8 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB8_FMP_A::Fmp)
    }
}
#[doc = "Fast Mode Plus (FM+) driving capability activation bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_PB9_FMP_A {
    #[doc = "0: PB9 pin operate in standard mode"]
    Standard = 0,
    #[doc = "1: I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    Fmp = 1,
}
impl From<I2C_PB9_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP_A>;
impl I2C_PB9_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_PB9_FMP_A {
        match self.bits {
            false => I2C_PB9_FMP_A::Standard,
            true => I2C_PB9_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C_PB9_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C_PB9_FMP_A::Fmp
    }
}
#[doc = "Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C_PB9_FMP_A, O>;
impl<'a, const O: u8> I2C_PB9_FMP_W<'a, O> {
    #[doc = "PB9 pin operate in standard mode"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Standard)
    }
    #[doc = "I2C FM+ mode enabled on PB9 and the Speed control is bypassed"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C_PB9_FMP_A::Fmp)
    }
}
#[doc = "I2C1 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    Fmp = 1,
}
impl From<I2C1_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1_FMP` reader - I2C1 Fast Mode Plus"]
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP_A>;
impl I2C1_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_FMP_A {
        match self.bits {
            false => I2C1_FMP_A::Standard,
            true => I2C1_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C1_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C1_FMP_A::Fmp
    }
}
#[doc = "Field `I2C1_FMP` writer - I2C1 Fast Mode Plus"]
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C1_FMP_A, O>;
impl<'a, const O: u8> I2C1_FMP_W<'a, O> {
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C1 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C1_FMP_A::Fmp)
    }
}
#[doc = "I2C2 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    Fmp = 1,
}
impl From<I2C2_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2_FMP` reader - I2C2 Fast Mode Plus"]
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP_A>;
impl I2C2_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_FMP_A {
        match self.bits {
            false => I2C2_FMP_A::Standard,
            true => I2C2_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C2_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C2_FMP_A::Fmp
    }
}
#[doc = "Field `I2C2_FMP` writer - I2C2 Fast Mode Plus"]
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C2_FMP_A, O>;
impl<'a, const O: u8> I2C2_FMP_W<'a, O> {
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C2 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C2_FMP_A::Fmp)
    }
}
#[doc = "Encoder mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ENCODER_MODE_A {
    #[doc = "0: No redirection"]
    NoRedirection = 0,
    #[doc = "1: TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    MapTim2tim15 = 1,
    #[doc = "2: TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    MapTim3tim15 = 2,
    #[doc = "3: TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    MapTim4tim15 = 3,
}
impl From<ENCODER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ENCODER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub type ENCODER_MODE_R = crate::FieldReader<u8, ENCODER_MODE_A>;
impl ENCODER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENCODER_MODE_A {
        match self.bits {
            0 => ENCODER_MODE_A::NoRedirection,
            1 => ENCODER_MODE_A::MapTim2tim15,
            2 => ENCODER_MODE_A::MapTim3tim15,
            3 => ENCODER_MODE_A::MapTim4tim15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoRedirection`"]
    #[inline(always)]
    pub fn is_no_redirection(&self) -> bool {
        *self == ENCODER_MODE_A::NoRedirection
    }
    #[doc = "Checks if the value of the field is `MapTim2tim15`"]
    #[inline(always)]
    pub fn is_map_tim2tim15(&self) -> bool {
        *self == ENCODER_MODE_A::MapTim2tim15
    }
    #[doc = "Checks if the value of the field is `MapTim3tim15`"]
    #[inline(always)]
    pub fn is_map_tim3tim15(&self) -> bool {
        *self == ENCODER_MODE_A::MapTim3tim15
    }
    #[doc = "Checks if the value of the field is `MapTim4tim15`"]
    #[inline(always)]
    pub fn is_map_tim4tim15(&self) -> bool {
        *self == ENCODER_MODE_A::MapTim4tim15
    }
}
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub type ENCODER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, ENCODER_MODE_A, 2, O>;
impl<'a, const O: u8> ENCODER_MODE_W<'a, O> {
    #[doc = "No redirection"]
    #[inline(always)]
    pub fn no_redirection(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::NoRedirection)
    }
    #[doc = "TIM2 IC1 and TIM2 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn map_tim2tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MapTim2tim15)
    }
    #[doc = "TIM3 IC1 and TIM3 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively"]
    #[inline(always)]
    pub fn map_tim3tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MapTim3tim15)
    }
    #[doc = "TIM4 IC1 and TIM4 IC2 are connected to TIM15 IC1 and TIM15 IC2 respectively (STM32F303xB/C and STM32F358xC devices only)"]
    #[inline(always)]
    pub fn map_tim4tim15(self) -> &'a mut W {
        self.variant(ENCODER_MODE_A::MapTim4tim15)
    }
}
#[doc = "Inexact interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE5_A {
    #[doc = "0: Inexact interrupt disable"]
    Disabled = 0,
    #[doc = "1: Inexact interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE5_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE5` reader - Inexact interrupt enable"]
pub type FPU_IE5_R = crate::BitReader<FPU_IE5_A>;
impl FPU_IE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE5_A {
        match self.bits {
            false => FPU_IE5_A::Disabled,
            true => FPU_IE5_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE5_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE5_A::Enabled
    }
}
#[doc = "Field `FPU_IE5` writer - Inexact interrupt enable"]
pub type FPU_IE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE5_A, O>;
impl<'a, const O: u8> FPU_IE5_W<'a, O> {
    #[doc = "Inexact interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE5_A::Disabled)
    }
    #[doc = "Inexact interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE5_A::Enabled)
    }
}
#[doc = "Input denormal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE4_A {
    #[doc = "0: Input denormal interrupt disable"]
    Disabled = 0,
    #[doc = "1: Input denormal interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE4_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE4` reader - Input denormal interrupt enable"]
pub type FPU_IE4_R = crate::BitReader<FPU_IE4_A>;
impl FPU_IE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE4_A {
        match self.bits {
            false => FPU_IE4_A::Disabled,
            true => FPU_IE4_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE4_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE4_A::Enabled
    }
}
#[doc = "Field `FPU_IE4` writer - Input denormal interrupt enable"]
pub type FPU_IE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE4_A, O>;
impl<'a, const O: u8> FPU_IE4_W<'a, O> {
    #[doc = "Input denormal interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE4_A::Disabled)
    }
    #[doc = "Input denormal interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE4_A::Enabled)
    }
}
#[doc = "Overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE3_A {
    #[doc = "0: Overflow interrupt disable"]
    Disabled = 0,
    #[doc = "1: Overflow interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE3_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE3` reader - Overflow interrupt enable"]
pub type FPU_IE3_R = crate::BitReader<FPU_IE3_A>;
impl FPU_IE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE3_A {
        match self.bits {
            false => FPU_IE3_A::Disabled,
            true => FPU_IE3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE3_A::Enabled
    }
}
#[doc = "Field `FPU_IE3` writer - Overflow interrupt enable"]
pub type FPU_IE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE3_A, O>;
impl<'a, const O: u8> FPU_IE3_W<'a, O> {
    #[doc = "Overflow interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE3_A::Disabled)
    }
    #[doc = "Overflow interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE3_A::Enabled)
    }
}
#[doc = "Underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE2_A {
    #[doc = "0: Underflow interrupt disable"]
    Disabled = 0,
    #[doc = "1: Underflow interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE2_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE2` reader - Underflow interrupt enable"]
pub type FPU_IE2_R = crate::BitReader<FPU_IE2_A>;
impl FPU_IE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE2_A {
        match self.bits {
            false => FPU_IE2_A::Disabled,
            true => FPU_IE2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE2_A::Enabled
    }
}
#[doc = "Field `FPU_IE2` writer - Underflow interrupt enable"]
pub type FPU_IE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE2_A, O>;
impl<'a, const O: u8> FPU_IE2_W<'a, O> {
    #[doc = "Underflow interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE2_A::Disabled)
    }
    #[doc = "Underflow interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE2_A::Enabled)
    }
}
#[doc = "Devide-by-zero interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE1_A {
    #[doc = "0: Devide-by-zero interrupt disable"]
    Disabled = 0,
    #[doc = "1: Devide-by-zero interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE1_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE1` reader - Devide-by-zero interrupt enable"]
pub type FPU_IE1_R = crate::BitReader<FPU_IE1_A>;
impl FPU_IE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE1_A {
        match self.bits {
            false => FPU_IE1_A::Disabled,
            true => FPU_IE1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE1_A::Enabled
    }
}
#[doc = "Field `FPU_IE1` writer - Devide-by-zero interrupt enable"]
pub type FPU_IE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE1_A, O>;
impl<'a, const O: u8> FPU_IE1_W<'a, O> {
    #[doc = "Devide-by-zero interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE1_A::Disabled)
    }
    #[doc = "Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE1_A::Enabled)
    }
}
#[doc = "Invalid operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPU_IE0_A {
    #[doc = "0: Invalid operation interrupt disable"]
    Disabled = 0,
    #[doc = "1: Invalid operation interrupt enable"]
    Enabled = 1,
}
impl From<FPU_IE0_A> for bool {
    #[inline(always)]
    fn from(variant: FPU_IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU_IE0` reader - Invalid operation interrupt enable"]
pub type FPU_IE0_R = crate::BitReader<FPU_IE0_A>;
impl FPU_IE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPU_IE0_A {
        match self.bits {
            false => FPU_IE0_A::Disabled,
            true => FPU_IE0_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPU_IE0_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FPU_IE0_A::Enabled
    }
}
#[doc = "Field `FPU_IE0` writer - Invalid operation interrupt enable"]
pub type FPU_IE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, FPU_IE0_A, O>;
impl<'a, const O: u8> FPU_IE0_W<'a, O> {
    #[doc = "Invalid operation interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::Disabled)
    }
    #[doc = "Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FPU_IE0_A::Enabled)
    }
}
#[doc = "DAC2 channel1 DMA remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC2_CH1_DMA_RMP_A {
    #[doc = "0: Not remapped"]
    NotRemapped = 0,
    #[doc = "1: DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    Remapped = 1,
}
impl From<DAC2_CH1_DMA_RMP_A> for bool {
    #[inline(always)]
    fn from(variant: DAC2_CH1_DMA_RMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC2_CH1_DMA_RMP` reader - DAC2 channel1 DMA remap"]
pub type DAC2_CH1_DMA_RMP_R = crate::BitReader<DAC2_CH1_DMA_RMP_A>;
impl DAC2_CH1_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC2_CH1_DMA_RMP_A {
        match self.bits {
            false => DAC2_CH1_DMA_RMP_A::NotRemapped,
            true => DAC2_CH1_DMA_RMP_A::Remapped,
        }
    }
    #[doc = "Checks if the value of the field is `NotRemapped`"]
    #[inline(always)]
    pub fn is_not_remapped(&self) -> bool {
        *self == DAC2_CH1_DMA_RMP_A::NotRemapped
    }
    #[doc = "Checks if the value of the field is `Remapped`"]
    #[inline(always)]
    pub fn is_remapped(&self) -> bool {
        *self == DAC2_CH1_DMA_RMP_A::Remapped
    }
}
#[doc = "Field `DAC2_CH1_DMA_RMP` writer - DAC2 channel1 DMA remap"]
pub type DAC2_CH1_DMA_RMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR1_SPEC, DAC2_CH1_DMA_RMP_A, O>;
impl<'a, const O: u8> DAC2_CH1_DMA_RMP_W<'a, O> {
    #[doc = "Not remapped"]
    #[inline(always)]
    pub fn not_remapped(self) -> &'a mut W {
        self.variant(DAC2_CH1_DMA_RMP_A::NotRemapped)
    }
    #[doc = "DAC2_CH1 DMA requests mapped on DMA1 channel 5"]
    #[inline(always)]
    pub fn remapped(self) -> &'a mut W {
        self.variant(DAC2_CH1_DMA_RMP_A::Remapped)
    }
}
#[doc = "I2C3 Fast Mode Plus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_FMP_A {
    #[doc = "0: FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    Standard = 0,
    #[doc = "1: FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    Fmp = 1,
}
impl From<I2C3_FMP_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_FMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3_FMP` reader - I2C3 Fast Mode Plus"]
pub type I2C3_FMP_R = crate::BitReader<I2C3_FMP_A>;
impl I2C3_FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_FMP_A {
        match self.bits {
            false => I2C3_FMP_A::Standard,
            true => I2C3_FMP_A::Fmp,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == I2C3_FMP_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fmp`"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == I2C3_FMP_A::Fmp
    }
}
#[doc = "Field `I2C3_FMP` writer - I2C3 Fast Mode Plus"]
pub type I2C3_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, I2C3_FMP_A, O>;
impl<'a, const O: u8> I2C3_FMP_W<'a, O> {
    #[doc = "FM+ mode is controlled by I2C_Pxx_FMP bits only"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::Standard)
    }
    #[doc = "FM+ mode is enabled on all I2C3 pins selected through selection through IOPORT control registers AF selection bits"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut W {
        self.variant(I2C3_FMP_A::Fmp)
    }
}
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&self) -> USB_IT_RMP_R {
        USB_IT_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr3_rmp(&self) -> TIM1_ITR3_RMP_R {
        TIM1_ITR3_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac1_trig_rmp(&self) -> DAC1_TRIG_RMP_R {
        DAC1_TRIG_RMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_ch1_dma_rmp(&self) -> TIM6_DAC1_CH1_DMA_RMP_R {
        TIM6_DAC1_CH1_DMA_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac1_ch2_dma_rmp(&self) -> TIM7_DAC1_CH2_DMA_RMP_R {
        TIM7_DAC1_CH2_DMA_RMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&self) -> FPU_IE5_R {
        FPU_IE5_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&self) -> FPU_IE4_R {
        FPU_IE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&self) -> FPU_IE3_R {
        FPU_IE3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&self) -> FPU_IE2_R {
        FPU_IE2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&self) -> FPU_IE1_R {
        FPU_IE1_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&self) -> FPU_IE0_R {
        FPU_IE0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC2 channel1 DMA remap"]
    #[inline(always)]
    pub fn dac2_ch1_dma_rmp(&self) -> DAC2_CH1_DMA_RMP_R {
        DAC2_CH1_DMA_RMP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C3 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&mut self) -> USB_IT_RMP_W<5> {
        USB_IT_RMP_W::new(self)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr3_rmp(&mut self) -> TIM1_ITR3_RMP_W<6> {
        TIM1_ITR3_RMP_W::new(self)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac1_trig_rmp(&mut self) -> DAC1_TRIG_RMP_W<7> {
        DAC1_TRIG_RMP_W::new(self)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W<8> {
        ADC2_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W<11> {
        TIM16_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W<12> {
        TIM17_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_ch1_dma_rmp(&mut self) -> TIM6_DAC1_CH1_DMA_RMP_W<13> {
        TIM6_DAC1_CH1_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac1_ch2_dma_rmp(&mut self) -> TIM7_DAC1_CH2_DMA_RMP_W<14> {
        TIM7_DAC1_CH2_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<16> {
        I2C_PB6_FMP_W::new(self)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<17> {
        I2C_PB7_FMP_W::new(self)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<18> {
        I2C_PB8_FMP_W::new(self)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<19> {
        I2C_PB9_FMP_W::new(self)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<20> {
        I2C1_FMP_W::new(self)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<21> {
        I2C2_FMP_W::new(self)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W<22> {
        ENCODER_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Inexact interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie5(&mut self) -> FPU_IE5_W<31> {
        FPU_IE5_W::new(self)
    }
    #[doc = "Bit 30 - Input denormal interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie4(&mut self) -> FPU_IE4_W<30> {
        FPU_IE4_W::new(self)
    }
    #[doc = "Bit 29 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie3(&mut self) -> FPU_IE3_W<29> {
        FPU_IE3_W::new(self)
    }
    #[doc = "Bit 28 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie2(&mut self) -> FPU_IE2_W<28> {
        FPU_IE2_W::new(self)
    }
    #[doc = "Bit 27 - Devide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie1(&mut self) -> FPU_IE1_W<27> {
        FPU_IE1_W::new(self)
    }
    #[doc = "Bit 26 - Invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fpu_ie0(&mut self) -> FPU_IE0_W<26> {
        FPU_IE0_W::new(self)
    }
    #[doc = "Bit 15 - DAC2 channel1 DMA remap"]
    #[inline(always)]
    pub fn dac2_ch1_dma_rmp(&mut self) -> DAC2_CH1_DMA_RMP_W<15> {
        DAC2_CH1_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 24 - I2C3 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W<24> {
        I2C3_FMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

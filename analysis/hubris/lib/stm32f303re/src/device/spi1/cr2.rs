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
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    Enabled = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::Disabled,
            true => RXDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RXDMAEN_A, O>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Disabled)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::Enabled)
    }
}
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    Enabled = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::Disabled,
            true => TXDMAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TXDMAEN_A, O>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Disabled)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::Enabled)
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode"]
    Disabled = 0,
    #[doc = "1: SS output is enabled in master mode"]
    Enabled = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SSOE_R = crate::BitReader<SSOE_A>;
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::Disabled,
            true => SSOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::Enabled
    }
}
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SSOE_A, O>;
impl<'a, const O: u8> SSOE_W<'a, O> {
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::Disabled)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::Enabled)
    }
}
#[doc = "NSS pulse management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSP_A {
    #[doc = "0: No NSS pulse"]
    NoPulse = 0,
    #[doc = "1: NSS pulse generated"]
    PulseGenerated = 1,
}
impl From<NSSP_A> for bool {
    #[inline(always)]
    fn from(variant: NSSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSSP` reader - NSS pulse management"]
pub type NSSP_R = crate::BitReader<NSSP_A>;
impl NSSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSP_A {
        match self.bits {
            false => NSSP_A::NoPulse,
            true => NSSP_A::PulseGenerated,
        }
    }
    #[doc = "Checks if the value of the field is `NoPulse`"]
    #[inline(always)]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSP_A::NoPulse
    }
    #[doc = "Checks if the value of the field is `PulseGenerated`"]
    #[inline(always)]
    pub fn is_pulse_generated(&self) -> bool {
        *self == NSSP_A::PulseGenerated
    }
}
#[doc = "Field `NSSP` writer - NSS pulse management"]
pub type NSSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, NSSP_A, O>;
impl<'a, const O: u8> NSSP_W<'a, O> {
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn no_pulse(self) -> &'a mut W {
        self.variant(NSSP_A::NoPulse)
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn pulse_generated(self) -> &'a mut W {
        self.variant(NSSP_A::PulseGenerated)
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: SPI Motorola mode"]
    Motorola = 0,
    #[doc = "1: SPI TI mode"]
    Ti = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::BitReader<FRF_A>;
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::Motorola,
            true => FRF_A::Ti,
        }
    }
    #[doc = "Checks if the value of the field is `Motorola`"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF_A::Motorola
    }
    #[doc = "Checks if the value of the field is `Ti`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::Ti
    }
}
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, FRF_A, O>;
impl<'a, const O: u8> FRF_W<'a, O> {
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRF_A::Motorola)
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::Ti)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt masked"]
    Masked = 0,
    #[doc = "1: Error interrupt not masked"]
    NotMasked = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::Masked,
            true => ERRIE_A::NotMasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE_A::Masked
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE_A::NotMasked
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ERRIE_A, O>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::Masked)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NotMasked)
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    #[doc = "0: RXE interrupt masked"]
    Masked = 0,
    #[doc = "1: RXE interrupt not masked"]
    NotMasked = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::Masked,
            true => RXNEIE_A::NotMasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE_A::Masked
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE_A::NotMasked
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RXNEIE_A, O>;
impl<'a, const O: u8> RXNEIE_W<'a, O> {
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::Masked)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NotMasked)
    }
}
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    #[doc = "0: TXE interrupt masked"]
    Masked = 0,
    #[doc = "1: TXE interrupt not masked"]
    NotMasked = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::Masked,
            true => TXEIE_A::NotMasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE_A::Masked
    }
    #[doc = "Checks if the value of the field is `NotMasked`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE_A::NotMasked
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TXEIE_A, O>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::Masked)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NotMasked)
    }
}
#[doc = "Data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DS_A {
    #[doc = "3: 4-bit"]
    FourBit = 3,
    #[doc = "4: 5-bit"]
    FiveBit = 4,
    #[doc = "5: 6-bit"]
    SixBit = 5,
    #[doc = "6: 7-bit"]
    SevenBit = 6,
    #[doc = "7: 8-bit"]
    EightBit = 7,
    #[doc = "8: 9-bit"]
    NineBit = 8,
    #[doc = "9: 10-bit"]
    TenBit = 9,
    #[doc = "10: 11-bit"]
    ElevenBit = 10,
    #[doc = "11: 12-bit"]
    TwelveBit = 11,
    #[doc = "12: 13-bit"]
    ThirteenBit = 12,
    #[doc = "13: 14-bit"]
    FourteenBit = 13,
    #[doc = "14: 15-bit"]
    FifteenBit = 14,
    #[doc = "15: 16-bit"]
    SixteenBit = 15,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DS` reader - Data size"]
pub type DS_R = crate::FieldReader<u8, DS_A>;
impl DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DS_A> {
        match self.bits {
            3 => Some(DS_A::FourBit),
            4 => Some(DS_A::FiveBit),
            5 => Some(DS_A::SixBit),
            6 => Some(DS_A::SevenBit),
            7 => Some(DS_A::EightBit),
            8 => Some(DS_A::NineBit),
            9 => Some(DS_A::TenBit),
            10 => Some(DS_A::ElevenBit),
            11 => Some(DS_A::TwelveBit),
            12 => Some(DS_A::ThirteenBit),
            13 => Some(DS_A::FourteenBit),
            14 => Some(DS_A::FifteenBit),
            15 => Some(DS_A::SixteenBit),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FourBit`"]
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == DS_A::FourBit
    }
    #[doc = "Checks if the value of the field is `FiveBit`"]
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        *self == DS_A::FiveBit
    }
    #[doc = "Checks if the value of the field is `SixBit`"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == DS_A::SixBit
    }
    #[doc = "Checks if the value of the field is `SevenBit`"]
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        *self == DS_A::SevenBit
    }
    #[doc = "Checks if the value of the field is `EightBit`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == DS_A::EightBit
    }
    #[doc = "Checks if the value of the field is `NineBit`"]
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        *self == DS_A::NineBit
    }
    #[doc = "Checks if the value of the field is `TenBit`"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == DS_A::TenBit
    }
    #[doc = "Checks if the value of the field is `ElevenBit`"]
    #[inline(always)]
    pub fn is_eleven_bit(&self) -> bool {
        *self == DS_A::ElevenBit
    }
    #[doc = "Checks if the value of the field is `TwelveBit`"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == DS_A::TwelveBit
    }
    #[doc = "Checks if the value of the field is `ThirteenBit`"]
    #[inline(always)]
    pub fn is_thirteen_bit(&self) -> bool {
        *self == DS_A::ThirteenBit
    }
    #[doc = "Checks if the value of the field is `FourteenBit`"]
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == DS_A::FourteenBit
    }
    #[doc = "Checks if the value of the field is `FifteenBit`"]
    #[inline(always)]
    pub fn is_fifteen_bit(&self) -> bool {
        *self == DS_A::FifteenBit
    }
    #[doc = "Checks if the value of the field is `SixteenBit`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DS_A::SixteenBit
    }
}
#[doc = "Field `DS` writer - Data size"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, DS_A, 4, O>;
impl<'a, const O: u8> DS_W<'a, O> {
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(DS_A::FourBit)
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(DS_A::FiveBit)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(DS_A::SixBit)
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(DS_A::SevenBit)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(DS_A::EightBit)
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(DS_A::NineBit)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(DS_A::TenBit)
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn eleven_bit(self) -> &'a mut W {
        self.variant(DS_A::ElevenBit)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(DS_A::TwelveBit)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn thirteen_bit(self) -> &'a mut W {
        self.variant(DS_A::ThirteenBit)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FourteenBit)
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn fifteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FifteenBit)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DS_A::SixteenBit)
    }
}
#[doc = "FIFO reception threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRXTH_A {
    #[doc = "0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    Half = 0,
    #[doc = "1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    Quarter = 1,
}
impl From<FRXTH_A> for bool {
    #[inline(always)]
    fn from(variant: FRXTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRXTH` reader - FIFO reception threshold"]
pub type FRXTH_R = crate::BitReader<FRXTH_A>;
impl FRXTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRXTH_A {
        match self.bits {
            false => FRXTH_A::Half,
            true => FRXTH_A::Quarter,
        }
    }
    #[doc = "Checks if the value of the field is `Half`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRXTH_A::Half
    }
    #[doc = "Checks if the value of the field is `Quarter`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRXTH_A::Quarter
    }
}
#[doc = "Field `FRXTH` writer - FIFO reception threshold"]
pub type FRXTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, FRXTH_A, O>;
impl<'a, const O: u8> FRXTH_W<'a, O> {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FRXTH_A::Half)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FRXTH_A::Quarter)
    }
}
#[doc = "Last DMA transfer for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_RX_A {
    #[doc = "0: Number of data to transfer for receive is even"]
    Even = 0,
    #[doc = "1: Number of data to transfer for receive is odd"]
    Odd = 1,
}
impl From<LDMA_RX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception"]
pub type LDMA_RX_R = crate::BitReader<LDMA_RX_A>;
impl LDMA_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMA_RX_A {
        match self.bits {
            false => LDMA_RX_A::Even,
            true => LDMA_RX_A::Odd,
        }
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RX_A::Even
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RX_A::Odd
    }
}
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception"]
pub type LDMA_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LDMA_RX_A, O>;
impl<'a, const O: u8> LDMA_RX_W<'a, O> {
    #[doc = "Number of data to transfer for receive is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_RX_A::Even)
    }
    #[doc = "Number of data to transfer for receive is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_RX_A::Odd)
    }
}
#[doc = "Last DMA transfer for transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMA_TX_A {
    #[doc = "0: Number of data to transfer for transmit is even"]
    Even = 0,
    #[doc = "1: Number of data to transfer for transmit is odd"]
    Odd = 1,
}
impl From<LDMA_TX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission"]
pub type LDMA_TX_R = crate::BitReader<LDMA_TX_A>;
impl LDMA_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMA_TX_A {
        match self.bits {
            false => LDMA_TX_A::Even,
            true => LDMA_TX_A::Odd,
        }
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TX_A::Even
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TX_A::Odd
    }
}
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission"]
pub type LDMA_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, LDMA_TX_A, O>;
impl<'a, const O: u8> LDMA_TX_W<'a, O> {
    #[doc = "Number of data to transfer for transmit is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_TX_A::Even)
    }
    #[doc = "Number of data to transfer for transmit is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_TX_A::Odd)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<0> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<1> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<2> {
        SSOE_W::new(self)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W<3> {
        NSSP_W::new(self)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<6> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<8> {
        DS_W::new(self)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W<12> {
        FRXTH_W::new(self)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<13> {
        LDMA_RX_W::new(self)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<14> {
        LDMA_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
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

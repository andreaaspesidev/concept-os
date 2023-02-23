#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1_A {
    #[doc = "0: Use M0 to set the data bits"]
    M0 = 0,
    #[doc = "1: 1 start bit, 7 data bits, n stop bits"]
    Bit7 = 1,
}
impl From<M1_A> for bool {
    #[inline(always)]
    fn from(variant: M1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M1` reader - Word length"]
pub type M1_R = crate::BitReader<M1_A>;
impl M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1_A {
        match self.bits {
            false => M1_A::M0,
            true => M1_A::Bit7,
        }
    }
    #[doc = "Checks if the value of the field is `M0`"]
    #[inline(always)]
    pub fn is_m0(&self) -> bool {
        *self == M1_A::M0
    }
    #[doc = "Checks if the value of the field is `Bit7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == M1_A::Bit7
    }
}
#[doc = "Field `M1` writer - Word length"]
pub type M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, M1_A, O>;
impl<'a, const O: u8> M1_W<'a, O> {
    #[doc = "Use M0 to set the data bits"]
    #[inline(always)]
    pub fn m0(self) -> &'a mut W {
        self.variant(M1_A::M0)
    }
    #[doc = "1 start bit, 7 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(M1_A::Bit7)
    }
}
#[doc = "Character match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated when the CMF bit is set in the ISR register"]
    Enabled = 1,
}
impl From<CMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMIE` reader - Character match interrupt enable"]
pub type CMIE_R = crate::BitReader<CMIE_A>;
impl CMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMIE_A {
        match self.bits {
            false => CMIE_A::Disabled,
            true => CMIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMIE_A::Enabled
    }
}
#[doc = "Field `CMIE` writer - Character match interrupt enable"]
pub type CMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CMIE_A, O>;
impl<'a, const O: u8> CMIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMIE_A::Disabled)
    }
    #[doc = "Interrupt is generated when the CMF bit is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMIE_A::Enabled)
    }
}
#[doc = "Mute mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MME_A {
    #[doc = "0: Receiver in active mode permanently"]
    Disabled = 0,
    #[doc = "1: Receiver can switch between mute mode and active mode"]
    Enabled = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MME` reader - Mute mode enable"]
pub type MME_R = crate::BitReader<MME_A>;
impl MME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::Disabled,
            true => MME_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MME_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MME_A::Enabled
    }
}
#[doc = "Field `MME` writer - Mute mode enable"]
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MME_A, O>;
impl<'a, const O: u8> MME_W<'a, O> {
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MME_A::Disabled)
    }
    #[doc = "Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MME_A::Enabled)
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0_A {
    #[doc = "0: 1 start bit, 8 data bits, n stop bits"]
    Bit8 = 0,
    #[doc = "1: 1 start bit, 9 data bits, n stop bits"]
    Bit9 = 1,
}
impl From<M0_A> for bool {
    #[inline(always)]
    fn from(variant: M0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M0` reader - Word length"]
pub type M0_R = crate::BitReader<M0_A>;
impl M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0_A {
        match self.bits {
            false => M0_A::Bit8,
            true => M0_A::Bit9,
        }
    }
    #[doc = "Checks if the value of the field is `Bit8`"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == M0_A::Bit8
    }
    #[doc = "Checks if the value of the field is `Bit9`"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == M0_A::Bit9
    }
}
#[doc = "Field `M0` writer - Word length"]
pub type M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, M0_A, O>;
impl<'a, const O: u8> M0_W<'a, O> {
    #[doc = "1 start bit, 8 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(M0_A::Bit8)
    }
    #[doc = "1 start bit, 9 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(M0_A::Bit9)
    }
}
#[doc = "Receiver wakeup method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    #[doc = "0: Idle line"]
    Idle = 0,
    #[doc = "1: Address mask"]
    Address = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Receiver wakeup method"]
pub type WAKE_R = crate::BitReader<WAKE_A>;
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::Idle,
            true => WAKE_A::Address,
        }
    }
    #[doc = "Checks if the value of the field is `Idle`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE_A::Idle
    }
    #[doc = "Checks if the value of the field is `Address`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WAKE_A::Address
    }
}
#[doc = "Field `WAKE` writer - Receiver wakeup method"]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, WAKE_A, O>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKE_A::Idle)
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WAKE_A::Address)
    }
}
#[doc = "Parity control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCE_A {
    #[doc = "0: Parity control disabled"]
    Disabled = 0,
    #[doc = "1: Parity control enabled"]
    Enabled = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PCE_R = crate::BitReader<PCE_A>;
impl PCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::Disabled,
            true => PCE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE_A::Enabled
    }
}
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PCE_A, O>;
impl<'a, const O: u8> PCE_W<'a, O> {
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCE_A::Disabled)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCE_A::Enabled)
    }
}
#[doc = "Parity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Parity selection"]
pub type PS_R = crate::BitReader<PS_A>;
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::Even,
            true => PS_A::Odd,
        }
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS_A::Even
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS_A::Odd
    }
}
#[doc = "Field `PS` writer - Parity selection"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PS_A, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::Odd)
    }
}
#[doc = "PE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever PE=1 in the ISR register"]
    Enabled = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PEIE_R = crate::BitReader<PEIE_A>;
impl PEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::Disabled,
            true => PEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE_A::Enabled
    }
}
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, PEIE_A, O>;
impl<'a, const O: u8> PEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::Disabled)
    }
    #[doc = "Interrupt is generated whenever PE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIE_A::Enabled)
    }
}
#[doc = "interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever TXE=1 in the ISR register"]
    Enabled = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::Disabled,
            true => TXEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE_A::Enabled
    }
}
#[doc = "Field `TXEIE` writer - interrupt enable"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TXEIE_A, O>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIE_A::Disabled)
    }
    #[doc = "Interrupt is generated whenever TXE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIE_A::Enabled)
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever TC=1 in the ISR register"]
    Enabled = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::Disabled,
            true => TCIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::Disabled)
    }
    #[doc = "Interrupt is generated whenever TC=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::Enabled)
    }
}
#[doc = "RXNE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNEIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    Enabled = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::Disabled,
            true => RXNEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE_A::Enabled
    }
}
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RXNEIE_A, O>;
impl<'a, const O: u8> RXNEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::Disabled)
    }
    #[doc = "Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::Enabled)
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLEIE_A {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever IDLE=1 in the ISR register"]
    Enabled = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IDLEIE_R = crate::BitReader<IDLEIE_A>;
impl IDLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::Disabled,
            true => IDLEIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE_A::Enabled
    }
}
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, IDLEIE_A, O>;
impl<'a, const O: u8> IDLEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::Disabled)
    }
    #[doc = "Interrupt is generated whenever IDLE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::Enabled)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: Transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter is enabled"]
    Enabled = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<TE_A>;
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::Disabled,
            true => TE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::Enabled
    }
}
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::Disabled)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::Enabled)
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Receiver is disabled"]
    Disabled = 0,
    #[doc = "1: Receiver is enabled"]
    Enabled = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<RE_A>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::Disabled,
            true => RE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::Enabled
    }
}
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::Disabled)
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::Enabled)
    }
}
#[doc = "USART enable in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UESM_A {
    #[doc = "0: USART not able to wake up the MCU from Stop mode"]
    Disabled = 0,
    #[doc = "1: USART able to wake up the MCU from Stop mode"]
    Enabled = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UESM_R = crate::BitReader<UESM_A>;
impl UESM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::Disabled,
            true => UESM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UESM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UESM_A::Enabled
    }
}
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UESM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, UESM_A, O>;
impl<'a, const O: u8> UESM_W<'a, O> {
    #[doc = "USART not able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UESM_A::Disabled)
    }
    #[doc = "USART able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UESM_A::Enabled)
    }
}
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UE_A {
    #[doc = "0: UART is disabled"]
    Disabled = 0,
    #[doc = "1: UART is enabled"]
    Enabled = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - USART enable"]
pub type UE_R = crate::BitReader<UE_A>;
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::Disabled,
            true => UE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE_A::Enabled
    }
}
#[doc = "Field `UE` writer - USART enable"]
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, UE_A, O>;
impl<'a, const O: u8> UE_W<'a, O> {
    #[doc = "UART is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UE_A::Disabled)
    }
    #[doc = "UART is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UE_A::Enabled)
    }
}
#[doc = "Field `DEAT` reader - Driver Enable assertion time"]
pub type DEAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEAT` writer - Driver Enable assertion time"]
pub type DEAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DEDT` reader - Driver Enable de-assertion time"]
pub type DEDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEDT` writer - Driver Enable de-assertion time"]
pub type DEDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 28 - Word length"]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Driver Enable de-assertion time"]
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 28 - Word length"]
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<28> {
        M1_W::new(self)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<14> {
        CMIE_W::new(self)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<13> {
        MME_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<12> {
        M0_W::new(self)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<10> {
        PCE_W::new(self)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<9> {
        PS_W::new(self)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<8> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<5> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<1> {
        UESM_W::new(self)
    }
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<0> {
        UE_W::new(self)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<21> {
        DEAT_W::new(self)
    }
    #[doc = "Bits 16:20 - Driver Enable de-assertion time"]
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<16> {
        DEDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

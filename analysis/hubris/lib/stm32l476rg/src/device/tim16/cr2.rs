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
#[doc = "Output Idle state 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1N_A {
    #[doc = "0: OC1N=0 after a dead-time when MOE=0"]
    Low = 0,
    #[doc = "1: OC1N=1 after a dead-time when MOE=0"]
    High = 1,
}
impl From<OIS1N_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output Idle state 1"]
pub type OIS1N_R = crate::BitReader<OIS1N_A>;
impl OIS1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIS1N_A {
        match self.bits {
            false => OIS1N_A::Low,
            true => OIS1N_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OIS1N_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OIS1N_A::High
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1"]
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, OIS1N_A, O>;
impl<'a, const O: u8> OIS1N_W<'a, O> {
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OIS1N_A::Low)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OIS1N_A::High)
    }
}
#[doc = "Output Idle state 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1_A {
    #[doc = "0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    Low = 0,
    #[doc = "1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    High = 1,
}
impl From<OIS1_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output Idle state 1"]
pub type OIS1_R = crate::BitReader<OIS1_A>;
impl OIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OIS1_A {
        match self.bits {
            false => OIS1_A::Low,
            true => OIS1_A::High,
        }
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OIS1_A::Low
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OIS1_A::High
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1"]
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, OIS1_A, O>;
impl<'a, const O: u8> OIS1_W<'a, O> {
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OIS1_A::Low)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OIS1_A::High)
    }
}
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    OnCompare = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    OnUpdate = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<CCDS_A>;
impl CCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::OnCompare,
            true => CCDS_A::OnUpdate,
        }
    }
    #[doc = "Checks if the value of the field is `OnCompare`"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS_A::OnCompare
    }
    #[doc = "Checks if the value of the field is `OnUpdate`"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS_A::OnUpdate
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCDS_A, O>;
impl<'a, const O: u8> CCDS_W<'a, O> {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDS_A::OnCompare)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDS_A::OnUpdate)
    }
}
#[doc = "Capture/compare control update selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUS_A {
    #[doc = "0: Capture/compare are updated only by setting the COMG bit"]
    Default = 0,
    #[doc = "1: Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    WithRisingEdge = 1,
}
impl From<CCUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection"]
pub type CCUS_R = crate::BitReader<CCUS_A>;
impl CCUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCUS_A {
        match self.bits {
            false => CCUS_A::Default,
            true => CCUS_A::WithRisingEdge,
        }
    }
    #[doc = "Checks if the value of the field is `Default`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CCUS_A::Default
    }
    #[doc = "Checks if the value of the field is `WithRisingEdge`"]
    #[inline(always)]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == CCUS_A::WithRisingEdge
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection"]
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCUS_A, O>;
impl<'a, const O: u8> CCUS_W<'a, O> {
    #[doc = "Capture/compare are updated only by setting the COMG bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(CCUS_A::Default)
    }
    #[doc = "Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut W {
        self.variant(CCUS_A::WithRisingEdge)
    }
}
#[doc = "Capture/compare preloaded control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPC_A {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    NotPreloaded = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded"]
    Preloaded = 1,
}
impl From<CCPC_A> for bool {
    #[inline(always)]
    fn from(variant: CCPC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control"]
pub type CCPC_R = crate::BitReader<CCPC_A>;
impl CCPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPC_A {
        match self.bits {
            false => CCPC_A::NotPreloaded,
            true => CCPC_A::Preloaded,
        }
    }
    #[doc = "Checks if the value of the field is `NotPreloaded`"]
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPC_A::NotPreloaded
    }
    #[doc = "Checks if the value of the field is `Preloaded`"]
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPC_A::Preloaded
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control"]
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCPC_A, O>;
impl<'a, const O: u8> CCPC_W<'a, O> {
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCPC_A::NotPreloaded)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded"]
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCPC_A::Preloaded)
    }
}
impl R {
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
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

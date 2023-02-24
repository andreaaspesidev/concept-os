#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Volatile data execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDE_A {
    #[doc = "0: Volatile data segment cannot be executed if VDS = 0"]
    NotExecutable = 0,
    #[doc = "1: Volatile data segment is declared executable whatever VDS bit value"]
    Executable = 1,
}
impl From<VDE_A> for bool {
    #[inline(always)]
    fn from(variant: VDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDE` reader - Volatile data execution"]
pub type VDE_R = crate::BitReader<VDE_A>;
impl VDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDE_A {
        match self.bits {
            false => VDE_A::NotExecutable,
            true => VDE_A::Executable,
        }
    }
    #[doc = "Checks if the value of the field is `NotExecutable`"]
    #[inline(always)]
    pub fn is_not_executable(&self) -> bool {
        *self == VDE_A::NotExecutable
    }
    #[doc = "Checks if the value of the field is `Executable`"]
    #[inline(always)]
    pub fn is_executable(&self) -> bool {
        *self == VDE_A::Executable
    }
}
#[doc = "Volatile data execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDE_AW {
    #[doc = "0: Resets volatile data execution bit"]
    Reset = 0,
}
impl From<VDE_AW> for bool {
    #[inline(always)]
    fn from(variant: VDE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDE` writer - Volatile data execution"]
pub type VDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, VDE_AW, O>;
impl<'a, const O: u8> VDE_W<'a, O> {
    #[doc = "Resets volatile data execution bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VDE_AW::Reset)
    }
}
#[doc = "Volatile data shared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDS_A {
    #[doc = "0: Volatile data segment is not shared and cannot be hit by a non protected executable code when the Firewall is closed"]
    NotShared = 0,
    #[doc = "1: Volatile data segment is shared with non protected application code"]
    Shared = 1,
}
impl From<VDS_A> for bool {
    #[inline(always)]
    fn from(variant: VDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDS` reader - Volatile data shared"]
pub type VDS_R = crate::BitReader<VDS_A>;
impl VDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDS_A {
        match self.bits {
            false => VDS_A::NotShared,
            true => VDS_A::Shared,
        }
    }
    #[doc = "Checks if the value of the field is `NotShared`"]
    #[inline(always)]
    pub fn is_not_shared(&self) -> bool {
        *self == VDS_A::NotShared
    }
    #[doc = "Checks if the value of the field is `Shared`"]
    #[inline(always)]
    pub fn is_shared(&self) -> bool {
        *self == VDS_A::Shared
    }
}
#[doc = "Volatile data shared\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDS_AW {
    #[doc = "0: Resets volatile data shared bit"]
    Reset = 0,
}
impl From<VDS_AW> for bool {
    #[inline(always)]
    fn from(variant: VDS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDS` writer - Volatile data shared"]
pub type VDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, VDS_AW, O>;
impl<'a, const O: u8> VDS_W<'a, O> {
    #[doc = "Resets volatile data shared bit"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(VDS_AW::Reset)
    }
}
#[doc = "Firewall pre alarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPA_A {
    #[doc = "0: Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    PreArmReset = 0,
    #[doc = "1: Any code executed outside the protected segment will close the Firewall"]
    PreArmSet = 1,
}
impl From<FPA_A> for bool {
    #[inline(always)]
    fn from(variant: FPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPA` reader - Firewall pre alarm"]
pub type FPA_R = crate::BitReader<FPA_A>;
impl FPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPA_A {
        match self.bits {
            false => FPA_A::PreArmReset,
            true => FPA_A::PreArmSet,
        }
    }
    #[doc = "Checks if the value of the field is `PreArmReset`"]
    #[inline(always)]
    pub fn is_pre_arm_reset(&self) -> bool {
        *self == FPA_A::PreArmReset
    }
    #[doc = "Checks if the value of the field is `PreArmSet`"]
    #[inline(always)]
    pub fn is_pre_arm_set(&self) -> bool {
        *self == FPA_A::PreArmSet
    }
}
#[doc = "Field `FPA` writer - Firewall pre alarm"]
pub type FPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FPA_A, O>;
impl<'a, const O: u8> FPA_W<'a, O> {
    #[doc = "Any code executed outside the protected segment when the Firewall is opened will generate a system reset"]
    #[inline(always)]
    pub fn pre_arm_reset(self) -> &'a mut W {
        self.variant(FPA_A::PreArmReset)
    }
    #[doc = "Any code executed outside the protected segment will close the Firewall"]
    #[inline(always)]
    pub fn pre_arm_set(self) -> &'a mut W {
        self.variant(FPA_A::PreArmSet)
    }
}
impl R {
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&self) -> VDE_R {
        VDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&self) -> VDS_R {
        VDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&self) -> FPA_R {
        FPA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Volatile data execution"]
    #[inline(always)]
    pub fn vde(&mut self) -> VDE_W<2> {
        VDE_W::new(self)
    }
    #[doc = "Bit 1 - Volatile data shared"]
    #[inline(always)]
    pub fn vds(&mut self) -> VDS_W<1> {
        VDS_W::new(self)
    }
    #[doc = "Bit 0 - Firewall pre alarm"]
    #[inline(always)]
    pub fn fpa(&mut self) -> FPA_W<0> {
        FPA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Key slot ID successfully selected by the KMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selected {
    #[doc = "0: No key slot ID selected by KMU"]
    Disabled = 0,
    #[doc = "1: Key slot ID successfully selected by KMU"]
    Enabled = 1,
}
impl From<Selected> for bool {
    #[inline(always)]
    fn from(variant: Selected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECTED` reader - Key slot ID successfully selected by the KMU"]
pub type SelectedR = crate::BitReader<Selected>;
impl SelectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selected {
        match self.bits {
            false => Selected::Disabled,
            true => Selected::Enabled,
        }
    }
    #[doc = "No key slot ID selected by KMU"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Selected::Disabled
    }
    #[doc = "Key slot ID successfully selected by KMU"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Selected::Enabled
    }
}
#[doc = "Violation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blocked {
    #[doc = "0: No access violation detected"]
    Disabled = 0,
    #[doc = "1: Access violation detected and blocked"]
    Enabled = 1,
}
impl From<Blocked> for bool {
    #[inline(always)]
    fn from(variant: Blocked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCKED` reader - Violation status"]
pub type BlockedR = crate::BitReader<Blocked>;
impl BlockedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blocked {
        match self.bits {
            false => Blocked::Disabled,
            true => Blocked::Enabled,
        }
    }
    #[doc = "No access violation detected"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Blocked::Disabled
    }
    #[doc = "Access violation detected and blocked"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Blocked::Enabled
    }
}
impl R {
    #[doc = "Bit 0 - Key slot ID successfully selected by the KMU"]
    #[inline(always)]
    pub fn selected(&self) -> SelectedR {
        SelectedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Violation status"]
    #[inline(always)]
    pub fn blocked(&self) -> BlockedR {
        BlockedR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status bits for KMU operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}

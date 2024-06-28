#[doc = "Register `VARIANT` reader"]
pub type R = crate::R<VariantSpec>;
#[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII\n\nValue on reset: 268435455"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Variant {
    #[doc = "1094795585: AAAA"]
    Aaaa = 1094795585,
    #[doc = "1094795568: AAA0"]
    Aaa0 = 1094795568,
    #[doc = "1094795824: AAB0"]
    Aab0 = 1094795824,
    #[doc = "1094796080: AAC0"]
    Aac0 = 1094796080,
}
impl From<Variant> for u32 {
    #[inline(always)]
    fn from(variant: Variant) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Variant {
    type Ux = u32;
}
impl crate::IsEnum for Variant {}
#[doc = "Field `VARIANT` reader - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
pub type VariantR = crate::FieldReader<Variant>;
impl VariantR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Variant> {
        match self.bits {
            1094795585 => Some(Variant::Aaaa),
            1094795568 => Some(Variant::Aaa0),
            1094795824 => Some(Variant::Aab0),
            1094796080 => Some(Variant::Aac0),
            _ => None,
        }
    }
    #[doc = "AAAA"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == Variant::Aaaa
    }
    #[doc = "AAA0"]
    #[inline(always)]
    pub fn is_aaa0(&self) -> bool {
        *self == Variant::Aaa0
    }
    #[doc = "AAB0"]
    #[inline(always)]
    pub fn is_aab0(&self) -> bool {
        *self == Variant::Aab0
    }
    #[doc = "AAC0"]
    #[inline(always)]
    pub fn is_aac0(&self) -> bool {
        *self == Variant::Aac0
    }
}
impl R {
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(self.bits)
    }
}
#[doc = "Part Variant, Hardware version and Production configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`variant::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VariantSpec;
impl crate::RegisterSpec for VariantSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`variant::R`](R) reader structure"]
impl crate::Readable for VariantSpec {}
#[doc = "`reset()` method sets VARIANT to value 0x0fff_ffff"]
impl crate::Resettable for VariantSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}

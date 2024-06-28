#[doc = "Register `PARTNO` reader"]
pub type R = crate::R<PartnoSpec>;
#[doc = "\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Partno {
    #[doc = "37216: Device is an nRF9160 sip"]
    _9160 = 37216,
}
impl From<Partno> for u32 {
    #[inline(always)]
    fn from(variant: Partno) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Partno {
    type Ux = u32;
}
impl crate::IsEnum for Partno {}
#[doc = "Field `PARTNO` reader - "]
pub type PartnoR = crate::FieldReader<Partno>;
impl PartnoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Partno> {
        match self.bits {
            37216 => Some(Partno::_9160),
            _ => None,
        }
    }
    #[doc = "Device is an nRF9160 sip"]
    #[inline(always)]
    pub fn is_9160(&self) -> bool {
        *self == Partno::_9160
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(self.bits)
    }
}
#[doc = "SIP part number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`partno::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartnoSpec;
impl crate::RegisterSpec for PartnoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`partno::R`](R) reader structure"]
impl crate::Readable for PartnoSpec {}
#[doc = "`reset()` method sets PARTNO to value 0xffff_ffff"]
impl crate::Resettable for PartnoSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

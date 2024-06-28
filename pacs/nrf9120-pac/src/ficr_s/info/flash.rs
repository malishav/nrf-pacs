#[doc = "Register `FLASH` reader"]
pub type R = crate::R<FlashSpec>;
#[doc = "Flash variant\n\nValue on reset: 1024"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Flash {
    #[doc = "1024: 1 MByte FLASH"]
    K1024 = 1024,
}
impl From<Flash> for u32 {
    #[inline(always)]
    fn from(variant: Flash) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flash {
    type Ux = u32;
}
impl crate::IsEnum for Flash {}
#[doc = "Field `FLASH` reader - Flash variant"]
pub type FlashR = crate::FieldReader<Flash>;
impl FlashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flash> {
        match self.bits {
            1024 => Some(Flash::K1024),
            _ => None,
        }
    }
    #[doc = "1 MByte FLASH"]
    #[inline(always)]
    pub fn is_k1024(&self) -> bool {
        *self == Flash::K1024
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(self.bits)
    }
}
#[doc = "Flash variant\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashSpec;
impl crate::RegisterSpec for FlashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash::R`](R) reader structure"]
impl crate::Readable for FlashSpec {}
#[doc = "`reset()` method sets FLASH to value 0x0400"]
impl crate::Resettable for FlashSpec {
    const RESET_VALUE: u32 = 0x0400;
}

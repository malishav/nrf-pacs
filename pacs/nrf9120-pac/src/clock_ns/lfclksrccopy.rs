#[doc = "Register `LFCLKSRCCOPY` reader"]
pub type R = crate::R<LfclksrccopySpec>;
#[doc = "Clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: Reserved for future use"]
    Rfu = 0,
    #[doc = "1: 32.768 kHz RC oscillator"]
    Lfrc = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    Lfxo = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Clock source"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Rfu),
            1 => Some(Src::Lfrc),
            2 => Some(Src::Lfxo),
            _ => None,
        }
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == Src::Rfu
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Src::Lfrc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Src::Lfxo
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksrccopy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclksrccopySpec;
impl crate::RegisterSpec for LfclksrccopySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksrccopy::R`](R) reader structure"]
impl crate::Readable for LfclksrccopySpec {}
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0x01"]
impl crate::Resettable for LfclksrccopySpec {
    const RESET_VALUE: u32 = 0x01;
}

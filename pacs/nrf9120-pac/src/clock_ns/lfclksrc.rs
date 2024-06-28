#[doc = "Register `LFCLKSRC` reader"]
pub type R = crate::R<LfclksrcSpec>;
#[doc = "Register `LFCLKSRC` writer"]
pub type W = crate::W<LfclksrcSpec>;
#[doc = "Clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: Reserved for future use (equals selecting LFRC)"]
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
    #[doc = "Reserved for future use (equals selecting LFRC)"]
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
#[doc = "Field `SRC` writer - Clock source"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved for future use (equals selecting LFRC)"]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Rfu)
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfrc)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Lfxo)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<LfclksrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclksrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclksrcSpec;
impl crate::RegisterSpec for LfclksrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksrc::R`](R) reader structure"]
impl crate::Readable for LfclksrcSpec {}
#[doc = "`write(|w| ..)` method takes [`lfclksrc::W`](W) writer structure"]
impl crate::Writable for LfclksrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFCLKSRC to value 0x01"]
impl crate::Resettable for LfclksrcSpec {
    const RESET_VALUE: u32 = 0x01;
}

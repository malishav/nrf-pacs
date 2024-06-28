#[doc = "Register `EXTPOFCON` reader"]
pub type R = crate::R<ExtpofconSpec>;
#[doc = "Register `EXTPOFCON` writer"]
pub type W = crate::W<ExtpofconSpec>;
#[doc = "Enable or disable external power failure warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pof {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Pof> for bool {
    #[inline(always)]
    fn from(variant: Pof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POF` reader - Enable or disable external power failure warning"]
pub type PofR = crate::BitReader<Pof>;
impl PofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pof {
        match self.bits {
            false => Pof::Disabled,
            true => Pof::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pof::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pof::Enabled
    }
}
#[doc = "Field `POF` writer - Enable or disable external power failure warning"]
pub type PofW<'a, REG> = crate::BitWriter<'a, REG, Pof>;
impl<'a, REG> PofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pof::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable external power failure warning"]
    #[inline(always)]
    pub fn pof(&self) -> PofR {
        PofR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable external power failure warning"]
    #[inline(always)]
    #[must_use]
    pub fn pof(&mut self) -> PofW<ExtpofconSpec> {
        PofW::new(self, 0)
    }
}
#[doc = "External power failure warning configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extpofcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extpofcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtpofconSpec;
impl crate::RegisterSpec for ExtpofconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extpofcon::R`](R) reader structure"]
impl crate::Readable for ExtpofconSpec {}
#[doc = "`write(|w| ..)` method takes [`extpofcon::W`](W) writer structure"]
impl crate::Writable for ExtpofconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTPOFCON to value 0"]
impl crate::Resettable for ExtpofconSpec {
    const RESET_VALUE: u32 = 0;
}

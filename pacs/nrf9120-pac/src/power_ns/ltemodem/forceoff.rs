#[doc = "Register `FORCEOFF` reader"]
pub type R = crate::R<ForceoffSpec>;
#[doc = "Register `FORCEOFF` writer"]
pub type W = crate::W<ForceoffSpec>;
#[doc = "Force off LTE modem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceoff {
    #[doc = "0: Release force off"]
    Release = 0,
    #[doc = "1: Hold force off active"]
    Hold = 1,
}
impl From<Forceoff> for bool {
    #[inline(always)]
    fn from(variant: Forceoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEOFF` reader - Force off LTE modem"]
pub type ForceoffR = crate::BitReader<Forceoff>;
impl ForceoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forceoff {
        match self.bits {
            false => Forceoff::Release,
            true => Forceoff::Hold,
        }
    }
    #[doc = "Release force off"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == Forceoff::Release
    }
    #[doc = "Hold force off active"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == Forceoff::Hold
    }
}
#[doc = "Field `FORCEOFF` writer - Force off LTE modem"]
pub type ForceoffW<'a, REG> = crate::BitWriter<'a, REG, Forceoff>;
impl<'a, REG> ForceoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Release force off"]
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(Forceoff::Release)
    }
    #[doc = "Hold force off active"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut crate::W<REG> {
        self.variant(Forceoff::Hold)
    }
}
impl R {
    #[doc = "Bit 0 - Force off LTE modem"]
    #[inline(always)]
    pub fn forceoff(&self) -> ForceoffR {
        ForceoffR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force off LTE modem"]
    #[inline(always)]
    #[must_use]
    pub fn forceoff(&mut self) -> ForceoffW<ForceoffSpec> {
        ForceoffW::new(self, 0)
    }
}
#[doc = "Force off LTE modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`forceoff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`forceoff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceoffSpec;
impl crate::RegisterSpec for ForceoffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`forceoff::R`](R) reader structure"]
impl crate::Readable for ForceoffSpec {}
#[doc = "`write(|w| ..)` method takes [`forceoff::W`](W) writer structure"]
impl crate::Writable for ForceoffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCEOFF to value 0"]
impl crate::Resettable for ForceoffSpec {
    const RESET_VALUE: u32 = 0;
}

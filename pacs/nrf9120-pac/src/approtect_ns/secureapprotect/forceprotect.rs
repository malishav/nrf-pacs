#[doc = "Register `FORCEPROTECT` reader"]
pub type R = crate::R<ForceprotectSpec>;
#[doc = "Register `FORCEPROTECT` writer"]
pub type W = crate::W<ForceprotectSpec>;
#[doc = "Write 0x1 to force enable SECUREAPPROTECT mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forceprotect {
    #[doc = "1: Software force enable SECUREAPPROTECT mechanism"]
    Force = 1,
}
impl From<Forceprotect> for bool {
    #[inline(always)]
    fn from(variant: Forceprotect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEPROTECT` reader - Write 0x1 to force enable SECUREAPPROTECT mechanism"]
pub type ForceprotectR = crate::BitReader<Forceprotect>;
impl ForceprotectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Forceprotect> {
        match self.bits {
            true => Some(Forceprotect::Force),
            _ => None,
        }
    }
    #[doc = "Software force enable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == Forceprotect::Force
    }
}
#[doc = "Field `FORCEPROTECT` writer - Write 0x1 to force enable SECUREAPPROTECT mechanism"]
pub type ForceprotectW<'a, REG> = crate::BitWriter1S<'a, REG, Forceprotect>;
impl<'a, REG> ForceprotectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software force enable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn force(self) -> &'a mut crate::W<REG> {
        self.variant(Forceprotect::Force)
    }
}
impl R {
    #[doc = "Bit 9 - Write 0x1 to force enable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> ForceprotectR {
        ForceprotectR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Write 0x1 to force enable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn forceprotect(&mut self) -> ForceprotectW<ForceprotectSpec> {
        ForceprotectW::new(self, 9)
    }
}
#[doc = "Software force SECUREAPPROTECT mechanism\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`forceprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`forceprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceprotectSpec;
impl crate::RegisterSpec for ForceprotectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`forceprotect::R`](R) reader structure"]
impl crate::Readable for ForceprotectSpec {}
#[doc = "`write(|w| ..)` method takes [`forceprotect::W`](W) writer structure"]
impl crate::Writable for ForceprotectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0200;
}
#[doc = "`reset()` method sets FORCEPROTECT to value 0x01"]
impl crate::Resettable for ForceprotectSpec {
    const RESET_VALUE: u32 = 0x01;
}

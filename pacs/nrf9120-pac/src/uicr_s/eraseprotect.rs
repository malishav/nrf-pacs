#[doc = "Register `ERASEPROTECT` reader"]
pub type R = crate::R<EraseprotectSpec>;
#[doc = "Register `ERASEPROTECT` writer"]
pub type W = crate::W<EraseprotectSpec>;
#[doc = "Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pall {
    #[doc = "4294967295: Unprotected"]
    Unprotected = 4294967295,
    #[doc = "0: Protected"]
    Protected = 0,
}
impl From<Pall> for u32 {
    #[inline(always)]
    fn from(variant: Pall) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pall {
    type Ux = u32;
}
impl crate::IsEnum for Pall {}
#[doc = "Field `PALL` reader - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
pub type PallR = crate::FieldReader<Pall>;
impl PallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pall> {
        match self.bits {
            4294967295 => Some(Pall::Unprotected),
            0 => Some(Pall::Protected),
            _ => None,
        }
    }
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn is_unprotected(&self) -> bool {
        *self == Pall::Unprotected
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == Pall::Protected
    }
}
#[doc = "Field `PALL` writer - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn unprotected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Unprotected)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Protected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
    #[inline(always)]
    pub fn pall(&self) -> PallR {
        PallR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks NVMC ERASEALL and CTRLAP ERASEALL functionality"]
    #[inline(always)]
    #[must_use]
    pub fn pall(&mut self) -> PallW<EraseprotectSpec> {
        PallW::new(self, 0)
    }
}
#[doc = "Erase protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eraseprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eraseprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EraseprotectSpec;
impl crate::RegisterSpec for EraseprotectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eraseprotect::R`](R) reader structure"]
impl crate::Readable for EraseprotectSpec {}
#[doc = "`write(|w| ..)` method takes [`eraseprotect::W`](W) writer structure"]
impl crate::Writable for EraseprotectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERASEPROTECT to value 0"]
impl crate::Resettable for EraseprotectSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SECUREAPPROTECT` reader"]
pub type R = crate::R<SecureapprotectSpec>;
#[doc = "Register `SECUREAPPROTECT` writer"]
pub type W = crate::W<SecureapprotectSpec>;
#[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pall {
    #[doc = "1358582010: HwUnprotected"]
    HwUnprotected = 1358582010,
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
#[doc = "Field `PALL` reader - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
pub type PallR = crate::FieldReader<Pall>;
impl PallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pall> {
        match self.bits {
            1358582010 => Some(Pall::HwUnprotected),
            0 => Some(Pall::Protected),
            _ => None,
        }
    }
    #[doc = "HwUnprotected"]
    #[inline(always)]
    pub fn is_hw_unprotected(&self) -> bool {
        *self == Pall::HwUnprotected
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == Pall::Protected
    }
}
#[doc = "Field `PALL` writer - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "HwUnprotected"]
    #[inline(always)]
    pub fn hw_unprotected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::HwUnprotected)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Protected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&self) -> PallR {
        PallR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
    #[inline(always)]
    #[must_use]
    pub fn pall(&mut self) -> PallW<SecureapprotectSpec> {
        PallW::new(self, 0)
    }
}
#[doc = "Secure access port protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secureapprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secureapprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecureapprotectSpec;
impl crate::RegisterSpec for SecureapprotectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secureapprotect::R`](R) reader structure"]
impl crate::Readable for SecureapprotectSpec {}
#[doc = "`write(|w| ..)` method takes [`secureapprotect::W`](W) writer structure"]
impl crate::Writable for SecureapprotectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECUREAPPROTECT to value 0"]
impl crate::Resettable for SecureapprotectSpec {
    const RESET_VALUE: u32 = 0;
}

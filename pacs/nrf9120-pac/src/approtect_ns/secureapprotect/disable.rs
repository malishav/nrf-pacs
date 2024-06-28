#[doc = "Register `DISABLE` reader"]
pub type R = crate::R<DisableSpec>;
#[doc = "Register `DISABLE` writer"]
pub type W = crate::W<DisableSpec>;
#[doc = "Software disable SECUREAPPROTECT mechanism\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Disable {
    #[doc = "90: Software disable SECUREAPPROTECT mechanism"]
    SwUnprotected = 90,
}
impl From<Disable> for u8 {
    #[inline(always)]
    fn from(variant: Disable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Disable {
    type Ux = u8;
}
impl crate::IsEnum for Disable {}
#[doc = "Field `DISABLE` reader - Software disable SECUREAPPROTECT mechanism"]
pub type DisableR = crate::FieldReader<Disable>;
impl DisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Disable> {
        match self.bits {
            90 => Some(Disable::SwUnprotected),
            _ => None,
        }
    }
    #[doc = "Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn is_sw_unprotected(&self) -> bool {
        *self == Disable::SwUnprotected
    }
}
#[doc = "Field `DISABLE` writer - Software disable SECUREAPPROTECT mechanism"]
pub type DisableW<'a, REG> = crate::FieldWriter<'a, REG, 8, Disable>;
impl<'a, REG> DisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn sw_unprotected(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::SwUnprotected)
    }
}
impl R {
    #[doc = "Bits 0:7 - Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software disable SECUREAPPROTECT mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DisableW<DisableSpec> {
        DisableW::new(self, 0)
    }
}
#[doc = "Software disable SECUREAPPROTECT mechanism\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableSpec;
impl crate::RegisterSpec for DisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disable::R`](R) reader structure"]
impl crate::Readable for DisableSpec {}
#[doc = "`write(|w| ..)` method takes [`disable::W`](W) writer structure"]
impl crate::Writable for DisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DISABLE to value 0x01"]
impl crate::Resettable for DisableSpec {
    const RESET_VALUE: u32 = 0x01;
}

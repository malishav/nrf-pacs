#[doc = "Register `APPNVMCPOFGUARD` reader"]
pub type R = crate::R<AppnvmcpofguardSpec>;
#[doc = "Register `APPNVMCPOFGUARD` writer"]
pub type W = crate::W<AppnvmcpofguardSpec>;
#[doc = "Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nvmcpofguarden {
    #[doc = "0: NVM WRITE and NVM ERASE are not blocked in POFWARN condition"]
    Disabled = 0,
    #[doc = "1: NVM WRITE and NVM ERASE are blocked in POFWARN condition"]
    Enabled = 1,
}
impl From<Nvmcpofguarden> for bool {
    #[inline(always)]
    fn from(variant: Nvmcpofguarden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NVMCPOFGUARDEN` reader - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
pub type NvmcpofguardenR = crate::BitReader<Nvmcpofguarden>;
impl NvmcpofguardenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nvmcpofguarden {
        match self.bits {
            false => Nvmcpofguarden::Disabled,
            true => Nvmcpofguarden::Enabled,
        }
    }
    #[doc = "NVM WRITE and NVM ERASE are not blocked in POFWARN condition"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nvmcpofguarden::Disabled
    }
    #[doc = "NVM WRITE and NVM ERASE are blocked in POFWARN condition"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Nvmcpofguarden::Enabled
    }
}
#[doc = "Field `NVMCPOFGUARDEN` writer - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
pub type NvmcpofguardenW<'a, REG> = crate::BitWriter<'a, REG, Nvmcpofguarden>;
impl<'a, REG> NvmcpofguardenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NVM WRITE and NVM ERASE are not blocked in POFWARN condition"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmcpofguarden::Disabled)
    }
    #[doc = "NVM WRITE and NVM ERASE are blocked in POFWARN condition"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nvmcpofguarden::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
    #[inline(always)]
    pub fn nvmcpofguarden(&self) -> NvmcpofguardenR {
        NvmcpofguardenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable blocking NVM WRITE and aborting NVM ERASE in POFWARN condition"]
    #[inline(always)]
    #[must_use]
    pub fn nvmcpofguarden(&mut self) -> NvmcpofguardenW<AppnvmcpofguardSpec> {
        NvmcpofguardenW::new(self, 0)
    }
}
#[doc = "Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appnvmcpofguard::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appnvmcpofguard::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppnvmcpofguardSpec;
impl crate::RegisterSpec for AppnvmcpofguardSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appnvmcpofguard::R`](R) reader structure"]
impl crate::Readable for AppnvmcpofguardSpec {}
#[doc = "`write(|w| ..)` method takes [`appnvmcpofguard::W`](W) writer structure"]
impl crate::Writable for AppnvmcpofguardSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPNVMCPOFGUARD to value 0xffff_ffff"]
impl crate::Resettable for AppnvmcpofguardSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

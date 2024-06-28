#[doc = "Register `POWERSTATUS` reader"]
pub type R = crate::R<PowerstatusSpec>;
#[doc = "LTE modem domain status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ltemodem {
    #[doc = "0: LTE modem domain is powered off"]
    Off = 0,
    #[doc = "1: LTE modem domain is powered on"]
    On = 1,
}
impl From<Ltemodem> for bool {
    #[inline(always)]
    fn from(variant: Ltemodem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTEMODEM` reader - LTE modem domain status"]
pub type LtemodemR = crate::BitReader<Ltemodem>;
impl LtemodemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ltemodem {
        match self.bits {
            false => Ltemodem::Off,
            true => Ltemodem::On,
        }
    }
    #[doc = "LTE modem domain is powered off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Ltemodem::Off
    }
    #[doc = "LTE modem domain is powered on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Ltemodem::On
    }
}
impl R {
    #[doc = "Bit 0 - LTE modem domain status"]
    #[inline(always)]
    pub fn ltemodem(&self) -> LtemodemR {
        LtemodemR::new((self.bits & 1) != 0)
    }
}
#[doc = "Modem domain power status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`powerstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerstatusSpec;
impl crate::RegisterSpec for PowerstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`powerstatus::R`](R) reader structure"]
impl crate::Readable for PowerstatusSpec {}
#[doc = "`reset()` method sets POWERSTATUS to value 0"]
impl crate::Resettable for PowerstatusSpec {
    const RESET_VALUE: u32 = 0;
}

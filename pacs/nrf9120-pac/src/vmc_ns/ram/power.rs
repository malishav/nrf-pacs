#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Keep RAM section S0 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S0power> for bool {
    #[inline(always)]
    fn from(variant: S0power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0POWER` reader - Keep RAM section S0 of RAM n on or off in System ON mode"]
pub type S0powerR = crate::BitReader<S0power>;
impl S0powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0power {
        match self.bits {
            false => S0power::Off,
            true => S0power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0power::On
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 of RAM n on or off in System ON mode"]
pub type S0powerW<'a, REG> = crate::BitWriter<'a, REG, S0power>;
impl<'a, REG> S0powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S0power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S0power::On)
    }
}
#[doc = "Keep RAM section S1 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S1power> for bool {
    #[inline(always)]
    fn from(variant: S1power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1POWER` reader - Keep RAM section S1 of RAM n on or off in System ON mode"]
pub type S1powerR = crate::BitReader<S1power>;
impl S1powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1power {
        match self.bits {
            false => S1power::Off,
            true => S1power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1power::On
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 of RAM n on or off in System ON mode"]
pub type S1powerW<'a, REG> = crate::BitWriter<'a, REG, S1power>;
impl<'a, REG> S1powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S1power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S1power::On)
    }
}
#[doc = "Keep RAM section S2 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S2power> for bool {
    #[inline(always)]
    fn from(variant: S2power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2POWER` reader - Keep RAM section S2 of RAM n on or off in System ON mode"]
pub type S2powerR = crate::BitReader<S2power>;
impl S2powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2power {
        match self.bits {
            false => S2power::Off,
            true => S2power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S2power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S2power::On
    }
}
#[doc = "Field `S2POWER` writer - Keep RAM section S2 of RAM n on or off in System ON mode"]
pub type S2powerW<'a, REG> = crate::BitWriter<'a, REG, S2power>;
impl<'a, REG> S2powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S2power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S2power::On)
    }
}
#[doc = "Keep RAM section S3 of RAM n on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3power {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S3power> for bool {
    #[inline(always)]
    fn from(variant: S3power) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3POWER` reader - Keep RAM section S3 of RAM n on or off in System ON mode"]
pub type S3powerR = crate::BitReader<S3power>;
impl S3powerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3power {
        match self.bits {
            false => S3power::Off,
            true => S3power::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S3power::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S3power::On
    }
}
#[doc = "Field `S3POWER` writer - Keep RAM section S3 of RAM n on or off in System ON mode"]
pub type S3powerW<'a, REG> = crate::BitWriter<'a, REG, S3power>;
impl<'a, REG> S3powerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S3power::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S3power::On)
    }
}
#[doc = "Keep retention on RAM section S0 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S0retention> for bool {
    #[inline(always)]
    fn from(variant: S0retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RETENTION` reader - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
pub type S0retentionR = crate::BitReader<S0retention>;
impl S0retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0retention {
        match self.bits {
            false => S0retention::Off,
            true => S0retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0retention::On
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
pub type S0retentionW<'a, REG> = crate::BitWriter<'a, REG, S0retention>;
impl<'a, REG> S0retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S0retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S0retention::On)
    }
}
#[doc = "Keep retention on RAM section S1 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S1retention> for bool {
    #[inline(always)]
    fn from(variant: S1retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RETENTION` reader - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
pub type S1retentionR = crate::BitReader<S1retention>;
impl S1retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1retention {
        match self.bits {
            false => S1retention::Off,
            true => S1retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1retention::On
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
pub type S1retentionW<'a, REG> = crate::BitWriter<'a, REG, S1retention>;
impl<'a, REG> S1retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S1retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S1retention::On)
    }
}
#[doc = "Keep retention on RAM section S2 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S2retention> for bool {
    #[inline(always)]
    fn from(variant: S2retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RETENTION` reader - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
pub type S2retentionR = crate::BitReader<S2retention>;
impl S2retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2retention {
        match self.bits {
            false => S2retention::Off,
            true => S2retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S2retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S2retention::On
    }
}
#[doc = "Field `S2RETENTION` writer - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
pub type S2retentionW<'a, REG> = crate::BitWriter<'a, REG, S2retention>;
impl<'a, REG> S2retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S2retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S2retention::On)
    }
}
#[doc = "Keep retention on RAM section S3 of RAM n when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3retention {
    #[doc = "0: Off"]
    Off = 0,
    #[doc = "1: On"]
    On = 1,
}
impl From<S3retention> for bool {
    #[inline(always)]
    fn from(variant: S3retention) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3RETENTION` reader - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
pub type S3retentionR = crate::BitReader<S3retention>;
impl S3retentionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3retention {
        match self.bits {
            false => S3retention::Off,
            true => S3retention::On,
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S3retention::Off
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S3retention::On
    }
}
#[doc = "Field `S3RETENTION` writer - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
pub type S3retentionW<'a, REG> = crate::BitWriter<'a, REG, S3retention>;
impl<'a, REG> S3retentionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(S3retention::Off)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(S3retention::On)
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM n on or off in System ON mode"]
    #[inline(always)]
    pub fn s0power(&self) -> S0powerR {
        S0powerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM n on or off in System ON mode"]
    #[inline(always)]
    pub fn s1power(&self) -> S1powerR {
        S1powerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM n on or off in System ON mode"]
    #[inline(always)]
    pub fn s2power(&self) -> S2powerR {
        S2powerR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM n on or off in System ON mode"]
    #[inline(always)]
    pub fn s3power(&self) -> S3powerR {
        S3powerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
    #[inline(always)]
    pub fn s0retention(&self) -> S0retentionR {
        S0retentionR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
    #[inline(always)]
    pub fn s1retention(&self) -> S1retentionR {
        S1retentionR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
    #[inline(always)]
    pub fn s2retention(&self) -> S2retentionR {
        S2retentionR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
    #[inline(always)]
    pub fn s3retention(&self) -> S3retentionR {
        S3retentionR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s0power(&mut self) -> S0powerW<PowerSpec> {
        S0powerW::new(self, 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s1power(&mut self) -> S1powerW<PowerSpec> {
        S1powerW::new(self, 1)
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s2power(&mut self) -> S2powerW<PowerSpec> {
        S2powerW::new(self, 2)
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM n on or off in System ON mode"]
    #[inline(always)]
    #[must_use]
    pub fn s3power(&mut self) -> S3powerW<PowerSpec> {
        S3powerW::new(self, 3)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s0retention(&mut self) -> S0retentionW<PowerSpec> {
        S0retentionW::new(self, 16)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s1retention(&mut self) -> S1retentionW<PowerSpec> {
        S1retentionW::new(self, 17)
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s2retention(&mut self) -> S2retentionW<PowerSpec> {
        S2retentionW::new(self, 18)
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn s3retention(&mut self) -> S3retentionW<PowerSpec> {
        S3retentionW::new(self, 19)
    }
}
#[doc = "Description cluster: RAMn power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER to value 0xffff"]
impl crate::Resettable for PowerSpec {
    const RESET_VALUE: u32 = 0xffff;
}

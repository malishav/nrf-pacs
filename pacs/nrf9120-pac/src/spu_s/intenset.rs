#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramaccerr {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ramaccerr> for bool {
    #[inline(always)]
    fn from(variant: Ramaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMACCERR` reader - Write '1' to enable interrupt for event RAMACCERR"]
pub type RamaccerrR = crate::BitReader<Ramaccerr>;
impl RamaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramaccerr {
        match self.bits {
            false => Ramaccerr::Disabled,
            true => Ramaccerr::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ramaccerr::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ramaccerr::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamaccerrWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RamaccerrWO> for bool {
    #[inline(always)]
    fn from(variant: RamaccerrWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMACCERR` writer - Write '1' to enable interrupt for event RAMACCERR"]
pub type RamaccerrW<'a, REG> = crate::BitWriter<'a, REG, RamaccerrWO>;
impl<'a, REG> RamaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RamaccerrWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashaccerr {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Flashaccerr> for bool {
    #[inline(always)]
    fn from(variant: Flashaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHACCERR` reader - Write '1' to enable interrupt for event FLASHACCERR"]
pub type FlashaccerrR = crate::BitReader<Flashaccerr>;
impl FlashaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashaccerr {
        match self.bits {
            false => Flashaccerr::Disabled,
            true => Flashaccerr::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flashaccerr::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flashaccerr::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashaccerrWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FlashaccerrWO> for bool {
    #[inline(always)]
    fn from(variant: FlashaccerrWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHACCERR` writer - Write '1' to enable interrupt for event FLASHACCERR"]
pub type FlashaccerrW<'a, REG> = crate::BitWriter<'a, REG, FlashaccerrWO>;
impl<'a, REG> FlashaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FlashaccerrWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Periphaccerr {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Periphaccerr> for bool {
    #[inline(always)]
    fn from(variant: Periphaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHACCERR` reader - Write '1' to enable interrupt for event PERIPHACCERR"]
pub type PeriphaccerrR = crate::BitReader<Periphaccerr>;
impl PeriphaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Periphaccerr {
        match self.bits {
            false => Periphaccerr::Disabled,
            true => Periphaccerr::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Periphaccerr::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Periphaccerr::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeriphaccerrWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<PeriphaccerrWO> for bool {
    #[inline(always)]
    fn from(variant: PeriphaccerrWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PERIPHACCERR` writer - Write '1' to enable interrupt for event PERIPHACCERR"]
pub type PeriphaccerrW<'a, REG> = crate::BitWriter<'a, REG, PeriphaccerrWO>;
impl<'a, REG> PeriphaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(PeriphaccerrWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&self) -> RamaccerrR {
        RamaccerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&self) -> FlashaccerrR {
        FlashaccerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&self) -> PeriphaccerrR {
        PeriphaccerrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RAMACCERR"]
    #[inline(always)]
    #[must_use]
    pub fn ramaccerr(&mut self) -> RamaccerrW<IntensetSpec> {
        RamaccerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event FLASHACCERR"]
    #[inline(always)]
    #[must_use]
    pub fn flashaccerr(&mut self) -> FlashaccerrW<IntensetSpec> {
        FlashaccerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    #[must_use]
    pub fn periphaccerr(&mut self) -> PeriphaccerrW<IntensetSpec> {
        PeriphaccerrW::new(self, 2)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}

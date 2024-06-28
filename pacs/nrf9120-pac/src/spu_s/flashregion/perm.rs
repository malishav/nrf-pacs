#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Configure instruction fetch permissions from flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Execute {
    #[doc = "1: Allow instruction fetches from flash region n"]
    Enable = 1,
    #[doc = "0: Block instruction fetches from flash region n"]
    Disable = 0,
}
impl From<Execute> for bool {
    #[inline(always)]
    fn from(variant: Execute) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXECUTE` reader - Configure instruction fetch permissions from flash region n"]
pub type ExecuteR = crate::BitReader<Execute>;
impl ExecuteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Execute {
        match self.bits {
            true => Execute::Enable,
            false => Execute::Disable,
        }
    }
    #[doc = "Allow instruction fetches from flash region n"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Execute::Enable
    }
    #[doc = "Block instruction fetches from flash region n"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Execute::Disable
    }
}
#[doc = "Field `EXECUTE` writer - Configure instruction fetch permissions from flash region n"]
pub type ExecuteW<'a, REG> = crate::BitWriter<'a, REG, Execute>;
impl<'a, REG> ExecuteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow instruction fetches from flash region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Enable)
    }
    #[doc = "Block instruction fetches from flash region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Execute::Disable)
    }
}
#[doc = "Configure write permission for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Write {
    #[doc = "1: Allow write operation to region n"]
    Enable = 1,
    #[doc = "0: Block write operation to region n"]
    Disable = 0,
}
impl From<Write> for bool {
    #[inline(always)]
    fn from(variant: Write) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE` reader - Configure write permission for flash region n"]
pub type WriteR = crate::BitReader<Write>;
impl WriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Write {
        match self.bits {
            true => Write::Enable,
            false => Write::Disable,
        }
    }
    #[doc = "Allow write operation to region n"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Write::Enable
    }
    #[doc = "Block write operation to region n"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Write::Disable
    }
}
#[doc = "Field `WRITE` writer - Configure write permission for flash region n"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG, Write>;
impl<'a, REG> WriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow write operation to region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Enable)
    }
    #[doc = "Block write operation to region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Write::Disable)
    }
}
#[doc = "Configure read permissions for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "1: Allow read operation from flash region n"]
    Enable = 1,
    #[doc = "0: Block read operation from flash region n"]
    Disable = 0,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Configure read permissions for flash region n"]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            true => Read::Enable,
            false => Read::Disable,
        }
    }
    #[doc = "Allow read operation from flash region n"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Read::Enable
    }
    #[doc = "Block read operation from flash region n"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Read::Disable
    }
}
#[doc = "Field `READ` writer - Configure read permissions for flash region n"]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allow read operation from flash region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Enable)
    }
    #[doc = "Block read operation from flash region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Disable)
    }
}
#[doc = "Security attribute for flash region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secattr {
    #[doc = "0: Flash region n security attribute is non-secure"]
    NonSecure = 0,
    #[doc = "1: Flash region n security attribute is secure"]
    Secure = 1,
}
impl From<Secattr> for bool {
    #[inline(always)]
    fn from(variant: Secattr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECATTR` reader - Security attribute for flash region n"]
pub type SecattrR = crate::BitReader<Secattr>;
impl SecattrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secattr {
        match self.bits {
            false => Secattr::NonSecure,
            true => Secattr::Secure,
        }
    }
    #[doc = "Flash region n security attribute is non-secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Secattr::NonSecure
    }
    #[doc = "Flash region n security attribute is secure"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Secattr::Secure
    }
}
#[doc = "Field `SECATTR` writer - Security attribute for flash region n"]
pub type SecattrW<'a, REG> = crate::BitWriter<'a, REG, Secattr>;
impl<'a, REG> SecattrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash region n security attribute is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::NonSecure)
    }
    #[doc = "Flash region n security attribute is secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Secattr::Secure)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: This register can be updated"]
    Unlocked = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG, Lock>;
impl<'a, REG> LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Unlocked)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lock::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure write permission for flash region n"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure read permissions for flash region n"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Security attribute for flash region n"]
    #[inline(always)]
    pub fn secattr(&self) -> SecattrR {
        SecattrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure instruction fetch permissions from flash region n"]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> ExecuteW<PermSpec> {
        ExecuteW::new(self, 0)
    }
    #[doc = "Bit 1 - Configure write permission for flash region n"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<PermSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Configure read permissions for flash region n"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<PermSpec> {
        ReadW::new(self, 2)
    }
    #[doc = "Bit 4 - Security attribute for flash region n"]
    #[inline(always)]
    #[must_use]
    pub fn secattr(&mut self) -> SecattrW<PermSpec> {
        SecattrW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<PermSpec> {
        LockW::new(self, 8)
    }
}
#[doc = "Description cluster: Access permissions for flash region n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PermSpec;
impl crate::RegisterSpec for PermSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perm::R`](R) reader structure"]
impl crate::Readable for PermSpec {}
#[doc = "`write(|w| ..)` method takes [`perm::W`](W) writer structure"]
impl crate::Writable for PermSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERM to value 0x17"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0x17;
}

#[doc = "Register `SIZE` reader"]
pub type R = crate::R<SizeSpec>;
#[doc = "Register `SIZE` writer"]
pub type W = crate::W<SizeSpec>;
#[doc = "Size of the non-secure callable (NSC) region n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    Disabled = 0,
    #[doc = "1: The region n is defined as non-secure callable with a 32-byte size"]
    _32 = 1,
    #[doc = "2: The region n is defined as non-secure callable with a 64-byte size"]
    _64 = 2,
    #[doc = "3: The region n is defined as non-secure callable with a 128-byte size"]
    _128 = 3,
    #[doc = "4: The region n is defined as non-secure callable with a 256-byte size"]
    _256 = 4,
    #[doc = "5: The region n is defined as non-secure callable with a 512-byte size"]
    _512 = 5,
    #[doc = "6: The region n is defined as non-secure callable with a 1024-byte size"]
    _1024 = 6,
    #[doc = "7: The region n is defined as non-secure callable with a 2048-byte size"]
    _2048 = 7,
    #[doc = "8: The region n is defined as non-secure callable with a 4096-byte size"]
    _4096 = 8,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - Size of the non-secure callable (NSC) region n"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            0 => Some(Size::Disabled),
            1 => Some(Size::_32),
            2 => Some(Size::_64),
            3 => Some(Size::_128),
            4 => Some(Size::_256),
            5 => Some(Size::_512),
            6 => Some(Size::_1024),
            7 => Some(Size::_2048),
            8 => Some(Size::_4096),
            _ => None,
        }
    }
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Size::Disabled
    }
    #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Size::_32
    }
    #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Size::_64
    }
    #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Size::_128
    }
    #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Size::_256
    }
    #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == Size::_512
    }
    #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Size::_1024
    }
    #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == Size::_2048
    }
    #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == Size::_4096
    }
}
#[doc = "Field `SIZE` writer - Size of the non-secure callable (NSC) region n"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Disabled)
    }
    #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_32)
    }
    #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_64)
    }
    #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_128)
    }
    #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_256)
    }
    #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_512)
    }
    #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_1024)
    }
    #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_2048)
    }
    #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(Size::_4096)
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
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<SizeSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<SizeSpec> {
        LockW::new(self, 8)
    }
}
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeSpec;
impl crate::RegisterSpec for SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size::R`](R) reader structure"]
impl crate::Readable for SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`size::W`](W) writer structure"]
impl crate::Writable for SizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIZE to value 0"]
impl crate::Resettable for SizeSpec {
    const RESET_VALUE: u32 = 0;
}

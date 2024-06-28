#[doc = "Register `WRITEUICRNS` writer"]
pub type W = crate::W<WriteuicrnsSpec>;
#[doc = "Allow non-secure code to set APPROTECT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Set {
    #[doc = "1: Set value"]
    Set = 1,
}
impl From<Set> for bool {
    #[inline(always)]
    fn from(variant: Set) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET` writer - Allow non-secure code to set APPROTECT"]
pub type SetW<'a, REG> = crate::BitWriter<'a, REG, Set>;
impl<'a, REG> SetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set value"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Set::Set)
    }
}
#[doc = "Key to write in order to validate the write operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Key {
    #[doc = "184280487: Key value"]
    Keyvalid = 184280487,
}
impl From<Key> for u32 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u32;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - Key to write in order to validate the write operation"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 28, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Key value"]
    #[inline(always)]
    pub fn keyvalid(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Keyvalid)
    }
}
impl W {
    #[doc = "Bit 0 - Allow non-secure code to set APPROTECT"]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<WriteuicrnsSpec> {
        SetW::new(self, 0)
    }
    #[doc = "Bits 4:31 - Key to write in order to validate the write operation"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<WriteuicrnsSpec> {
        KeyW::new(self, 4)
    }
}
#[doc = "Non-secure APPROTECT enable register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writeuicrns::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WriteuicrnsSpec;
impl crate::RegisterSpec for WriteuicrnsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`writeuicrns::W`](W) writer structure"]
impl crate::Writable for WriteuicrnsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITEUICRNS to value 0"]
impl crate::Resettable for WriteuicrnsSpec {
    const RESET_VALUE: u32 = 0;
}

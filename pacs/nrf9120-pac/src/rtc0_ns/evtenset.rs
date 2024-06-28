#[doc = "Register `EVTENSET` reader"]
pub type R = crate::R<EvtensetSpec>;
#[doc = "Register `EVTENSET` writer"]
pub type W = crate::W<EvtensetSpec>;
#[doc = "Write '1' to enable event routing for event TICK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tick {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Tick> for bool {
    #[inline(always)]
    fn from(variant: Tick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK` reader - Write '1' to enable event routing for event TICK"]
pub type TickR = crate::BitReader<Tick>;
impl TickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tick {
        match self.bits {
            false => Tick::Disabled,
            true => Tick::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tick::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tick::Enabled
    }
}
#[doc = "Write '1' to enable event routing for event TICK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TickWO> for bool {
    #[inline(always)]
    fn from(variant: TickWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK` writer - Write '1' to enable event routing for event TICK"]
pub type TickW<'a, REG> = crate::BitWriter<'a, REG, TickWO>;
impl<'a, REG> TickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TickWO::Set)
    }
}
#[doc = "Write '1' to enable event routing for event OVRFLW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrflw {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ovrflw> for bool {
    #[inline(always)]
    fn from(variant: Ovrflw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRFLW` reader - Write '1' to enable event routing for event OVRFLW"]
pub type OvrflwR = crate::BitReader<Ovrflw>;
impl OvrflwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrflw {
        match self.bits {
            false => Ovrflw::Disabled,
            true => Ovrflw::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ovrflw::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ovrflw::Enabled
    }
}
#[doc = "Write '1' to enable event routing for event OVRFLW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvrflwWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<OvrflwWO> for bool {
    #[inline(always)]
    fn from(variant: OvrflwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRFLW` writer - Write '1' to enable event routing for event OVRFLW"]
pub type OvrflwW<'a, REG> = crate::BitWriter<'a, REG, OvrflwWO>;
impl<'a, REG> OvrflwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OvrflwWO::Set)
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare0> for bool {
    #[inline(always)]
    fn from(variant: Compare0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - Write '1' to enable event routing for event COMPARE\\[0\\]"]
pub type Compare0R = crate::BitReader<Compare0>;
impl Compare0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare0 {
        match self.bits {
            false => Compare0::Disabled,
            true => Compare0::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare0::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare0::Enabled
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare0WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Compare0WO> for bool {
    #[inline(always)]
    fn from(variant: Compare0WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` writer - Write '1' to enable event routing for event COMPARE\\[0\\]"]
pub type Compare0W<'a, REG> = crate::BitWriter<'a, REG, Compare0WO>;
impl<'a, REG> Compare0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Compare0WO::Set)
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare1> for bool {
    #[inline(always)]
    fn from(variant: Compare1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - Write '1' to enable event routing for event COMPARE\\[1\\]"]
pub type Compare1R = crate::BitReader<Compare1>;
impl Compare1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare1 {
        match self.bits {
            false => Compare1::Disabled,
            true => Compare1::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare1::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare1::Enabled
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare1WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Compare1WO> for bool {
    #[inline(always)]
    fn from(variant: Compare1WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` writer - Write '1' to enable event routing for event COMPARE\\[1\\]"]
pub type Compare1W<'a, REG> = crate::BitWriter<'a, REG, Compare1WO>;
impl<'a, REG> Compare1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Compare1WO::Set)
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare2> for bool {
    #[inline(always)]
    fn from(variant: Compare2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - Write '1' to enable event routing for event COMPARE\\[2\\]"]
pub type Compare2R = crate::BitReader<Compare2>;
impl Compare2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare2 {
        match self.bits {
            false => Compare2::Disabled,
            true => Compare2::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare2::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare2::Enabled
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare2WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Compare2WO> for bool {
    #[inline(always)]
    fn from(variant: Compare2WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` writer - Write '1' to enable event routing for event COMPARE\\[2\\]"]
pub type Compare2W<'a, REG> = crate::BitWriter<'a, REG, Compare2WO>;
impl<'a, REG> Compare2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Compare2WO::Set)
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3 {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Compare3> for bool {
    #[inline(always)]
    fn from(variant: Compare3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - Write '1' to enable event routing for event COMPARE\\[3\\]"]
pub type Compare3R = crate::BitReader<Compare3>;
impl Compare3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Compare3 {
        match self.bits {
            false => Compare3::Disabled,
            true => Compare3::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Compare3::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Compare3::Enabled
    }
}
#[doc = "Write '1' to enable event routing for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compare3WO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Compare3WO> for bool {
    #[inline(always)]
    fn from(variant: Compare3WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` writer - Write '1' to enable event routing for event COMPARE\\[3\\]"]
pub type Compare3W<'a, REG> = crate::BitWriter<'a, REG, Compare3WO>;
impl<'a, REG> Compare3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Compare3WO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable event routing for event TICK"]
    #[inline(always)]
    pub fn tick(&self) -> TickR {
        TickR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable event routing for event OVRFLW"]
    #[inline(always)]
    pub fn ovrflw(&self) -> OvrflwR {
        OvrflwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to enable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> Compare0R {
        Compare0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> Compare1R {
        Compare1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> Compare2R {
        Compare2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> Compare3R {
        Compare3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable event routing for event TICK"]
    #[inline(always)]
    #[must_use]
    pub fn tick(&mut self) -> TickW<EvtensetSpec> {
        TickW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable event routing for event OVRFLW"]
    #[inline(always)]
    #[must_use]
    pub fn ovrflw(&mut self) -> OvrflwW<EvtensetSpec> {
        OvrflwW::new(self, 1)
    }
    #[doc = "Bit 16 - Write '1' to enable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare0(&mut self) -> Compare0W<EvtensetSpec> {
        Compare0W::new(self, 16)
    }
    #[doc = "Bit 17 - Write '1' to enable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare1(&mut self) -> Compare1W<EvtensetSpec> {
        Compare1W::new(self, 17)
    }
    #[doc = "Bit 18 - Write '1' to enable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare2(&mut self) -> Compare2W<EvtensetSpec> {
        Compare2W::new(self, 18)
    }
    #[doc = "Bit 19 - Write '1' to enable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare3(&mut self) -> Compare3W<EvtensetSpec> {
        Compare3W::new(self, 19)
    }
}
#[doc = "Enable event routing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtenset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtenset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtensetSpec;
impl crate::RegisterSpec for EvtensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtenset::R`](R) reader structure"]
impl crate::Readable for EvtensetSpec {}
#[doc = "`write(|w| ..)` method takes [`evtenset::W`](W) writer structure"]
impl crate::Writable for EvtensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTENSET to value 0"]
impl crate::Resettable for EvtensetSpec {
    const RESET_VALUE: u32 = 0;
}

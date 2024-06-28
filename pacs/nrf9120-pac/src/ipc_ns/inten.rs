#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive0> for bool {
    #[inline(always)]
    fn from(variant: Receive0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type Receive0R = crate::BitReader<Receive0>;
impl Receive0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive0 {
        match self.bits {
            false => Receive0::Disabled,
            true => Receive0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive0::Enabled
    }
}
#[doc = "Field `RECEIVE0` writer - Enable or disable interrupt for event RECEIVE\\[0\\]"]
pub type Receive0W<'a, REG> = crate::BitWriter<'a, REG, Receive0>;
impl<'a, REG> Receive0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive1> for bool {
    #[inline(always)]
    fn from(variant: Receive1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type Receive1R = crate::BitReader<Receive1>;
impl Receive1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive1 {
        match self.bits {
            false => Receive1::Disabled,
            true => Receive1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive1::Enabled
    }
}
#[doc = "Field `RECEIVE1` writer - Enable or disable interrupt for event RECEIVE\\[1\\]"]
pub type Receive1W<'a, REG> = crate::BitWriter<'a, REG, Receive1>;
impl<'a, REG> Receive1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive2> for bool {
    #[inline(always)]
    fn from(variant: Receive2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type Receive2R = crate::BitReader<Receive2>;
impl Receive2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive2 {
        match self.bits {
            false => Receive2::Disabled,
            true => Receive2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive2::Enabled
    }
}
#[doc = "Field `RECEIVE2` writer - Enable or disable interrupt for event RECEIVE\\[2\\]"]
pub type Receive2W<'a, REG> = crate::BitWriter<'a, REG, Receive2>;
impl<'a, REG> Receive2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive2::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive3> for bool {
    #[inline(always)]
    fn from(variant: Receive3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type Receive3R = crate::BitReader<Receive3>;
impl Receive3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive3 {
        match self.bits {
            false => Receive3::Disabled,
            true => Receive3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive3::Enabled
    }
}
#[doc = "Field `RECEIVE3` writer - Enable or disable interrupt for event RECEIVE\\[3\\]"]
pub type Receive3W<'a, REG> = crate::BitWriter<'a, REG, Receive3>;
impl<'a, REG> Receive3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive3::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive4 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive4> for bool {
    #[inline(always)]
    fn from(variant: Receive4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type Receive4R = crate::BitReader<Receive4>;
impl Receive4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive4 {
        match self.bits {
            false => Receive4::Disabled,
            true => Receive4::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive4::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive4::Enabled
    }
}
#[doc = "Field `RECEIVE4` writer - Enable or disable interrupt for event RECEIVE\\[4\\]"]
pub type Receive4W<'a, REG> = crate::BitWriter<'a, REG, Receive4>;
impl<'a, REG> Receive4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive4::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive4::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive5 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive5> for bool {
    #[inline(always)]
    fn from(variant: Receive5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type Receive5R = crate::BitReader<Receive5>;
impl Receive5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive5 {
        match self.bits {
            false => Receive5::Disabled,
            true => Receive5::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive5::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive5::Enabled
    }
}
#[doc = "Field `RECEIVE5` writer - Enable or disable interrupt for event RECEIVE\\[5\\]"]
pub type Receive5W<'a, REG> = crate::BitWriter<'a, REG, Receive5>;
impl<'a, REG> Receive5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive5::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive5::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive6 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive6> for bool {
    #[inline(always)]
    fn from(variant: Receive6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type Receive6R = crate::BitReader<Receive6>;
impl Receive6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive6 {
        match self.bits {
            false => Receive6::Disabled,
            true => Receive6::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive6::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive6::Enabled
    }
}
#[doc = "Field `RECEIVE6` writer - Enable or disable interrupt for event RECEIVE\\[6\\]"]
pub type Receive6W<'a, REG> = crate::BitWriter<'a, REG, Receive6>;
impl<'a, REG> Receive6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive6::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive6::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive7 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Receive7> for bool {
    #[inline(always)]
    fn from(variant: Receive7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type Receive7R = crate::BitReader<Receive7>;
impl Receive7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive7 {
        match self.bits {
            false => Receive7::Disabled,
            true => Receive7::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Receive7::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Receive7::Enabled
    }
}
#[doc = "Field `RECEIVE7` writer - Enable or disable interrupt for event RECEIVE\\[7\\]"]
pub type Receive7W<'a, REG> = crate::BitWriter<'a, REG, Receive7>;
impl<'a, REG> Receive7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive7::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Receive7::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> Receive0R {
        Receive0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> Receive1R {
        Receive1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> Receive2R {
        Receive2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> Receive3R {
        Receive3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> Receive4R {
        Receive4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> Receive5R {
        Receive5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> Receive6R {
        Receive6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> Receive7R {
        Receive7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive0(&mut self) -> Receive0W<IntenSpec> {
        Receive0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive1(&mut self) -> Receive1W<IntenSpec> {
        Receive1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive2(&mut self) -> Receive2W<IntenSpec> {
        Receive2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive3(&mut self) -> Receive3W<IntenSpec> {
        Receive3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive4(&mut self) -> Receive4W<IntenSpec> {
        Receive4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive5(&mut self) -> Receive5W<IntenSpec> {
        Receive5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive6(&mut self) -> Receive6W<IntenSpec> {
        Receive6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn receive7(&mut self) -> Receive7W<IntenSpec> {
        Receive7W::new(self, 7)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}

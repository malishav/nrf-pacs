#[doc = "Register `INTPEND` reader"]
pub type R = crate::R<IntpendSpec>;
#[doc = "Read pending status of interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive0 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive0> for bool {
    #[inline(always)]
    fn from(variant: Receive0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE0` reader - Read pending status of interrupt for event RECEIVE\\[0\\]"]
pub type Receive0R = crate::BitReader<Receive0>;
impl Receive0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive0 {
        match self.bits {
            false => Receive0::NotPending,
            true => Receive0::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive0::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive0::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive1 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive1> for bool {
    #[inline(always)]
    fn from(variant: Receive1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE1` reader - Read pending status of interrupt for event RECEIVE\\[1\\]"]
pub type Receive1R = crate::BitReader<Receive1>;
impl Receive1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive1 {
        match self.bits {
            false => Receive1::NotPending,
            true => Receive1::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive1::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive1::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive2 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive2> for bool {
    #[inline(always)]
    fn from(variant: Receive2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE2` reader - Read pending status of interrupt for event RECEIVE\\[2\\]"]
pub type Receive2R = crate::BitReader<Receive2>;
impl Receive2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive2 {
        match self.bits {
            false => Receive2::NotPending,
            true => Receive2::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive2::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive2::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive3 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive3> for bool {
    #[inline(always)]
    fn from(variant: Receive3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE3` reader - Read pending status of interrupt for event RECEIVE\\[3\\]"]
pub type Receive3R = crate::BitReader<Receive3>;
impl Receive3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive3 {
        match self.bits {
            false => Receive3::NotPending,
            true => Receive3::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive3::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive3::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive4 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive4> for bool {
    #[inline(always)]
    fn from(variant: Receive4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE4` reader - Read pending status of interrupt for event RECEIVE\\[4\\]"]
pub type Receive4R = crate::BitReader<Receive4>;
impl Receive4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive4 {
        match self.bits {
            false => Receive4::NotPending,
            true => Receive4::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive4::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive4::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive5 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive5> for bool {
    #[inline(always)]
    fn from(variant: Receive5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE5` reader - Read pending status of interrupt for event RECEIVE\\[5\\]"]
pub type Receive5R = crate::BitReader<Receive5>;
impl Receive5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive5 {
        match self.bits {
            false => Receive5::NotPending,
            true => Receive5::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive5::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive5::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive6 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive6> for bool {
    #[inline(always)]
    fn from(variant: Receive6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE6` reader - Read pending status of interrupt for event RECEIVE\\[6\\]"]
pub type Receive6R = crate::BitReader<Receive6>;
impl Receive6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive6 {
        match self.bits {
            false => Receive6::NotPending,
            true => Receive6::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive6::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive6::Pending
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receive7 {
    #[doc = "0: Read: Not pending"]
    NotPending = 0,
    #[doc = "1: Read: Pending"]
    Pending = 1,
}
impl From<Receive7> for bool {
    #[inline(always)]
    fn from(variant: Receive7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RECEIVE7` reader - Read pending status of interrupt for event RECEIVE\\[7\\]"]
pub type Receive7R = crate::BitReader<Receive7>;
impl Receive7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Receive7 {
        match self.bits {
            false => Receive7::NotPending,
            true => Receive7::Pending,
        }
    }
    #[doc = "Read: Not pending"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == Receive7::NotPending
    }
    #[doc = "Read: Pending"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Receive7::Pending
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> Receive0R {
        Receive0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> Receive1R {
        Receive1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> Receive2R {
        Receive2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> Receive3R {
        Receive3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> Receive4R {
        Receive4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> Receive5R {
        Receive5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> Receive6R {
        Receive6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> Receive7R {
        Receive7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Pending interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpendSpec;
impl crate::RegisterSpec for IntpendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpend::R`](R) reader structure"]
impl crate::Readable for IntpendSpec {}
#[doc = "`reset()` method sets INTPEND to value 0"]
impl crate::Resettable for IntpendSpec {
    const RESET_VALUE: u32 = 0;
}

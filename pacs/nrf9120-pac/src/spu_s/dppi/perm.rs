#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel0 {
    #[doc = "1: Channel0 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel0 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel0> for bool {
    #[inline(always)]
    fn from(variant: Channel0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL0` reader - Select secure attribute."]
pub type Channel0R = crate::BitReader<Channel0>;
impl Channel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel0 {
        match self.bits {
            true => Channel0::Secure,
            false => Channel0::NonSecure,
        }
    }
    #[doc = "Channel0 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel0::Secure
    }
    #[doc = "Channel0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel0::NonSecure
    }
}
#[doc = "Field `CHANNEL0` writer - Select secure attribute."]
pub type Channel0W<'a, REG> = crate::BitWriter<'a, REG, Channel0>;
impl<'a, REG> Channel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel0::Secure)
    }
    #[doc = "Channel0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel0::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel1 {
    #[doc = "1: Channel1 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel1 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel1> for bool {
    #[inline(always)]
    fn from(variant: Channel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL1` reader - Select secure attribute."]
pub type Channel1R = crate::BitReader<Channel1>;
impl Channel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel1 {
        match self.bits {
            true => Channel1::Secure,
            false => Channel1::NonSecure,
        }
    }
    #[doc = "Channel1 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel1::Secure
    }
    #[doc = "Channel1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel1::NonSecure
    }
}
#[doc = "Field `CHANNEL1` writer - Select secure attribute."]
pub type Channel1W<'a, REG> = crate::BitWriter<'a, REG, Channel1>;
impl<'a, REG> Channel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel1::Secure)
    }
    #[doc = "Channel1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel1::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel2 {
    #[doc = "1: Channel2 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel2 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel2> for bool {
    #[inline(always)]
    fn from(variant: Channel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL2` reader - Select secure attribute."]
pub type Channel2R = crate::BitReader<Channel2>;
impl Channel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel2 {
        match self.bits {
            true => Channel2::Secure,
            false => Channel2::NonSecure,
        }
    }
    #[doc = "Channel2 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel2::Secure
    }
    #[doc = "Channel2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel2::NonSecure
    }
}
#[doc = "Field `CHANNEL2` writer - Select secure attribute."]
pub type Channel2W<'a, REG> = crate::BitWriter<'a, REG, Channel2>;
impl<'a, REG> Channel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel2::Secure)
    }
    #[doc = "Channel2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel2::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel3 {
    #[doc = "1: Channel3 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel3 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel3> for bool {
    #[inline(always)]
    fn from(variant: Channel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL3` reader - Select secure attribute."]
pub type Channel3R = crate::BitReader<Channel3>;
impl Channel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel3 {
        match self.bits {
            true => Channel3::Secure,
            false => Channel3::NonSecure,
        }
    }
    #[doc = "Channel3 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel3::Secure
    }
    #[doc = "Channel3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel3::NonSecure
    }
}
#[doc = "Field `CHANNEL3` writer - Select secure attribute."]
pub type Channel3W<'a, REG> = crate::BitWriter<'a, REG, Channel3>;
impl<'a, REG> Channel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel3::Secure)
    }
    #[doc = "Channel3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel3::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel4 {
    #[doc = "1: Channel4 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel4 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel4> for bool {
    #[inline(always)]
    fn from(variant: Channel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL4` reader - Select secure attribute."]
pub type Channel4R = crate::BitReader<Channel4>;
impl Channel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel4 {
        match self.bits {
            true => Channel4::Secure,
            false => Channel4::NonSecure,
        }
    }
    #[doc = "Channel4 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel4::Secure
    }
    #[doc = "Channel4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel4::NonSecure
    }
}
#[doc = "Field `CHANNEL4` writer - Select secure attribute."]
pub type Channel4W<'a, REG> = crate::BitWriter<'a, REG, Channel4>;
impl<'a, REG> Channel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel4::Secure)
    }
    #[doc = "Channel4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel4::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel5 {
    #[doc = "1: Channel5 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel5 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel5> for bool {
    #[inline(always)]
    fn from(variant: Channel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL5` reader - Select secure attribute."]
pub type Channel5R = crate::BitReader<Channel5>;
impl Channel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel5 {
        match self.bits {
            true => Channel5::Secure,
            false => Channel5::NonSecure,
        }
    }
    #[doc = "Channel5 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel5::Secure
    }
    #[doc = "Channel5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel5::NonSecure
    }
}
#[doc = "Field `CHANNEL5` writer - Select secure attribute."]
pub type Channel5W<'a, REG> = crate::BitWriter<'a, REG, Channel5>;
impl<'a, REG> Channel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel5::Secure)
    }
    #[doc = "Channel5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel5::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel6 {
    #[doc = "1: Channel6 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel6 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel6> for bool {
    #[inline(always)]
    fn from(variant: Channel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL6` reader - Select secure attribute."]
pub type Channel6R = crate::BitReader<Channel6>;
impl Channel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel6 {
        match self.bits {
            true => Channel6::Secure,
            false => Channel6::NonSecure,
        }
    }
    #[doc = "Channel6 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel6::Secure
    }
    #[doc = "Channel6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel6::NonSecure
    }
}
#[doc = "Field `CHANNEL6` writer - Select secure attribute."]
pub type Channel6W<'a, REG> = crate::BitWriter<'a, REG, Channel6>;
impl<'a, REG> Channel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel6::Secure)
    }
    #[doc = "Channel6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel6::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel7 {
    #[doc = "1: Channel7 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel7 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel7> for bool {
    #[inline(always)]
    fn from(variant: Channel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL7` reader - Select secure attribute."]
pub type Channel7R = crate::BitReader<Channel7>;
impl Channel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel7 {
        match self.bits {
            true => Channel7::Secure,
            false => Channel7::NonSecure,
        }
    }
    #[doc = "Channel7 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel7::Secure
    }
    #[doc = "Channel7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel7::NonSecure
    }
}
#[doc = "Field `CHANNEL7` writer - Select secure attribute."]
pub type Channel7W<'a, REG> = crate::BitWriter<'a, REG, Channel7>;
impl<'a, REG> Channel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel7::Secure)
    }
    #[doc = "Channel7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel7::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel8 {
    #[doc = "1: Channel8 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel8 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel8> for bool {
    #[inline(always)]
    fn from(variant: Channel8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL8` reader - Select secure attribute."]
pub type Channel8R = crate::BitReader<Channel8>;
impl Channel8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel8 {
        match self.bits {
            true => Channel8::Secure,
            false => Channel8::NonSecure,
        }
    }
    #[doc = "Channel8 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel8::Secure
    }
    #[doc = "Channel8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel8::NonSecure
    }
}
#[doc = "Field `CHANNEL8` writer - Select secure attribute."]
pub type Channel8W<'a, REG> = crate::BitWriter<'a, REG, Channel8>;
impl<'a, REG> Channel8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel8::Secure)
    }
    #[doc = "Channel8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel8::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel9 {
    #[doc = "1: Channel9 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel9 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel9> for bool {
    #[inline(always)]
    fn from(variant: Channel9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL9` reader - Select secure attribute."]
pub type Channel9R = crate::BitReader<Channel9>;
impl Channel9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel9 {
        match self.bits {
            true => Channel9::Secure,
            false => Channel9::NonSecure,
        }
    }
    #[doc = "Channel9 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel9::Secure
    }
    #[doc = "Channel9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel9::NonSecure
    }
}
#[doc = "Field `CHANNEL9` writer - Select secure attribute."]
pub type Channel9W<'a, REG> = crate::BitWriter<'a, REG, Channel9>;
impl<'a, REG> Channel9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel9::Secure)
    }
    #[doc = "Channel9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel9::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel10 {
    #[doc = "1: Channel10 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel10 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel10> for bool {
    #[inline(always)]
    fn from(variant: Channel10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL10` reader - Select secure attribute."]
pub type Channel10R = crate::BitReader<Channel10>;
impl Channel10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel10 {
        match self.bits {
            true => Channel10::Secure,
            false => Channel10::NonSecure,
        }
    }
    #[doc = "Channel10 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel10::Secure
    }
    #[doc = "Channel10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel10::NonSecure
    }
}
#[doc = "Field `CHANNEL10` writer - Select secure attribute."]
pub type Channel10W<'a, REG> = crate::BitWriter<'a, REG, Channel10>;
impl<'a, REG> Channel10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel10::Secure)
    }
    #[doc = "Channel10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel10::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel11 {
    #[doc = "1: Channel11 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel11 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel11> for bool {
    #[inline(always)]
    fn from(variant: Channel11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL11` reader - Select secure attribute."]
pub type Channel11R = crate::BitReader<Channel11>;
impl Channel11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel11 {
        match self.bits {
            true => Channel11::Secure,
            false => Channel11::NonSecure,
        }
    }
    #[doc = "Channel11 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel11::Secure
    }
    #[doc = "Channel11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel11::NonSecure
    }
}
#[doc = "Field `CHANNEL11` writer - Select secure attribute."]
pub type Channel11W<'a, REG> = crate::BitWriter<'a, REG, Channel11>;
impl<'a, REG> Channel11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel11::Secure)
    }
    #[doc = "Channel11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel11::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel12 {
    #[doc = "1: Channel12 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel12 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel12> for bool {
    #[inline(always)]
    fn from(variant: Channel12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL12` reader - Select secure attribute."]
pub type Channel12R = crate::BitReader<Channel12>;
impl Channel12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel12 {
        match self.bits {
            true => Channel12::Secure,
            false => Channel12::NonSecure,
        }
    }
    #[doc = "Channel12 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel12::Secure
    }
    #[doc = "Channel12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel12::NonSecure
    }
}
#[doc = "Field `CHANNEL12` writer - Select secure attribute."]
pub type Channel12W<'a, REG> = crate::BitWriter<'a, REG, Channel12>;
impl<'a, REG> Channel12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel12::Secure)
    }
    #[doc = "Channel12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel12::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel13 {
    #[doc = "1: Channel13 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel13 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel13> for bool {
    #[inline(always)]
    fn from(variant: Channel13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL13` reader - Select secure attribute."]
pub type Channel13R = crate::BitReader<Channel13>;
impl Channel13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel13 {
        match self.bits {
            true => Channel13::Secure,
            false => Channel13::NonSecure,
        }
    }
    #[doc = "Channel13 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel13::Secure
    }
    #[doc = "Channel13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel13::NonSecure
    }
}
#[doc = "Field `CHANNEL13` writer - Select secure attribute."]
pub type Channel13W<'a, REG> = crate::BitWriter<'a, REG, Channel13>;
impl<'a, REG> Channel13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel13::Secure)
    }
    #[doc = "Channel13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel13::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel14 {
    #[doc = "1: Channel14 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel14 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel14> for bool {
    #[inline(always)]
    fn from(variant: Channel14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL14` reader - Select secure attribute."]
pub type Channel14R = crate::BitReader<Channel14>;
impl Channel14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel14 {
        match self.bits {
            true => Channel14::Secure,
            false => Channel14::NonSecure,
        }
    }
    #[doc = "Channel14 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel14::Secure
    }
    #[doc = "Channel14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel14::NonSecure
    }
}
#[doc = "Field `CHANNEL14` writer - Select secure attribute."]
pub type Channel14W<'a, REG> = crate::BitWriter<'a, REG, Channel14>;
impl<'a, REG> Channel14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel14::Secure)
    }
    #[doc = "Channel14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel14::NonSecure)
    }
}
#[doc = "Select secure attribute.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel15 {
    #[doc = "1: Channel15 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Channel15 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Channel15> for bool {
    #[inline(always)]
    fn from(variant: Channel15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHANNEL15` reader - Select secure attribute."]
pub type Channel15R = crate::BitReader<Channel15>;
impl Channel15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Channel15 {
        match self.bits {
            true => Channel15::Secure,
            false => Channel15::NonSecure,
        }
    }
    #[doc = "Channel15 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Channel15::Secure
    }
    #[doc = "Channel15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Channel15::NonSecure
    }
}
#[doc = "Field `CHANNEL15` writer - Select secure attribute."]
pub type Channel15W<'a, REG> = crate::BitWriter<'a, REG, Channel15>;
impl<'a, REG> Channel15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel15::Secure)
    }
    #[doc = "Channel15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Channel15::NonSecure)
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline(always)]
    pub fn channel0(&self) -> Channel0R {
        Channel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline(always)]
    pub fn channel1(&self) -> Channel1R {
        Channel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline(always)]
    pub fn channel2(&self) -> Channel2R {
        Channel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline(always)]
    pub fn channel3(&self) -> Channel3R {
        Channel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline(always)]
    pub fn channel4(&self) -> Channel4R {
        Channel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline(always)]
    pub fn channel5(&self) -> Channel5R {
        Channel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline(always)]
    pub fn channel6(&self) -> Channel6R {
        Channel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline(always)]
    pub fn channel7(&self) -> Channel7R {
        Channel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline(always)]
    pub fn channel8(&self) -> Channel8R {
        Channel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline(always)]
    pub fn channel9(&self) -> Channel9R {
        Channel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline(always)]
    pub fn channel10(&self) -> Channel10R {
        Channel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline(always)]
    pub fn channel11(&self) -> Channel11R {
        Channel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline(always)]
    pub fn channel12(&self) -> Channel12R {
        Channel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline(always)]
    pub fn channel13(&self) -> Channel13R {
        Channel13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline(always)]
    pub fn channel14(&self) -> Channel14R {
        Channel14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline(always)]
    pub fn channel15(&self) -> Channel15R {
        Channel15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> Channel0W<PermSpec> {
        Channel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> Channel1W<PermSpec> {
        Channel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> Channel2W<PermSpec> {
        Channel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> Channel3W<PermSpec> {
        Channel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel4(&mut self) -> Channel4W<PermSpec> {
        Channel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel5(&mut self) -> Channel5W<PermSpec> {
        Channel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel6(&mut self) -> Channel6W<PermSpec> {
        Channel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel7(&mut self) -> Channel7W<PermSpec> {
        Channel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel8(&mut self) -> Channel8W<PermSpec> {
        Channel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel9(&mut self) -> Channel9W<PermSpec> {
        Channel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel10(&mut self) -> Channel10W<PermSpec> {
        Channel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel11(&mut self) -> Channel11W<PermSpec> {
        Channel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel12(&mut self) -> Channel12W<PermSpec> {
        Channel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel13(&mut self) -> Channel13W<PermSpec> {
        Channel13W::new(self, 13)
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel14(&mut self) -> Channel14W<PermSpec> {
        Channel14W::new(self, 14)
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline(always)]
    #[must_use]
    pub fn channel15(&mut self) -> Channel15W<PermSpec> {
        Channel15W::new(self, 15)
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PERM to value 0xffff"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0xffff;
}

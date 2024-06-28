#[doc = "Register `PERM` reader"]
pub type R = crate::R<PermSpec>;
#[doc = "Register `PERM` writer"]
pub type W = crate::W<PermSpec>;
#[doc = "Select secure attribute attribute for PIN 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin0 {
    #[doc = "1: Pin 0 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 0 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin0> for bool {
    #[inline(always)]
    fn from(variant: Pin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN0` reader - Select secure attribute attribute for PIN 0."]
pub type Pin0R = crate::BitReader<Pin0>;
impl Pin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin0 {
        match self.bits {
            true => Pin0::Secure,
            false => Pin0::NonSecure,
        }
    }
    #[doc = "Pin 0 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin0::Secure
    }
    #[doc = "Pin 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin0::NonSecure
    }
}
#[doc = "Field `PIN0` writer - Select secure attribute attribute for PIN 0."]
pub type Pin0W<'a, REG> = crate::BitWriter<'a, REG, Pin0>;
impl<'a, REG> Pin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 0 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::Secure)
    }
    #[doc = "Pin 0 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin0::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin1 {
    #[doc = "1: Pin 1 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 1 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin1> for bool {
    #[inline(always)]
    fn from(variant: Pin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN1` reader - Select secure attribute attribute for PIN 1."]
pub type Pin1R = crate::BitReader<Pin1>;
impl Pin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin1 {
        match self.bits {
            true => Pin1::Secure,
            false => Pin1::NonSecure,
        }
    }
    #[doc = "Pin 1 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin1::Secure
    }
    #[doc = "Pin 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin1::NonSecure
    }
}
#[doc = "Field `PIN1` writer - Select secure attribute attribute for PIN 1."]
pub type Pin1W<'a, REG> = crate::BitWriter<'a, REG, Pin1>;
impl<'a, REG> Pin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 1 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::Secure)
    }
    #[doc = "Pin 1 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin1::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin2 {
    #[doc = "1: Pin 2 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 2 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin2> for bool {
    #[inline(always)]
    fn from(variant: Pin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN2` reader - Select secure attribute attribute for PIN 2."]
pub type Pin2R = crate::BitReader<Pin2>;
impl Pin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin2 {
        match self.bits {
            true => Pin2::Secure,
            false => Pin2::NonSecure,
        }
    }
    #[doc = "Pin 2 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin2::Secure
    }
    #[doc = "Pin 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin2::NonSecure
    }
}
#[doc = "Field `PIN2` writer - Select secure attribute attribute for PIN 2."]
pub type Pin2W<'a, REG> = crate::BitWriter<'a, REG, Pin2>;
impl<'a, REG> Pin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 2 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::Secure)
    }
    #[doc = "Pin 2 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin2::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin3 {
    #[doc = "1: Pin 3 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 3 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin3> for bool {
    #[inline(always)]
    fn from(variant: Pin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN3` reader - Select secure attribute attribute for PIN 3."]
pub type Pin3R = crate::BitReader<Pin3>;
impl Pin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin3 {
        match self.bits {
            true => Pin3::Secure,
            false => Pin3::NonSecure,
        }
    }
    #[doc = "Pin 3 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin3::Secure
    }
    #[doc = "Pin 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin3::NonSecure
    }
}
#[doc = "Field `PIN3` writer - Select secure attribute attribute for PIN 3."]
pub type Pin3W<'a, REG> = crate::BitWriter<'a, REG, Pin3>;
impl<'a, REG> Pin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 3 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::Secure)
    }
    #[doc = "Pin 3 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin3::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin4 {
    #[doc = "1: Pin 4 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 4 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin4> for bool {
    #[inline(always)]
    fn from(variant: Pin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN4` reader - Select secure attribute attribute for PIN 4."]
pub type Pin4R = crate::BitReader<Pin4>;
impl Pin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin4 {
        match self.bits {
            true => Pin4::Secure,
            false => Pin4::NonSecure,
        }
    }
    #[doc = "Pin 4 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin4::Secure
    }
    #[doc = "Pin 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin4::NonSecure
    }
}
#[doc = "Field `PIN4` writer - Select secure attribute attribute for PIN 4."]
pub type Pin4W<'a, REG> = crate::BitWriter<'a, REG, Pin4>;
impl<'a, REG> Pin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 4 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::Secure)
    }
    #[doc = "Pin 4 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin4::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin5 {
    #[doc = "1: Pin 5 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 5 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin5> for bool {
    #[inline(always)]
    fn from(variant: Pin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN5` reader - Select secure attribute attribute for PIN 5."]
pub type Pin5R = crate::BitReader<Pin5>;
impl Pin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin5 {
        match self.bits {
            true => Pin5::Secure,
            false => Pin5::NonSecure,
        }
    }
    #[doc = "Pin 5 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin5::Secure
    }
    #[doc = "Pin 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin5::NonSecure
    }
}
#[doc = "Field `PIN5` writer - Select secure attribute attribute for PIN 5."]
pub type Pin5W<'a, REG> = crate::BitWriter<'a, REG, Pin5>;
impl<'a, REG> Pin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 5 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::Secure)
    }
    #[doc = "Pin 5 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin5::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin6 {
    #[doc = "1: Pin 6 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 6 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin6> for bool {
    #[inline(always)]
    fn from(variant: Pin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN6` reader - Select secure attribute attribute for PIN 6."]
pub type Pin6R = crate::BitReader<Pin6>;
impl Pin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin6 {
        match self.bits {
            true => Pin6::Secure,
            false => Pin6::NonSecure,
        }
    }
    #[doc = "Pin 6 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin6::Secure
    }
    #[doc = "Pin 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin6::NonSecure
    }
}
#[doc = "Field `PIN6` writer - Select secure attribute attribute for PIN 6."]
pub type Pin6W<'a, REG> = crate::BitWriter<'a, REG, Pin6>;
impl<'a, REG> Pin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 6 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::Secure)
    }
    #[doc = "Pin 6 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin6::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin7 {
    #[doc = "1: Pin 7 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 7 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin7> for bool {
    #[inline(always)]
    fn from(variant: Pin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN7` reader - Select secure attribute attribute for PIN 7."]
pub type Pin7R = crate::BitReader<Pin7>;
impl Pin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin7 {
        match self.bits {
            true => Pin7::Secure,
            false => Pin7::NonSecure,
        }
    }
    #[doc = "Pin 7 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin7::Secure
    }
    #[doc = "Pin 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin7::NonSecure
    }
}
#[doc = "Field `PIN7` writer - Select secure attribute attribute for PIN 7."]
pub type Pin7W<'a, REG> = crate::BitWriter<'a, REG, Pin7>;
impl<'a, REG> Pin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 7 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::Secure)
    }
    #[doc = "Pin 7 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin7::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin8 {
    #[doc = "1: Pin 8 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 8 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin8> for bool {
    #[inline(always)]
    fn from(variant: Pin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN8` reader - Select secure attribute attribute for PIN 8."]
pub type Pin8R = crate::BitReader<Pin8>;
impl Pin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin8 {
        match self.bits {
            true => Pin8::Secure,
            false => Pin8::NonSecure,
        }
    }
    #[doc = "Pin 8 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin8::Secure
    }
    #[doc = "Pin 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin8::NonSecure
    }
}
#[doc = "Field `PIN8` writer - Select secure attribute attribute for PIN 8."]
pub type Pin8W<'a, REG> = crate::BitWriter<'a, REG, Pin8>;
impl<'a, REG> Pin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 8 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::Secure)
    }
    #[doc = "Pin 8 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin8::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin9 {
    #[doc = "1: Pin 9 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 9 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin9> for bool {
    #[inline(always)]
    fn from(variant: Pin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN9` reader - Select secure attribute attribute for PIN 9."]
pub type Pin9R = crate::BitReader<Pin9>;
impl Pin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin9 {
        match self.bits {
            true => Pin9::Secure,
            false => Pin9::NonSecure,
        }
    }
    #[doc = "Pin 9 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin9::Secure
    }
    #[doc = "Pin 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin9::NonSecure
    }
}
#[doc = "Field `PIN9` writer - Select secure attribute attribute for PIN 9."]
pub type Pin9W<'a, REG> = crate::BitWriter<'a, REG, Pin9>;
impl<'a, REG> Pin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 9 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::Secure)
    }
    #[doc = "Pin 9 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin9::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin10 {
    #[doc = "1: Pin 10 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 10 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin10> for bool {
    #[inline(always)]
    fn from(variant: Pin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN10` reader - Select secure attribute attribute for PIN 10."]
pub type Pin10R = crate::BitReader<Pin10>;
impl Pin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin10 {
        match self.bits {
            true => Pin10::Secure,
            false => Pin10::NonSecure,
        }
    }
    #[doc = "Pin 10 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin10::Secure
    }
    #[doc = "Pin 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin10::NonSecure
    }
}
#[doc = "Field `PIN10` writer - Select secure attribute attribute for PIN 10."]
pub type Pin10W<'a, REG> = crate::BitWriter<'a, REG, Pin10>;
impl<'a, REG> Pin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 10 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::Secure)
    }
    #[doc = "Pin 10 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin10::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin11 {
    #[doc = "1: Pin 11 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 11 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin11> for bool {
    #[inline(always)]
    fn from(variant: Pin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN11` reader - Select secure attribute attribute for PIN 11."]
pub type Pin11R = crate::BitReader<Pin11>;
impl Pin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin11 {
        match self.bits {
            true => Pin11::Secure,
            false => Pin11::NonSecure,
        }
    }
    #[doc = "Pin 11 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin11::Secure
    }
    #[doc = "Pin 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin11::NonSecure
    }
}
#[doc = "Field `PIN11` writer - Select secure attribute attribute for PIN 11."]
pub type Pin11W<'a, REG> = crate::BitWriter<'a, REG, Pin11>;
impl<'a, REG> Pin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 11 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::Secure)
    }
    #[doc = "Pin 11 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin11::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin12 {
    #[doc = "1: Pin 12 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 12 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin12> for bool {
    #[inline(always)]
    fn from(variant: Pin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN12` reader - Select secure attribute attribute for PIN 12."]
pub type Pin12R = crate::BitReader<Pin12>;
impl Pin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin12 {
        match self.bits {
            true => Pin12::Secure,
            false => Pin12::NonSecure,
        }
    }
    #[doc = "Pin 12 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin12::Secure
    }
    #[doc = "Pin 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin12::NonSecure
    }
}
#[doc = "Field `PIN12` writer - Select secure attribute attribute for PIN 12."]
pub type Pin12W<'a, REG> = crate::BitWriter<'a, REG, Pin12>;
impl<'a, REG> Pin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 12 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::Secure)
    }
    #[doc = "Pin 12 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin12::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin13 {
    #[doc = "1: Pin 13 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 13 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin13> for bool {
    #[inline(always)]
    fn from(variant: Pin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN13` reader - Select secure attribute attribute for PIN 13."]
pub type Pin13R = crate::BitReader<Pin13>;
impl Pin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin13 {
        match self.bits {
            true => Pin13::Secure,
            false => Pin13::NonSecure,
        }
    }
    #[doc = "Pin 13 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin13::Secure
    }
    #[doc = "Pin 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin13::NonSecure
    }
}
#[doc = "Field `PIN13` writer - Select secure attribute attribute for PIN 13."]
pub type Pin13W<'a, REG> = crate::BitWriter<'a, REG, Pin13>;
impl<'a, REG> Pin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 13 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::Secure)
    }
    #[doc = "Pin 13 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin13::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin14 {
    #[doc = "1: Pin 14 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 14 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin14> for bool {
    #[inline(always)]
    fn from(variant: Pin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN14` reader - Select secure attribute attribute for PIN 14."]
pub type Pin14R = crate::BitReader<Pin14>;
impl Pin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin14 {
        match self.bits {
            true => Pin14::Secure,
            false => Pin14::NonSecure,
        }
    }
    #[doc = "Pin 14 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin14::Secure
    }
    #[doc = "Pin 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin14::NonSecure
    }
}
#[doc = "Field `PIN14` writer - Select secure attribute attribute for PIN 14."]
pub type Pin14W<'a, REG> = crate::BitWriter<'a, REG, Pin14>;
impl<'a, REG> Pin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 14 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::Secure)
    }
    #[doc = "Pin 14 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin14::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin15 {
    #[doc = "1: Pin 15 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 15 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin15> for bool {
    #[inline(always)]
    fn from(variant: Pin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN15` reader - Select secure attribute attribute for PIN 15."]
pub type Pin15R = crate::BitReader<Pin15>;
impl Pin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin15 {
        match self.bits {
            true => Pin15::Secure,
            false => Pin15::NonSecure,
        }
    }
    #[doc = "Pin 15 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin15::Secure
    }
    #[doc = "Pin 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin15::NonSecure
    }
}
#[doc = "Field `PIN15` writer - Select secure attribute attribute for PIN 15."]
pub type Pin15W<'a, REG> = crate::BitWriter<'a, REG, Pin15>;
impl<'a, REG> Pin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 15 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::Secure)
    }
    #[doc = "Pin 15 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin15::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin16 {
    #[doc = "1: Pin 16 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 16 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin16> for bool {
    #[inline(always)]
    fn from(variant: Pin16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN16` reader - Select secure attribute attribute for PIN 16."]
pub type Pin16R = crate::BitReader<Pin16>;
impl Pin16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin16 {
        match self.bits {
            true => Pin16::Secure,
            false => Pin16::NonSecure,
        }
    }
    #[doc = "Pin 16 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin16::Secure
    }
    #[doc = "Pin 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin16::NonSecure
    }
}
#[doc = "Field `PIN16` writer - Select secure attribute attribute for PIN 16."]
pub type Pin16W<'a, REG> = crate::BitWriter<'a, REG, Pin16>;
impl<'a, REG> Pin16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 16 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16::Secure)
    }
    #[doc = "Pin 16 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin16::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin17 {
    #[doc = "1: Pin 17 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 17 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin17> for bool {
    #[inline(always)]
    fn from(variant: Pin17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN17` reader - Select secure attribute attribute for PIN 17."]
pub type Pin17R = crate::BitReader<Pin17>;
impl Pin17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin17 {
        match self.bits {
            true => Pin17::Secure,
            false => Pin17::NonSecure,
        }
    }
    #[doc = "Pin 17 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin17::Secure
    }
    #[doc = "Pin 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin17::NonSecure
    }
}
#[doc = "Field `PIN17` writer - Select secure attribute attribute for PIN 17."]
pub type Pin17W<'a, REG> = crate::BitWriter<'a, REG, Pin17>;
impl<'a, REG> Pin17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 17 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17::Secure)
    }
    #[doc = "Pin 17 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin17::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin18 {
    #[doc = "1: Pin 18 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 18 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin18> for bool {
    #[inline(always)]
    fn from(variant: Pin18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN18` reader - Select secure attribute attribute for PIN 18."]
pub type Pin18R = crate::BitReader<Pin18>;
impl Pin18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin18 {
        match self.bits {
            true => Pin18::Secure,
            false => Pin18::NonSecure,
        }
    }
    #[doc = "Pin 18 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin18::Secure
    }
    #[doc = "Pin 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin18::NonSecure
    }
}
#[doc = "Field `PIN18` writer - Select secure attribute attribute for PIN 18."]
pub type Pin18W<'a, REG> = crate::BitWriter<'a, REG, Pin18>;
impl<'a, REG> Pin18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 18 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18::Secure)
    }
    #[doc = "Pin 18 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin18::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin19 {
    #[doc = "1: Pin 19 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 19 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin19> for bool {
    #[inline(always)]
    fn from(variant: Pin19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN19` reader - Select secure attribute attribute for PIN 19."]
pub type Pin19R = crate::BitReader<Pin19>;
impl Pin19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin19 {
        match self.bits {
            true => Pin19::Secure,
            false => Pin19::NonSecure,
        }
    }
    #[doc = "Pin 19 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin19::Secure
    }
    #[doc = "Pin 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin19::NonSecure
    }
}
#[doc = "Field `PIN19` writer - Select secure attribute attribute for PIN 19."]
pub type Pin19W<'a, REG> = crate::BitWriter<'a, REG, Pin19>;
impl<'a, REG> Pin19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 19 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19::Secure)
    }
    #[doc = "Pin 19 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin19::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin20 {
    #[doc = "1: Pin 20 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 20 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin20> for bool {
    #[inline(always)]
    fn from(variant: Pin20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN20` reader - Select secure attribute attribute for PIN 20."]
pub type Pin20R = crate::BitReader<Pin20>;
impl Pin20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin20 {
        match self.bits {
            true => Pin20::Secure,
            false => Pin20::NonSecure,
        }
    }
    #[doc = "Pin 20 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin20::Secure
    }
    #[doc = "Pin 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin20::NonSecure
    }
}
#[doc = "Field `PIN20` writer - Select secure attribute attribute for PIN 20."]
pub type Pin20W<'a, REG> = crate::BitWriter<'a, REG, Pin20>;
impl<'a, REG> Pin20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 20 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20::Secure)
    }
    #[doc = "Pin 20 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin20::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin21 {
    #[doc = "1: Pin 21 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 21 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin21> for bool {
    #[inline(always)]
    fn from(variant: Pin21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN21` reader - Select secure attribute attribute for PIN 21."]
pub type Pin21R = crate::BitReader<Pin21>;
impl Pin21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin21 {
        match self.bits {
            true => Pin21::Secure,
            false => Pin21::NonSecure,
        }
    }
    #[doc = "Pin 21 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin21::Secure
    }
    #[doc = "Pin 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin21::NonSecure
    }
}
#[doc = "Field `PIN21` writer - Select secure attribute attribute for PIN 21."]
pub type Pin21W<'a, REG> = crate::BitWriter<'a, REG, Pin21>;
impl<'a, REG> Pin21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 21 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21::Secure)
    }
    #[doc = "Pin 21 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin21::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin22 {
    #[doc = "1: Pin 22 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 22 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin22> for bool {
    #[inline(always)]
    fn from(variant: Pin22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN22` reader - Select secure attribute attribute for PIN 22."]
pub type Pin22R = crate::BitReader<Pin22>;
impl Pin22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin22 {
        match self.bits {
            true => Pin22::Secure,
            false => Pin22::NonSecure,
        }
    }
    #[doc = "Pin 22 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin22::Secure
    }
    #[doc = "Pin 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin22::NonSecure
    }
}
#[doc = "Field `PIN22` writer - Select secure attribute attribute for PIN 22."]
pub type Pin22W<'a, REG> = crate::BitWriter<'a, REG, Pin22>;
impl<'a, REG> Pin22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 22 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22::Secure)
    }
    #[doc = "Pin 22 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin22::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin23 {
    #[doc = "1: Pin 23 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 23 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin23> for bool {
    #[inline(always)]
    fn from(variant: Pin23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN23` reader - Select secure attribute attribute for PIN 23."]
pub type Pin23R = crate::BitReader<Pin23>;
impl Pin23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin23 {
        match self.bits {
            true => Pin23::Secure,
            false => Pin23::NonSecure,
        }
    }
    #[doc = "Pin 23 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin23::Secure
    }
    #[doc = "Pin 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin23::NonSecure
    }
}
#[doc = "Field `PIN23` writer - Select secure attribute attribute for PIN 23."]
pub type Pin23W<'a, REG> = crate::BitWriter<'a, REG, Pin23>;
impl<'a, REG> Pin23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 23 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23::Secure)
    }
    #[doc = "Pin 23 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin23::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin24 {
    #[doc = "1: Pin 24 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 24 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin24> for bool {
    #[inline(always)]
    fn from(variant: Pin24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN24` reader - Select secure attribute attribute for PIN 24."]
pub type Pin24R = crate::BitReader<Pin24>;
impl Pin24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin24 {
        match self.bits {
            true => Pin24::Secure,
            false => Pin24::NonSecure,
        }
    }
    #[doc = "Pin 24 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin24::Secure
    }
    #[doc = "Pin 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin24::NonSecure
    }
}
#[doc = "Field `PIN24` writer - Select secure attribute attribute for PIN 24."]
pub type Pin24W<'a, REG> = crate::BitWriter<'a, REG, Pin24>;
impl<'a, REG> Pin24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 24 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24::Secure)
    }
    #[doc = "Pin 24 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin24::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin25 {
    #[doc = "1: Pin 25 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 25 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin25> for bool {
    #[inline(always)]
    fn from(variant: Pin25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN25` reader - Select secure attribute attribute for PIN 25."]
pub type Pin25R = crate::BitReader<Pin25>;
impl Pin25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin25 {
        match self.bits {
            true => Pin25::Secure,
            false => Pin25::NonSecure,
        }
    }
    #[doc = "Pin 25 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin25::Secure
    }
    #[doc = "Pin 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin25::NonSecure
    }
}
#[doc = "Field `PIN25` writer - Select secure attribute attribute for PIN 25."]
pub type Pin25W<'a, REG> = crate::BitWriter<'a, REG, Pin25>;
impl<'a, REG> Pin25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 25 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25::Secure)
    }
    #[doc = "Pin 25 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin25::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin26 {
    #[doc = "1: Pin 26 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 26 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin26> for bool {
    #[inline(always)]
    fn from(variant: Pin26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN26` reader - Select secure attribute attribute for PIN 26."]
pub type Pin26R = crate::BitReader<Pin26>;
impl Pin26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin26 {
        match self.bits {
            true => Pin26::Secure,
            false => Pin26::NonSecure,
        }
    }
    #[doc = "Pin 26 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin26::Secure
    }
    #[doc = "Pin 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin26::NonSecure
    }
}
#[doc = "Field `PIN26` writer - Select secure attribute attribute for PIN 26."]
pub type Pin26W<'a, REG> = crate::BitWriter<'a, REG, Pin26>;
impl<'a, REG> Pin26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 26 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26::Secure)
    }
    #[doc = "Pin 26 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin26::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin27 {
    #[doc = "1: Pin 27 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 27 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin27> for bool {
    #[inline(always)]
    fn from(variant: Pin27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN27` reader - Select secure attribute attribute for PIN 27."]
pub type Pin27R = crate::BitReader<Pin27>;
impl Pin27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin27 {
        match self.bits {
            true => Pin27::Secure,
            false => Pin27::NonSecure,
        }
    }
    #[doc = "Pin 27 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin27::Secure
    }
    #[doc = "Pin 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin27::NonSecure
    }
}
#[doc = "Field `PIN27` writer - Select secure attribute attribute for PIN 27."]
pub type Pin27W<'a, REG> = crate::BitWriter<'a, REG, Pin27>;
impl<'a, REG> Pin27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 27 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27::Secure)
    }
    #[doc = "Pin 27 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin27::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin28 {
    #[doc = "1: Pin 28 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 28 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin28> for bool {
    #[inline(always)]
    fn from(variant: Pin28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN28` reader - Select secure attribute attribute for PIN 28."]
pub type Pin28R = crate::BitReader<Pin28>;
impl Pin28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin28 {
        match self.bits {
            true => Pin28::Secure,
            false => Pin28::NonSecure,
        }
    }
    #[doc = "Pin 28 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin28::Secure
    }
    #[doc = "Pin 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin28::NonSecure
    }
}
#[doc = "Field `PIN28` writer - Select secure attribute attribute for PIN 28."]
pub type Pin28W<'a, REG> = crate::BitWriter<'a, REG, Pin28>;
impl<'a, REG> Pin28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 28 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28::Secure)
    }
    #[doc = "Pin 28 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin28::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 29.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin29 {
    #[doc = "1: Pin 29 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 29 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin29> for bool {
    #[inline(always)]
    fn from(variant: Pin29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN29` reader - Select secure attribute attribute for PIN 29."]
pub type Pin29R = crate::BitReader<Pin29>;
impl Pin29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin29 {
        match self.bits {
            true => Pin29::Secure,
            false => Pin29::NonSecure,
        }
    }
    #[doc = "Pin 29 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin29::Secure
    }
    #[doc = "Pin 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin29::NonSecure
    }
}
#[doc = "Field `PIN29` writer - Select secure attribute attribute for PIN 29."]
pub type Pin29W<'a, REG> = crate::BitWriter<'a, REG, Pin29>;
impl<'a, REG> Pin29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 29 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29::Secure)
    }
    #[doc = "Pin 29 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin29::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 30.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin30 {
    #[doc = "1: Pin 30 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 30 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin30> for bool {
    #[inline(always)]
    fn from(variant: Pin30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN30` reader - Select secure attribute attribute for PIN 30."]
pub type Pin30R = crate::BitReader<Pin30>;
impl Pin30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin30 {
        match self.bits {
            true => Pin30::Secure,
            false => Pin30::NonSecure,
        }
    }
    #[doc = "Pin 30 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin30::Secure
    }
    #[doc = "Pin 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin30::NonSecure
    }
}
#[doc = "Field `PIN30` writer - Select secure attribute attribute for PIN 30."]
pub type Pin30W<'a, REG> = crate::BitWriter<'a, REG, Pin30>;
impl<'a, REG> Pin30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 30 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30::Secure)
    }
    #[doc = "Pin 30 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin30::NonSecure)
    }
}
#[doc = "Select secure attribute attribute for PIN 31.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin31 {
    #[doc = "1: Pin 31 has its secure attribute set"]
    Secure = 1,
    #[doc = "0: Pin 31 has its non-secure attribute set"]
    NonSecure = 0,
}
impl From<Pin31> for bool {
    #[inline(always)]
    fn from(variant: Pin31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN31` reader - Select secure attribute attribute for PIN 31."]
pub type Pin31R = crate::BitReader<Pin31>;
impl Pin31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin31 {
        match self.bits {
            true => Pin31::Secure,
            false => Pin31::NonSecure,
        }
    }
    #[doc = "Pin 31 has its secure attribute set"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Pin31::Secure
    }
    #[doc = "Pin 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Pin31::NonSecure
    }
}
#[doc = "Field `PIN31` writer - Select secure attribute attribute for PIN 31."]
pub type Pin31W<'a, REG> = crate::BitWriter<'a, REG, Pin31>;
impl<'a, REG> Pin31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin 31 has its secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31::Secure)
    }
    #[doc = "Pin 31 has its non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Pin31::NonSecure)
    }
}
impl R {
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    pub fn pin0(&self) -> Pin0R {
        Pin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    pub fn pin1(&self) -> Pin1R {
        Pin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    pub fn pin2(&self) -> Pin2R {
        Pin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    pub fn pin3(&self) -> Pin3R {
        Pin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    pub fn pin4(&self) -> Pin4R {
        Pin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    pub fn pin5(&self) -> Pin5R {
        Pin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    pub fn pin6(&self) -> Pin6R {
        Pin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    pub fn pin7(&self) -> Pin7R {
        Pin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    pub fn pin8(&self) -> Pin8R {
        Pin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    pub fn pin9(&self) -> Pin9R {
        Pin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    pub fn pin10(&self) -> Pin10R {
        Pin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    pub fn pin11(&self) -> Pin11R {
        Pin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    pub fn pin12(&self) -> Pin12R {
        Pin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    pub fn pin13(&self) -> Pin13R {
        Pin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    pub fn pin14(&self) -> Pin14R {
        Pin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    pub fn pin15(&self) -> Pin15R {
        Pin15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    pub fn pin16(&self) -> Pin16R {
        Pin16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    pub fn pin17(&self) -> Pin17R {
        Pin17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    pub fn pin18(&self) -> Pin18R {
        Pin18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    pub fn pin19(&self) -> Pin19R {
        Pin19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    pub fn pin20(&self) -> Pin20R {
        Pin20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    pub fn pin21(&self) -> Pin21R {
        Pin21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    pub fn pin22(&self) -> Pin22R {
        Pin22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    pub fn pin23(&self) -> Pin23R {
        Pin23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    pub fn pin24(&self) -> Pin24R {
        Pin24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    pub fn pin25(&self) -> Pin25R {
        Pin25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    pub fn pin26(&self) -> Pin26R {
        Pin26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    pub fn pin27(&self) -> Pin27R {
        Pin27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    pub fn pin28(&self) -> Pin28R {
        Pin28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    pub fn pin29(&self) -> Pin29R {
        Pin29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    pub fn pin30(&self) -> Pin30R {
        Pin30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    pub fn pin31(&self) -> Pin31R {
        Pin31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> Pin0W<PermSpec> {
        Pin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> Pin1W<PermSpec> {
        Pin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> Pin2W<PermSpec> {
        Pin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> Pin3W<PermSpec> {
        Pin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> Pin4W<PermSpec> {
        Pin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> Pin5W<PermSpec> {
        Pin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> Pin6W<PermSpec> {
        Pin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> Pin7W<PermSpec> {
        Pin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> Pin8W<PermSpec> {
        Pin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> Pin9W<PermSpec> {
        Pin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> Pin10W<PermSpec> {
        Pin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> Pin11W<PermSpec> {
        Pin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> Pin12W<PermSpec> {
        Pin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> Pin13W<PermSpec> {
        Pin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> Pin14W<PermSpec> {
        Pin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> Pin15W<PermSpec> {
        Pin15W::new(self, 15)
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline(always)]
    #[must_use]
    pub fn pin16(&mut self) -> Pin16W<PermSpec> {
        Pin16W::new(self, 16)
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline(always)]
    #[must_use]
    pub fn pin17(&mut self) -> Pin17W<PermSpec> {
        Pin17W::new(self, 17)
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline(always)]
    #[must_use]
    pub fn pin18(&mut self) -> Pin18W<PermSpec> {
        Pin18W::new(self, 18)
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline(always)]
    #[must_use]
    pub fn pin19(&mut self) -> Pin19W<PermSpec> {
        Pin19W::new(self, 19)
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline(always)]
    #[must_use]
    pub fn pin20(&mut self) -> Pin20W<PermSpec> {
        Pin20W::new(self, 20)
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline(always)]
    #[must_use]
    pub fn pin21(&mut self) -> Pin21W<PermSpec> {
        Pin21W::new(self, 21)
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline(always)]
    #[must_use]
    pub fn pin22(&mut self) -> Pin22W<PermSpec> {
        Pin22W::new(self, 22)
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline(always)]
    #[must_use]
    pub fn pin23(&mut self) -> Pin23W<PermSpec> {
        Pin23W::new(self, 23)
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline(always)]
    #[must_use]
    pub fn pin24(&mut self) -> Pin24W<PermSpec> {
        Pin24W::new(self, 24)
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline(always)]
    #[must_use]
    pub fn pin25(&mut self) -> Pin25W<PermSpec> {
        Pin25W::new(self, 25)
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline(always)]
    #[must_use]
    pub fn pin26(&mut self) -> Pin26W<PermSpec> {
        Pin26W::new(self, 26)
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline(always)]
    #[must_use]
    pub fn pin27(&mut self) -> Pin27W<PermSpec> {
        Pin27W::new(self, 27)
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline(always)]
    #[must_use]
    pub fn pin28(&mut self) -> Pin28W<PermSpec> {
        Pin28W::new(self, 28)
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline(always)]
    #[must_use]
    pub fn pin29(&mut self) -> Pin29W<PermSpec> {
        Pin29W::new(self, 29)
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline(always)]
    #[must_use]
    pub fn pin30(&mut self) -> Pin30W<PermSpec> {
        Pin30W::new(self, 30)
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline(always)]
    #[must_use]
    pub fn pin31(&mut self) -> Pin31W<PermSpec> {
        Pin31W::new(self, 31)
    }
}
#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PERM to value 0xffff_ffff"]
impl crate::Resettable for PermSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

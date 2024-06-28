#[doc = "Register `CHEN` reader"]
pub type R = crate::R<ChenSpec>;
#[doc = "Register `CHEN` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Enable or disable channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Enable or disable channel 0"]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Disabled,
            true => Ch0::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0::Enabled
    }
}
#[doc = "Field `CH0` writer - Enable or disable channel 0"]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Enabled)
    }
}
#[doc = "Enable or disable channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch1> for bool {
    #[inline(always)]
    fn from(variant: Ch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Enable or disable channel 1"]
pub type Ch1R = crate::BitReader<Ch1>;
impl Ch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1 {
        match self.bits {
            false => Ch1::Disabled,
            true => Ch1::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1::Enabled
    }
}
#[doc = "Field `CH1` writer - Enable or disable channel 1"]
pub type Ch1W<'a, REG> = crate::BitWriter<'a, REG, Ch1>;
impl<'a, REG> Ch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1::Enabled)
    }
}
#[doc = "Enable or disable channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch2> for bool {
    #[inline(always)]
    fn from(variant: Ch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Enable or disable channel 2"]
pub type Ch2R = crate::BitReader<Ch2>;
impl Ch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2 {
        match self.bits {
            false => Ch2::Disabled,
            true => Ch2::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2::Enabled
    }
}
#[doc = "Field `CH2` writer - Enable or disable channel 2"]
pub type Ch2W<'a, REG> = crate::BitWriter<'a, REG, Ch2>;
impl<'a, REG> Ch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2::Enabled)
    }
}
#[doc = "Enable or disable channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch3> for bool {
    #[inline(always)]
    fn from(variant: Ch3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Enable or disable channel 3"]
pub type Ch3R = crate::BitReader<Ch3>;
impl Ch3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3 {
        match self.bits {
            false => Ch3::Disabled,
            true => Ch3::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3::Enabled
    }
}
#[doc = "Field `CH3` writer - Enable or disable channel 3"]
pub type Ch3W<'a, REG> = crate::BitWriter<'a, REG, Ch3>;
impl<'a, REG> Ch3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3::Enabled)
    }
}
#[doc = "Enable or disable channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch4> for bool {
    #[inline(always)]
    fn from(variant: Ch4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Enable or disable channel 4"]
pub type Ch4R = crate::BitReader<Ch4>;
impl Ch4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4 {
        match self.bits {
            false => Ch4::Disabled,
            true => Ch4::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch4::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch4::Enabled
    }
}
#[doc = "Field `CH4` writer - Enable or disable channel 4"]
pub type Ch4W<'a, REG> = crate::BitWriter<'a, REG, Ch4>;
impl<'a, REG> Ch4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4::Enabled)
    }
}
#[doc = "Enable or disable channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch5> for bool {
    #[inline(always)]
    fn from(variant: Ch5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Enable or disable channel 5"]
pub type Ch5R = crate::BitReader<Ch5>;
impl Ch5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5 {
        match self.bits {
            false => Ch5::Disabled,
            true => Ch5::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch5::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch5::Enabled
    }
}
#[doc = "Field `CH5` writer - Enable or disable channel 5"]
pub type Ch5W<'a, REG> = crate::BitWriter<'a, REG, Ch5>;
impl<'a, REG> Ch5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5::Enabled)
    }
}
#[doc = "Enable or disable channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch6> for bool {
    #[inline(always)]
    fn from(variant: Ch6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Enable or disable channel 6"]
pub type Ch6R = crate::BitReader<Ch6>;
impl Ch6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6 {
        match self.bits {
            false => Ch6::Disabled,
            true => Ch6::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch6::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch6::Enabled
    }
}
#[doc = "Field `CH6` writer - Enable or disable channel 6"]
pub type Ch6W<'a, REG> = crate::BitWriter<'a, REG, Ch6>;
impl<'a, REG> Ch6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6::Enabled)
    }
}
#[doc = "Enable or disable channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch7> for bool {
    #[inline(always)]
    fn from(variant: Ch7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Enable or disable channel 7"]
pub type Ch7R = crate::BitReader<Ch7>;
impl Ch7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7 {
        match self.bits {
            false => Ch7::Disabled,
            true => Ch7::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch7::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch7::Enabled
    }
}
#[doc = "Field `CH7` writer - Enable or disable channel 7"]
pub type Ch7W<'a, REG> = crate::BitWriter<'a, REG, Ch7>;
impl<'a, REG> Ch7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7::Enabled)
    }
}
#[doc = "Enable or disable channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch8 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch8> for bool {
    #[inline(always)]
    fn from(variant: Ch8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` reader - Enable or disable channel 8"]
pub type Ch8R = crate::BitReader<Ch8>;
impl Ch8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch8 {
        match self.bits {
            false => Ch8::Disabled,
            true => Ch8::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch8::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch8::Enabled
    }
}
#[doc = "Field `CH8` writer - Enable or disable channel 8"]
pub type Ch8W<'a, REG> = crate::BitWriter<'a, REG, Ch8>;
impl<'a, REG> Ch8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch8::Enabled)
    }
}
#[doc = "Enable or disable channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch9 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch9> for bool {
    #[inline(always)]
    fn from(variant: Ch9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` reader - Enable or disable channel 9"]
pub type Ch9R = crate::BitReader<Ch9>;
impl Ch9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch9 {
        match self.bits {
            false => Ch9::Disabled,
            true => Ch9::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch9::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch9::Enabled
    }
}
#[doc = "Field `CH9` writer - Enable or disable channel 9"]
pub type Ch9W<'a, REG> = crate::BitWriter<'a, REG, Ch9>;
impl<'a, REG> Ch9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch9::Enabled)
    }
}
#[doc = "Enable or disable channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch10 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch10> for bool {
    #[inline(always)]
    fn from(variant: Ch10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` reader - Enable or disable channel 10"]
pub type Ch10R = crate::BitReader<Ch10>;
impl Ch10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch10 {
        match self.bits {
            false => Ch10::Disabled,
            true => Ch10::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch10::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch10::Enabled
    }
}
#[doc = "Field `CH10` writer - Enable or disable channel 10"]
pub type Ch10W<'a, REG> = crate::BitWriter<'a, REG, Ch10>;
impl<'a, REG> Ch10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch10::Enabled)
    }
}
#[doc = "Enable or disable channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch11 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch11> for bool {
    #[inline(always)]
    fn from(variant: Ch11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` reader - Enable or disable channel 11"]
pub type Ch11R = crate::BitReader<Ch11>;
impl Ch11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch11 {
        match self.bits {
            false => Ch11::Disabled,
            true => Ch11::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch11::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch11::Enabled
    }
}
#[doc = "Field `CH11` writer - Enable or disable channel 11"]
pub type Ch11W<'a, REG> = crate::BitWriter<'a, REG, Ch11>;
impl<'a, REG> Ch11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch11::Enabled)
    }
}
#[doc = "Enable or disable channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch12 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch12> for bool {
    #[inline(always)]
    fn from(variant: Ch12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` reader - Enable or disable channel 12"]
pub type Ch12R = crate::BitReader<Ch12>;
impl Ch12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch12 {
        match self.bits {
            false => Ch12::Disabled,
            true => Ch12::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch12::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch12::Enabled
    }
}
#[doc = "Field `CH12` writer - Enable or disable channel 12"]
pub type Ch12W<'a, REG> = crate::BitWriter<'a, REG, Ch12>;
impl<'a, REG> Ch12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch12::Enabled)
    }
}
#[doc = "Enable or disable channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch13 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch13> for bool {
    #[inline(always)]
    fn from(variant: Ch13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` reader - Enable or disable channel 13"]
pub type Ch13R = crate::BitReader<Ch13>;
impl Ch13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch13 {
        match self.bits {
            false => Ch13::Disabled,
            true => Ch13::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch13::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch13::Enabled
    }
}
#[doc = "Field `CH13` writer - Enable or disable channel 13"]
pub type Ch13W<'a, REG> = crate::BitWriter<'a, REG, Ch13>;
impl<'a, REG> Ch13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch13::Enabled)
    }
}
#[doc = "Enable or disable channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch14 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch14> for bool {
    #[inline(always)]
    fn from(variant: Ch14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` reader - Enable or disable channel 14"]
pub type Ch14R = crate::BitReader<Ch14>;
impl Ch14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch14 {
        match self.bits {
            false => Ch14::Disabled,
            true => Ch14::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch14::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch14::Enabled
    }
}
#[doc = "Field `CH14` writer - Enable or disable channel 14"]
pub type Ch14W<'a, REG> = crate::BitWriter<'a, REG, Ch14>;
impl<'a, REG> Ch14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch14::Enabled)
    }
}
#[doc = "Enable or disable channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch15 {
    #[doc = "0: Disable channel"]
    Disabled = 0,
    #[doc = "1: Enable channel"]
    Enabled = 1,
}
impl From<Ch15> for bool {
    #[inline(always)]
    fn from(variant: Ch15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` reader - Enable or disable channel 15"]
pub type Ch15R = crate::BitReader<Ch15>;
impl Ch15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch15 {
        match self.bits {
            false => Ch15::Disabled,
            true => Ch15::Enabled,
        }
    }
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch15::Disabled
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch15::Enabled
    }
}
#[doc = "Field `CH15` writer - Enable or disable channel 15"]
pub type Ch15W<'a, REG> = crate::BitWriter<'a, REG, Ch15>;
impl<'a, REG> Ch15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Disabled)
    }
    #[doc = "Enable channel"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch15::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable channel 8"]
    #[inline(always)]
    pub fn ch8(&self) -> Ch8R {
        Ch8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable channel 9"]
    #[inline(always)]
    pub fn ch9(&self) -> Ch9R {
        Ch9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable channel 10"]
    #[inline(always)]
    pub fn ch10(&self) -> Ch10R {
        Ch10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable channel 11"]
    #[inline(always)]
    pub fn ch11(&self) -> Ch11R {
        Ch11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable channel 12"]
    #[inline(always)]
    pub fn ch12(&self) -> Ch12R {
        Ch12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable channel 13"]
    #[inline(always)]
    pub fn ch13(&self) -> Ch13R {
        Ch13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable channel 14"]
    #[inline(always)]
    pub fn ch14(&self) -> Ch14R {
        Ch14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable channel 15"]
    #[inline(always)]
    pub fn ch15(&self) -> Ch15R {
        Ch15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> Ch0W<ChenSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> Ch1W<ChenSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> Ch2W<ChenSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> Ch3W<ChenSpec> {
        Ch3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> Ch4W<ChenSpec> {
        Ch4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> Ch5W<ChenSpec> {
        Ch5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> Ch6W<ChenSpec> {
        Ch6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> Ch7W<ChenSpec> {
        Ch7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> Ch8W<ChenSpec> {
        Ch8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> Ch9W<ChenSpec> {
        Ch9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable or disable channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> Ch10W<ChenSpec> {
        Ch10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable or disable channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> Ch11W<ChenSpec> {
        Ch11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable or disable channel 12"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> Ch12W<ChenSpec> {
        Ch12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable or disable channel 13"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> Ch13W<ChenSpec> {
        Ch13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable or disable channel 14"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> Ch14W<ChenSpec> {
        Ch14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable or disable channel 15"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> Ch15W<ChenSpec> {
        Ch15W::new(self, 15)
    }
}
#[doc = "Channel enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for ChenSpec {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for ChenSpec {
    const RESET_VALUE: u32 = 0;
}

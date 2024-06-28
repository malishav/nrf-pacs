#[doc = "Register `TRACEDATA0` reader"]
pub type R = crate::R<Tracedata0Spec>;
#[doc = "Register `TRACEDATA0` writer"]
pub type W = crate::W<Tracedata0Spec>;
#[doc = "Pin number\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin {
    #[doc = "22: TRACEDATA0 pin"]
    Tracedata0 = 22,
}
impl From<Pin> for u8 {
    #[inline(always)]
    fn from(variant: Pin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pin {
    type Ux = u8;
}
impl crate::IsEnum for Pin {}
#[doc = "Field `PIN` reader - Pin number"]
pub type PinR = crate::FieldReader<Pin>;
impl PinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pin> {
        match self.bits {
            22 => Some(Pin::Tracedata0),
            _ => None,
        }
    }
    #[doc = "TRACEDATA0 pin"]
    #[inline(always)]
    pub fn is_tracedata0(&self) -> bool {
        *self == Pin::Tracedata0
    }
}
#[doc = "Field `PIN` writer - Pin number"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pin>;
impl<'a, REG> PinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TRACEDATA0 pin"]
    #[inline(always)]
    pub fn tracedata0(self) -> &'a mut crate::W<REG> {
        self.variant(Pin::Tracedata0)
    }
}
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connect {
    #[doc = "1: Disconnect"]
    Disconnected = 1,
    #[doc = "0: Connect"]
    Connected = 0,
}
impl From<Connect> for bool {
    #[inline(always)]
    fn from(variant: Connect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECT` reader - Connection"]
pub type ConnectR = crate::BitReader<Connect>;
impl ConnectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connect {
        match self.bits {
            true => Connect::Disconnected,
            false => Connect::Connected,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Connect::Disconnected
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Connect::Connected
    }
}
#[doc = "Field `CONNECT` writer - Connection"]
pub type ConnectW<'a, REG> = crate::BitWriter<'a, REG, Connect>;
impl<'a, REG> ConnectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Disconnected)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Connect::Connected)
    }
}
impl R {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> ConnectR {
        ConnectR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<Tracedata0Spec> {
        PinW::new(self, 0)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    #[must_use]
    pub fn connect(&mut self) -> ConnectW<Tracedata0Spec> {
        ConnectW::new(self, 31)
    }
}
#[doc = "Pin configuration for TRACEDATA\\[0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracedata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracedata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tracedata0Spec;
impl crate::RegisterSpec for Tracedata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tracedata0::R`](R) reader structure"]
impl crate::Readable for Tracedata0Spec {}
#[doc = "`write(|w| ..)` method takes [`tracedata0::W`](W) writer structure"]
impl crate::Writable for Tracedata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACEDATA0 to value 0xffff_ffff"]
impl crate::Resettable for Tracedata0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

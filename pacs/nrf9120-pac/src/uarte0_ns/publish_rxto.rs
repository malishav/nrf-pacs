#[doc = "Register `PUBLISH_RXTO` reader"]
pub type R = crate::R<PublishRxtoSpec>;
#[doc = "Register `PUBLISH_RXTO` writer"]
pub type W = crate::W<PublishRxtoSpec>;
#[doc = "Field `CHIDX` reader - DPPI channel that event RXTO will publish to"]
pub type ChidxR = crate::FieldReader;
#[doc = "Field `CHIDX` writer - DPPI channel that event RXTO will publish to"]
pub type ChidxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable publishing"]
    Disabled = 0,
    #[doc = "1: Enable publishing"]
    Enabled = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enabled,
        }
    }
    #[doc = "Disable publishing"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Enable publishing"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == En::Enabled
    }
}
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable publishing"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Enable publishing"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - DPPI channel that event RXTO will publish to"]
    #[inline(always)]
    pub fn chidx(&self) -> ChidxR {
        ChidxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DPPI channel that event RXTO will publish to"]
    #[inline(always)]
    #[must_use]
    pub fn chidx(&mut self) -> ChidxW<PublishRxtoSpec> {
        ChidxW::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<PublishRxtoSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Publish configuration for event RXTO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_rxto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_rxto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PublishRxtoSpec;
impl crate::RegisterSpec for PublishRxtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`publish_rxto::R`](R) reader structure"]
impl crate::Readable for PublishRxtoSpec {}
#[doc = "`write(|w| ..)` method takes [`publish_rxto::W`](W) writer structure"]
impl crate::Writable for PublishRxtoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUBLISH_RXTO to value 0"]
impl crate::Resettable for PublishRxtoSpec {
    const RESET_VALUE: u32 = 0;
}

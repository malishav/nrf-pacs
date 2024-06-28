#[doc = "Register `EVENTS_KEYSLOT_REVOKED` reader"]
pub type R = crate::R<EventsKeyslotRevokedSpec>;
#[doc = "Register `EVENTS_KEYSLOT_REVOKED` writer"]
pub type W = crate::W<EventsKeyslotRevokedSpec>;
#[doc = "Key slot has been revoked and cannot be tasked for selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsKeyslotRevoked {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsKeyslotRevoked> for bool {
    #[inline(always)]
    fn from(variant: EventsKeyslotRevoked) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_KEYSLOT_REVOKED` reader - Key slot has been revoked and cannot be tasked for selection"]
pub type EventsKeyslotRevokedR = crate::BitReader<EventsKeyslotRevoked>;
impl EventsKeyslotRevokedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsKeyslotRevoked {
        match self.bits {
            false => EventsKeyslotRevoked::NotGenerated,
            true => EventsKeyslotRevoked::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsKeyslotRevoked::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsKeyslotRevoked::Generated
    }
}
#[doc = "Field `EVENTS_KEYSLOT_REVOKED` writer - Key slot has been revoked and cannot be tasked for selection"]
pub type EventsKeyslotRevokedW<'a, REG> = crate::BitWriter<'a, REG, EventsKeyslotRevoked>;
impl<'a, REG> EventsKeyslotRevokedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsKeyslotRevoked::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsKeyslotRevoked::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Key slot has been revoked and cannot be tasked for selection"]
    #[inline(always)]
    pub fn events_keyslot_revoked(&self) -> EventsKeyslotRevokedR {
        EventsKeyslotRevokedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key slot has been revoked and cannot be tasked for selection"]
    #[inline(always)]
    #[must_use]
    pub fn events_keyslot_revoked(&mut self) -> EventsKeyslotRevokedW<EventsKeyslotRevokedSpec> {
        EventsKeyslotRevokedW::new(self, 0)
    }
}
#[doc = "Key slot has been revoked and cannot be tasked for selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_keyslot_revoked::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_keyslot_revoked::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsKeyslotRevokedSpec;
impl crate::RegisterSpec for EventsKeyslotRevokedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_keyslot_revoked::R`](R) reader structure"]
impl crate::Readable for EventsKeyslotRevokedSpec {}
#[doc = "`write(|w| ..)` method takes [`events_keyslot_revoked::W`](W) writer structure"]
impl crate::Writable for EventsKeyslotRevokedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_KEYSLOT_REVOKED to value 0"]
impl crate::Resettable for EventsKeyslotRevokedSpec {
    const RESET_VALUE: u32 = 0;
}

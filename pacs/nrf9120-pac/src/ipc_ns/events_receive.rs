#[doc = "Register `EVENTS_RECEIVE[%s]` reader"]
pub type R = crate::R<EventsReceiveSpec>;
#[doc = "Register `EVENTS_RECEIVE[%s]` writer"]
pub type W = crate::W<EventsReceiveSpec>;
#[doc = "Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsReceive {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsReceive> for bool {
    #[inline(always)]
    fn from(variant: EventsReceive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_RECEIVE` reader - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
pub type EventsReceiveR = crate::BitReader<EventsReceive>;
impl EventsReceiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsReceive {
        match self.bits {
            false => EventsReceive::NotGenerated,
            true => EventsReceive::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsReceive::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsReceive::Generated
    }
}
#[doc = "Field `EVENTS_RECEIVE` writer - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
pub type EventsReceiveW<'a, REG> = crate::BitWriter<'a, REG, EventsReceive>;
impl<'a, REG> EventsReceiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReceive::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsReceive::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    pub fn events_receive(&self) -> EventsReceiveR {
        EventsReceiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    #[must_use]
    pub fn events_receive(&mut self) -> EventsReceiveW<EventsReceiveSpec> {
        EventsReceiveW::new(self, 0)
    }
}
#[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_receive::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_receive::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsReceiveSpec;
impl crate::RegisterSpec for EventsReceiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_receive::R`](R) reader structure"]
impl crate::Readable for EventsReceiveSpec {}
#[doc = "`write(|w| ..)` method takes [`events_receive::W`](W) writer structure"]
impl crate::Writable for EventsReceiveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_RECEIVE[%s]
to value 0"]
impl crate::Resettable for EventsReceiveSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `EVENTS_KEYSLOT_ERROR` reader"]
pub type R = crate::R<EventsKeyslotErrorSpec>;
#[doc = "Register `EVENTS_KEYSLOT_ERROR` writer"]
pub type W = crate::W<EventsKeyslotErrorSpec>;
#[doc = "No key slot selected, no destination address defined, or error during push operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsKeyslotError {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsKeyslotError> for bool {
    #[inline(always)]
    fn from(variant: EventsKeyslotError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_KEYSLOT_ERROR` reader - No key slot selected, no destination address defined, or error during push operation"]
pub type EventsKeyslotErrorR = crate::BitReader<EventsKeyslotError>;
impl EventsKeyslotErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsKeyslotError {
        match self.bits {
            false => EventsKeyslotError::NotGenerated,
            true => EventsKeyslotError::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsKeyslotError::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsKeyslotError::Generated
    }
}
#[doc = "Field `EVENTS_KEYSLOT_ERROR` writer - No key slot selected, no destination address defined, or error during push operation"]
pub type EventsKeyslotErrorW<'a, REG> = crate::BitWriter<'a, REG, EventsKeyslotError>;
impl<'a, REG> EventsKeyslotErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsKeyslotError::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsKeyslotError::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - No key slot selected, no destination address defined, or error during push operation"]
    #[inline(always)]
    pub fn events_keyslot_error(&self) -> EventsKeyslotErrorR {
        EventsKeyslotErrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No key slot selected, no destination address defined, or error during push operation"]
    #[inline(always)]
    #[must_use]
    pub fn events_keyslot_error(&mut self) -> EventsKeyslotErrorW<EventsKeyslotErrorSpec> {
        EventsKeyslotErrorW::new(self, 0)
    }
}
#[doc = "No key slot selected, no destination address defined, or error during push operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_keyslot_error::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_keyslot_error::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsKeyslotErrorSpec;
impl crate::RegisterSpec for EventsKeyslotErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_keyslot_error::R`](R) reader structure"]
impl crate::Readable for EventsKeyslotErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`events_keyslot_error::W`](W) writer structure"]
impl crate::Writable for EventsKeyslotErrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_KEYSLOT_ERROR to value 0"]
impl crate::Resettable for EventsKeyslotErrorSpec {
    const RESET_VALUE: u32 = 0;
}

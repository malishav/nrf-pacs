#[doc = "Register `EVENTS_FLASHACCERR` reader"]
pub type R = crate::R<EventsFlashaccerrSpec>;
#[doc = "Register `EVENTS_FLASHACCERR` writer"]
pub type W = crate::W<EventsFlashaccerrSpec>;
#[doc = "A security violation has been detected for the flash memory space\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsFlashaccerr {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsFlashaccerr> for bool {
    #[inline(always)]
    fn from(variant: EventsFlashaccerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_FLASHACCERR` reader - A security violation has been detected for the flash memory space"]
pub type EventsFlashaccerrR = crate::BitReader<EventsFlashaccerr>;
impl EventsFlashaccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsFlashaccerr {
        match self.bits {
            false => EventsFlashaccerr::NotGenerated,
            true => EventsFlashaccerr::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsFlashaccerr::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsFlashaccerr::Generated
    }
}
#[doc = "Field `EVENTS_FLASHACCERR` writer - A security violation has been detected for the flash memory space"]
pub type EventsFlashaccerrW<'a, REG> = crate::BitWriter<'a, REG, EventsFlashaccerr>;
impl<'a, REG> EventsFlashaccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFlashaccerr::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsFlashaccerr::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - A security violation has been detected for the flash memory space"]
    #[inline(always)]
    pub fn events_flashaccerr(&self) -> EventsFlashaccerrR {
        EventsFlashaccerrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A security violation has been detected for the flash memory space"]
    #[inline(always)]
    #[must_use]
    pub fn events_flashaccerr(&mut self) -> EventsFlashaccerrW<EventsFlashaccerrSpec> {
        EventsFlashaccerrW::new(self, 0)
    }
}
#[doc = "A security violation has been detected for the flash memory space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_flashaccerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_flashaccerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsFlashaccerrSpec;
impl crate::RegisterSpec for EventsFlashaccerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_flashaccerr::R`](R) reader structure"]
impl crate::Readable for EventsFlashaccerrSpec {}
#[doc = "`write(|w| ..)` method takes [`events_flashaccerr::W`](W) writer structure"]
impl crate::Writable for EventsFlashaccerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_FLASHACCERR to value 0"]
impl crate::Resettable for EventsFlashaccerrSpec {
    const RESET_VALUE: u32 = 0;
}

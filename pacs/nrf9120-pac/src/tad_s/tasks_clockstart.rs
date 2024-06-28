#[doc = "Register `TASKS_CLOCKSTART` writer"]
pub type W = crate::W<TasksClockstartSpec>;
#[doc = "Start all trace and debug clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksClockstart {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksClockstart> for bool {
    #[inline(always)]
    fn from(variant: TasksClockstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CLOCKSTART` writer - Start all trace and debug clocks."]
pub type TasksClockstartW<'a, REG> = crate::BitWriter<'a, REG, TasksClockstart>;
impl<'a, REG> TasksClockstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksClockstart::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Start all trace and debug clocks."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_clockstart(&mut self) -> TasksClockstartW<TasksClockstartSpec> {
        TasksClockstartW::new(self, 0)
    }
}
#[doc = "Start all trace and debug clocks.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_clockstart::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClockstartSpec;
impl crate::RegisterSpec for TasksClockstartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clockstart::W`](W) writer structure"]
impl crate::Writable for TasksClockstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CLOCKSTART to value 0"]
impl crate::Resettable for TasksClockstartSpec {
    const RESET_VALUE: u32 = 0;
}

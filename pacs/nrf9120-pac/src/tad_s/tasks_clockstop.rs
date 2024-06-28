#[doc = "Register `TASKS_CLOCKSTOP` writer"]
pub type W = crate::W<TasksClockstopSpec>;
#[doc = "Stop all trace and debug clocks.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksClockstop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksClockstop> for bool {
    #[inline(always)]
    fn from(variant: TasksClockstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CLOCKSTOP` writer - Stop all trace and debug clocks."]
pub type TasksClockstopW<'a, REG> = crate::BitWriter<'a, REG, TasksClockstop>;
impl<'a, REG> TasksClockstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksClockstop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop all trace and debug clocks."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_clockstop(&mut self) -> TasksClockstopW<TasksClockstopSpec> {
        TasksClockstopW::new(self, 0)
    }
}
#[doc = "Stop all trace and debug clocks.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_clockstop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClockstopSpec;
impl crate::RegisterSpec for TasksClockstopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clockstop::W`](W) writer structure"]
impl crate::Writable for TasksClockstopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CLOCKSTOP to value 0"]
impl crate::Resettable for TasksClockstopSpec {
    const RESET_VALUE: u32 = 0;
}

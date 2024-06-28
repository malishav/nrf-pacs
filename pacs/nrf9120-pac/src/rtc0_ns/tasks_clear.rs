#[doc = "Register `TASKS_CLEAR` writer"]
pub type W = crate::W<TasksClearSpec>;
#[doc = "Clear RTC counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksClear {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksClear> for bool {
    #[inline(always)]
    fn from(variant: TasksClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_CLEAR` writer - Clear RTC counter"]
pub type TasksClearW<'a, REG> = crate::BitWriter<'a, REG, TasksClear>;
impl<'a, REG> TasksClearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksClear::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Clear RTC counter"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_clear(&mut self) -> TasksClearW<TasksClearSpec> {
        TasksClearW::new(self, 0)
    }
}
#[doc = "Clear RTC counter\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksClearSpec;
impl crate::RegisterSpec for TasksClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_clear::W`](W) writer structure"]
impl crate::Writable for TasksClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_CLEAR to value 0"]
impl crate::Resettable for TasksClearSpec {
    const RESET_VALUE: u32 = 0;
}

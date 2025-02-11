#[doc = "Register `TPRCR` reader"]
pub type R = crate::R<TprcrSpec>;
#[doc = "Register `TPRCR` writer"]
pub type W = crate::W<TprcrSpec>;
#[doc = "Field `PATTCOUNT` reader - 8-bit counter value to indicate the number of traceclkin cycles for which a pattern runs before it switches to the next pattern."]
pub type PattcountR = crate::FieldReader;
#[doc = "Field `PATTCOUNT` writer - 8-bit counter value to indicate the number of traceclkin cycles for which a pattern runs before it switches to the next pattern."]
pub type PattcountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8-bit counter value to indicate the number of traceclkin cycles for which a pattern runs before it switches to the next pattern."]
    #[inline(always)]
    pub fn pattcount(&self) -> PattcountR {
        PattcountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8-bit counter value to indicate the number of traceclkin cycles for which a pattern runs before it switches to the next pattern."]
    #[inline(always)]
    #[must_use]
    pub fn pattcount(&mut self) -> PattcountW<TprcrSpec> {
        PattcountW::new(self, 0)
    }
}
#[doc = "The TPRCR register is an 8-bit counter start value that is decremented. A write sets the initial counter value and a read returns the programmed value.\n\nYou can [`read`](crate::Reg::read) this register and get [`tprcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tprcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TprcrSpec;
impl crate::RegisterSpec for TprcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tprcr::R`](R) reader structure"]
impl crate::Readable for TprcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tprcr::W`](W) writer structure"]
impl crate::Writable for TprcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPRCR to value 0"]
impl crate::Resettable for TprcrSpec {
    const RESET_VALUE: u32 = 0;
}

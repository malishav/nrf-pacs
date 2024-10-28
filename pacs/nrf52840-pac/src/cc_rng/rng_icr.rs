#[doc = "Register `RNG_ICR` writer"]
pub type W = crate::W<RngIcrSpec>;
#[doc = "Field `EHR_VALID_CLEAR` writer - Writing value '1' clears corresponding bit in RNG_ISR"]
pub type EhrValidClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCORR_ERR_CLEAR` writer - Cannot be cleared by software! Only RNG reset clears this bit."]
pub type AutocorrErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRNGT_ERR_CLEAR` writer - Writing value '1' clears corresponding bit in RNG_ISR"]
pub type CrngtErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VNC_ERR_CLEAR` writer - Writing value '1' clears corresponding bit in RNG_ISR"]
pub type VncErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WATCHDOG_CLEAR` writer - Writing value '1' clears corresponding bit in RNG_ISR"]
pub type WatchdogClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE_CLEAR` writer - Writing value '1' clears corresponding bit in RNG_ISR"]
pub type DmaDoneClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing value '1' clears corresponding bit in RNG_ISR"]
    #[inline(always)]
    #[must_use]
    pub fn ehr_valid_clear(&mut self) -> EhrValidClearW<RngIcrSpec> {
        EhrValidClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Cannot be cleared by software! Only RNG reset clears this bit."]
    #[inline(always)]
    #[must_use]
    pub fn autocorr_err_clear(&mut self) -> AutocorrErrClearW<RngIcrSpec> {
        AutocorrErrClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing value '1' clears corresponding bit in RNG_ISR"]
    #[inline(always)]
    #[must_use]
    pub fn crngt_err_clear(&mut self) -> CrngtErrClearW<RngIcrSpec> {
        CrngtErrClearW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing value '1' clears corresponding bit in RNG_ISR"]
    #[inline(always)]
    #[must_use]
    pub fn vnc_err_clear(&mut self) -> VncErrClearW<RngIcrSpec> {
        VncErrClearW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing value '1' clears corresponding bit in RNG_ISR"]
    #[inline(always)]
    #[must_use]
    pub fn watchdog_clear(&mut self) -> WatchdogClearW<RngIcrSpec> {
        WatchdogClearW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing value '1' clears corresponding bit in RNG_ISR"]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_clear(&mut self) -> DmaDoneClearW<RngIcrSpec> {
        DmaDoneClearW::new(self, 5)
    }
}
#[doc = "Interrupt clear register. Writing a 1 bit into a field in this register will clear the corresponding bit in RNG_ISR.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngIcrSpec;
impl crate::RegisterSpec for RngIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rng_icr::W`](W) writer structure"]
impl crate::Writable for RngIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_ICR to value 0"]
impl crate::Resettable for RngIcrSpec {
    const RESET_VALUE: u32 = 0;
}

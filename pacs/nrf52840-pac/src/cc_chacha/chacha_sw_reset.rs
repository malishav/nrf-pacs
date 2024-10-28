#[doc = "Register `CHACHA_SW_RESET` writer"]
pub type W = crate::W<ChachaSwResetSpec>;
#[doc = "Writing any value to this address resets the CHACHA engine. The reset takes 4 CPU clock cycles to complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "1: Reset CHACHA engine."]
    Enable = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` writer - Writing any value to this address resets the CHACHA engine. The reset takes 4 CPU clock cycles to complete."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset CHACHA engine."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Writing any value to this address resets the CHACHA engine. The reset takes 4 CPU clock cycles to complete."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<ChachaSwResetSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "Reset the CHACHA engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chacha_sw_reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChachaSwResetSpec;
impl crate::RegisterSpec for ChachaSwResetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chacha_sw_reset::W`](W) writer structure"]
impl crate::Writable for ChachaSwResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHACHA_SW_RESET to value 0"]
impl crate::Resettable for ChachaSwResetSpec {
    const RESET_VALUE: u32 = 0;
}

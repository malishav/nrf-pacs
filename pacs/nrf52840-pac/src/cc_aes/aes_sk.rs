#[doc = "Register `AES_SK` writer"]
pub type W = crate::W<AesSkSpec>;
#[doc = "Field `AES_SK` writer - Sample HW key to AES_KEY_0 registers."]
pub type AesSkW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Sample HW key to AES_KEY_0 registers."]
    #[inline(always)]
    #[must_use]
    pub fn aes_sk(&mut self) -> AesSkW<AesSkSpec> {
        AesSkW::new(self, 0)
    }
}
#[doc = "Writing to this address trigger sampling of the HW key to the AES_KEY_0 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_sk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesSkSpec;
impl crate::RegisterSpec for AesSkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_sk::W`](W) writer structure"]
impl crate::Writable for AesSkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_SK to value 0"]
impl crate::Resettable for AesSkSpec {
    const RESET_VALUE: u32 = 0;
}

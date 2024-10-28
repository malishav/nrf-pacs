#[doc = "Register `SRC_MEM_ADDR` writer"]
pub type W = crate::W<SrcMemAddrSpec>;
#[doc = "Field `ADDR` writer - Source address in memory."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Source address in memory."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<SrcMemAddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Data source address in memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_mem_addr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcMemAddrSpec;
impl crate::RegisterSpec for SrcMemAddrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`src_mem_addr::W`](W) writer structure"]
impl crate::Writable for SrcMemAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_MEM_ADDR to value 0"]
impl crate::Resettable for SrcMemAddrSpec {
    const RESET_VALUE: u32 = 0;
}

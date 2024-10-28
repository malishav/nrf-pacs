#[doc = "Register `DST_MEM_SIZE` writer"]
pub type W = crate::W<DstMemSizeSpec>;
#[doc = "Field `SIZE` writer - Total number of bytes to write to memory."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `FIRST` writer - This field is reserved"]
pub type FirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST` writer - This field is reserved"]
pub type LastW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:29 - Total number of bytes to write to memory."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<DstMemSizeSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bit 30 - This field is reserved"]
    #[inline(always)]
    #[must_use]
    pub fn first(&mut self) -> FirstW<DstMemSizeSpec> {
        FirstW::new(self, 30)
    }
    #[doc = "Bit 31 - This field is reserved"]
    #[inline(always)]
    #[must_use]
    pub fn last(&mut self) -> LastW<DstMemSizeSpec> {
        LastW::new(self, 31)
    }
}
#[doc = "The number of bytes to be written to memory.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_mem_size::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstMemSizeSpec;
impl crate::RegisterSpec for DstMemSizeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dst_mem_size::W`](W) writer structure"]
impl crate::Writable for DstMemSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_MEM_SIZE to value 0"]
impl crate::Resettable for DstMemSizeSpec {
    const RESET_VALUE: u32 = 0;
}

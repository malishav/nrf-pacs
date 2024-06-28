#[doc = "Register `MAXCNT` reader"]
pub type R = crate::R<MaxcntSpec>;
#[doc = "Register `MAXCNT` writer"]
pub type W = crate::W<MaxcntSpec>;
#[doc = "Field `MAXCNT` reader - Maximum number of bytes in TXD buffer"]
pub type MaxcntR = crate::FieldReader<u16>;
#[doc = "Field `MAXCNT` writer - Maximum number of bytes in TXD buffer"]
pub type MaxcntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Maximum number of bytes in TXD buffer"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MaxcntR {
        MaxcntR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Maximum number of bytes in TXD buffer"]
    #[inline(always)]
    #[must_use]
    pub fn maxcnt(&mut self) -> MaxcntW<MaxcntSpec> {
        MaxcntW::new(self, 0)
    }
}
#[doc = "Maximum number of bytes in TXD buffer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxcntSpec;
impl crate::RegisterSpec for MaxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxcnt::R`](R) reader structure"]
impl crate::Readable for MaxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`maxcnt::W`](W) writer structure"]
impl crate::Writable for MaxcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXCNT to value 0"]
impl crate::Resettable for MaxcntSpec {
    const RESET_VALUE: u32 = 0;
}

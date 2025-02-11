#[doc = "Register `TRCITIATBINR` reader"]
pub type R = crate::R<TrcitiatbinrSpec>;
#[doc = "Register `TRCITIATBINR` writer"]
pub type W = crate::W<TrcitiatbinrSpec>;
#[doc = "Field `ATVALID` reader - Returns the value of the ATVALIDMI input pin."]
pub type AtvalidR = crate::BitReader;
#[doc = "Field `ATVALID` writer - Returns the value of the ATVALIDMI input pin."]
pub type AtvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADY` reader - Returns the value of the AFREADYMI input pin."]
pub type AfreadyR = crate::BitReader;
#[doc = "Field `AFREADY` writer - Returns the value of the AFREADYMI input pin."]
pub type AfreadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Returns the value of the ATVALIDMI input pin."]
    #[inline(always)]
    pub fn atvalid(&self) -> AtvalidR {
        AtvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Returns the value of the AFREADYMI input pin."]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the value of the ATVALIDMI input pin."]
    #[inline(always)]
    #[must_use]
    pub fn atvalid(&mut self) -> AtvalidW<TrcitiatbinrSpec> {
        AtvalidW::new(self, 0)
    }
    #[doc = "Bit 1 - Returns the value of the AFREADYMI input pin."]
    #[inline(always)]
    #[must_use]
    pub fn afready(&mut self) -> AfreadyW<TrcitiatbinrSpec> {
        AfreadyW::new(self, 1)
    }
}
#[doc = "Reads the state of the input pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`trcitiatbinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trcitiatbinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrcitiatbinrSpec;
impl crate::RegisterSpec for TrcitiatbinrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trcitiatbinr::R`](R) reader structure"]
impl crate::Readable for TrcitiatbinrSpec {}
#[doc = "`write(|w| ..)` method takes [`trcitiatbinr::W`](W) writer structure"]
impl crate::Writable for TrcitiatbinrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCITIATBINR to value 0"]
impl crate::Resettable for TrcitiatbinrSpec {
    const RESET_VALUE: u32 = 0;
}

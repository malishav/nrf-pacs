#[doc = "Register `XOSC32M` reader"]
pub type R = crate::R<Xosc32mSpec>;
#[doc = "Register `XOSC32M` writer"]
pub type W = crate::W<Xosc32mSpec>;
#[doc = "Field `CTRL` reader - Pierce current DAC control signals"]
pub type CtrlR = crate::FieldReader;
#[doc = "Field `CTRL` writer - Pierce current DAC control signals"]
pub type CtrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Pierce current DAC control signals"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pierce current DAC control signals"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl(&mut self) -> CtrlW<Xosc32mSpec> {
        CtrlW::new(self, 0)
    }
}
#[doc = "Oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc32m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc32m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32mSpec;
impl crate::RegisterSpec for Xosc32mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc32m::R`](R) reader structure"]
impl crate::Readable for Xosc32mSpec {}
#[doc = "`write(|w| ..)` method takes [`xosc32m::W`](W) writer structure"]
impl crate::Writable for Xosc32mSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XOSC32M to value 0xffff_ffcf"]
impl crate::Resettable for Xosc32mSpec {
    const RESET_VALUE: u32 = 0xffff_ffcf;
}

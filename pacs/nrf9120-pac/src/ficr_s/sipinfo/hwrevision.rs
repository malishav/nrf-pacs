#[doc = "Register `HWREVISION[%s]` reader"]
pub type R = crate::R<HwrevisionSpec>;
#[doc = "Field `HWREVISION` reader - "]
pub type HwrevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hwrevision(&self) -> HwrevisionR {
        HwrevisionR::new(self.bits)
    }
}
#[doc = "Description collection: SIP hardware revision, encoded in ASCII, ex B0A or B1A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwrevision::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwrevisionSpec;
impl crate::RegisterSpec for HwrevisionSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hwrevision::R`](R) reader structure"]
impl crate::Readable for HwrevisionSpec {}
#[doc = "`reset()` method sets HWREVISION[%s]
to value 0xff"]
impl crate::Resettable for HwrevisionSpec {
    const RESET_VALUE: u8 = 0xff;
}

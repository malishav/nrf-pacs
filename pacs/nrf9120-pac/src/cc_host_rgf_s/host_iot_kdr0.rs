#[doc = "Register `HOST_IOT_KDR0` reader"]
pub type R = crate::R<HostIotKdr0Spec>;
#[doc = "Register `HOST_IOT_KDR0` writer"]
pub type W = crate::W<HostIotKdr0Spec>;
#[doc = "Field `HOST_IOT_KDR0` reader - Write: K_DR bits 31:0. Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain. Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
pub type HostIotKdr0R = crate::FieldReader<u32>;
#[doc = "Field `HOST_IOT_KDR0` writer - Write: K_DR bits 31:0. Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain. Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
pub type HostIotKdr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write: K_DR bits 31:0. Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain. Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    pub fn host_iot_kdr0(&self) -> HostIotKdr0R {
        HostIotKdr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write: K_DR bits 31:0. Read: 0x00000000 when 128-bit K_DR key value is not yet retained in the CRYPTOCELL AO power domain. Read: 0x00000001 when 128-bit K_DR key value is successfully retained in the CRYPTOCELL AO power domain."]
    #[inline(always)]
    #[must_use]
    pub fn host_iot_kdr0(&mut self) -> HostIotKdr0W<HostIotKdr0Spec> {
        HostIotKdr0W::new(self, 0)
    }
}
#[doc = "This register holds bits 31:0 of K_DR. The value of this register is saved in the CRYPTOCELL AO power domain. Reading from this address returns the K_DR valid status indicating if K_DR is successfully retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_iot_kdr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_iot_kdr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotKdr0Spec;
impl crate::RegisterSpec for HostIotKdr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_iot_kdr0::R`](R) reader structure"]
impl crate::Readable for HostIotKdr0Spec {}
#[doc = "`write(|w| ..)` method takes [`host_iot_kdr0::W`](W) writer structure"]
impl crate::Writable for HostIotKdr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_IOT_KDR0 to value 0"]
impl crate::Resettable for HostIotKdr0Spec {
    const RESET_VALUE: u32 = 0;
}

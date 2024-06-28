#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "ERASEPROTECT")]
pub struct Eraseprotect {
    lock: Lock,
    disable: Disable,
}
impl Eraseprotect {
    #[doc = "0x00 - This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x04 - This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
    #[inline(always)]
    pub const fn disable(&self) -> &Disable {
        &self.disable
    }
}
#[doc = "LOCK (rw) register accessor: This register locks the ERASEPROTECT.DISABLE register from being written until next reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "This register locks the ERASEPROTECT.DISABLE register from being written until next reset."]
pub mod lock;
#[doc = "DISABLE (rw) register accessor: This register disables the ERASEPROTECT register and performs an ERASEALL operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disable`]
module"]
#[doc(alias = "DISABLE")]
pub type Disable = crate::Reg<disable::DisableSpec>;
#[doc = "This register disables the ERASEPROTECT register and performs an ERASEALL operation."]
pub mod disable;

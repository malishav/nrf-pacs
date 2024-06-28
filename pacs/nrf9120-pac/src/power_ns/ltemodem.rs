#[repr(C)]
#[doc = "LTE Modem"]
#[doc(alias = "LTEMODEM")]
pub struct Ltemodem {
    startn: Startn,
    forceoff: Forceoff,
}
impl Ltemodem {
    #[doc = "0x00 - Start LTE modem"]
    #[inline(always)]
    pub const fn startn(&self) -> &Startn {
        &self.startn
    }
    #[doc = "0x04 - Force off LTE modem"]
    #[inline(always)]
    pub const fn forceoff(&self) -> &Forceoff {
        &self.forceoff
    }
}
#[doc = "STARTN (rw) register accessor: Start LTE modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startn`]
module"]
#[doc(alias = "STARTN")]
pub type Startn = crate::Reg<startn::StartnSpec>;
#[doc = "Start LTE modem"]
pub mod startn;
#[doc = "FORCEOFF (rw) register accessor: Force off LTE modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`forceoff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`forceoff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forceoff`]
module"]
#[doc(alias = "FORCEOFF")]
pub type Forceoff = crate::Reg<forceoff::ForceoffSpec>;
#[doc = "Force off LTE modem"]
pub mod forceoff;

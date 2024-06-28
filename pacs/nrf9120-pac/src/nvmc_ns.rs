#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    ready: Ready,
    _reserved1: [u8; 0x04],
    readynext: Readynext,
    _reserved2: [u8; 0xf8],
    config: Config,
    _reserved3: [u8; 0x04],
    eraseall: Eraseall,
    _reserved4: [u8; 0x0c],
    erasepagepartialcfg: Erasepagepartialcfg,
    _reserved5: [u8; 0x20],
    icachecnf: Icachecnf,
    _reserved6: [u8; 0x04],
    ihit: Ihit,
    imiss: Imiss,
    _reserved8: [u8; 0x34],
    configns: Configns,
    writeuicrns: Writeuicrns,
}
impl RegisterBlock {
    #[doc = "0x400 - Ready flag"]
    #[inline(always)]
    pub const fn ready(&self) -> &Ready {
        &self.ready
    }
    #[doc = "0x408 - Ready flag"]
    #[inline(always)]
    pub const fn readynext(&self) -> &Readynext {
        &self.readynext
    }
    #[doc = "0x504 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    #[inline(always)]
    pub const fn eraseall(&self) -> &Eraseall {
        &self.eraseall
    }
    #[doc = "0x51c - Register for partial erase configuration"]
    #[inline(always)]
    pub const fn erasepagepartialcfg(&self) -> &Erasepagepartialcfg {
        &self.erasepagepartialcfg
    }
    #[doc = "0x540 - I-code cache configuration register"]
    #[inline(always)]
    pub const fn icachecnf(&self) -> &Icachecnf {
        &self.icachecnf
    }
    #[doc = "0x548 - I-code cache hit counter"]
    #[inline(always)]
    pub const fn ihit(&self) -> &Ihit {
        &self.ihit
    }
    #[doc = "0x54c - I-code cache miss counter"]
    #[inline(always)]
    pub const fn imiss(&self) -> &Imiss {
        &self.imiss
    }
    #[doc = "0x584 - Unspecified"]
    #[inline(always)]
    pub const fn configns(&self) -> &Configns {
        &self.configns
    }
    #[doc = "0x588 - Non-secure APPROTECT enable register"]
    #[inline(always)]
    pub const fn writeuicrns(&self) -> &Writeuicrns {
        &self.writeuicrns
    }
}
#[doc = "READY (r) register accessor: Ready flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ready::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ready`]
module"]
#[doc(alias = "READY")]
pub type Ready = crate::Reg<ready::ReadySpec>;
#[doc = "Ready flag"]
pub mod ready;
#[doc = "READYNEXT (r) register accessor: Ready flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readynext::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readynext`]
module"]
#[doc(alias = "READYNEXT")]
pub type Readynext = crate::Reg<readynext::ReadynextSpec>;
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ERASEALL (w) register accessor: Register for erasing all non-volatile user memory\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eraseall::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseall`]
module"]
#[doc(alias = "ERASEALL")]
pub type Eraseall = crate::Reg<eraseall::EraseallSpec>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "ERASEPAGEPARTIALCFG (rw) register accessor: Register for partial erase configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`erasepagepartialcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`erasepagepartialcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@erasepagepartialcfg`]
module"]
#[doc(alias = "ERASEPAGEPARTIALCFG")]
pub type Erasepagepartialcfg = crate::Reg<erasepagepartialcfg::ErasepagepartialcfgSpec>;
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "ICACHECNF (rw) register accessor: I-code cache configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icachecnf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icachecnf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icachecnf`]
module"]
#[doc(alias = "ICACHECNF")]
pub type Icachecnf = crate::Reg<icachecnf::IcachecnfSpec>;
#[doc = "I-code cache configuration register"]
pub mod icachecnf;
#[doc = "IHIT (rw) register accessor: I-code cache hit counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ihit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ihit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihit`]
module"]
#[doc(alias = "IHIT")]
pub type Ihit = crate::Reg<ihit::IhitSpec>;
#[doc = "I-code cache hit counter"]
pub mod ihit;
#[doc = "IMISS (rw) register accessor: I-code cache miss counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imiss::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imiss::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imiss`]
module"]
#[doc(alias = "IMISS")]
pub type Imiss = crate::Reg<imiss::ImissSpec>;
#[doc = "I-code cache miss counter"]
pub mod imiss;
#[doc = "CONFIGNS (rw) register accessor: Unspecified\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configns::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configns::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configns`]
module"]
#[doc(alias = "CONFIGNS")]
pub type Configns = crate::Reg<configns::ConfignsSpec>;
#[doc = "Unspecified"]
pub mod configns;
#[doc = "WRITEUICRNS (w) register accessor: Non-secure APPROTECT enable register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writeuicrns::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writeuicrns`]
module"]
#[doc(alias = "WRITEUICRNS")]
pub type Writeuicrns = crate::Reg<writeuicrns::WriteuicrnsSpec>;
#[doc = "Non-secure APPROTECT enable register"]
pub mod writeuicrns;

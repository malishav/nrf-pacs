#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "TRIMCNF")]
pub struct Trimcnf {
    addr: Addr,
    data: Data,
}
impl Trimcnf {
    #[doc = "0x00 - Description cluster: Address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - Description cluster: Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "ADDR (r) register accessor: Description cluster: Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Description cluster: Address"]
pub mod addr;
#[doc = "DATA (r) register accessor: Description cluster: Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Description cluster: Data"]
pub mod data;

#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "RAMNSC")]
pub struct Ramnsc {
    region: Region,
    size: Size,
}
impl Ramnsc {
    #[doc = "0x00 - Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn region(&self) -> &Region {
        &self.region
    }
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub const fn size(&self) -> &Size {
        &self.size
    }
}
#[doc = "REGION (rw) register accessor: Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`region::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`region::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@region`]
module"]
#[doc(alias = "REGION")]
pub type Region = crate::Reg<region::RegionSpec>;
#[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
pub mod region;
#[doc = "SIZE (rw) register accessor: Description cluster: Define the size of the non-secure callable (NSC) region n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@size`]
module"]
#[doc(alias = "SIZE")]
pub type Size = crate::Reg<size::SizeSpec>;
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
pub mod size;

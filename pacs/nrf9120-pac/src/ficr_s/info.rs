#[repr(C)]
#[doc = "Device info"]
#[doc(alias = "INFO")]
pub struct Info {
    _reserved0: [u8; 0x04],
    deviceid: [Deviceid; 2],
    part: Part,
    variant: Variant,
    package: Package,
    ram: Ram,
    flash: Flash,
    codepagesize: Codepagesize,
    codesize: Codesize,
    devicetype: Devicetype,
}
impl Info {
    #[doc = "0x04..0x0c - Description collection: Device identifier"]
    #[inline(always)]
    pub const fn deviceid(&self, n: usize) -> &Deviceid {
        &self.deviceid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Description collection: Device identifier"]
    #[inline(always)]
    pub fn deviceid_iter(&self) -> impl Iterator<Item = &Deviceid> {
        self.deviceid.iter()
    }
    #[doc = "0x0c - Part code"]
    #[inline(always)]
    pub const fn part(&self) -> &Part {
        &self.part
    }
    #[doc = "0x10 - Part Variant, Hardware version and Production configuration"]
    #[inline(always)]
    pub const fn variant(&self) -> &Variant {
        &self.variant
    }
    #[doc = "0x14 - Package option"]
    #[inline(always)]
    pub const fn package(&self) -> &Package {
        &self.package
    }
    #[doc = "0x18 - RAM variant"]
    #[inline(always)]
    pub const fn ram(&self) -> &Ram {
        &self.ram
    }
    #[doc = "0x1c - Flash variant"]
    #[inline(always)]
    pub const fn flash(&self) -> &Flash {
        &self.flash
    }
    #[doc = "0x20 - Code memory page size"]
    #[inline(always)]
    pub const fn codepagesize(&self) -> &Codepagesize {
        &self.codepagesize
    }
    #[doc = "0x24 - Code memory size"]
    #[inline(always)]
    pub const fn codesize(&self) -> &Codesize {
        &self.codesize
    }
    #[doc = "0x28 - Device type"]
    #[inline(always)]
    pub const fn devicetype(&self) -> &Devicetype {
        &self.devicetype
    }
}
#[doc = "DEVICEID (r) register accessor: Description collection: Device identifier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`]
module"]
#[doc(alias = "DEVICEID")]
pub type Deviceid = crate::Reg<deviceid::DeviceidSpec>;
#[doc = "Description collection: Device identifier"]
pub mod deviceid;
#[doc = "PART (r) register accessor: Part code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`part::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part`]
module"]
#[doc(alias = "PART")]
pub type Part = crate::Reg<part::PartSpec>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT (r) register accessor: Part Variant, Hardware version and Production configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`variant::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@variant`]
module"]
#[doc(alias = "VARIANT")]
pub type Variant = crate::Reg<variant::VariantSpec>;
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "PACKAGE (r) register accessor: Package option\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`package::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@package`]
module"]
#[doc(alias = "PACKAGE")]
pub type Package = crate::Reg<package::PackageSpec>;
#[doc = "Package option"]
pub mod package;
#[doc = "RAM (r) register accessor: RAM variant\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram`]
module"]
#[doc(alias = "RAM")]
pub type Ram = crate::Reg<ram::RamSpec>;
#[doc = "RAM variant"]
pub mod ram;
#[doc = "FLASH (r) register accessor: Flash variant\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash`]
module"]
#[doc(alias = "FLASH")]
pub type Flash = crate::Reg<flash::FlashSpec>;
#[doc = "Flash variant"]
pub mod flash;
#[doc = "CODEPAGESIZE (r) register accessor: Code memory page size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codepagesize::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codepagesize`]
module"]
#[doc(alias = "CODEPAGESIZE")]
pub type Codepagesize = crate::Reg<codepagesize::CodepagesizeSpec>;
#[doc = "Code memory page size"]
pub mod codepagesize;
#[doc = "CODESIZE (r) register accessor: Code memory size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codesize::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codesize`]
module"]
#[doc(alias = "CODESIZE")]
pub type Codesize = crate::Reg<codesize::CodesizeSpec>;
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "DEVICETYPE (r) register accessor: Device type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devicetype::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicetype`]
module"]
#[doc(alias = "DEVICETYPE")]
pub type Devicetype = crate::Reg<devicetype::DevicetypeSpec>;
#[doc = "Device type"]
pub mod devicetype;

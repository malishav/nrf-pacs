#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    mailbox: Mailbox,
    _reserved1: [u8; 0x78],
    eraseprotect: Eraseprotect,
}
impl RegisterBlock {
    #[doc = "0x400..0x488 - Unspecified"]
    #[inline(always)]
    pub const fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }
    #[doc = "0x500..0x508 - Unspecified"]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
}
#[doc = "Unspecified"]
pub use self::mailbox::Mailbox;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = "Unspecified"]
pub use self::eraseprotect::Eraseprotect;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod eraseprotect;

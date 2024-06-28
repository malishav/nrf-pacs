#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_send: [TasksSend; 8],
    _reserved1: [u8; 0x60],
    subscribe_send: [SubscribeSend; 8],
    _reserved2: [u8; 0x60],
    events_receive: [EventsReceive; 8],
    _reserved3: [u8; 0x60],
    publish_receive: [PublishReceive; 8],
    _reserved4: [u8; 0x0160],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    intpend: Intpend,
    _reserved8: [u8; 0x0200],
    send_cnf: [SendCnf; 8],
    _reserved9: [u8; 0x60],
    receive_cnf: [ReceiveCnf; 8],
    _reserved10: [u8; 0x60],
    gpmem: [Gpmem; 4],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
    #[inline(always)]
    pub const fn tasks_send(&self, n: usize) -> &TasksSend {
        &self.tasks_send[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
    #[inline(always)]
    pub fn tasks_send_iter(&self) -> impl Iterator<Item = &TasksSend> {
        self.tasks_send.iter()
    }
    #[doc = "0x80..0xa0 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    #[inline(always)]
    pub const fn subscribe_send(&self, n: usize) -> &SubscribeSend {
        &self.subscribe_send[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xa0 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    #[inline(always)]
    pub fn subscribe_send_iter(&self) -> impl Iterator<Item = &SubscribeSend> {
        self.subscribe_send.iter()
    }
    #[doc = "0x100..0x120 - Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    pub const fn events_receive(&self, n: usize) -> &EventsReceive {
        &self.events_receive[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
    #[inline(always)]
    pub fn events_receive_iter(&self) -> impl Iterator<Item = &EventsReceive> {
        self.events_receive.iter()
    }
    #[doc = "0x180..0x1a0 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    #[inline(always)]
    pub const fn publish_receive(&self, n: usize) -> &PublishReceive {
        &self.publish_receive[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1a0 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    #[inline(always)]
    pub fn publish_receive_iter(&self) -> impl Iterator<Item = &PublishReceive> {
        self.publish_receive.iter()
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x30c - Pending interrupts"]
    #[inline(always)]
    pub const fn intpend(&self) -> &Intpend {
        &self.intpend
    }
    #[doc = "0x510..0x530 - Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
    #[inline(always)]
    pub const fn send_cnf(&self, n: usize) -> &SendCnf {
        &self.send_cnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x530 - Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
    #[inline(always)]
    pub fn send_cnf_iter(&self) -> impl Iterator<Item = &SendCnf> {
        self.send_cnf.iter()
    }
    #[doc = "0x590..0x5b0 - Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
    #[inline(always)]
    pub const fn receive_cnf(&self, n: usize) -> &ReceiveCnf {
        &self.receive_cnf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x590..0x5b0 - Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
    #[inline(always)]
    pub fn receive_cnf_iter(&self) -> impl Iterator<Item = &ReceiveCnf> {
        self.receive_cnf.iter()
    }
    #[doc = "0x610..0x620 - Description collection: General purpose memory"]
    #[inline(always)]
    pub const fn gpmem(&self, n: usize) -> &Gpmem {
        &self.gpmem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x610..0x620 - Description collection: General purpose memory"]
    #[inline(always)]
    pub fn gpmem_iter(&self) -> impl Iterator<Item = &Gpmem> {
        self.gpmem.iter()
    }
}
#[doc = "TASKS_SEND (w) register accessor: Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_send::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_send`]
module"]
#[doc(alias = "TASKS_SEND")]
pub type TasksSend = crate::Reg<tasks_send::TasksSendSpec>;
#[doc = "Description collection: Trigger events on IPC channel enabled in SEND_CNF\\[n\\]"]
pub mod tasks_send;
#[doc = "SUBSCRIBE_SEND (rw) register accessor: Description collection: Subscribe configuration for task SEND\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_send::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_send::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_send`]
module"]
#[doc(alias = "SUBSCRIBE_SEND")]
pub type SubscribeSend = crate::Reg<subscribe_send::SubscribeSendSpec>;
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub mod subscribe_send;
#[doc = "EVENTS_RECEIVE (rw) register accessor: Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_receive::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_receive::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_receive`]
module"]
#[doc(alias = "EVENTS_RECEIVE")]
pub type EventsReceive = crate::Reg<events_receive::EventsReceiveSpec>;
#[doc = "Description collection: Event received on one or more of the enabled IPC channels in RECEIVE_CNF\\[n\\]"]
pub mod events_receive;
#[doc = "PUBLISH_RECEIVE (rw) register accessor: Description collection: Publish configuration for event RECEIVE\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_receive::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_receive::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_receive`]
module"]
#[doc(alias = "PUBLISH_RECEIVE")]
pub type PublishReceive = crate::Reg<publish_receive::PublishReceiveSpec>;
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub mod publish_receive;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "INTPEND (r) register accessor: Pending interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpend::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpend`]
module"]
#[doc(alias = "INTPEND")]
pub type Intpend = crate::Reg<intpend::IntpendSpec>;
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "SEND_CNF (rw) register accessor: Description collection: Send event configuration for TASKS_SEND\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`send_cnf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send_cnf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@send_cnf`]
module"]
#[doc(alias = "SEND_CNF")]
pub type SendCnf = crate::Reg<send_cnf::SendCnfSpec>;
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]"]
pub mod send_cnf;
#[doc = "RECEIVE_CNF (rw) register accessor: Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_cnf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_cnf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_cnf`]
module"]
#[doc(alias = "RECEIVE_CNF")]
pub type ReceiveCnf = crate::Reg<receive_cnf::ReceiveCnfSpec>;
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]"]
pub mod receive_cnf;
#[doc = "GPMEM (rw) register accessor: Description collection: General purpose memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpmem`]
module"]
#[doc(alias = "GPMEM")]
pub type Gpmem = crate::Reg<gpmem::GpmemSpec>;
#[doc = "Description collection: General purpose memory"]
pub mod gpmem;

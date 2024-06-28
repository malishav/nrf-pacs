#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_startrx: TasksStartrx,
    _reserved1: [u8; 0x04],
    tasks_starttx: TasksStarttx,
    _reserved2: [u8; 0x08],
    tasks_stop: TasksStop,
    _reserved3: [u8; 0x04],
    tasks_suspend: TasksSuspend,
    tasks_resume: TasksResume,
    _reserved5: [u8; 0x5c],
    subscribe_startrx: SubscribeStartrx,
    _reserved6: [u8; 0x04],
    subscribe_starttx: SubscribeStarttx,
    _reserved7: [u8; 0x08],
    subscribe_stop: SubscribeStop,
    _reserved8: [u8; 0x04],
    subscribe_suspend: SubscribeSuspend,
    subscribe_resume: SubscribeResume,
    _reserved10: [u8; 0x60],
    events_stopped: EventsStopped,
    _reserved11: [u8; 0x1c],
    events_error: EventsError,
    _reserved12: [u8; 0x20],
    events_suspended: EventsSuspended,
    events_rxstarted: EventsRxstarted,
    events_txstarted: EventsTxstarted,
    _reserved15: [u8; 0x08],
    events_lastrx: EventsLastrx,
    events_lasttx: EventsLasttx,
    _reserved17: [u8; 0x20],
    publish_stopped: PublishStopped,
    _reserved18: [u8; 0x1c],
    publish_error: PublishError,
    _reserved19: [u8; 0x20],
    publish_suspended: PublishSuspended,
    publish_rxstarted: PublishRxstarted,
    publish_txstarted: PublishTxstarted,
    _reserved22: [u8; 0x08],
    publish_lastrx: PublishLastrx,
    publish_lasttx: PublishLasttx,
    _reserved24: [u8; 0x1c],
    shorts: Shorts,
    _reserved25: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved28: [u8; 0x01b8],
    errorsrc: Errorsrc,
    _reserved29: [u8; 0x38],
    enable: Enable,
    _reserved30: [u8; 0x04],
    psel: Psel,
    _reserved31: [u8; 0x14],
    frequency: Frequency,
    _reserved32: [u8; 0x0c],
    rxd: Rxd,
    txd: Txd,
    _reserved34: [u8; 0x34],
    address: Address,
}
impl RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    #[inline(always)]
    pub const fn tasks_startrx(&self) -> &TasksStartrx {
        &self.tasks_startrx
    }
    #[doc = "0x08 - Start TWI transmit sequence"]
    #[inline(always)]
    pub const fn tasks_starttx(&self) -> &TasksStarttx {
        &self.tasks_starttx
    }
    #[doc = "0x14 - Stop TWI transaction. Must be issued while the TWI master is not suspended."]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x1c - Suspend TWI transaction"]
    #[inline(always)]
    pub const fn tasks_suspend(&self) -> &TasksSuspend {
        &self.tasks_suspend
    }
    #[doc = "0x20 - Resume TWI transaction"]
    #[inline(always)]
    pub const fn tasks_resume(&self) -> &TasksResume {
        &self.tasks_resume
    }
    #[doc = "0x80 - Subscribe configuration for task STARTRX"]
    #[inline(always)]
    pub const fn subscribe_startrx(&self) -> &SubscribeStartrx {
        &self.subscribe_startrx
    }
    #[doc = "0x88 - Subscribe configuration for task STARTTX"]
    #[inline(always)]
    pub const fn subscribe_starttx(&self) -> &SubscribeStarttx {
        &self.subscribe_starttx
    }
    #[doc = "0x94 - Subscribe configuration for task STOP"]
    #[inline(always)]
    pub const fn subscribe_stop(&self) -> &SubscribeStop {
        &self.subscribe_stop
    }
    #[doc = "0x9c - Subscribe configuration for task SUSPEND"]
    #[inline(always)]
    pub const fn subscribe_suspend(&self) -> &SubscribeSuspend {
        &self.subscribe_suspend
    }
    #[doc = "0xa0 - Subscribe configuration for task RESUME"]
    #[inline(always)]
    pub const fn subscribe_resume(&self) -> &SubscribeResume {
        &self.subscribe_resume
    }
    #[doc = "0x104 - TWI stopped"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x124 - TWI error"]
    #[inline(always)]
    pub const fn events_error(&self) -> &EventsError {
        &self.events_error
    }
    #[doc = "0x148 - SUSPEND task has been issued, TWI traffic is now suspended."]
    #[inline(always)]
    pub const fn events_suspended(&self) -> &EventsSuspended {
        &self.events_suspended
    }
    #[doc = "0x14c - Receive sequence started"]
    #[inline(always)]
    pub const fn events_rxstarted(&self) -> &EventsRxstarted {
        &self.events_rxstarted
    }
    #[doc = "0x150 - Transmit sequence started"]
    #[inline(always)]
    pub const fn events_txstarted(&self) -> &EventsTxstarted {
        &self.events_txstarted
    }
    #[doc = "0x15c - Byte boundary, starting to receive the last byte"]
    #[inline(always)]
    pub const fn events_lastrx(&self) -> &EventsLastrx {
        &self.events_lastrx
    }
    #[doc = "0x160 - Byte boundary, starting to transmit the last byte"]
    #[inline(always)]
    pub const fn events_lasttx(&self) -> &EventsLasttx {
        &self.events_lasttx
    }
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(&self) -> &PublishStopped {
        &self.publish_stopped
    }
    #[doc = "0x1a4 - Publish configuration for event ERROR"]
    #[inline(always)]
    pub const fn publish_error(&self) -> &PublishError {
        &self.publish_error
    }
    #[doc = "0x1c8 - Publish configuration for event SUSPENDED"]
    #[inline(always)]
    pub const fn publish_suspended(&self) -> &PublishSuspended {
        &self.publish_suspended
    }
    #[doc = "0x1cc - Publish configuration for event RXSTARTED"]
    #[inline(always)]
    pub const fn publish_rxstarted(&self) -> &PublishRxstarted {
        &self.publish_rxstarted
    }
    #[doc = "0x1d0 - Publish configuration for event TXSTARTED"]
    #[inline(always)]
    pub const fn publish_txstarted(&self) -> &PublishTxstarted {
        &self.publish_txstarted
    }
    #[doc = "0x1dc - Publish configuration for event LASTRX"]
    #[inline(always)]
    pub const fn publish_lastrx(&self) -> &PublishLastrx {
        &self.publish_lastrx
    }
    #[doc = "0x1e0 - Publish configuration for event LASTTX"]
    #[inline(always)]
    pub const fn publish_lasttx(&self) -> &PublishLasttx {
        &self.publish_lasttx
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x4c4 - Error source"]
    #[inline(always)]
    pub const fn errorsrc(&self) -> &Errorsrc {
        &self.errorsrc
    }
    #[doc = "0x500 - Enable TWIM"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508..0x510 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x524 - TWI frequency. Accuracy depends on the HFCLK source selected."]
    #[inline(always)]
    pub const fn frequency(&self) -> &Frequency {
        &self.frequency
    }
    #[doc = "0x534..0x544 - RXD EasyDMA channel"]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x544..0x554 - TXD EasyDMA channel"]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x588 - Address used in the TWI transfer"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
}
#[doc = "TASKS_STARTRX (w) register accessor: Start TWI receive sequence\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_startrx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_startrx`]
module"]
#[doc(alias = "TASKS_STARTRX")]
pub type TasksStartrx = crate::Reg<tasks_startrx::TasksStartrxSpec>;
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "TASKS_STARTTX (w) register accessor: Start TWI transmit sequence\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_starttx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_starttx`]
module"]
#[doc(alias = "TASKS_STARTTX")]
pub type TasksStarttx = crate::Reg<tasks_starttx::TasksStarttxSpec>;
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
#[doc = "TASKS_STOP (w) register accessor: Stop TWI transaction. Must be issued while the TWI master is not suspended.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop TWI transaction. Must be issued while the TWI master is not suspended."]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: Suspend TWI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_suspend::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_suspend`]
module"]
#[doc(alias = "TASKS_SUSPEND")]
pub type TasksSuspend = crate::Reg<tasks_suspend::TasksSuspendSpec>;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: Resume TWI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_resume::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_resume`]
module"]
#[doc(alias = "TASKS_RESUME")]
pub type TasksResume = crate::Reg<tasks_resume::TasksResumeSpec>;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "SUBSCRIBE_STARTRX (rw) register accessor: Subscribe configuration for task STARTRX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_startrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_startrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_startrx`]
module"]
#[doc(alias = "SUBSCRIBE_STARTRX")]
pub type SubscribeStartrx = crate::Reg<subscribe_startrx::SubscribeStartrxSpec>;
#[doc = "Subscribe configuration for task STARTRX"]
pub mod subscribe_startrx;
#[doc = "SUBSCRIBE_STARTTX (rw) register accessor: Subscribe configuration for task STARTTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_starttx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_starttx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_starttx`]
module"]
#[doc(alias = "SUBSCRIBE_STARTTX")]
pub type SubscribeStarttx = crate::Reg<subscribe_starttx::SubscribeStarttxSpec>;
#[doc = "Subscribe configuration for task STARTTX"]
pub mod subscribe_starttx;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: Subscribe configuration for task STOP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_stop`]
module"]
#[doc(alias = "SUBSCRIBE_STOP")]
pub type SubscribeStop = crate::Reg<subscribe_stop::SubscribeStopSpec>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_SUSPEND (rw) register accessor: Subscribe configuration for task SUSPEND\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_suspend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_suspend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_suspend`]
module"]
#[doc(alias = "SUBSCRIBE_SUSPEND")]
pub type SubscribeSuspend = crate::Reg<subscribe_suspend::SubscribeSuspendSpec>;
#[doc = "Subscribe configuration for task SUSPEND"]
pub mod subscribe_suspend;
#[doc = "SUBSCRIBE_RESUME (rw) register accessor: Subscribe configuration for task RESUME\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_resume::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_resume::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_resume`]
module"]
#[doc(alias = "SUBSCRIBE_RESUME")]
pub type SubscribeResume = crate::Reg<subscribe_resume::SubscribeResumeSpec>;
#[doc = "Subscribe configuration for task RESUME"]
pub mod subscribe_resume;
#[doc = "EVENTS_STOPPED (rw) register accessor: TWI stopped\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_stopped::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ERROR (rw) register accessor: TWI error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_error`]
module"]
#[doc(alias = "EVENTS_ERROR")]
pub type EventsError = crate::Reg<events_error::EventsErrorSpec>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_SUSPENDED (rw) register accessor: SUSPEND task has been issued, TWI traffic is now suspended.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_suspended::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_suspended::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_suspended`]
module"]
#[doc(alias = "EVENTS_SUSPENDED")]
pub type EventsSuspended = crate::Reg<events_suspended::EventsSuspendedSpec>;
#[doc = "SUSPEND task has been issued, TWI traffic is now suspended."]
pub mod events_suspended;
#[doc = "EVENTS_RXSTARTED (rw) register accessor: Receive sequence started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_rxstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_rxstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxstarted`]
module"]
#[doc(alias = "EVENTS_RXSTARTED")]
pub type EventsRxstarted = crate::Reg<events_rxstarted::EventsRxstartedSpec>;
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "EVENTS_TXSTARTED (rw) register accessor: Transmit sequence started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_txstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_txstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txstarted`]
module"]
#[doc(alias = "EVENTS_TXSTARTED")]
pub type EventsTxstarted = crate::Reg<events_txstarted::EventsTxstartedSpec>;
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "EVENTS_LASTRX (rw) register accessor: Byte boundary, starting to receive the last byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_lastrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_lastrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lastrx`]
module"]
#[doc(alias = "EVENTS_LASTRX")]
pub type EventsLastrx = crate::Reg<events_lastrx::EventsLastrxSpec>;
#[doc = "Byte boundary, starting to receive the last byte"]
pub mod events_lastrx;
#[doc = "EVENTS_LASTTX (rw) register accessor: Byte boundary, starting to transmit the last byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_lasttx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_lasttx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_lasttx`]
module"]
#[doc(alias = "EVENTS_LASTTX")]
pub type EventsLasttx = crate::Reg<events_lasttx::EventsLasttxSpec>;
#[doc = "Byte boundary, starting to transmit the last byte"]
pub mod events_lasttx;
#[doc = "PUBLISH_STOPPED (rw) register accessor: Publish configuration for event STOPPED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_stopped::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_stopped::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_stopped`]
module"]
#[doc(alias = "PUBLISH_STOPPED")]
pub type PublishStopped = crate::Reg<publish_stopped::PublishStoppedSpec>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_ERROR (rw) register accessor: Publish configuration for event ERROR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_error`]
module"]
#[doc(alias = "PUBLISH_ERROR")]
pub type PublishError = crate::Reg<publish_error::PublishErrorSpec>;
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "PUBLISH_SUSPENDED (rw) register accessor: Publish configuration for event SUSPENDED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_suspended::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_suspended::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_suspended`]
module"]
#[doc(alias = "PUBLISH_SUSPENDED")]
pub type PublishSuspended = crate::Reg<publish_suspended::PublishSuspendedSpec>;
#[doc = "Publish configuration for event SUSPENDED"]
pub mod publish_suspended;
#[doc = "PUBLISH_RXSTARTED (rw) register accessor: Publish configuration for event RXSTARTED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_rxstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_rxstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_rxstarted`]
module"]
#[doc(alias = "PUBLISH_RXSTARTED")]
pub type PublishRxstarted = crate::Reg<publish_rxstarted::PublishRxstartedSpec>;
#[doc = "Publish configuration for event RXSTARTED"]
pub mod publish_rxstarted;
#[doc = "PUBLISH_TXSTARTED (rw) register accessor: Publish configuration for event TXSTARTED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_txstarted::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_txstarted::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_txstarted`]
module"]
#[doc(alias = "PUBLISH_TXSTARTED")]
pub type PublishTxstarted = crate::Reg<publish_txstarted::PublishTxstartedSpec>;
#[doc = "Publish configuration for event TXSTARTED"]
pub mod publish_txstarted;
#[doc = "PUBLISH_LASTRX (rw) register accessor: Publish configuration for event LASTRX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_lastrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_lastrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lastrx`]
module"]
#[doc(alias = "PUBLISH_LASTRX")]
pub type PublishLastrx = crate::Reg<publish_lastrx::PublishLastrxSpec>;
#[doc = "Publish configuration for event LASTRX"]
pub mod publish_lastrx;
#[doc = "PUBLISH_LASTTX (rw) register accessor: Publish configuration for event LASTTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_lasttx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_lasttx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_lasttx`]
module"]
#[doc(alias = "PUBLISH_LASTTX")]
pub type PublishLasttx = crate::Reg<publish_lasttx::PublishLasttxSpec>;
#[doc = "Publish configuration for event LASTTX"]
pub mod publish_lasttx;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shorts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "ERRORSRC (rw) register accessor: Error source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`errorsrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`errorsrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errorsrc`]
module"]
#[doc(alias = "ERRORSRC")]
pub type Errorsrc = crate::Reg<errorsrc::ErrorsrcSpec>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: Enable TWIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable TWIM"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "FREQUENCY (rw) register accessor: TWI frequency. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frequency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`]
module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "TWI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "RXD EasyDMA channel"]
pub use self::rxd::Rxd;
#[doc = r"Cluster"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = "TXD EasyDMA channel"]
pub use self::txd::Txd;
#[doc = r"Cluster"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "ADDRESS (rw) register accessor: Address used in the TWI transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Address used in the TWI transfer"]
pub mod address;

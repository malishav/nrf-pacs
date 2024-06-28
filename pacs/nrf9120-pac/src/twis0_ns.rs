#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    tasks_stop: TasksStop,
    _reserved1: [u8; 0x04],
    tasks_suspend: TasksSuspend,
    tasks_resume: TasksResume,
    _reserved3: [u8; 0x0c],
    tasks_preparerx: TasksPreparerx,
    tasks_preparetx: TasksPreparetx,
    _reserved5: [u8; 0x5c],
    subscribe_stop: SubscribeStop,
    _reserved6: [u8; 0x04],
    subscribe_suspend: SubscribeSuspend,
    subscribe_resume: SubscribeResume,
    _reserved8: [u8; 0x0c],
    subscribe_preparerx: SubscribePreparerx,
    subscribe_preparetx: SubscribePreparetx,
    _reserved10: [u8; 0x4c],
    events_stopped: EventsStopped,
    _reserved11: [u8; 0x1c],
    events_error: EventsError,
    _reserved12: [u8; 0x24],
    events_rxstarted: EventsRxstarted,
    events_txstarted: EventsTxstarted,
    _reserved14: [u8; 0x10],
    events_write: EventsWrite,
    events_read: EventsRead,
    _reserved16: [u8; 0x18],
    publish_stopped: PublishStopped,
    _reserved17: [u8; 0x1c],
    publish_error: PublishError,
    _reserved18: [u8; 0x24],
    publish_rxstarted: PublishRxstarted,
    publish_txstarted: PublishTxstarted,
    _reserved20: [u8; 0x10],
    publish_write: PublishWrite,
    publish_read: PublishRead,
    _reserved22: [u8; 0x14],
    shorts: Shorts,
    _reserved23: [u8; 0xfc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved26: [u8; 0x01c4],
    errorsrc: Errorsrc,
    match_: Match,
    _reserved28: [u8; 0x28],
    enable: Enable,
    _reserved29: [u8; 0x04],
    psel: Psel,
    _reserved30: [u8; 0x24],
    rxd: Rxd,
    txd: Txd,
    _reserved32: [u8; 0x34],
    address: [Address; 2],
    _reserved33: [u8; 0x04],
    config: Config,
    _reserved34: [u8; 0x28],
    orc: Orc,
}
impl RegisterBlock {
    #[doc = "0x14 - Stop TWI transaction"]
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
    #[doc = "0x30 - Prepare the TWI slave to respond to a write command"]
    #[inline(always)]
    pub const fn tasks_preparerx(&self) -> &TasksPreparerx {
        &self.tasks_preparerx
    }
    #[doc = "0x34 - Prepare the TWI slave to respond to a read command"]
    #[inline(always)]
    pub const fn tasks_preparetx(&self) -> &TasksPreparetx {
        &self.tasks_preparetx
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
    #[doc = "0xb0 - Subscribe configuration for task PREPARERX"]
    #[inline(always)]
    pub const fn subscribe_preparerx(&self) -> &SubscribePreparerx {
        &self.subscribe_preparerx
    }
    #[doc = "0xb4 - Subscribe configuration for task PREPARETX"]
    #[inline(always)]
    pub const fn subscribe_preparetx(&self) -> &SubscribePreparetx {
        &self.subscribe_preparetx
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
    #[doc = "0x164 - Write command received"]
    #[inline(always)]
    pub const fn events_write(&self) -> &EventsWrite {
        &self.events_write
    }
    #[doc = "0x168 - Read command received"]
    #[inline(always)]
    pub const fn events_read(&self) -> &EventsRead {
        &self.events_read
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
    #[doc = "0x1e4 - Publish configuration for event WRITE"]
    #[inline(always)]
    pub const fn publish_write(&self) -> &PublishWrite {
        &self.publish_write
    }
    #[doc = "0x1e8 - Publish configuration for event READ"]
    #[inline(always)]
    pub const fn publish_read(&self) -> &PublishRead {
        &self.publish_read
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
    #[doc = "0x4d0 - Error source"]
    #[inline(always)]
    pub const fn errorsrc(&self) -> &Errorsrc {
        &self.errorsrc
    }
    #[doc = "0x4d4 - Status register indicating which address had a match"]
    #[inline(always)]
    pub const fn match_(&self) -> &Match {
        &self.match_
    }
    #[doc = "0x500 - Enable TWIS"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508..0x510 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
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
    #[doc = "0x588..0x590 - Description collection: TWI slave address n"]
    #[inline(always)]
    pub const fn address(&self, n: usize) -> &Address {
        &self.address[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x588..0x590 - Description collection: TWI slave address n"]
    #[inline(always)]
    pub fn address_iter(&self) -> impl Iterator<Item = &Address> {
        self.address.iter()
    }
    #[doc = "0x594 - Configuration register for the address match mechanism"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x5c0 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub const fn orc(&self) -> &Orc {
        &self.orc
    }
}
#[doc = "TASKS_STOP (w) register accessor: Stop TWI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop TWI transaction"]
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
#[doc = "TASKS_PREPARERX (w) register accessor: Prepare the TWI slave to respond to a write command\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_preparerx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_preparerx`]
module"]
#[doc(alias = "TASKS_PREPARERX")]
pub type TasksPreparerx = crate::Reg<tasks_preparerx::TasksPreparerxSpec>;
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "TASKS_PREPARETX (w) register accessor: Prepare the TWI slave to respond to a read command\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_preparetx::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_preparetx`]
module"]
#[doc(alias = "TASKS_PREPARETX")]
pub type TasksPreparetx = crate::Reg<tasks_preparetx::TasksPreparetxSpec>;
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
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
#[doc = "SUBSCRIBE_PREPARERX (rw) register accessor: Subscribe configuration for task PREPARERX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_preparerx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_preparerx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_preparerx`]
module"]
#[doc(alias = "SUBSCRIBE_PREPARERX")]
pub type SubscribePreparerx = crate::Reg<subscribe_preparerx::SubscribePreparerxSpec>;
#[doc = "Subscribe configuration for task PREPARERX"]
pub mod subscribe_preparerx;
#[doc = "SUBSCRIBE_PREPARETX (rw) register accessor: Subscribe configuration for task PREPARETX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_preparetx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_preparetx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_preparetx`]
module"]
#[doc(alias = "SUBSCRIBE_PREPARETX")]
pub type SubscribePreparetx = crate::Reg<subscribe_preparetx::SubscribePreparetxSpec>;
#[doc = "Subscribe configuration for task PREPARETX"]
pub mod subscribe_preparetx;
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
#[doc = "EVENTS_WRITE (rw) register accessor: Write command received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_write`]
module"]
#[doc(alias = "EVENTS_WRITE")]
pub type EventsWrite = crate::Reg<events_write::EventsWriteSpec>;
#[doc = "Write command received"]
pub mod events_write;
#[doc = "EVENTS_READ (rw) register accessor: Read command received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_read`]
module"]
#[doc(alias = "EVENTS_READ")]
pub type EventsRead = crate::Reg<events_read::EventsReadSpec>;
#[doc = "Read command received"]
pub mod events_read;
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
#[doc = "PUBLISH_WRITE (rw) register accessor: Publish configuration for event WRITE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_write`]
module"]
#[doc(alias = "PUBLISH_WRITE")]
pub type PublishWrite = crate::Reg<publish_write::PublishWriteSpec>;
#[doc = "Publish configuration for event WRITE"]
pub mod publish_write;
#[doc = "PUBLISH_READ (rw) register accessor: Publish configuration for event READ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_read`]
module"]
#[doc(alias = "PUBLISH_READ")]
pub type PublishRead = crate::Reg<publish_read::PublishReadSpec>;
#[doc = "Publish configuration for event READ"]
pub mod publish_read;
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
#[doc = "MATCH (r) register accessor: Status register indicating which address had a match\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`match_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`]
module"]
#[doc(alias = "MATCH")]
pub type Match = crate::Reg<match_::MatchSpec>;
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "ENABLE (rw) register accessor: Enable TWIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
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
#[doc = "ADDRESS (rw) register accessor: Description collection: TWI slave address n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "Description collection: TWI slave address n"]
pub mod address;
#[doc = "CONFIG (rw) register accessor: Configuration register for the address match mechanism\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "ORC (rw) register accessor: Over-read character. Character sent out in case of an over-read of the transmit buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`orc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orc`]
module"]
#[doc(alias = "ORC")]
pub type Orc = crate::Reg<orc::OrcSpec>;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;

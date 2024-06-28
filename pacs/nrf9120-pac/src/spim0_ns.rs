#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    _reserved2: [u8; 0x04],
    tasks_suspend: TasksSuspend,
    tasks_resume: TasksResume,
    _reserved4: [u8; 0x6c],
    subscribe_start: SubscribeStart,
    subscribe_stop: SubscribeStop,
    _reserved6: [u8; 0x04],
    subscribe_suspend: SubscribeSuspend,
    subscribe_resume: SubscribeResume,
    _reserved8: [u8; 0x60],
    events_stopped: EventsStopped,
    _reserved9: [u8; 0x08],
    events_endrx: EventsEndrx,
    _reserved10: [u8; 0x04],
    events_end: EventsEnd,
    _reserved11: [u8; 0x04],
    events_endtx: EventsEndtx,
    _reserved12: [u8; 0x28],
    events_started: EventsStarted,
    _reserved13: [u8; 0x34],
    publish_stopped: PublishStopped,
    _reserved14: [u8; 0x08],
    publish_endrx: PublishEndrx,
    _reserved15: [u8; 0x04],
    publish_end: PublishEnd,
    _reserved16: [u8; 0x04],
    publish_endtx: PublishEndtx,
    _reserved17: [u8; 0x28],
    publish_started: PublishStarted,
    _reserved18: [u8; 0x30],
    shorts: Shorts,
    _reserved19: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved21: [u8; 0x01f4],
    enable: Enable,
    _reserved22: [u8; 0x04],
    psel: Psel,
    _reserved23: [u8; 0x10],
    frequency: Frequency,
    _reserved24: [u8; 0x0c],
    rxd: Rxd,
    txd: Txd,
    config: Config,
    _reserved27: [u8; 0x68],
    orc: Orc,
}
impl RegisterBlock {
    #[doc = "0x10 - Start SPI transaction"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x14 - Stop SPI transaction"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x1c - Suspend SPI transaction"]
    #[inline(always)]
    pub const fn tasks_suspend(&self) -> &TasksSuspend {
        &self.tasks_suspend
    }
    #[doc = "0x20 - Resume SPI transaction"]
    #[inline(always)]
    pub const fn tasks_resume(&self) -> &TasksResume {
        &self.tasks_resume
    }
    #[doc = "0x90 - Subscribe configuration for task START"]
    #[inline(always)]
    pub const fn subscribe_start(&self) -> &SubscribeStart {
        &self.subscribe_start
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
    #[doc = "0x104 - SPI transaction has stopped"]
    #[inline(always)]
    pub const fn events_stopped(&self) -> &EventsStopped {
        &self.events_stopped
    }
    #[doc = "0x110 - End of RXD buffer reached"]
    #[inline(always)]
    pub const fn events_endrx(&self) -> &EventsEndrx {
        &self.events_endrx
    }
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x120 - End of TXD buffer reached"]
    #[inline(always)]
    pub const fn events_endtx(&self) -> &EventsEndtx {
        &self.events_endtx
    }
    #[doc = "0x14c - Transaction started"]
    #[inline(always)]
    pub const fn events_started(&self) -> &EventsStarted {
        &self.events_started
    }
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    #[inline(always)]
    pub const fn publish_stopped(&self) -> &PublishStopped {
        &self.publish_stopped
    }
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    #[inline(always)]
    pub const fn publish_endrx(&self) -> &PublishEndrx {
        &self.publish_endrx
    }
    #[doc = "0x198 - Publish configuration for event END"]
    #[inline(always)]
    pub const fn publish_end(&self) -> &PublishEnd {
        &self.publish_end
    }
    #[doc = "0x1a0 - Publish configuration for event ENDTX"]
    #[inline(always)]
    pub const fn publish_endtx(&self) -> &PublishEndtx {
        &self.publish_endtx
    }
    #[doc = "0x1cc - Publish configuration for event STARTED"]
    #[inline(always)]
    pub const fn publish_started(&self) -> &PublishStarted {
        &self.publish_started
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
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
    #[doc = "0x500 - Enable SPIM"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x508..0x514 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
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
    #[doc = "0x554 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x5c0 - Over-read character. Character clocked out in case an over-read of the TXD buffer."]
    #[inline(always)]
    pub const fn orc(&self) -> &Orc {
        &self.orc
    }
}
#[doc = "TASKS_START (w) register accessor: Start SPI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop SPI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: Suspend SPI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_suspend::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_suspend`]
module"]
#[doc(alias = "TASKS_SUSPEND")]
pub type TasksSuspend = crate::Reg<tasks_suspend::TasksSuspendSpec>;
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: Resume SPI transaction\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tasks_resume::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_resume`]
module"]
#[doc(alias = "TASKS_RESUME")]
pub type TasksResume = crate::Reg<tasks_resume::TasksResumeSpec>;
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "SUBSCRIBE_START (rw) register accessor: Subscribe configuration for task START\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subscribe_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subscribe_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subscribe_start`]
module"]
#[doc(alias = "SUBSCRIBE_START")]
pub type SubscribeStart = crate::Reg<subscribe_start::SubscribeStartSpec>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
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
#[doc = "EVENTS_STOPPED (rw) register accessor: SPI transaction has stopped\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_stopped::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_stopped::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_stopped`]
module"]
#[doc(alias = "EVENTS_STOPPED")]
pub type EventsStopped = crate::Reg<events_stopped::EventsStoppedSpec>;
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ENDRX (rw) register accessor: End of RXD buffer reached\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_endrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_endrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endrx`]
module"]
#[doc(alias = "EVENTS_ENDRX")]
pub type EventsEndrx = crate::Reg<events_endrx::EventsEndrxSpec>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_END (rw) register accessor: End of RXD buffer and TXD buffer reached\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "EVENTS_ENDTX (rw) register accessor: End of TXD buffer reached\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_endtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_endtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_endtx`]
module"]
#[doc(alias = "EVENTS_ENDTX")]
pub type EventsEndtx = crate::Reg<events_endtx::EventsEndtxSpec>;
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "EVENTS_STARTED (rw) register accessor: Transaction started\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`events_started::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`events_started::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_started`]
module"]
#[doc(alias = "EVENTS_STARTED")]
pub type EventsStarted = crate::Reg<events_started::EventsStartedSpec>;
#[doc = "Transaction started"]
pub mod events_started;
#[doc = "PUBLISH_STOPPED (rw) register accessor: Publish configuration for event STOPPED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_stopped::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_stopped::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_stopped`]
module"]
#[doc(alias = "PUBLISH_STOPPED")]
pub type PublishStopped = crate::Reg<publish_stopped::PublishStoppedSpec>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_ENDRX (rw) register accessor: Publish configuration for event ENDRX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_endrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_endrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endrx`]
module"]
#[doc(alias = "PUBLISH_ENDRX")]
pub type PublishEndrx = crate::Reg<publish_endrx::PublishEndrxSpec>;
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "PUBLISH_END (rw) register accessor: Publish configuration for event END\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_end`]
module"]
#[doc(alias = "PUBLISH_END")]
pub type PublishEnd = crate::Reg<publish_end::PublishEndSpec>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_ENDTX (rw) register accessor: Publish configuration for event ENDTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_endtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_endtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_endtx`]
module"]
#[doc(alias = "PUBLISH_ENDTX")]
pub type PublishEndtx = crate::Reg<publish_endtx::PublishEndtxSpec>;
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "PUBLISH_STARTED (rw) register accessor: Publish configuration for event STARTED\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`publish_started::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`publish_started::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@publish_started`]
module"]
#[doc(alias = "PUBLISH_STARTED")]
pub type PublishStarted = crate::Reg<publish_started::PublishStartedSpec>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shorts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "ENABLE (rw) register accessor: Enable SPIM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "FREQUENCY (rw) register accessor: SPI frequency. Accuracy depends on the HFCLK source selected.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frequency::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`]
module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
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
#[doc = "CONFIG (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ORC (rw) register accessor: Over-read character. Character clocked out in case an over-read of the TXD buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`orc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`orc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@orc`]
module"]
#[doc(alias = "ORC")]
pub type Orc = crate::Reg<orc::OrcSpec>;
#[doc = "Over-read character. Character clocked out in case an over-read of the TXD buffer."]
pub mod orc;

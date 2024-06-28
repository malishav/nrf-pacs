#[doc = "Register `HFCLKSTAT` reader"]
pub type R = crate::R<HfclkstatSpec>;
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "0: HFINT - 64 MHz on-chip oscillator"]
    Hfint = 0,
    #[doc = "1: HFXO - 64 MHz clock derived from external 32 MHz crystal oscillator"]
    Hfxo = 1,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Active clock source"]
pub type SrcR = crate::BitReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            false => Src::Hfint,
            true => Src::Hfxo,
        }
    }
    #[doc = "HFINT - 64 MHz on-chip oscillator"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        *self == Src::Hfint
    }
    #[doc = "HFXO - 64 MHz clock derived from external 32 MHz crystal oscillator"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == Src::Hfxo
    }
}
#[doc = "HFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: HFXO has not been started or HFCLKSTOP task has been triggered"]
    NotRunning = 0,
    #[doc = "1: HFXO has been started (HFCLKSTARTED event has been generated)"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - HFCLK state"]
pub type StateR = crate::BitReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> State {
        match self.bits {
            false => State::NotRunning,
            true => State::Running,
        }
    }
    #[doc = "HFXO has not been started or HFCLKSTOP task has been triggered"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "HFXO has been started (HFCLKSTARTED event has been generated)"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bit 0 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - HFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfclkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfclkstatSpec;
impl crate::RegisterSpec for HfclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfclkstat::R`](R) reader structure"]
impl crate::Readable for HfclkstatSpec {}
#[doc = "`reset()` method sets HFCLKSTAT to value 0"]
impl crate::Resettable for HfclkstatSpec {
    const RESET_VALUE: u32 = 0;
}

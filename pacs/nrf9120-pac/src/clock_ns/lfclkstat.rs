#[doc = "Register `LFCLKSTAT` reader"]
pub type R = crate::R<LfclkstatSpec>;
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: Reserved for future use"]
    Rfu = 0,
    #[doc = "1: 32.768 kHz RC oscillator"]
    Lfrc = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    Lfxo = 2,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Active clock source"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::Rfu),
            1 => Some(Src::Lfrc),
            2 => Some(Src::Lfxo),
            _ => None,
        }
    }
    #[doc = "Reserved for future use"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == Src::Rfu
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == Src::Lfrc
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == Src::Lfxo
    }
}
#[doc = "LFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    #[doc = "0: Requested LFCLK source has not been started or LFCLKSTOP task has been triggered"]
    NotRunning = 0,
    #[doc = "1: Requested LFCLK source has been started (LFCLKSTARTED event has been generated)"]
    Running = 1,
}
impl From<State> for bool {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - LFCLK state"]
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
    #[doc = "Requested LFCLK source has not been started or LFCLKSTOP task has been triggered"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == State::NotRunning
    }
    #[doc = "Requested LFCLK source has been started (LFCLKSTARTED event has been generated)"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == State::Running
    }
}
impl R {
    #[doc = "Bits 0:1 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkstatSpec;
impl crate::RegisterSpec for LfclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclkstat::R`](R) reader structure"]
impl crate::Readable for LfclkstatSpec {}
#[doc = "`reset()` method sets LFCLKSTAT to value 0"]
impl crate::Resettable for LfclkstatSpec {
    const RESET_VALUE: u32 = 0;
}

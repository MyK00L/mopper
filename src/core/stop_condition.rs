use std::ops::{Add, Sub};

use crate::core::Objective;

/// timer trait to use as stopping conditions for solvers
pub trait Timer: Clone + Default {
    type Instant: Sub<Output = std::time::Duration>
        + Add<std::time::Duration, Output = Self::Instant>
        + Ord
        + Copy;
    fn time(&self) -> Self::Instant;
}
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[derive(Clone, Default)]
pub struct RdtscTimer<const TICKS_PER_SEC: u64>;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RdtscTimerInstant<const TICKS_PER_SEC: u64>(u64);
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
impl<const TICKS_PER_SEC: u64> Sub for RdtscTimerInstant<TICKS_PER_SEC> {
    type Output = std::time::Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        let ticks = self.0 - rhs.0;
        let secs = ticks / TICKS_PER_SEC;
        let nanos = (ticks % TICKS_PER_SEC) * 1_000_000_000 / TICKS_PER_SEC;
        std::time::Duration::new(secs, nanos as u32)
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
impl<const TICKS_PER_SEC: u64> Add<std::time::Duration> for RdtscTimerInstant<TICKS_PER_SEC> {
    type Output = Self;
    fn add(self, rhs: std::time::Duration) -> Self::Output {
        let ticks = rhs.as_secs() * TICKS_PER_SEC
            + (rhs.subsec_nanos() as u64 * TICKS_PER_SEC) / 1_000_000_000;
        Self(self.0 + ticks)
    }
}
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
impl<const TICKS_PER_SEC: u64> Timer for RdtscTimer<TICKS_PER_SEC> {
    type Instant = RdtscTimerInstant<TICKS_PER_SEC>;
    fn time(&self) -> Self::Instant {
        #[cfg(target_arch = "x86_64")]
        return RdtscTimerInstant::<TICKS_PER_SEC>(unsafe { core::arch::x86_64::_rdtsc() });
        #[cfg(target_arch = "x86")]
        return RdtscTimerInstant::<TICKS_PER_SEC>(unsafe { core::arch::x86::_rdtsc() });
    }
}

#[derive(Clone, Default)]
pub struct StdTimer;
impl Timer for StdTimer {
    type Instant = std::time::Instant;
    fn time(&self) -> Self::Instant {
        std::time::Instant::now()
    }
}

pub type DefaultTimer = StdTimer;
// TODO: use RdtscTimer instead

/// represent a stopping condition for solvers
/// could be called many times, so it should be fast to add low overhead
pub trait StopCondition<Obj: Objective>: Clone {
    fn stop(&mut self, primal_bound: Obj, dual_bound: Obj) -> bool;
}

pub struct TimeStop<T: Timer> {
    timer: T,
    start: T::Instant,
    duration: std::time::Duration,
}
impl<T: Timer> Clone for TimeStop<T> {
    fn clone(&self) -> Self {
        let time = self.timer.time();
        Self {
            timer: self.timer.clone(),
            start: time,
            duration: self.duration,
        }
    }
}
impl<T: Timer> TimeStop<T> {
    pub fn new(timer: T, duration: std::time::Duration) -> Self {
        let time = timer.time();
        Self {
            timer,
            start: time,
            duration,
        }
    }
}
impl<T: Timer, Obj: Objective> StopCondition<Obj> for TimeStop<T> {
    fn stop(&mut self, _primal_bound: Obj, _dual_bound: Obj) -> bool {
        self.timer.time() >= self.start + self.duration
    }
}

// TODO: more stop conditions

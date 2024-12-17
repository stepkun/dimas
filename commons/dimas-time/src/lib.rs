// Copyright © 2024 Stephan Kunz
#![no_std]

//! Module `dimas-time` provides a set of `Timer` variants which can be created using the `TimerBuilder`.
//! When fired, a `Timer` calls his assigned `TimerCallback`.

#[doc(hidden)]
extern crate alloc;

mod error;
mod interval_timer;
mod interval_timer_parameter;
mod timer;
mod timer_lib;
mod timer_variant;

// region:    --- modules
use alloc::{boxed::Box, sync::Arc};
use anyhow::Result;
use dimas_core::Agent;
use futures::future::BoxFuture;
#[cfg(feature = "std")]
use tokio::sync::Mutex;
// endregion:   --- modules

// region:		--- types
/// Type definition for the functions called by a timer
pub type TimerCallback =
	Box<dyn FnMut(Agent) -> BoxFuture<'static, Result<()>> + Send + Sync + 'static>;

/// Type definition for a timers atomic reference counted callback
/// @ TODO: remove pub if possible
pub type ArcTimerCallback = Arc<Mutex<TimerCallback>>;
// endregion:	--- types

// flatten
pub use interval_timer::IntervalTimer;
//pub use interval_timer_old::IntervalTimerOld;
pub use interval_timer_parameter::IntervalTimerParameter;
pub use timer::{Timer, TimerFactory};
pub use timer_lib::TimerLib;
//pub use timer_old::TimerOld;
pub use timer_variant::TimerVariant;
// endregion: --- modules

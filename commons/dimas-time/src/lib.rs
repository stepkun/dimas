// Copyright © 2024 Stephan Kunz
#![no_std]

//! Module `dimas-time` provides a set of `Timer` variants which can be created using the `TimerBuilder`.
//! When fired, a `Timer` calls his assigned `TimerCallback`.

#[doc(hidden)]
extern crate alloc;

// region:    --- modules
mod error;
mod interval_timer;
mod interval_timer_parameter;
mod timer;
mod timer_lib;
mod timer_variant;

use alloc::sync::Arc;
use anyhow::Result;
use dimas_core::traits::Context;
use parking_lot::Mutex;
// endregion:   --- modules

// region:		--- types
/// type definition for the functions called by a timer
/// @ TODO: remove pub if possible
pub type ArcTimerCallback<P> =
	Arc<Mutex<dyn FnMut(Context<P>) -> Result<()> + Send + Sync + 'static>>;
// endregion:	--- types

// flatten
pub use interval_timer::IntervalTimer;
pub use interval_timer_parameter::IntervalTimerParameter;
pub use timer::Timer;
pub use timer_lib::TimerLib;
pub use timer_variant::TimerVariant;
// endregion: --- modules

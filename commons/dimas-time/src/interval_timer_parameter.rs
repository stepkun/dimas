// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `timer_builder` provides a builder for the different [`TimerVariant`]s

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;

#[cfg(doc)]
use crate::IntervalTimer;

use super::{ArcTimerCallback, TimerVariant};
// endregion:   --- modules

// region:      --- TimerBuilder
/// Parameters for an [`IntervalTimer`]
#[dimas_macros::parameter]
pub struct IntervalTimerParameter {
	/// The interval in which the Timer is fired
	/// The default value is 1 seconds
	pub(crate) interval: Duration,
	/// The optional delay
	pub(crate) delay: Option<Duration>,
}

impl Default for IntervalTimerParameter {
	fn default() -> Self {
		Self {
			interval: Duration::from_millis(1000),
			delay: None,
		}
	}
}

impl IntervalTimerParameter {
	/// Create an [`IntervalTimerParameter`] set with 
	/// an `interval` and an optional 'delay'
	#[must_use]
	pub const fn new(interval: Duration, delay: Option<Duration>) -> Self {
		Self {
			interval,
			delay,
		}
	}
}
// endregion:   --- TimerBuilder

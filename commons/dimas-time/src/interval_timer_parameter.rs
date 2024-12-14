// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `interval_timer_parameter` provides a parameter struct for [`IntervalTimer`]s

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::{traits::Context, OperationState, OperationalData};

#[cfg(doc)]
use crate::IntervalTimerOld;
// endregion:   --- modules

// region:      --- IntervalTimerParameter
/// Parameters for an [`IntervalTimer`]
#[dimas_macros::parameter]
pub struct IntervalTimerParameter {
	/// The interval in which the Timer is fired
	/// The default value is 1 seconds
	pub(crate) interval: Duration,
	/// The optional delay
	pub(crate) delay: Option<Duration>,
	/// The [`OperationalData`]
	pub(crate) operational: OperationalData,
}

impl Default for IntervalTimerParameter {
	#[inline]
	fn default() -> Self {
		Self::new(
			Duration::from_millis(1000),
			None,
			OperationalData::default(),
		)
	}
}

impl IntervalTimerParameter {
	/// Create an [`IntervalTimerParameter`] set with
	/// an `interval` and an optional 'delay'
	#[inline]
	#[must_use]
	pub const fn new(
		interval: Duration,
		delay: Option<Duration>,
		operational: OperationalData,
	) -> Self {
		Self {
			interval,
			delay,
			operational,
		}
	}
}
// endregion:   --- IntervalTimerParameter

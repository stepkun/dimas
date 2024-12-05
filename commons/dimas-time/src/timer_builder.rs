// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `timer_builder` provides a builder for the different [`TimerVariant`]s

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;

use super::{ArcTimerCallback, TimerVariant};
// endregion:   --- modules

// region:      --- TimerBuilder
/// Builder for [`TimerVariant`]s
pub struct TimerBuilder<P>
where
	P: Send + Sync + 'static,
{
	/// The interval in which the Timer is fired
	interval: Option<Duration>,
	/// The optional delay
	delay: Option<Duration>,
	/// Timers Callback function called, when Timer is fired
	callback: Option<ArcTimerCallback<P>>,
	/// Context for the Timer
	context: Option<Context<P>>,
}

impl<P> Default for TimerBuilder<P>
where
	P: Send + Sync + 'static,
{
	fn default() -> Self {
		Self {
			interval: None,
			delay: None,
			callback: None,
			context: None,
		}
	}
}

impl<P> TimerBuilder<P>
where
	P: Send + Sync + 'static,
{
	/// Create a [`TimerBuilder`]
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}
}
// endregion:   --- TimerBuilder

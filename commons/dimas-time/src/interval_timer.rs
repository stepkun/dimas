// Copyright © 2023 Stephan Kunz
#![allow(unused)]

//! Module `interval_timer` provides an [`IntervalTimer`].
//! When fired, an [`IntervalTimer`] calls his assigned [`TimerCallback`].
//! The timer may have a delay for first start.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, string::String, sync::Arc};
use anyhow::Result;
use core::{fmt::Debug, future::Future, time::Duration};
use dimas_core::{
	enums::TaskSignal, traits::Context, Activity, ActivityData, ActivityId, ActivityType, Agent, OperationState, Operational, OperationalType, Transitions
};
#[cfg(feature = "std")]
use tokio::{sync::Mutex, task::JoinHandle, time};
use tracing::{error, event, info, instrument, warn, Level};

use crate::{IntervalTimerParameter, Timer, TimerCallback};

use super::ArcTimerCallback;
// endregion:	--- modules

// region:		--- IntervalTimer
/// A timer that fires in regular intervals.
/// May have a delay otherwise it fires at once.
//#[dimas_macros::activity]
pub struct IntervalTimer {
	/// The timers parameter
	parameter: IntervalTimerParameter,
	/// Timers Callback function called, when Timer is fired
	callback: ArcTimerCallback,
	/// The handle to stop the Timer
	handle: parking_lot::Mutex<Option<JoinHandle<()>>>,
}

impl Activity for IntervalTimer {
	fn id(&self) -> ActivityId {
		String::from("IntervalTimer")
	}
}

impl Debug for IntervalTimer {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Timer")
			.field("interval", &self.parameter.interval)
			.field("delay", &self.parameter.delay)
			.finish_non_exhaustive()
	}
}

impl Operational for IntervalTimer {
	fn activation_state(&self) -> OperationState {
		self.parameter.activity.operational.activation
	}

	fn set_activation_state(&mut self, state: OperationState) {
		self.parameter.activity.operational.activation = state;
	}

	fn state(&self) -> OperationState {
		self.parameter.activity.operational.current
	}

	fn set_state(&mut self, state: OperationState) {
		self.parameter.activity.operational.current = state;
	}
}

impl Timer for IntervalTimer {}

impl Transitions for IntervalTimer {
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		//let key = self.id();
		let delay = self.parameter.delay;
		let interval = self.parameter.interval;
		let cb = self.callback.clone();
		let ctx = self.parameter.activity.ctx.clone().expect("snh");
		//let ctx1 = self.context.clone();

		self.handle
			.lock()
			.replace(tokio::task::spawn(async move {
				// if there is a delay, wait
				if let Some(delay) = delay {
					tokio::time::sleep(delay).await;
				}
				run_timer(interval, cb, ctx).await;
			}));
		Ok(())
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn deactivate(&mut self) -> Result<()> {
		let handle = self.handle.lock().take();
		if let Some(handle) = handle {
			handle.abort();
		};
		Ok(())
	}
}

impl IntervalTimer {
	/// Constructor for an [`IntervalTimer`]
	#[must_use]
	pub fn new<CB, F>(parameter: IntervalTimerParameter, mut callback: CB) -> Self
	where
		CB: FnMut(Agent) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		// wrap the callback
		let callback: TimerCallback = Box::new(move |ctx| Box::pin(callback(ctx)));
		let callback: ArcTimerCallback = Arc::new(Mutex::new(callback));

		Self {
			parameter,
			callback,
			handle: parking_lot::Mutex::new(None),
		}
	}
}

#[instrument(name="timer", level = Level::TRACE, skip_all)]
async fn run_timer(interval: Duration, cb: ArcTimerCallback, ctx: Agent) {
	let mut interval = time::interval(interval);
	loop {
		let ctx = ctx.clone();
		interval.tick().await;

		let mut lock = cb.lock().await;
		if let Err(error) = lock(ctx).await {
			error!("timer callback failed with {error}");
		}
	}
}
// endregion:	--- IntervalTimer

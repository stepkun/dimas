// Copyright © 2023 Stephan Kunz

//! Module `interval_timer` provides an `IntervalTimer`.
//! When fired, an `IntervalTimer` calls his assigned `TimerCallback`.
//! The timer may have a delay at first start.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, string::String, sync::Arc};
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::{
	enums::TaskSignal, traits::Context, Activity, ActivityType, OperationState, Operational,
	OperationalType, Transitions,
};
use parking_lot::Mutex;
#[cfg(feature = "std")]
use tokio::{task::JoinHandle, time};
use tracing::{error, event, info, instrument, warn, Level};

use crate::IntervalTimerParameter;

use super::ArcTimerCallbackOld;
// endregion:	--- modules

// region:		--- IntervalTimer
/// A timer that fires in regular intervals.
/// May have a delay otherwise it fires at once.
#[dimas_macros::activity]
pub struct IntervalTimerOld<P>
where
	P: Send + Sync + 'static,
{
	/// The timers parameter
	parameter: IntervalTimerParameter,
	/// Timers Callback function called, when Timer is fired
	callback: ArcTimerCallbackOld<P>,
	/// Context for the Timer
	context: Context<P>,
	/// The handle to stop the Timer
	handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl<P> Debug for IntervalTimerOld<P>
where
	P: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Timer")
			.field("interval", &self.parameter.interval)
			.field("delay", &self.parameter.delay)
			.finish_non_exhaustive()
	}
}

impl<P> Transitions for IntervalTimerOld<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		let key = self.id();
		let delay = self.parameter.delay;
		let interval = self.parameter.interval;
		let cb = self.callback.clone();
		let ctx1 = self.context.clone();
		let ctx2 = self.context.clone();

		self.handle
			.lock()
			.replace(tokio::task::spawn(async move {
				std::panic::set_hook(Box::new(move |reason| {
					error!("delayed timer panic: {}", reason);
					if let Err(reason) = ctx1
						.sender()
						.blocking_send(TaskSignal::RestartTimer(key.clone()))
					{
						error!("could not restart timer: {}", reason);
					} else {
						info!("restarting timer!");
					};
				}));
				// if there is a delay, wait
				if let Some(delay) = delay {
					tokio::time::sleep(delay).await;
				}
				run_timer(interval, cb, ctx2).await;
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

impl<P> IntervalTimerOld<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for an [`IntervalTimer`]
	#[must_use]
	pub fn new(
		activity: ActivityType,
		operational: OperationalType,
		parameter: IntervalTimerParameter,
		callback: ArcTimerCallbackOld<P>,
		context: Context<P>,
	) -> Self {
		Self {
			activity,
			operational,
			parameter,
			callback,
			context,
			handle: Arc::new(Mutex::new(None)),
		}
	}
}

#[instrument(name="timer", level = Level::TRACE, skip_all)]
async fn run_timer<P>(interval: Duration, cb: ArcTimerCallbackOld<P>, ctx: Context<P>)
where
	P: Send + Sync + 'static,
{
	let mut interval = time::interval(interval);
	loop {
		let ctx = ctx.clone();
		interval.tick().await;

		if let Err(error) = cb.lock()(ctx) {
			error!("timer callback failed with {error}");
		}
	}
}
// endregion:	--- IntervalTimer

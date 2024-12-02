// Copyright © 2023 Stephan Kunz

//! Module `timer` provides a set of `Timer` variants which can be created using the `TimerBuilder`.
//! When fired, a `Timer` calls his assigned `TimerCallback`.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::{boxed::Box, string::String, sync::Arc};
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::{
	enums::TaskSignal, traits::Context, Component, ComponentType, OperationState, Operational,
	Transitions,
};
#[cfg(feature = "std")]
use parking_lot::Mutex;
#[cfg(feature = "std")]
use tokio::{task::JoinHandle, time};
use tracing::{error, info, instrument, warn, Level};
// endregion:	--- modules

// region:		--- types
/// type definition for the functions called by a timer
pub type ArcTimerCallback<P> =
	Arc<Mutex<dyn FnMut(Context<P>) -> Result<()> + Send + Sync + 'static>>;
// endregion:	--- types

// region:		--- Timer
/// Timer
pub struct Timer<P>
where
	P: Send + Sync + 'static,
{
	/// Inheritance of necessary fields & methods for [`Component`]
	component: ComponentType,
	/// Context for the Timer
	context: Context<P>,
	/// Timers Callback function called, when Timer is fired
	callback: ArcTimerCallback<P>,
	/// The interval in which the Timer is fired
	interval: Duration,
	/// The optional delay
	delay: Option<Duration>,
	/// The handle to stop the Timer
	handle: Mutex<Option<JoinHandle<()>>>,
}

impl<P> Debug for Timer<P>
where
	P: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Timer")
			.field("interval", &self.interval)
			.field("delay", &self.delay)
			.finish_non_exhaustive()
	}
}

impl<P> Transitions for Timer<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		let key = self.component.id();
		let delay = self.delay;
		let interval = self.interval;
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

impl<P> Operational for Timer<P>
where
	P: Send + Sync + 'static,
{
	fn activation_state(&self) -> OperationState {
		self.component.activation_state()
	}

	fn desired_state(&self, state: OperationState) -> OperationState {
		self.component.desired_state(state)
	}

	fn state(&self) -> OperationState {
		self.component.state()
	}

	fn set_state(&mut self, state: OperationState) {
		self.component.set_state(state);
	}

	fn set_activation_state(&mut self, _state: OperationState) {
		todo!()
	}
}

impl<P> Timer<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for a [Timer]
	#[must_use]
	pub fn new(
		name: impl Into<String>,
		context: Context<P>,
		activation_state: OperationState,
		callback: ArcTimerCallback<P>,
		interval: Duration,
		delay: Option<Duration>,
	) -> Self {
		Self {
			component: ComponentType::with_activation_state(name.into(), activation_state),
			context,
			delay,
			interval,
			callback,
			handle: Mutex::new(None),
		}
	}
}

#[instrument(name="timer", level = Level::ERROR, skip_all)]
async fn run_timer<P>(interval: Duration, cb: ArcTimerCallback<P>, ctx: Context<P>)
where
	P: Send + Sync + 'static,
{
	let mut interval = time::interval(interval);
	loop {
		let ctx = ctx.clone();
		interval.tick().await;

		if let Err(error) = cb.lock()(ctx) {
			error!("callback failed with {error}");
		}
	}
}
// endregion:	--- Timer

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug)]
	struct Props {}

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Timer<Props>>();
	}
}

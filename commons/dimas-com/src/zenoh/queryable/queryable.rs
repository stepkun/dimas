// Copyright © 2023 Stephan Kunz

//! Module `queryable` provides an information/compute provider `Queryable` which can be created using the `QueryableBuilder`.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::sync::Arc;
use alloc::{boxed::Box, string::String};
use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{
	enums::TaskSignal, message_types::QueryMsg, traits::Context, Activity, ActivityType,
	OperationState, Operational, OperationalType, Transitions,
};
use futures::future::BoxFuture;
#[cfg(feature = "std")]
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::{error, event, info, instrument, warn, Level};
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;
use zenoh::Session;

use super::QueryableParameter;
// endregion:	--- modules

// region:    	--- types
/// type defnition for a queryables `request` callback
pub type GetCallback<P> =
	Box<dyn FnMut(Context<P>, QueryMsg) -> BoxFuture<'static, Result<()>> + Send + Sync>;
/// type defnition for a queryables atomic reference counted `request` callback
pub type ArcGetCallback<P> = Arc<Mutex<GetCallback<P>>>;
// endregion: 	--- types

// region:		--- Queryable
/// Queryable
#[dimas_macros::activity]
pub struct Queryable<P>
where
	P: Send + Sync + 'static,
{
	selector: String,
	parameter: QueryableParameter,
	/// the zenoh session this queryable belongs to
	session: Arc<Session>,
	/// Context for the Subscriber
	context: Context<P>,
	callback: ArcGetCallback<P>,
	handle: parking_lot::Mutex<Option<JoinHandle<()>>>,
}

impl<P> Debug for Queryable<P>
where
	P: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Queryable")
			.field("selector", &self.selector)
			.field("complete", &self.parameter.completeness)
			.finish_non_exhaustive()
	}
}

impl<P> crate::traits::Responder for Queryable<P>
where
	P: Send + Sync + 'static,
{
	/// Get `selector`
	fn selector(&self) -> &str {
		&self.selector
	}
}

impl<P> Transitions for Queryable<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		let selector = self.selector.clone();
		let cb = self.callback.clone();
		let ctx1 = self.context.clone();
		let ctx2 = self.context.clone();
		let session = self.session.clone();
		let parameter = self.parameter.clone();

		self.handle
			.lock()
			.replace(tokio::task::spawn(async move {
				let key = selector.clone();
				std::panic::set_hook(Box::new(move |reason| {
					error!("queryable panic: {}", reason);
					if let Err(reason) = ctx1
						.sender()
						.blocking_send(TaskSignal::RestartQueryable(key.clone()))
					{
						error!("could not restart queryable: {}", reason);
					} else {
						info!("restarting queryable!");
					};
				}));
				if let Err(error) = run_queryable(session, selector, parameter, cb, ctx2).await {
					error!("queryable failed with {error}");
				};
			}));
		Ok(())
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn deactivate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "deactivate");
		self.handle.lock().take();
		Ok(())
	}
}

impl<P> Queryable<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for a [`Queryable`]
	#[must_use]
	pub fn new(
		activity: ActivityType,
		selector: impl Into<String>,
		parameter: QueryableParameter,
		session: Arc<Session>,
		context: Context<P>,
		request_callback: ArcGetCallback<P>,
	) -> Self {
		Self {
			activity,
			selector: selector.into(),
			parameter,
			session,
			context,
			callback: request_callback,
			handle: parking_lot::Mutex::new(None),
		}
	}
}

#[instrument(name="queryable", level = Level::ERROR, skip_all)]
async fn run_queryable<P>(
	session: Arc<Session>,
	selector: String,
	parameter: QueryableParameter,
	callback: ArcGetCallback<P>,
	ctx: Context<P>,
) -> core::result::Result<(), Box<dyn core::error::Error + Send + Sync + 'static>>
where
	P: Send + Sync + 'static,
{
	let builder = session
		.declare_queryable(&selector)
		.complete(parameter.completeness);
	#[cfg(feature = "unstable")]
	let builder = builder.allowed_origin(allowed_origin);

	let queryable = builder.await?;

	loop {
		let query = queryable.recv_async().await?;
		let request = QueryMsg(query);

		let ctx = ctx.clone();
		let mut lock = callback.lock().await;
		if let Err(error) = lock(ctx, request).await {
			error!("queryable callback failed with {error}");
		}
	}
}
// endregion:	--- Queryable

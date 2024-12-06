// Copyright © 2023 Stephan Kunz

//! Module `subscriber` provides a message `Subscriber` which can be created using the `SubscriberBuilder`.
//! A `Subscriber` can optional subscribe on a delete message.

// region:		--- modules
use crate::error::Error;
use anyhow::Result;
use dimas_core::Transitions;
use dimas_core::{
	enums::TaskSignal, message_types::Message, traits::Context, Activity, ActivityType,
	OperationState, Operational, OperationalType,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::{error, event, info, instrument, warn, Level};
use zenoh::sample::SampleKind;
use zenoh::Session;

use super::SubscriberParameter;
// endregion:	--- modules

// region:    	--- types
/// Type definition for a subscribers `put` callback
pub type PutCallback<P> =
	Box<dyn FnMut(Context<P>, Message) -> BoxFuture<'static, Result<()>> + Send + Sync>;
/// Type definition for a subscribers atomic reference counted `put` callback
pub type ArcPutCallback<P> = Arc<Mutex<PutCallback<P>>>;
/// Type definition for a subscribers `delete` callback
pub type DeleteCallback<P> =
	Box<dyn FnMut(Context<P>) -> BoxFuture<'static, Result<()>> + Send + Sync>;
/// Type definition for a subscribers atomic reference counted `delete` callback
pub type ArcDeleteCallback<P> = Arc<Mutex<DeleteCallback<P>>>;
// endregion: 	--- types

// region:		--- Subscriber
/// Subscriber
#[dimas_macros::activity]
pub struct Subscriber<P>
where
	P: Send + Sync + 'static,
{
	/// the zenoh session this subscriber belongs to
	session: Arc<Session>,
	/// The subscribers key expression
	selector: String,
	/// Subscriber parameter
	parameter: SubscriberParameter,
	/// Context for the Subscriber
	context: Context<P>,
	put_callback: ArcPutCallback<P>,
	delete_callback: Option<ArcDeleteCallback<P>>,
	handle: parking_lot::Mutex<Option<JoinHandle<()>>>,
}

impl<P> core::fmt::Debug for Subscriber<P>
where
	P: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Subscriber")
			.field("selector", &self.selector)
			.finish_non_exhaustive()
	}
}

impl<P> crate::traits::Responder for Subscriber<P>
where
	P: Send + Sync + 'static,
{
	/// Get `selector`
	fn selector(&self) -> &str {
		&self.selector
	}
}

impl<P> Transitions for Subscriber<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		let selector = self.selector.clone();
		let p_cb = self.put_callback.clone();
		let d_cb = self.delete_callback.clone();
		let ctx1 = self.context.clone();
		let ctx2 = self.context.clone();
		let session = self.session.clone();
		let parameter = self.parameter.clone();

		self.handle
			.lock()
			.replace(tokio::task::spawn(async move {
				let key = selector.clone();
				std::panic::set_hook(Box::new(move |reason| {
					error!("subscriber panic: {}", reason);
					if let Err(reason) = ctx1
						.sender()
						.blocking_send(TaskSignal::RestartSubscriber(key.clone()))
					{
						error!("could not restart subscriber: {}", reason);
					} else {
						info!("restarting subscriber!");
					};
				}));
				if let Err(error) =
					run_subscriber(session, selector, parameter, p_cb, d_cb, ctx2).await
				{
					error!("spawning subscriber failed with {error}");
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

impl<P> Subscriber<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for a [`Subscriber`].
	#[must_use]
	pub fn new(
		activity: ActivityType,
		selector: impl Into<String>,
		parameter: SubscriberParameter,
		session: Arc<Session>,
		context: Context<P>,
		put_callback: ArcPutCallback<P>,
		delete_callback: Option<ArcDeleteCallback<P>>,
	) -> Self {
		Self {
			activity,
			selector: selector.into(),
			parameter,
			session,
			context,
			put_callback,
			delete_callback,
			handle: parking_lot::Mutex::new(None),
		}
	}
}

#[allow(unused_variables)]
#[instrument(name="subscriber", level = Level::ERROR, skip_all)]
async fn run_subscriber<P>(
	session: Arc<Session>,
	selector: String,
	parameter: SubscriberParameter,
	p_cb: ArcPutCallback<P>,
	d_cb: Option<ArcDeleteCallback<P>>,
	ctx: Context<P>,
) -> core::result::Result<(), Box<dyn core::error::Error + Send + Sync + 'static>>
where
	P: Send + Sync + 'static,
{
	let builder = session.declare_subscriber(&selector);

	#[cfg(feature = "unstable")]
	let builder = builder.allowed_origin(parameter.allowed_origin);

	let subscriber = builder.await?;

	loop {
		let sample = subscriber
			.recv_async()
			.await
			.map_err(|source| Error::SubscriberCreation { source })?;

		match sample.kind() {
			SampleKind::Put => {
				let content: Vec<u8> = sample.payload().to_bytes().into_owned();
				let msg = Message::new(content);
				let mut lock = p_cb.lock().await;
				let ctx = ctx.clone();
				if let Err(error) = lock(ctx, msg).await {
					error!("subscriber put callback failed with {error}");
				}
			}
			SampleKind::Delete => {
				if let Some(cb) = d_cb.clone() {
					let ctx = ctx.clone();
					let mut lock = cb.lock().await;
					if let Err(error) = lock(ctx).await {
						error!("subscriber delete callback failed with {error}");
					}
				}
			}
		}
	}
}
// endregion:	--- Subscriber

// Copyright © 2023 Stephan Kunz

//! Module `subscriber` provides a message `Subscriber` which can be created using the `SubscriberBuilder`.
//! A `Subscriber` can optional subscribe on a delete message.

// region:		--- modules
use crate::error::Error;
use anyhow::Result;
use dimas_core::Transitions;
use dimas_core::{
	enums::TaskSignal, message_types::Message, traits::Context, OperationState, Operational,
};
use futures::future::BoxFuture;
use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};
use tracing::{error, event, info, instrument, warn, Level};
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;
use zenoh::sample::SampleKind;
use zenoh::Session;
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
pub struct Subscriber<P>
where
	P: Send + Sync + 'static,
{
	/// The current state for [`Operational`]
	current_state: OperationState,
	/// the zenoh session this subscriber belongs to
	session: Arc<Session>,
	/// The subscribers key expression
	selector: String,
	/// Context for the Subscriber
	context: Context<P>,
	/// The state from parent, at which [`OperationState::Active`] should be reached
	activation_state: OperationState,
	#[cfg(feature = "unstable")]
	allowed_origin: Locality,
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
		#[cfg(feature = "unstable")]
		let allowed_origin = self.allowed_origin;

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
				if let Err(error) = run_subscriber(
					session,
					selector,
					#[cfg(feature = "unstable")]
					allowed_origin,
					p_cb,
					d_cb,
					ctx2.clone(),
				)
				.await
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

impl<P> Operational for Subscriber<P>
where
	P: Send + Sync + 'static,
{
	fn activation_state(&self) -> OperationState {
		self.activation_state
	}

	fn desired_state(&self, _state: OperationState) -> OperationState {
		todo!()
	}

	fn state(&self) -> OperationState {
		self.current_state
	}

	fn set_state(&mut self, state: OperationState) {
		self.current_state = state;
	}

	fn set_activation_state(&mut self, _state: OperationState) {
		todo!()
	}
}

impl<P> Subscriber<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for a [`Subscriber`].
	#[must_use]
	pub fn new(
		session: Arc<Session>,
		selector: impl Into<String>,
		context: Context<P>,
		activation_state: OperationState,
		#[cfg(feature = "unstable")] allowed_origin: Locality,
		put_callback: ArcPutCallback<P>,
		delete_callback: Option<ArcDeleteCallback<P>>,
	) -> Self {
		Self {
			current_state: OperationState::default(),
			session,
			selector: selector.into(),
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			put_callback,
			delete_callback,
			handle: parking_lot::Mutex::new(None),
		}
	}
}

#[instrument(name="subscriber", level = Level::ERROR, skip_all)]
async fn run_subscriber<P>(
	session: Arc<Session>,
	selector: String,
	#[cfg(feature = "unstable")] allowed_origin: Locality,
	p_cb: ArcPutCallback<P>,
	d_cb: Option<ArcDeleteCallback<P>>,
	ctx: Context<P>,
) -> core::result::Result<(), Box<dyn core::error::Error + Send + Sync + 'static>>
where
	P: Send + Sync + 'static,
{
	let builder = session.declare_subscriber(&selector);

	#[cfg(feature = "unstable")]
	let builder = builder.allowed_origin(allowed_origin);

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

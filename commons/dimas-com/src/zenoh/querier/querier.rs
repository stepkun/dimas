// Copyright © 2023 Stephan Kunz

//! Module `Querier` provides an information/compute requestor `Querier` which can be created using the `QuerierBuilder`.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use alloc::sync::Arc;
use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{
	message_types::{Message, QueryableMsg},
	traits::Context,
	Activity, ActivityType, OperationState, Operational, OperationalType, Transitions,
};
use futures::future::BoxFuture;
#[cfg(feature = "std")]
use std::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
#[cfg(feature = "std")]
use tokio::sync::Mutex;
use tracing::{error, event, instrument, warn, Level};
use zenoh::{sample::SampleKind, Session, Wait};

use crate::error::Error;

use super::QuerierParameter;
// endregion:	--- modules

// region:    	--- types
/// type definition for a queriers `response` callback
pub type GetCallback<P> =
	Box<dyn FnMut(Context<P>, QueryableMsg) -> BoxFuture<'static, Result<()>> + Send + Sync>;
/// type definition for a queriers atomic reference counted `response` callback
pub type ArcGetCallback<P> = Arc<Mutex<GetCallback<P>>>;
// endregion: 	--- types

// region:		--- Querier
/// Querier
#[dimas_macros::activity]
pub struct Querier<P>
where
	P: Send + Sync + 'static,
{
	selector: String,
	parameter: QuerierParameter,
	/// the zenoh session this querier belongs to
	session: Arc<Session>,
	/// Context for the Querier
	context: Context<P>,
	callback: ArcGetCallback<P>,
	handle: parking_lot::Mutex<Option<zenoh::key_expr::KeyExpr<'static>>>,
}

impl<P> Debug for Querier<P>
where
	P: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let res = f
			.debug_struct("Querier")
			.field("selector", &self.selector)
			.finish_non_exhaustive();
		res
	}
}

impl<P> crate::traits::Querier for Querier<P>
where
	P: Send + Sync + 'static,
{
	/// Get `selector`
	fn selector(&self) -> &str {
		&self.selector
	}

	/// Run a Querier with an optional [`Message`].
	#[instrument(name="Querier", level = Level::ERROR, skip_all)]
	fn get(
		&self,
		message: Option<Message>,
		mut callback: Option<&mut dyn FnMut(QueryableMsg) -> Result<()>>,
	) -> Result<()> {
		let cb = self.callback.clone();
		let key_expr = self
			.handle
			.lock()
			.clone()
			.ok_or(Error::InvalidSelector("querier".into()))?;

		let builder = message
			.map_or_else(
				|| self.session.get(&key_expr),
				|msg| {
					self.session
						.get(&self.selector)
						.payload(msg.value())
				},
			)
			.encoding(self.parameter.encoding.clone())
			.target(self.parameter.target)
			.consolidation(self.parameter.mode)
			.timeout(self.parameter.timeout);

		#[cfg(feature = "unstable")]
		let builder = builder.allowed_destination(self.parameter.allowed_destination);

		let query = builder
			.wait()
			.map_err(|source| Error::QueryCreation { source })?;

		let mut unreached = true;
		let mut retry_count = 0u8;

		while unreached && retry_count <= 5 {
			retry_count += 1;
			while let Ok(reply) = query.recv() {
				match reply.result() {
					Ok(sample) => match sample.kind() {
						SampleKind::Put => {
							let content: Vec<u8> = sample.payload().to_bytes().into_owned();
							let msg = QueryableMsg(content);
							if callback.is_none() {
								let cb = cb.clone();
								let ctx = self.context.clone();
								tokio::task::spawn(async move {
									let mut lock = cb.lock().await;
									if let Err(error) = lock(ctx, msg).await {
										error!("querier callback failed with {error}");
									}
								});
							} else {
								let callback =
									callback.as_mut().ok_or(Error::AccessingQuerier {
										selector: key_expr.to_string(),
									})?;
								callback(msg).map_err(|source| Error::QueryCallback { source })?;
							}
						}
						SampleKind::Delete => {
							error!("Delete in Querier");
						}
					},
					Err(err) => error!("receive error: {:?})", err),
				}
				unreached = false;
			}
			if unreached {
				if retry_count < 5 {
					std::thread::sleep(self.parameter.timeout);
				} else {
					return Err(Error::AccessingQueryable {
						selector: key_expr.to_string(),
					}
					.into());
				}
			}
		}

		Ok(())
	}
}

impl<P> Transitions for Querier<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		let mut key_expr = self.handle.lock();
		self.session
			.declare_keyexpr(self.selector.clone())
			.wait()
			.map_or_else(
				|_| Err(Error::Unexpected(file!().into(), line!()).into()),
				|new_key_expr| {
					key_expr.replace(new_key_expr);
					Ok(())
				},
			)
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn deactivate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "deactivate");
		self.handle.lock().take();
		Ok(())
	}
}

impl<P> Querier<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for a [`Querier`]
	#[must_use]
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		activity: ActivityType,
		operational: OperationalType,
		selector: impl Into<String>,
		parameter: QuerierParameter,
		session: Arc<Session>,
		context: Context<P>,
		callback: ArcGetCallback<P>,
	) -> Self {
		Self {
			activity,
			operational,
			selector: selector.into(),
			parameter,
			session,
			context,
			callback,
			handle: parking_lot::Mutex::new(None),
		}
	}
}
// endregion:	--- Querier

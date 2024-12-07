// Copyright © 2023 Stephan Kunz

//! Module `subscriber_builder`.

// region:		--- modules
use anyhow::Result;
use dimas_com::zenoh::subscriber::{
	ArcDeleteCallback, ArcPutCallback, DeleteCallback, PutCallback, Subscriber, SubscriberParameter,
};
use dimas_core::{
	message_types::Message, traits::Context, utils::selector_from, ActivityType, Component,
	ComponentType, OperationState, OperationalType,
};
use futures::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;

use super::{
	builder_states::{Callback, NoCallback, NoSelector, NoStorage, Selector, Storage},
	error::Error,
};
// endregion:	--- modules

// region:		--- SubscriberBuilder
/// A builder for a subscriber
#[derive(Clone)]
pub struct SubscriberBuilder<P, K, C, S>
where
	P: Send + Sync + 'static,
{
	session_id: String,
	context: Context<P>,
	activation_state: OperationState,
	#[cfg(feature = "unstable")]
	allowed_origin: Locality,
	selector: K,
	put_callback: C,
	storage: S,
	delete_callback: Option<ArcDeleteCallback<P>>,
}

impl<P> SubscriberBuilder<P, NoSelector, NoCallback, NoStorage>
where
	P: Send + Sync + 'static,
{
	/// Construct a `SubscriberBuilder` in initial state
	#[must_use]
	pub fn new(session_id: impl Into<String>, context: Context<P>) -> Self {
		Self {
			session_id: session_id.into(),
			context,
			activation_state: OperationState::Active,
			#[cfg(feature = "unstable")]
			allowed_origin: Locality::Any,
			selector: NoSelector,
			put_callback: NoCallback,
			storage: NoStorage,
			delete_callback: None,
		}
	}
}

impl<P, K, C, S> SubscriberBuilder<P, K, C, S>
where
	P: Send + Sync + 'static,
{
	/// Set the activation state.
	#[must_use]
	pub const fn activation_state(mut self, state: OperationState) -> Self {
		self.activation_state = state;
		self
	}

	/// Set the allowed origin.
	#[cfg(feature = "unstable")]
	#[must_use]
	pub const fn allowed_origin(mut self, allowed_origin: Locality) -> Self {
		self.allowed_origin = allowed_origin;
		self
	}

	/// Set subscribers callback for `delete` messages
	#[must_use]
	pub fn delete_callback<CB, F>(mut self, mut callback: CB) -> Self
	where
		CB: FnMut(Context<P>) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		let callback: DeleteCallback<P> = Box::new(move |ctx| Box::pin(callback(ctx)));
		self.delete_callback
			.replace(Arc::new(Mutex::new(callback)));
		self
	}

	/// Set the session id.
	#[must_use]
	pub fn session_id(mut self, session_id: &str) -> Self {
		self.session_id = session_id.into();
		self
	}
}

impl<P, C, S> SubscriberBuilder<P, NoSelector, C, S>
where
	P: Send + Sync + 'static,
{
	/// Set the full key expression for the [`Subscriber`].
	#[must_use]
	pub fn selector(self, selector: &str) -> SubscriberBuilder<P, Selector, C, S> {
		let Self {
			session_id,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			storage,
			put_callback,
			delete_callback,
			..
		} = self;
		SubscriberBuilder {
			session_id,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector: Selector {
				selector: selector.into(),
			},
			put_callback,
			storage,
			delete_callback,
		}
	}

	/// Set only the message qualifing part of the [`Subscriber`].
	/// Will be prefixed with `Agent`s prefix.
	#[must_use]
	pub fn topic(self, topic: &str) -> SubscriberBuilder<P, Selector, C, S> {
		let selector = selector_from(topic, self.context.prefix());
		self.selector(&selector)
	}
}

impl<P, K, S> SubscriberBuilder<P, K, NoCallback, S>
where
	P: Send + Sync + 'static,
{
	/// Set callback for put messages
	#[must_use]
	pub fn put_callback<CB, F>(
		self,
		mut callback: CB,
	) -> SubscriberBuilder<P, K, Callback<ArcPutCallback<P>>, S>
	where
		CB: FnMut(Context<P>, Message) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		let Self {
			session_id,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			storage,
			delete_callback,
			..
		} = self;
		let callback: PutCallback<P> = Box::new(move |ctx, msg| Box::pin(callback(ctx, msg)));
		let callback: ArcPutCallback<P> = Arc::new(Mutex::new(callback));
		SubscriberBuilder {
			session_id,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			put_callback: Callback { callback },
			storage,
			delete_callback,
		}
	}
}

impl<P, K, C> SubscriberBuilder<P, K, C, NoStorage>
where
	P: Send + Sync + 'static,
{
	/// Provide agents storage for the subscriber
	#[must_use]
	pub fn storage(self, storage: &mut ComponentType) -> SubscriberBuilder<P, K, C, Storage> {
		let Self {
			session_id,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			put_callback,
			delete_callback,
			..
		} = self;
		SubscriberBuilder {
			session_id,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			put_callback,
			storage: Storage { storage },
			delete_callback,
		}
	}
}

impl<P, S> SubscriberBuilder<P, Selector, Callback<ArcPutCallback<P>>, S>
where
	P: Send + Sync + 'static,
{
	/// Build the [`Subscriber`].
	///
	/// # Errors
	/// Currently none
	pub fn build(self) -> Result<Subscriber<P>> {
		let Self {
			session_id,
			selector,
			context,
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_origin,
			put_callback,
			delete_callback,
			..
		} = self;

		let session = context
			.session(&session_id)
			.ok_or(Error::NoZenohSession)?;

		let selector = selector.selector;
		let activity = ActivityType::new(selector.clone());
		let operational = OperationalType::new(activation_state);
		#[cfg(not(feature = "unstable"))]
		let parameter = SubscriberParameter::new();
		#[cfg(feature = "unstable")]
		let parameter = SubscriberParameter::new(allowed_origin);

		Ok(Subscriber::new(
			activity,
			operational,
			selector,
			parameter,
			session,
			context,
			put_callback.callback,
			delete_callback,
		))
	}
}

impl<'a, P> SubscriberBuilder<P, Selector, Callback<ArcPutCallback<P>>, Storage<'a>>
where
	P: Send + Sync + 'static,
{
	/// Build and add the [`Subscriber`] to the `Agent`.
	///
	/// # Errors
	/// Currently none
	pub fn add(self) -> Result<()> {
		let mut collection = self.storage.storage.clone();
		let s = self.build()?;
		collection.add_activity(Box::new(s));
		Ok(())
	}
}
// endregion:	--- SubscriberBuilder

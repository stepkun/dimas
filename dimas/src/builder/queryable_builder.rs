// Copyright © 2023 Stephan Kunz

//! Module `queryable_builder`.

// region:		--- modules
use anyhow::Result;
use dimas_com::zenoh::queryable::{ArcGetCallback, GetCallback, Queryable, QueryableParameter};
use dimas_core::{
	message_types::QueryMsg, traits::Context, utils::selector_from, ActivityType, OperationState,
	System, SystemType,
};
use futures::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;

use super::{
	builder_states::{Callback, NoCallback, NoSelector, NoStorage, Selector, StorageNew},
	error::Error,
};
// endregion:	--- modules

// region:		--- QueryableBuilder
/// The builder for a queryable.
#[derive(Clone)]
pub struct QueryableBuilder<P, K, C, S>
where
	P: Send + Sync + 'static,
{
	session_id: String,
	context: Context<P>,
	activation_state: OperationState,
	completeness: bool,
	#[cfg(feature = "unstable")]
	allowed_origin: Locality,
	selector: K,
	callback: C,
	storage: S,
}

impl<P> QueryableBuilder<P, NoSelector, NoCallback, NoStorage>
where
	P: Send + Sync + 'static,
{
	/// Construct a `QueryableBuilder` in initial state
	#[must_use]
	pub fn new(session_id: impl Into<String>, context: Context<P>) -> Self {
		Self {
			session_id: session_id.into(),
			context,
			activation_state: OperationState::Active,
			completeness: true,
			#[cfg(feature = "unstable")]
			allowed_origin: Locality::Any,
			selector: NoSelector,
			callback: NoCallback,
			storage: NoStorage,
		}
	}
}

impl<P, K, C, S> QueryableBuilder<P, K, C, S>
where
	P: Send + Sync + 'static,
{
	/// Set the activation state.
	#[must_use]
	pub const fn activation_state(mut self, state: OperationState) -> Self {
		self.activation_state = state;
		self
	}

	/// Set the completeness of the [`Queryable`].
	#[must_use]
	pub const fn completeness(mut self, completeness: bool) -> Self {
		self.completeness = completeness;
		self
	}

	/// Set the allowed origin of the [`Queryable`].
	#[cfg(feature = "unstable")]
	#[must_use]
	pub const fn allowed_origin(mut self, allowed_origin: Locality) -> Self {
		self.allowed_origin = allowed_origin;
		self
	}

	/// Set the session id.
	#[must_use]
	pub fn session_id(mut self, session_id: &str) -> Self {
		self.session_id = session_id.into();
		self
	}
}

impl<P, C, S> QueryableBuilder<P, NoSelector, C, S>
where
	P: Send + Sync + 'static,
{
	/// Set the full expression for the [`Queryable`].
	#[must_use]
	pub fn selector(self, selector: &str) -> QueryableBuilder<P, Selector, C, S> {
		let Self {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			storage,
			callback,
			..
		} = self;
		QueryableBuilder {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector: Selector {
				selector: selector.into(),
			},
			callback,
			storage,
		}
	}

	/// Set only the topic of the [`Queryable`].
	/// Will be prefixed with agents prefix.
	#[must_use]
	pub fn topic(self, topic: &str) -> QueryableBuilder<P, Selector, C, S> {
		let selector = selector_from(topic, self.context.prefix());
		self.selector(&selector)
	}
}

impl<P, K, S> QueryableBuilder<P, K, NoCallback, S>
where
	P: Send + Sync + 'static,
{
	/// Set callback for request messages
	#[must_use]
	pub fn callback<C, F>(
		self,
		mut callback: C,
	) -> QueryableBuilder<P, K, Callback<ArcGetCallback<P>>, S>
	where
		C: FnMut(Context<P>, QueryMsg) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		let Self {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			storage,
			..
		} = self;
		let callback: GetCallback<P> = Box::new(move |ctx, msg| Box::pin(callback(ctx, msg)));
		let callback: ArcGetCallback<P> = Arc::new(Mutex::new(callback));
		QueryableBuilder {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			callback: Callback { callback },
			storage,
		}
	}
}

impl<P, K, C> QueryableBuilder<P, K, C, NoStorage>
where
	P: Send + Sync + 'static,
{
	/// Provide agents storage for the queryable
	#[must_use]
	pub fn storage(self, storage: &mut SystemType) -> QueryableBuilder<P, K, C, StorageNew> {
		let Self {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			callback,
			..
		} = self;
		QueryableBuilder {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			callback,
			storage: StorageNew { storage },
		}
	}
}

impl<P, S> QueryableBuilder<P, Selector, Callback<ArcGetCallback<P>>, S>
where
	P: Send + Sync + 'static,
{
	/// Build the [`Queryable`]
	/// # Errors
	///
	pub fn build(self) -> Result<Queryable<P>> {
		let Self {
			session_id,
			context,
			activation_state,
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
			selector,
			callback,
			..
		} = self;

		let session = context
			.session(&session_id)
			.ok_or(Error::NoZenohSession)?;

		let selector = selector.selector;
		let activity = ActivityType::with_activation_state(selector.clone(), activation_state);
		#[cfg(not(feature = "unstable"))]
		let parameter = QueryableParameter::new(completeness);
		#[cfg(feature = "unstable")]
		let parameter = QueryableParameter::new(completeness, allowed_origin);

		Ok(Queryable::new(
			activity,
			selector,
			parameter,
			session,
			context,
			callback.callback,
		))
	}
}

impl<'a, P> QueryableBuilder<P, Selector, Callback<ArcGetCallback<P>>, StorageNew<'a>>
where
	P: Send + Sync + 'static,
{
	/// Build and add the queryable to the agents context
	/// # Errors
	///
	pub fn add(self) -> Result<()> {
		let mut collection = self.storage.storage.clone();
		let q = self.build()?;
		collection.add_activity(Box::new(q));
		Ok(())
	}
}
// endregion:	--- QueryableBuilder

// Copyright © 2023 Stephan Kunz

//! Builder for a [`LivelinessSubscriber`]
//!

// region:		--- modules
use anyhow::Result;
use dimas_com::zenoh::liveliness::{
	ArcLivelinessCallback, LivelinessCallback, LivelinessSubscriber, LivelinessSubscriberParameter,
};
use dimas_core::{
	traits::Context, utils::selector_from, ActivityType, OperationState, System, SystemType,
};
use futures::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
	builder_states::{Callback, NoCallback, NoStorage, StorageNew},
	error::Error,
};
// endregion:	--- modules

// region:		--- LivelinessSubscriberBuilder
/// The builder for the liveliness subscriber
#[allow(clippy::module_name_repetitions)]
pub struct LivelinessSubscriberBuilder<P, C, S>
where
	P: Send + Sync + 'static,
{
	session_id: String,
	token: String,
	context: Context<P>,
	activation_state: OperationState,
	put_callback: C,
	storage: S,
	delete_callback: Option<ArcLivelinessCallback<P>>,
}

impl<P> LivelinessSubscriberBuilder<P, NoCallback, NoStorage>
where
	P: Send + Sync + 'static,
{
	/// Construct a `LivelinessSubscriberBuilder` in initial state
	#[must_use]
	pub fn new(session_id: impl Into<String>, context: Context<P>) -> Self {
		//let token = context
		//	.prefix()
		//	.map_or("*".to_string(), |prefix| format!("{prefix}/*"));
		let token = selector_from("*", context.prefix());
		Self {
			session_id: session_id.into(),
			token,
			context,
			activation_state: OperationState::Created,
			put_callback: NoCallback,
			storage: NoStorage,
			delete_callback: None,
		}
	}
}

impl<P, C, S> LivelinessSubscriberBuilder<P, C, S>
where
	P: Send + Sync + 'static,
{
	/// Set the activation state.
	#[must_use]
	pub const fn activation_state(mut self, state: OperationState) -> Self {
		self.activation_state = state;
		self
	}

	/// Set a different prefix for the liveliness subscriber.
	#[must_use]
	pub fn prefix(self, prefix: &str) -> Self {
		let token = format!("{prefix}/*");
		let Self {
			session_id,
			context,
			activation_state,
			put_callback,
			storage,
			delete_callback,
			..
		} = self;
		Self {
			session_id,
			token,
			context,
			activation_state,
			put_callback,
			storage,
			delete_callback,
		}
	}

	/// Set the session id.
	#[must_use]
	pub fn session_id(mut self, session_id: &str) -> Self {
		self.session_id = session_id.into();
		self
	}

	/// Set an explicite token for the liveliness subscriber.
	#[must_use]
	pub fn token(self, token: impl Into<String>) -> Self {
		let Self {
			session_id,
			context,
			activation_state,
			put_callback,
			storage,
			delete_callback,
			..
		} = self;
		Self {
			session_id,
			token: token.into(),
			context,
			activation_state,
			put_callback,
			storage,
			delete_callback,
		}
	}

	/// Set liveliness subscribers callback for `delete` messages
	#[must_use]
	pub fn delete_callback<CB, F>(self, mut callback: CB) -> Self
	where
		CB: FnMut(Context<P>, String) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		let Self {
			session_id,
			token,
			context,
			activation_state,
			put_callback,
			storage,
			..
		} = self;

		let callback: LivelinessCallback<P> =
			Box::new(move |ctx, txt| Box::pin(callback(ctx, txt)));
		let delete_callback: Option<ArcLivelinessCallback<P>> =
			Some(Arc::new(Mutex::new(callback)));
		Self {
			session_id,
			token,
			context,
			activation_state,
			put_callback,
			storage,
			delete_callback,
		}
	}
}

impl<P, S> LivelinessSubscriberBuilder<P, NoCallback, S>
where
	P: Send + Sync + 'static,
{
	/// Set liveliness subscribers callback for `put` messages
	#[must_use]
	pub fn put_callback<CB, F>(
		self,
		mut callback: CB,
	) -> LivelinessSubscriberBuilder<P, Callback<ArcLivelinessCallback<P>>, S>
	where
		CB: FnMut(Context<P>, String) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		let Self {
			session_id,
			token,
			context,
			activation_state,
			storage,
			delete_callback,
			..
		} = self;
		let callback: LivelinessCallback<P> =
			Box::new(move |ctx, txt| Box::pin(callback(ctx, txt)));
		let put_callback: ArcLivelinessCallback<P> = Arc::new(Mutex::new(callback));
		LivelinessSubscriberBuilder {
			session_id,
			token,
			context,
			activation_state,
			put_callback: Callback {
				callback: put_callback,
			},
			storage,
			delete_callback,
		}
	}
}

impl<P, C> LivelinessSubscriberBuilder<P, C, NoStorage>
where
	P: Send + Sync + 'static,
{
	/// Provide agents storage for the liveliness subscriber
	#[must_use]
	pub fn storage(
		self,
		storage: &mut SystemType,
	) -> LivelinessSubscriberBuilder<P, C, StorageNew> {
		let Self {
			session_id,
			token,
			context,
			activation_state,
			put_callback,
			delete_callback,
			..
		} = self;
		LivelinessSubscriberBuilder {
			session_id,
			token,
			context,
			activation_state,
			put_callback,
			storage: StorageNew { storage },
			delete_callback,
		}
	}
}

impl<P, S> LivelinessSubscriberBuilder<P, Callback<ArcLivelinessCallback<P>>, S>
where
	P: Send + Sync + 'static,
{
	/// Build the [`LivelinessSubscriber`]
	/// # Errors
	///
	pub fn build(self) -> Result<LivelinessSubscriber<P>> {
		let Self {
			session_id,
			token,
			context,
			activation_state,
			put_callback,
			delete_callback,
			..
		} = self;

		let session = context
			.session(&session_id)
			.ok_or(Error::NoZenohSession)?;

		let activity = ActivityType::with_activation_state(token.clone(), activation_state);
		let parameter = LivelinessSubscriberParameter::new();
		Ok(LivelinessSubscriber::new(
			activity,
			token,
			parameter,
			session,
			context,
			put_callback.callback,
			delete_callback,
		))
	}
}

impl<'a, P> LivelinessSubscriberBuilder<P, Callback<ArcLivelinessCallback<P>>, StorageNew<'a>>
where
	P: Send + Sync + 'static,
{
	/// Build and add the liveliness subscriber to the agent
	/// # Errors
	///
	pub fn add(self) -> Result<()> {
		let mut collection = self.storage.storage.clone();
		let ls = self.build()?;
		collection.add_activity(Box::new(ls));
		Ok(())
	}
}
// endregion:	--- LivelinessSubscriberBuilder

// Copyright © 2023 Stephan Kunz

//! Module `liveliness` provides a `LivelinessSubscriber` which can be created using the `LivelinessSubscriberBuilder`.
//! A `LivelinessSubscriber` can optional subscribe on a delete message.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::error::Error;
use crate::{
	traits::LivelinessSubscriber as LivelinessSubscriberTrait,
	zenoh::liveliness::{ArcLivelinessCallback, LivelinessCallback, LivelinessSubscriber},
};
use alloc::{boxed::Box, format, string::String, sync::Arc};
use dimas_core::builder_states::{Callback, NoCallback, NoStorage, Storage};
use dimas_core::{Result, enums::OperationState, traits::Context, utils::selector_from};
use futures::future::Future;
#[cfg(feature = "std")]
use std::{collections::HashMap, sync::RwLock};
#[cfg(feature = "std")]
use tokio::sync::Mutex;
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
		storage: Arc<RwLock<HashMap<String, Box<dyn LivelinessSubscriberTrait>>>>,
	) -> LivelinessSubscriberBuilder<P, C, Storage<Box<dyn LivelinessSubscriberTrait>>> {
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
			storage: Storage { storage },
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
			.ok_or_else(|| Error::NoZenohSession)?;
		Ok(LivelinessSubscriber::new(
			session,
			token,
			context,
			activation_state,
			put_callback.callback,
			delete_callback,
		))
	}
}

impl<P>
	LivelinessSubscriberBuilder<
		P,
		Callback<ArcLivelinessCallback<P>>,
		Storage<Box<dyn LivelinessSubscriberTrait>>,
	>
where
	P: Send + Sync + 'static,
{
	/// Build and add the liveliness subscriber to the agent
	/// # Errors
	///
	pub fn add(self) -> Result<Option<Box<dyn LivelinessSubscriberTrait>>> {
		let c = self.storage.storage.clone();
		let s = self.build()?;

		let r = c
			.write()
			.map_err(|_| Error::MutexPoison(String::from("LivelinessSubscriberBuilder")))?
			.insert(s.token().into(), Box::new(s));
		Ok(r)
	}
}
// endregion:	--- LivelinessSubscriberBuilder

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug)]
	struct Props {}

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<LivelinessSubscriberBuilder<Props, NoCallback, NoStorage>>();
	}
}

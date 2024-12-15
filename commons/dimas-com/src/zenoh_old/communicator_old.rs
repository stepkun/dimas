// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]

//! Implements the zenoh communication capabilities.
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::{error_old::Error, traits_old::CommunicatorImplementationMethods};
use alloc::{
	borrow::ToOwned,
	boxed::Box,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::{
	message_types::{Message, QueryableMsg},
	OperationState, Operational, Transitions,
};
use zenoh::config::WhatAmI;
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;
use zenoh::{
	query::{ConsolidationMode, QueryTarget},
	sample::SampleKind,
	Session, Wait,
};
// endregion:	--- modules

// region:		--- Communicator
/// [`Communicator`] handles all communication aspects
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct CommunicatorOld {
	/// The current state for [`Operational`]
	current_state: OperationState,
	session: Arc<Session>,
	/// Mode of the session (router|peer|client)
	mode: String,
	/// The state from parent, at which [`OperationState::Active`] should be reached
	activation_state: OperationState,
}

impl Transitions for CommunicatorOld {}

impl Operational for CommunicatorOld {
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

impl CommunicatorImplementationMethods for CommunicatorOld {
	/// Send a put message [`Message`] using the given `selector`
	/// # Errors
	#[allow(clippy::needless_pass_by_value)]
	fn put(&self, selector: &str, message: Message) -> Result<()> {
		self.session
			.put(selector, message.value())
			.wait()
			.map_err(|source| Error::PublishingPut { source }.into())
	}

	/// Send a delete message using the given `selector`.
	/// # Errors
	fn delete(&self, selector: &str) -> Result<()> {
		self.session
			.delete(selector)
			.wait()
			.map_err(|source| Error::PublishingDelete { source }.into())
	}

	/// Send a query with an optional [`Message`] using the given `selector`.
	/// Answers are collected via callback
	/// # Errors
	/// # Panics
	fn get(
		&self,
		selector: &str,
		message: Option<Message>,
		mut callback: Option<&mut dyn FnMut(QueryableMsg) -> Result<()>>,
	) -> Result<()> {
		let builder = message
			.map_or_else(
				|| self.session.get(selector),
				|msg| self.session.get(selector).payload(msg.value()),
			)
			.consolidation(ConsolidationMode::None)
			.target(QueryTarget::All);

		#[cfg(feature = "unstable")]
		let builder = builder.allowed_destination(Locality::Any);

		let query = builder
			.timeout(Duration::from_millis(250))
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
							// CommunicatorImplementation::Zenoh(zenoh) =>
							callback.as_deref_mut().map_or_else(
								|| Err(Error::NotImplemented),
								|callback| {
									callback(QueryableMsg(content))
										.map_err(|source| Error::QueryCallback { source })
								},
							)?;
						}
						SampleKind::Delete => {
							todo!("Delete in Query");
						}
					},
					Err(err) => {
						let content: Vec<u8> = err.payload().to_bytes().into_owned();
						todo!(">> Received (ERROR: '{:?}' for {})", &content, &selector);
					}
				}
				unreached = false;
			}
			if unreached {
				if retry_count < 5 {
					std::thread::sleep(Duration::from_millis(1000));
				} else {
					return Err(Error::AccessingQueryable {
						selector: selector.to_string(),
					}
					.into());
				}
			}
		}
		Ok(())
	}
}

impl CommunicatorOld {
	/// Constructor
	/// # Errors
	pub fn new(config: &zenoh::Config) -> Result<Self> {
		#[cfg(feature = "unstable")]
		let kind = config.mode().unwrap_or(WhatAmI::Peer).to_string();
		#[cfg(not(feature = "unstable"))]
		let kind = WhatAmI::Peer.to_string();
		let session = Arc::new(
			zenoh::open(config.to_owned())
				.wait()
				.map_err(|source| Error::CreateCommunicator { source })?,
		);
		Ok(Self {
			current_state: OperationState::default(),
			session,
			mode: kind,
			activation_state: OperationState::Active,
		})
	}

	/// Get globally unique ID
	#[must_use]
	pub fn uuid(&self) -> String {
		self.session.zid().to_string()
	}

	/// Get session reference
	#[must_use]
	pub fn session(&self) -> Arc<Session> {
		self.session.clone()
	}

	/// Get session mode
	#[must_use]
	pub const fn mode(&self) -> &String {
		&self.mode
	}
}
// endregion:	--- Communicator

#[cfg(test)]
mod tests {
	use super::*;
	//use serial_test::serial;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<CommunicatorOld>();
	}

	#[tokio::test(flavor = "multi_thread")]
	//#[serial]
	async fn communicator_create() -> Result<()> {
		let cfg = dimas_config::Config::default();
		let _peer = CommunicatorOld::new(cfg.zenoh_config())?;
		Ok(())
	}
}

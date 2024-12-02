// Copyright © 2023 Stephan Kunz

//! Module `publisher` provides a message sender `Publisher` which can be created using the `PublisherBuilder`.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::error::Error;
use alloc::{string::String, sync::Arc};
use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{message_types::Message, OperationState, Operational, Transitions};
use tracing::{event, instrument, Level};
#[cfg(feature = "unstable")]
use zenoh::{qos::Reliability, sample::Locality};
use zenoh::{
	qos::{CongestionControl, Priority},
	Session, Wait,
};
// endregion:	--- modules

// region:		--- Publisher
/// Publisher
pub struct Publisher {
	/// The current state for [`Operational`]
	current_state: OperationState,
	/// the zenoh session this publisher belongs to
	session: Arc<Session>,
	selector: String,
	/// The state from parent, at which [`OperationState::Active`] should be reached
	activation_state: OperationState,
	#[cfg(feature = "unstable")]
	allowed_destination: Locality,
	congestion_control: CongestionControl,
	encoding: String,
	express: bool,
	priority: Priority,
	#[cfg(feature = "unstable")]
	reliability: Reliability,
	publisher: parking_lot::Mutex<Option<zenoh::pubsub::Publisher<'static>>>,
}

impl Debug for Publisher {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Publisher")
			.field("selector", &self.selector)
			.field("initialized", &self.publisher)
			.finish_non_exhaustive()
	}
}

impl crate::traits::Publisher for Publisher {
	/// Get `selector`
	fn selector(&self) -> &str {
		&self.selector
	}

	/// Send a "put" message
	/// # Errors
	///
	#[instrument(name="publish", level = Level::ERROR, skip_all)]
	fn put(&self, message: Message) -> Result<()> {
		match self
			.publisher
			.lock()
			.as_ref()
			.ok_or(Error::AccessPublisher)?
			.put(message.value())
			.wait()
		{
			Ok(()) => Ok(()),
			Err(source) => Err(Error::PublishingPut { source }.into()),
		}
	}

	/// Send a "delete" message
	/// # Errors
	///
	#[instrument(level = Level::ERROR, skip_all)]
	fn delete(&self) -> Result<()> {
		match self
			.publisher
			.lock()
			.as_ref()
			.ok_or(Error::AccessPublisher)?
			.delete()
			.wait()
		{
			Ok(()) => Ok(()),
			Err(source) => Err(Error::PublishingDelete { source }.into()),
		}
	}
}

impl Transitions for Publisher {
	#[instrument(level = Level::DEBUG, skip_all)]
	fn activate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "activate");
		println!("Test");
		let builder = self
			.session
			.declare_publisher(self.selector.clone())
			.congestion_control(self.congestion_control)
			.encoding(self.encoding.as_str())
			.express(self.express)
			.priority(self.priority);

		#[cfg(feature = "unstable")]
		let builder = builder
			.allowed_destination(self.allowed_destination)
			.reliability(self.reliability);

		builder.wait().map_or_else(
			|_| Err(Error::Unexpected(file!().into(), line!()).into()),
			|new_publisher| {
				self.publisher.lock().replace(new_publisher);
				Ok(())
			},
		)
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn deactivate(&mut self) -> Result<()> {
		event!(Level::DEBUG, "deactivate");
		self.publisher.lock().take();
		Ok(())
	}
}

impl Operational for Publisher {
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

impl Publisher {
	/// Constructor for a [`Publisher`]
	#[allow(clippy::too_many_arguments)]
	#[must_use]
	pub fn new(
		session: Arc<Session>,
		selector: impl Into<String>,
		activation_state: OperationState,
		#[cfg(feature = "unstable")] allowed_destination: Locality,
		congestion_control: CongestionControl,
		encoding: impl Into<String>,
		express: bool,
		priority: Priority,
		#[cfg(feature = "unstable")] reliability: Reliability,
	) -> Self {
		Self {
			current_state: OperationState::default(),
			session,
			selector: selector.into(),
			activation_state,
			#[cfg(feature = "unstable")]
			allowed_destination,
			congestion_control,
			encoding: encoding.into(),
			express,
			priority,
			#[cfg(feature = "unstable")]
			reliability,
			publisher: parking_lot::Mutex::new(None),
		}
	}
}
// endregion:	--- Publisher

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Publisher>();
	}
}

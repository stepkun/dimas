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
use dimas_core::{message_types::Message, OperationState, Operational};
use tracing::{instrument, Level};
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
	/// the zenoh session this publisher belongs to
	session: Arc<Session>,
	selector: String,
	activation_state: OperationState,
	#[cfg(feature = "unstable")]
	allowed_destination: Locality,
	congestion_control: CongestionControl,
	encoding: String,
	express: bool,
	priority: Priority,
	#[cfg(feature = "unstable")]
	reliability: Reliability,
	publisher: std::sync::Mutex<Option<zenoh::pubsub::Publisher<'static>>>,
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
		self.publisher.lock().map_or_else(
			|_| Err(Error::Unexpected(file!().into(), line!()).into()),
			|publisher| match publisher
				.as_ref()
				.ok_or(Error::AccessPublisher)?
				.put(message.value())
				.wait()
			{
				Ok(()) => Ok(()),
				Err(source) => Err(Error::PublishingPut { source }.into()),
			},
		)
	}

	/// Send a "delete" message
	/// # Errors
	///
	#[instrument(level = Level::ERROR, skip_all)]
	fn delete(&self) -> Result<()> {
		self.publisher.lock().map_or_else(
			|_| Err(Error::Unexpected(file!().into(), line!()).into()),
			|publisher| match publisher
				.as_ref()
				.ok_or(Error::AccessPublisher)?
				.delete()
				.wait()
			{
				Ok(()) => Ok(()),
				Err(source) => Err(Error::PublishingDelete { source }.into()),
			},
		)
	}
}

impl Operational for Publisher {
	fn manage_operation_state_old(&self, state: OperationState) -> Result<()> {
		if state >= self.activation_state {
			return self.init();
		} else if state < self.activation_state {
			return self.de_init();
		}
		Ok(())
	}
	
	fn state(&self) -> OperationState {
			todo!()
		}
	
	fn set_state(&mut self, _state: OperationState) {
			todo!()
		}
	
	fn operationals(&mut self) -> &mut Vec<Box<dyn Operational>> {
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
			publisher: std::sync::Mutex::new(None),
		}
	}

	/// Initialize
	/// # Errors
	///
	fn init(&self) -> Result<()> {
		self.de_init()?;

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
				self.publisher.lock().map_or_else(
					|_| Err(Error::Unexpected(file!().into(), line!()).into()),
					|mut publisher| {
						publisher.replace(new_publisher);
						Ok(())
					},
				)
			},
		)
	}

	/// De-Initialize
	/// # Errors
	///
	#[allow(clippy::unnecessary_wraps)]
	fn de_init(&self) -> Result<()> {
		self.publisher.lock().map_or_else(
			|_| Err(Error::Unexpected(file!().into(), line!()).into()),
			|mut publisher| {
				publisher.take();
				Ok(())
			},
		)
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

// Copyright © 2023 Stephan Kunz

//! Module `publisher` provides a message sender `Publisher` which can be created using the `PublisherBuilder`.

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:		--- modules
use crate::error_old::Error;
use alloc::{string::String, sync::Arc};
use anyhow::Result;
use core::fmt::Debug;
use dimas_core::{
	message_types::Message, Activity, ActivityType, OperationState, Operational, OperationalType,
	Transitions,
};
use tracing::{event, instrument, Level};
use zenoh::{Session, Wait};

use super::PublisherParameter;
// endregion:	--- modules

// region:		--- Publisher
/// Publisher
#[dimas_macros::activity]
pub struct Publisher {
	selector: String,
	parameter: PublisherParameter,
	session: Arc<Session>,
	publisher: parking_lot::Mutex<Option<zenoh::pubsub::Publisher<'static>>>,
}

impl Debug for Publisher {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Publisher")
			.field("selector", &self.selector)
			.finish_non_exhaustive()
	}
}

impl crate::traits_old::Publisher for Publisher {
	/// Get `selector`
	fn selector(&self) -> &str {
		&self.selector
	}

	/// Send a "put" message
	/// # Errors
	///
	#[instrument(name="publish", level = Level::TRACE, skip_all)]
	fn put(&self, message: Message) -> Result<()> {
		event!(Level::TRACE, "put");
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
	#[instrument(level = Level::TRACE, skip_all)]
	fn delete(&self) -> Result<()> {
		event!(Level::TRACE, "delete");
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
		let builder = self
			.session
			.declare_publisher(self.selector.clone())
			.congestion_control(self.parameter.congestion_control)
			.encoding(self.parameter.encoding.clone())
			.express(self.parameter.express)
			.priority(self.parameter.priority);

		#[cfg(feature = "unstable")]
		let builder = builder
			.allowed_destination(self.parameter.allowed_destination)
			.reliability(self.parameter.reliability);

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

impl Publisher {
	/// Constructor for a [`Publisher`]
	#[allow(clippy::too_many_arguments)]
	#[must_use]
	pub fn new(
		activity: ActivityType,
		operational: OperationalType,
		selector: impl Into<String>,
		parameter: PublisherParameter,
		session: Arc<Session>,
	) -> Self {
		let selector = selector.into();

		Self {
			activity,
			operational,
			parameter,
			session,
			selector,
			publisher: parking_lot::Mutex::new(None),
		}
	}
}
// endregion:	--- Publisher

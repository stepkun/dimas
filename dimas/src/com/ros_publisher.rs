// Copyright © 2024 Stephan Kunz

// region:		--- modules
use crate::prelude::*;
#[allow(unused_imports)]
use std::collections::HashMap;
use std::{any::Any, fmt::Debug};
// endregion:	--- modules

// region:		--- states
/// State signaling that the [`RosPublisherBuilder`] has no storage value set
pub struct NoStorage;
/// State signaling that the [`RosPublisherBuilder`] has the storage value set
#[cfg(feature = "ros_publisher")]
pub struct Storage {
	/// Thread safe reference to a [`HashMap`] to store the created [`RosPublisher`]
	pub storage: Arc<RwLock<std::collections::HashMap<String, RosPublisher>>>,
}

/// State signaling that the [`RosPublisherBuilder`] has no topic set
pub struct NoTopic;
/// State signaling that the [`RosPublisherBuilder`] has the topic set
pub struct Topic {
	/// The topic name
	name: String,
	/// The messages package
	package: String,
	/// The messages name within package
	message_name: String,
	/// The message
	message: Arc<dyn Any + Send + Sync + Unpin>,
}
// endregion:	--- states

// region:		--- RosPublisherBuilder
/// `RosPublisherBuilder`
#[allow(clippy::module_name_repetitions)]
pub struct RosPublisherBuilder<K, S> {
	prefix: Option<String>,
	pub(crate) topic: K,
	pub(crate) storage: S,
}

impl RosPublisherBuilder<NoTopic, NoStorage> {
	/// Construct a `RosPublisherBuilder` in initial state
	#[must_use]
	pub const fn new(prefix: Option<String>) -> Self {
		Self {
			prefix,
			topic: NoTopic,
			storage: NoStorage,
		}
	}

	/// set a different namespace
	#[must_use]
	pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
		self.prefix = Some(namespace.into());
		self
	}
}

#[cfg(feature = "ros_publisher")]
impl<K> RosPublisherBuilder<K, NoStorage> {
	/// Provide agents storage for the publisher
	#[must_use]
	pub fn storage(
		self,
		storage: Arc<RwLock<std::collections::HashMap<String, RosPublisher>>>,
	) -> RosPublisherBuilder<K, Storage> {
		let Self { prefix, topic, .. } = self;
		RosPublisherBuilder {
			prefix,
			topic,
			storage: Storage { storage },
		}
	}
}

impl<S> RosPublisherBuilder<NoTopic, S> {
	/// Set the topic of the [`Publisher`].
	/// Will be prefixed with [`Agent`]s prefix as namespace.
	#[must_use]
	pub fn topic(
		self,
		name: &str,
		package: &str,
		message_name: &str,
		message: Arc<dyn Any + Send + Sync + Unpin>,
	) -> RosPublisherBuilder<Topic, S> {
		let Self {
			prefix, storage, ..
		} = self;
		RosPublisherBuilder {
			prefix,
			topic: Topic {
				name: name.into(),
				package: package.into(),
				message_name: message_name.into(),
				message,
			},
			storage,
		}
	}
}

impl<S> RosPublisherBuilder<Topic, S> {
	/// Build the [`RosPublisher`]
	///
	/// # Errors
	/// Currently none
	pub fn build(self) -> Result<RosPublisher> {
		Ok(RosPublisher {
			name: self.topic.name,
			package: self.topic.package,
			message_name: self.topic.message_name,
			message: self.topic.message,
		})
	}
}

#[cfg(feature = "ros_publisher")]
impl RosPublisherBuilder<Topic, Storage> {
	/// Build and add the [`RosPublisher`] to the [`Agent`]s context
	///
	/// # Errors
	/// Currently none
	#[cfg_attr(any(nightly, docrs), doc, doc(cfg(feature = "ros_publisher")))]
	pub fn add(self) -> Result<Option<RosPublisher>> {
		let key = format!("/{}/{}", &self.topic.package, &self.topic.message_name);
		let collection = self.storage.storage.clone();
		let p = self.build()?;
		let r = collection
			.write()
			.map_err(|_| DimasError::ShouldNotHappen)?
			.insert(key, p);
		Ok(r)
	}
}
// endregion:	--- RosPublisherBuilder

// region:		--- RosPublisher
/// `RosPublisher`
pub struct RosPublisher {
	pub(crate) name: String,
	pub(crate) package: String,
	pub(crate) message_name: String,
	pub(crate) message: Arc<dyn Any + Send + Sync + Unpin>,
}

impl Debug for RosPublisher {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RosPublisher")
			.field("package", &self.package)
			.field("name", &self.message_name)
			//.field("initialized", &self.publisher.is_some())
			.finish_non_exhaustive()
	}
}
// endregion:	--- RosPublisher

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync + Unpin>() {}

	#[test]
	const fn normal_types() {
		is_normal::<RosPublisher>();
		is_normal::<RosPublisherBuilder<NoTopic, NoStorage>>();
	}
}

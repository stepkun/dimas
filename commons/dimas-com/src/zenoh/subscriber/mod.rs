// Copyright © 2024 Stephan Kunz

//! Module `publisher` provides a zenoh based publisher

#[doc(hidden)]
extern crate alloc;

// region:    --- modules
mod error;
#[allow(clippy::module_inception)]
mod subscriber;
mod subscriber_parameter;
// endregion:   --- modules

// flatten
pub use subscriber::{ArcDeleteCallback, ArcPutCallback, DeleteCallback, PutCallback, Subscriber};
#[allow(clippy::module_name_repetitions)]
pub use subscriber_parameter::SubscriberParameter;
// endregion: --- modules

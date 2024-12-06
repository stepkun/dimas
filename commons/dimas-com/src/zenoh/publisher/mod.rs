// Copyright © 2024 Stephan Kunz

//! Module `publisher` provides a zenoh based publisher

#[doc(hidden)]
extern crate alloc;

// region:    --- modules
mod error;
#[allow(clippy::module_inception)]
mod publisher;
mod publisher_parameter;
// endregion:   --- modules

// flatten
pub use publisher::Publisher;
#[allow(clippy::module_name_repetitions)]
pub use publisher_parameter::PublisherParameter;
// endregion: --- modules

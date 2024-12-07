// Copyright © 2024 Stephan Kunz

//! Module `publisher` provides a zenoh based publisher

mod error;
#[allow(clippy::module_inception)]
mod publisher;
mod publisher_parameter;

// flatten
pub use publisher::Publisher;
#[allow(clippy::module_name_repetitions)]
pub use publisher_parameter::PublisherParameter;

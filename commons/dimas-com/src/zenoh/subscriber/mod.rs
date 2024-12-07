// Copyright © 2024 Stephan Kunz

//! Module `subscriber` provides a zenoh based subscriber

mod error;
#[allow(clippy::module_inception)]
mod subscriber;
mod subscriber_parameter;

// flatten
pub use subscriber::{ArcDeleteCallback, ArcPutCallback, DeleteCallback, PutCallback, Subscriber};
#[allow(clippy::module_name_repetitions)]
pub use subscriber_parameter::SubscriberParameter;

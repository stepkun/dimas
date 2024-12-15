// Copyright © 2024 Stephan Kunz

//! Module `observable` provides a zenoh based observable

mod error;
#[allow(clippy::module_inception)]
mod liveliness;
mod liveliness_parameter;

// flatten
#[allow(clippy::module_name_repetitions)]
pub use liveliness::{ArcLivelinessCallback, LivelinessCallback, LivelinessSubscriber};
#[allow(clippy::module_name_repetitions)]
pub use liveliness_parameter::LivelinessSubscriberParameter;

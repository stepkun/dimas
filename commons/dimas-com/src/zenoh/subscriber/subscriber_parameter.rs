// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `timer_builder` provides a builder for the different [`TimerVariant`]s

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;
use zenoh::{
	bytes::Encoding,
	qos::{CongestionControl, Priority},
	Session, Wait,
};
#[cfg(feature = "unstable")]
use zenoh::{qos::Reliability, sample::Locality};

// endregion:   --- modules

// region:      --- SubscriberParameter
/// Parameters for an [`IntervalTimer`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct SubscriberParameter {
	#[cfg(feature = "unstable")]
	allowed_origin: Locality,
}

#[allow(clippy::derivable_impls)]
impl Default for SubscriberParameter {
	fn default() -> Self {
		Self {
			#[cfg(feature = "unstable")]
			allowed_origin: Locality::Any,
		}
	}
}

impl SubscriberParameter {
	/// Create a [`PublisherParamter`] set.
	#[must_use]
	pub const fn new(#[cfg(feature = "unstable")] allowed_destination: Locality) -> Self {
		Self {
			#[cfg(feature = "unstable")]
			allowed_destination,
		}
	}
}
// endregion:   --- SubscriberParameter

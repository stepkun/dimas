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

// region:      --- PublisherParameter
/// Parameters for an [`IntervalTimer`]
#[dimas_macros::parameter]
pub struct PublisherParameter {
	pub(crate) congestion_control: CongestionControl,
	pub(crate) encoding: Encoding,
	pub(crate) express: bool,
	pub(crate) priority: Priority,
	#[cfg(feature = "unstable")]
	pub(crate) reliability: Reliability,
	#[cfg(feature = "unstable")]
	pub(crate) allowed_destination: Locality,
}

impl Default for PublisherParameter {
	fn default() -> Self {
		Self {
			congestion_control: CongestionControl::Drop,
			encoding: Encoding::default(),
			express: false,
			priority: Priority::Data,
			#[cfg(feature = "unstable")]
			reliability: Reliability::BestEffort,
			#[cfg(feature = "unstable")]
			allowed_destination: Locality::Any,
		}
	}
}

impl PublisherParameter {
	/// Create a [`PublisherParamter`] set.
	#[must_use]
	pub const fn new(
		congestion_control: CongestionControl,
		encoding: Encoding,
		express: bool,
		priority: Priority,
		#[cfg(feature = "unstable")] reliability: Reliability,
		#[cfg(feature = "unstable")] allowed_destination: Locality,
	) -> Self {
		Self {
			congestion_control,
			encoding,
			express,
			priority,
			#[cfg(feature = "unstable")]
			reliability,
			#[cfg(feature = "unstable")]
			allowed_destination,
		}
	}
}
// endregion:   --- PublisherParameter

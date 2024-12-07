// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `subscriber_parameter`

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;

// endregion:   --- modules

// region:      --- SubscriberParameter
/// Parameters for a [`Subscriber`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct SubscriberParameter {
	#[cfg(feature = "unstable")]
	pub(crate) allowed_origin: Locality,
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
	/// Create a [`SubscriberParameter`] set.
	#[must_use]
	pub const fn new(#[cfg(feature = "unstable")] allowed_origin: Locality) -> Self {
		Self {
			#[cfg(feature = "unstable")]
			allowed_origin,
		}
	}
}
// endregion:   --- SubscriberParameter

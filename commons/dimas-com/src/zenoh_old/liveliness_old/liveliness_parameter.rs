// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `liveliness_subscriber_parameter`

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;
// endregion:   --- modules

// region:      --- LivelinessSubscriberParameter
/// Parameters for a [`Subscriber`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct LivelinessSubscriberParameter {}

#[allow(clippy::derivable_impls)]
impl Default for LivelinessSubscriberParameter {
	fn default() -> Self {
		Self {}
	}
}

impl LivelinessSubscriberParameter {
	/// Create a [`LivelinessSubscriberParameter`] set.
	#[must_use]
	pub const fn new() -> Self {
		Self {}
	}
}
// endregion:   --- LivelinessSubscriberParameter

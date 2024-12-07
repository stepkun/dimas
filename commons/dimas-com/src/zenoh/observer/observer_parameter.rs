// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `observer_parameter`.

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;
use zenoh::{
	bytes::Encoding,
	query::{ConsolidationMode, QueryTarget},
};

// endregion:   --- modules

// region:      --- ObserverParameter
/// Parameters for a [`Querier`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct ObserverParameter {
	pub(crate) timeout: Duration,
}

#[allow(clippy::derivable_impls)]
impl Default for ObserverParameter {
	fn default() -> Self {
		Self {
			timeout: Duration::from_millis(100),
		}
	}
}

impl ObserverParameter {
	/// Create a [`ObserverParameter`] set.
	#[must_use]
	pub const fn new(timeout: Duration) -> Self {
		Self { timeout }
	}
}
// endregion:   --- ObserverParameter

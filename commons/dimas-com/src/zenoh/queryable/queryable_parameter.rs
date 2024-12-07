// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `queryable_parameter`.

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;
#[cfg(feature = "unstable")]
use zenoh::sample::Locality;

// endregion:   --- modules

// region:      --- QueryableParameter
/// Parameters for a [`Queryable`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct QueryableParameter {
	pub(crate) completeness: bool,
	#[cfg(feature = "unstable")]
	pub(crate) allowed_origin: Locality,
}

#[allow(clippy::derivable_impls)]
impl Default for QueryableParameter {
	fn default() -> Self {
		Self {
			completeness: true,
			#[cfg(feature = "unstable")]
			allowed_origin: Locality::Any,
		}
	}
}

impl QueryableParameter {
	/// Create a [`QueryableParameter`] set.
	#[must_use]
	pub const fn new(
		completeness: bool,
		#[cfg(feature = "unstable")] allowed_origin: Locality,
	) -> Self {
		Self {
			completeness,
			#[cfg(feature = "unstable")]
			allowed_origin,
		}
	}
}
// endregion:   --- QueryableParameter

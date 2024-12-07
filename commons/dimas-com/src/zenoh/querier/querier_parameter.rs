// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `subscriber_parameter`.

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

// region:      --- QuerierParameter
/// Parameters for a [`Querier`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct QuerierParameter {
	pub(crate) mode: ConsolidationMode,
	pub(crate) timeout: Duration,
	pub(crate) encoding: Encoding,
	pub(crate) target: QueryTarget,
	#[cfg(feature = "unstable")]
	pub(crate) allowed_destination: Locality,
}

#[allow(clippy::derivable_impls)]
impl Default for QuerierParameter {
	fn default() -> Self {
		Self {
			mode: ConsolidationMode::None,
			timeout: Duration::from_millis(100),
			encoding: Encoding::default(),
			target: QueryTarget::All,
			#[cfg(feature = "unstable")]
			allowed_destination: Locality::Any,
		}
	}
}

impl QuerierParameter {
	/// Create a [`QuerierParameter`] set.
	#[must_use]
	pub const fn new(
		mode: ConsolidationMode,
		timeout: Duration,
		encoding: Encoding,
		target: QueryTarget,
		#[cfg(feature = "unstable")] allowed_destination: Locality,
	) -> Self {
		Self {
			mode,
			timeout,
			encoding,
			target,
			#[cfg(feature = "unstable")]
			allowed_destination,
		}
	}
}
// endregion:   --- QuerierParameter

// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
#![allow(dead_code)]

//! Module `observable_parameter`.

// region:		--- modules
use anyhow::Result;
use core::{fmt::Debug, time::Duration};
use dimas_core::traits::Context;

// endregion:   --- modules

// region:      --- ObservableParameter
/// Parameters for a [`Observable`]
#[dimas_macros::parameter]
#[derive(Clone)]
pub struct ObservableParameter {
	pub(crate) feedback_interval: Duration,
}

#[allow(clippy::derivable_impls)]
impl Default for ObservableParameter {
	fn default() -> Self {
		Self {
			feedback_interval: Duration::from_millis(100),
		}
	}
}

impl ObservableParameter {
	/// Create a [`ObservableParameter`] set.
	#[must_use]
	pub const fn new(feedback_interval: Duration) -> Self {
		Self { feedback_interval }
	}
}
// endregion:   --- ObservableParameter

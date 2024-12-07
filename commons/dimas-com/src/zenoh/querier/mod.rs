// Copyright © 2024 Stephan Kunz

//! Module `querier` provides a zenoh based querier

mod error;
#[allow(clippy::module_inception)]
mod querier;
mod querier_parameter;

// flatten
pub use querier::{ArcGetCallback, GetCallback, Querier};
#[allow(clippy::module_name_repetitions)]
pub use querier_parameter::QuerierParameter;

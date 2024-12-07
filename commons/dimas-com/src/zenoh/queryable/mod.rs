// Copyright © 2024 Stephan Kunz

//! Module `queryable` provides a zenoh based queryable

mod error;
#[allow(clippy::module_inception)]
mod queryable;
mod queryable_parameter;

// flatten
pub use queryable::{ArcGetCallback, GetCallback, Queryable};
#[allow(clippy::module_name_repetitions)]
pub use queryable_parameter::QueryableParameter;

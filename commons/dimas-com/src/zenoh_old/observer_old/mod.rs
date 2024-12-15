// Copyright © 2024 Stephan Kunz

//! Module `querier` provides a zenoh based querier

mod error;
#[allow(clippy::module_inception)]
mod observer;
mod observer_parameter;

// flatten
pub use observer::{
	ArcControlCallback, ArcResponseCallback, ControlCallback, Observer, ResponseCallback,
};
#[allow(clippy::module_name_repetitions)]
pub use observer_parameter::ObserverParameter;

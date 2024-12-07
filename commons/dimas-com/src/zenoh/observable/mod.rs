// Copyright © 2024 Stephan Kunz

//! Module `observable` provides a zenoh based observable

mod error;
#[allow(clippy::module_inception)]
mod observable;
mod observable_parameter;

// flatten
pub use observable::{
	ArcControlCallback, ArcExecutionCallback, ArcFeedbackCallback, ControlCallback,
	ExecutionCallback, FeedbackCallback, Observable,
};
#[allow(clippy::module_name_repetitions)]
pub use observable_parameter::ObservableParameter;

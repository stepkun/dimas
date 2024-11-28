// Copyright © 2024 Stephan Kunz

//! Operational
//!

mod error;
mod operation_state;
#[allow(clippy::module_inception)]
mod operational;

// flatten
pub use error::Error;
pub use operation_state::OperationState;
pub use operational::Operational;

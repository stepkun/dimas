// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

//! Operational
//!

mod error;
mod operation_state;
mod operational;
mod operational_type;

// flatten
pub use error::Error;
pub use operation_state::OperationState;
pub use operational::{Operational, Transitions};
pub use operational_type::OperationalType;

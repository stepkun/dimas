// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

//! Operational traits & structs
//!

mod error;
mod manage_operation_state;
mod operation_state;
mod operational;
mod operational_data;
mod operational_type;

// flatten
pub use error::Error;
pub use manage_operation_state::ManageOperationState;
pub use operation_state::OperationState;
pub use operational::{Operational, Transitions};
pub use operational_data::OperationalData;
pub use operational_type::OperationalType;

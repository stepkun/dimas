// Copyright © 2024 Stephan Kunz
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
#![allow(unused)]

//! System traits
//!

mod error;
mod system;
mod system_type;

// flatten
pub use error::Error;
pub use system::{System, SystemId};
pub use system_type::SystemType;

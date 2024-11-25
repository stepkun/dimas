// Copyright © 2023 Stephan Kunz

//! dimas-com implements the communication capabilities.
//!

/// Communicator
pub mod communicator;
/// Enums
pub mod enums;
/// Modules errors
pub mod error;
/// `Communicator` trait
pub mod traits;
/// zenoh implementation
pub mod zenoh;

// flatten
pub use communicator::*;

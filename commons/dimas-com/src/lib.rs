// Copyright © 2023 Stephan Kunz

//! dimas-com implements the communication capabilities.
//!

/// Zenoh based communication component
mod zenoh;

/// Communicator
pub mod communicator_old;
/// Enums
pub mod enums_old;
/// Modules errors
pub mod error_old;
/// `Communicator` trait
pub mod traits_old;
/// zenoh implementation
pub mod zenoh_old;

// flatten
pub use communicator_old::*;
pub use zenoh::Zenoh;

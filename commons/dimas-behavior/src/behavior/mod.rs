// Copyright © 2024 Stephan Kunz

//! Behavior of `DiMAS`

#[allow(clippy::module_inception)]
pub mod behavior;
pub mod error;
pub mod function;
pub mod macros;
pub mod tree;

// flatten
pub use behavior::*;
pub use function::*;

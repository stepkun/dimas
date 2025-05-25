// Copyright © 2024 Stephan Kunz
#![no_std]

//! Behavior library of `DiMAS`.

#[doc(hidden)]
extern crate alloc;

// modules
pub mod behavior;
pub mod blackboard;
pub mod factory;
pub mod port;
pub mod tree;

// flatten:
pub use behavior::Behavior;

// re-export
pub use dimas_behavior_macros::Behavior;

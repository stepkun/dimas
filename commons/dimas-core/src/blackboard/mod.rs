// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! Blackboard implementation of `DiMAS`

#[allow(clippy::module_inception)]
mod blackboard;
pub mod error;
mod string;

// flatten
pub use blackboard::*;
pub use string::*;

// Copyright © 2025 Stephan Kunz

//! [`BehaviorTree`] library
//!

pub mod error;
#[allow(clippy::module_inception)]
mod tree;

// flatten
pub use tree::{BehaviorTree, BehaviorTreeComponent};

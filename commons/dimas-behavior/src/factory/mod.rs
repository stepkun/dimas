// Copyright © 2025 Stephan Kunz

//! Factory library
//!

mod behavior_registry;
pub mod error;
#[allow(clippy::module_inception)]
mod factory;
mod xml_parser;

// flatten
pub use behavior_registry::BehaviorRegistry;
pub use factory::BehaviorTreeFactory;

// Copyright © 2025 Stephan Kunz

//! Action behavior library
//!

pub mod force_failure;
pub mod inverter;
pub mod retry_until_successful;

// region:      --- modules
use crate::behavior::BehaviorInstanceMethods;
// endregion:   --- modules

// region:      --- DecoratorBehavior
/// Common methods for control behaviors.
pub trait DecoratorBehavior: BehaviorInstanceMethods {}
// endregion:   --- DecoratorBehavior

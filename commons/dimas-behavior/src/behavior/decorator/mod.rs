// Copyright © 2025 Stephan Kunz

//! Action behavior library
//!

pub mod force_failure;
pub mod inverter;
pub mod loop_queue;
pub mod retry_until_successful;
pub mod script_precondition;
pub mod subtree;

// region:      --- modules
use crate::behavior::BehaviorInstance;
// endregion:   --- modules

// region:      --- DecoratorBehavior
/// Common methods for control behaviors.
pub trait DecoratorBehavior: BehaviorInstance {}
// endregion:   --- DecoratorBehavior

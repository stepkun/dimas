// Copyright © 2025 Stephan Kunz

//! Condition behavior library
//!

pub mod script_condition;

// region:      --- modules
use crate::behavior::BehaviorInstance;
// endregion:   --- modules

// region:      --- ConditionBehavior
/// Common methods for control behaviors.
pub trait ConditionBehavior: BehaviorInstance {}
// endregion:   --- ConditionBehavior

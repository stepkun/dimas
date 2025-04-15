// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]

//! Condition behavior library
//!

// region:      --- modules
use crate::new_behavior::BehaviorMethods;
// endregion:   --- modules

// region:      --- ConditionBehavior
/// Common methods for control behaviors.
pub trait ConditionBehavior: BehaviorMethods {}
// endregion:   --- ConditionBehavior

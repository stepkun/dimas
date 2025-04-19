// Copyright © 2025 Stephan Kunz

//! Control behavior library
//!

pub mod fallback;
pub mod parallel;
pub mod parallel_all;
pub mod reactive_fallback;
pub mod reactive_sequence;
pub mod sequence;
pub mod sequence_with_memory;

// region:      --- modules
use crate::new_behavior::BehaviorInstanceMethods;
// endregion:   --- modules

// region:      --- ControlBehavior
/// Common methods for control behaviors.
pub trait ControlBehavior: BehaviorInstanceMethods {}
// endregion:   --- ControlBehavior

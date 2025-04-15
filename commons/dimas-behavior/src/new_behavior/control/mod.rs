// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]

//! Control behavior library
//!

pub mod sequence;

// region:      --- modules
use crate::new_behavior::BehaviorMethods;
// endregion:   --- modules

// region:      --- ControlBehavior
/// Common methods for control behaviors.
pub trait ControlBehavior: BehaviorMethods {}
// endregion:   --- ControlBehavior

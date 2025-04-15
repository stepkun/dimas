// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]

//! Action behavior library
//!

// region:      --- modules
use crate::new_behavior::BehaviorMethods;
// endregion:   --- modules

// region:      --- ActionBehavior
/// Common methods for control behaviors.
pub trait ActionBehavior: BehaviorMethods {}
// endregion:   --- ActionBehavior

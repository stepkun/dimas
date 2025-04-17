// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]

//! Action behavior library
//!

// region:      --- modules
use crate::new_behavior::BehaviorInstanceMethods;
// endregion:   --- modules

// region:      --- DecoratorBehavior
/// Common methods for control behaviors.
pub trait DecoratorBehavior: BehaviorInstanceMethods {}
// endregion:   --- DecoratorBehavior

// Copyright © 2025 Stephan Kunz
#![allow(clippy::unused_async)]
#![allow(dead_code)]

//! Action behavior library
//!

mod script;

// flatten
pub use script::Script;

// region:      --- modules
use crate::new_behavior::BehaviorInstanceMethods;
// endregion:   --- modules

// region:      --- ActionBehavior
/// Common methods for control behaviors.
pub trait ActionBehavior: BehaviorInstanceMethods {}
// endregion:   --- ActionBehavior

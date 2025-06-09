// Copyright © 2025 Stephan Kunz

//! Action behavior library
//!

mod always_after;
mod script;

// flatten
pub use always_after::AlwaysAfter;
pub use script::Script;

// region:      --- modules
use crate::behavior::BehaviorInstance;
// endregion:   --- modules

// region:      --- ActionBehavior
/// Common methods for control behaviors.
pub trait ActionBehavior: BehaviorInstance {}
// endregion:   --- ActionBehavior

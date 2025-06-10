// Copyright © 2025 Stephan Kunz

//! Action behavior library
//!

mod script;
mod state_after;

// flatten
pub use script::Script;
pub use state_after::StateAfter;

// region:      --- modules
use crate::behavior::BehaviorInstance;
// endregion:   --- modules

// region:      --- ActionBehavior
/// Common methods for control behaviors.
pub trait ActionBehavior: BehaviorInstance {}
// endregion:   --- ActionBehavior

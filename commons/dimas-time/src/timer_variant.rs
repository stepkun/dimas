// Copyright © 2023 Stephan Kunz
#![allow(dead_code)]

//! Module `timer_variant` defines the known/implemented timer variants.

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
// endregion:   --- modules

// region:      --- TimerVariant
/// All known timer variants
pub enum TimerVariant {
	/// An interval timer w/o delay
	InteralTimer,
}
// endregion:   --- TimerVariant

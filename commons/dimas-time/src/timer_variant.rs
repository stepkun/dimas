// Copyright © 2023 Stephan Kunz
#![allow(dead_code)]

//! Module `timer_variant` defines the known/implemented timer variants.

use crate::IntervalTimerParameter;

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
// endregion:   --- modules

// region:      --- TimerVariant
/// All implemented timer variants
pub enum TimerVariant {
	/// An interval timer without or with delay
	Interval(IntervalTimerParameter),
}
// endregion:   --- TimerVariant

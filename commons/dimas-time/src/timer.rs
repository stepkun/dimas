// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! `timer`defines the interfaces

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::boxed::Box;
use anyhow::Result;
use core::future::Future;
use dimas_core::Agent;

use crate::TimerVariant;
// endregion:   --- modules

// region:      --- Timer
/// `Timer` trait.
pub trait Timer: Send + Sync {}
// endregion:   --- Timer

// region:      --- TimerFactory
/// `TimerFactory` trait.
#[allow(clippy::module_name_repetitions)]
pub trait TimerFactory {
	/// Factory method for timer creation
	fn create_timer<CB, F>(&self, variant: TimerVariant, callback: CB) -> Box<dyn Timer>
	where
		CB: FnMut(Agent) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static;
}
// endregion:   --- TimerFactory

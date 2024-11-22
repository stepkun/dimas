// Copyright © 2024 Stephan Kunz

//! Core enums of `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::string::String;
use core::fmt::Debug;
// endregion:	--- modules

// region:		--- TaskSignal
/// Internal signals, used by panic hooks to inform that someting has happened.
#[derive(Debug, Clone)]
pub enum TaskSignal {
	/// Restart a certain liveliness subscriber, identified by its key expression
	#[cfg(feature = "unstable")]
	RestartLiveliness(String),
	/// Restart a certain observable, identified by its key expression
	RestartObservable(String),
	/// Restart a certain queryable, identified by its key expression
	RestartQueryable(String),
	/// Restart a certain lsubscriber, identified by its key expression
	RestartSubscriber(String),
	/// Restart a certain timer, identified by its key expression
	RestartTimer(String),
	/// Shutdown whole process
	Shutdown,
}
// endregion:	--- TaskSignal

// Copyright © 2024 Stephan Kunz

//! Core enums of `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::vec::Vec;
use bitcode::{Decode, Encode};
use core::fmt::Debug;

use super::OperationState;
// endregion:	--- modules

// region:		--- Signal
/// All defined commands of `DiMAS`
#[derive(Debug, Decode, Encode)]
pub enum Signal {
	/// About
	About,
	/// respond to Ping
	Ping {
		/// the utc time coordinate when the request was sent
		sent: i64,
	},
	/// Shutdown application
	Shutdown,
	/// State
	State {
		/// Optional `OperationState` to set
		state: Option<OperationState>,
	},
}
// endregion:	--- Signal

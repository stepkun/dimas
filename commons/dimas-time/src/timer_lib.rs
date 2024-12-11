// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(missing_docs)]

//! Module `timer` implements a component which provides a set of timer-variants.
//! Currently there are:
//! - [`IntervalTimer`]

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use dimas_core::{
	Activity, ActivityId, Component, ComponentData, ComponentId, ComponentStruct, ComponentType,
	ManageOperationState, OperationState, Operational, OperationalType, Transitions,
};
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[cfg(doc)]
use crate::{IntervalTimer, TimerVariant};
// endregion:   --- modules

// region:      --- Timer
/// Timer component.
#[dimas_macros::component]
#[derive(Debug)]
pub struct TimerLib {}

impl Default for TimerLib {
	fn default() -> Self {
		Self {
			data: ComponentData::new(Uuid::new_v4(), "Timer", 1),
			structure: ComponentStruct::default(),
		}
	}
}

impl Transitions for TimerLib {}

impl ManageOperationState for TimerLib {
	#[instrument(level = Level::DEBUG, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "manage_operation_state");
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}
// endregion:   --- Timer

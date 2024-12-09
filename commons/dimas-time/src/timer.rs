// Copyright © 2024 Stephan Kunz

//! Module `timer` implements a component which provides a set of timer-variants.
//! Currently there are:
//! - [`IntervalTimer`]

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use core::marker::PhantomData;
use dimas_core::{
	Activity, ActivityId, Component, ComponentId, ComponentType, ManageOperationState,
	OperationState, Operational, OperationalType, Transitions,
};
use tracing::{event, instrument, Level};

#[cfg(doc)]
use crate::{IntervalTimer, TimerVariant};
// endregion:   --- modules

// region:      --- Timer
/// Timer component.
#[dimas_macros::component]
pub struct Timer<P>
where
	P: Send + Sync + 'static,
{
	phantom: PhantomData<P>,
}

impl<P> Transitions for Timer<P> where P: Send + Sync + 'static {}

impl<P> ManageOperationState for Timer<P>
where
	P: Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "manage_operation_state");
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}
// endregion:   --- Timer

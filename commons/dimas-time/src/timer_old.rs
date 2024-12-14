// Copyright © 2024 Stephan Kunz

//! Module `timer` implements a component which provides a set of timer-variants.
//! Currently there are:
//! - [`IntervalTimer`]

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{boxed::Box, string::String};
use anyhow::Result;
use core::fmt::Debug;
use core::marker::PhantomData;
use dimas_core::{
	Activity, ActivityId, Agent, Component, ComponentId, ComponentType, ManageOperationState,
	OperationState, Operational, OperationalType, Transitions,
};
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[cfg(doc)]
use crate::{IntervalTimerOld, TimerVariant};
// endregion:   --- modules

// region:      --- Timer
/// Timer component.
#[dimas_macros::component_old]
#[derive(Debug)]
pub struct TimerOld<P>
where
	P: Debug + Send + Sync + 'static,
{
	phantom: PhantomData<P>,
}

impl<P> Transitions for TimerOld<P> where P: Debug + Send + Sync + 'static {}

impl<P> ManageOperationState for TimerOld<P>
where
	P: Debug + Send + Sync + 'static,
{
	#[instrument(level = Level::DEBUG, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "manage_operation_state");
		assert_ne!(state, OperationState::Undefined);
		Ok(())
	}
}
// endregion:   --- Timer

// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! Module `timer` implements a component which provides a set of timer-variants.
//! Currently there are:
//! - [`IntervalTimer`]

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use anyhow::Result;
use core::future::Future;
use dimas_core::{
	Activity, ActivityId, Agent, Component, ComponentData, ComponentId, ComponentStruct,
	ComponentType, ManageOperationState, OperationState, Operational, OperationalType, Transitions,
};
use tracing::{event, instrument, Level};
use uuid::Uuid;

use crate::{timer::TimerFactory, IntervalTimer, IntervalTimerOld, Timer, TimerVariant};
#[cfg(doc)]
use crate::{IntervalTimerOld, TimerVariant};
// endregion:   --- modules

// region:      --- TimerLib
/// Timer library.
#[derive(Debug)]
pub struct TimerLib {
	id: String,
	agent: Agent,
}

impl TimerLib {
	/// Create a [`TimerLib`] with an [`Agent`] as context
	#[must_use]
	pub fn new(agent: Agent) -> Self {
		Self {
			id: String::from("TimerLib"),
			agent,
		}
	}

	/// Create a [`Timer`]
	pub fn create_timer<CB, F>(&self, variant: TimerVariant, callback: CB) -> Box<dyn Activity>
	where
		CB: FnMut(Agent) -> F + Send + Sync + 'static,
		F: Future<Output = Result<()>> + Send + Sync + 'static,
	{
		let agent = self.agent.clone();
		match variant {
			TimerVariant::Interval(parameter) => {
				Box::new(IntervalTimer::new(parameter, callback, agent))
			}
		}
	}
}
// endregion:   --- TimerLib

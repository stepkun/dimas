// Copyright © 2024 Stephan Kunz

//! Module `timer` implements a component which provides a set of timer-variants.
//! Currently there are:
//! - [`IntervalTimer`]

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{fmt::Debug, marker::PhantomData};
use dimas_core::{
	Activity, ActivityId, Component, ComponentId, ComponentType, OperationState, Operational,
	OperationalType, Transitions,
};

#[cfg(doc)]
use crate::{IntervalTimer, TimerVariant};
// endregion:   --- modules

// region:      --- Timer
/// Timer component, providing a [`TimerBuilder`] for building [`TimerVariant`]s.
/// The timers are stored within the component and can be accessed via there ID.
#[dimas_macros::component]
#[derive(Debug)]
pub struct Timer<P>
where
	P: Debug + Send + Sync + 'static,
{
	phantom: PhantomData<P>,
}

impl<P> Transitions for Timer<P> where P: Debug + Send + Sync + 'static {}
// endregion:   --- Timer

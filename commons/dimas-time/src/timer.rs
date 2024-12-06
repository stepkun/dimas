// Copyright © 2024 Stephan Kunz

//! Module `timer` implements a component which provides a set of timer-variants.
//! Currently there are:
//! - [`IntervalTimer`]

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{boxed::Box, string::String, vec::Vec};
use core::marker::PhantomData;
use dimas_core::{
	Activity, ActivityId, Component, ComponentId, ComponentType, OperationState, Operational,
	OperationalType, Transitions,
};

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
// endregion:   --- Timer

// Copyright © 2023 Stephan Kunz
#![allow(unused_imports)]
//! Implementation of an [`Agent`]'s internal and user defined properties [`ContextImpl`].
//!
//! Never use it directly but through the type [`Context`], which provides thread safe access.
//! A [`Context`] is handed into every callback function.
//!
//! # Examples
//! ```rust,no_run
//! # use dimas::prelude::*;
//! // The [`Agent`]s properties
//! #[derive(Debug)]
//! struct AgentProps {
//!   counter: i32,
//! }
//! // A [`Timer`] callback
//! fn timer_callback(context: Context<AgentProps>) -> Result<()> {
//!   // reading properties
//!   let mut value = context.read().counter;
//!   value +=1;
//!   // writing properties
//!   context.write().counter = value;
//!   Ok(())
//! }
//! # #[tokio::main(flavor = "multi_thread")]
//! # async fn main() -> Result<()> {
//! # Ok(())
//! # }
//! ```
//!

// region:		--- modules
// only for doc needed
#[cfg(doc)]
use crate::agent_old::AgentOld;
use crate::error_old::Error;
use anyhow::Result;
use core::fmt::Debug;
#[cfg(feature = "unstable")]
use dimas_com::traits_old::LivelinessSubscriber;
use dimas_com::traits_old::{
	Communicator, CommunicatorMethods, Observer, Publisher, Querier, Responder,
};
use dimas_config::Config;
#[cfg(doc)]
use dimas_core::traits::Context;
use dimas_core::{
	enums::TaskSignal,
	message_types::{Message, QueryableMsg},
	traits::ContextAbstraction,
	OperationState, Operational,
};
use dimas_time::IntervalTimer;
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::Sender;
use tracing::{event, info, instrument, Level};
use zenoh::Session;
// endregion:	--- modules

// region:		--- types
// the initial size of the HashMaps
const INITIAL_SIZE: usize = 9;
// endregion:	--- types

// region:		--- ContextImpl
/// [`ContextImpl`] makes all relevant data of the [`Agent`] accessible via accessor methods.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ContextImpl<P>
where
	P: Send + Sync + 'static,
{
	/// The [`Agent`]s uuid
	uuid: String,
	/// The [`Agent`]s name
	/// Name must not, but should be unique.
	name: Option<String>,
	/// A prefix to separate communication for different groups
	prefix: Option<String>,
	/// The [`Agent`]s current operational state
	#[allow(unused)]
	state: Arc<RwLock<OperationState>>,
	/// A sender for sending signals to owner of context
	sender: Sender<TaskSignal>,
	/// The [`Agent`]s property structure
	props: Arc<RwLock<P>>,
	/// The [`Agent`]s [`Communicator`]
	communicator: Arc<RwLock<dyn Communicator>>,
	/// Registered [`Timer`]
	timers: Arc<RwLock<HashMap<String, IntervalTimer>>>,
}

impl<P> ContextAbstraction for ContextImpl<P>
where
	P: Send + Sync + 'static,
{
	type Props = P;
	/// Get the name
	#[must_use]
	fn name(&self) -> Option<&String> {
		self.name.as_ref()
	}

	#[must_use]
	fn fq_name(&self) -> Option<String> {
		self.name().and_then(|name| {
			self.prefix().map_or_else(
				|| Some(name.into()),
				|prefix| Some(format!("{prefix}/{name}")),
			)
		})
	}

	#[must_use]
	fn uuid(&self) -> String {
		self.uuid.clone()
	}

	#[must_use]
	fn prefix(&self) -> Option<&String> {
		self.prefix.as_ref()
	}

	#[must_use]
	fn sender(&self) -> &Sender<TaskSignal> {
		&self.sender
	}

	fn read(&self) -> parking_lot::lock_api::RwLockReadGuard<'_, parking_lot::RawRwLock, P> {
		self.props.read()
	}

	fn write(&self) -> parking_lot::lock_api::RwLockWriteGuard<'_, parking_lot::RawRwLock, P> {
		self.props.write()
	}

	#[instrument(level = Level::ERROR, skip_all)]
	fn put_with(&self, selector: &str, message: Message) -> Result<()> {
		if self.publishers().read().get(selector).is_some() {
			self.publishers()
				.read()
				.get(selector)
				.ok_or_else(|| Error::Get("publishers".into()))?
				.put(message)?;
		} else {
			self.communicator.read().put(selector, message)?;
		};
		Ok(())
	}

	#[instrument(level = Level::ERROR, skip_all)]
	fn delete_with(&self, selector: &str) -> Result<()> {
		if self.publishers().read().get(selector).is_some() {
			self.publishers()
				.read()
				.get(selector)
				.ok_or_else(|| Error::Get("publishers".into()))?
				.delete()?;
		} else {
			self.communicator.read().delete(selector)?;
		}
		Ok(())
	}

	#[instrument(level = Level::ERROR, skip_all)]
	fn get_with(
		&self,
		selector: &str,
		message: Option<Message>,
		callback: Option<&mut dyn FnMut(QueryableMsg) -> Result<()>>,
	) -> Result<()> {
		if self.queriers().read().get(selector).is_some() {
			self.queriers()
				.read()
				.get(selector)
				.ok_or_else(|| Error::Get("queries".into()))?
				.get(message, callback)?;
		} else {
			self.communicator
				.read()
				.get(selector, message, callback)?;
		};
		Ok(())
	}

	#[instrument(level = Level::ERROR, skip_all)]
	fn observe_with(&self, selector: &str, message: Option<Message>) -> Result<()> {
		self.observers()
			.read()
			.get(selector)
			.ok_or_else(|| Error::Get("observers".into()))?
			.request(message)?;
		Ok(())
	}

	#[instrument(level = Level::ERROR, skip_all)]
	fn cancel_observe_with(&self, selector: &str) -> Result<()> {
		self.observers()
			.read()
			.get(selector)
			.ok_or_else(|| Error::Get("observers".into()))?
			.cancel()?;
		Ok(())
	}

	fn mode(&self) -> String {
		self.communicator.read().mode()
	}

	fn default_session(&self) -> Arc<Session> {
		self.communicator.read().default_session()
	}

	fn session(&self, session_id: &str) -> Option<Arc<Session>> {
		if session_id == "default" {
			Some(self.communicator.read().default_session())
		} else {
			self.communicator.read().session(session_id)
		}
	}
}

impl<P> ContextImpl<P>
where
	P: Send + Sync + 'static,
{
	/// Constructor for the [`ContextImpl`]
	/// # Errors
	pub fn new(
		config: &Config,
		props: P,
		name: Option<String>,
		sender: Sender<TaskSignal>,
		prefix: Option<String>,
	) -> Result<Self> {
		let communicator = dimas_com::communicator_old::from(config)?;
		let uuid = communicator.read().uuid();
		Ok(Self {
			uuid,
			name,
			prefix,
			state: Arc::new(RwLock::new(OperationState::Created)),
			sender,
			communicator,
			props: Arc::new(RwLock::new(props)),
			timers: Arc::new(RwLock::new(HashMap::with_capacity(INITIAL_SIZE))),
		})
	}

	/// Set the [`Context`]s state
	/// # Errors
	fn modify_state_property(&self, state: OperationState) {
		*(self.state.write()) = state;
	}

	/// Get the liveliness subscribers
	#[cfg(feature = "unstable")]
	#[must_use]
	pub fn liveliness_subscribers(
		&self,
	) -> Arc<RwLock<HashMap<String, Box<dyn LivelinessSubscriber>>>> {
		self.communicator.read().liveliness_subscribers()
	}

	/// Get the observers
	#[must_use]
	pub fn observers(&self) -> Arc<RwLock<HashMap<String, Box<dyn Observer>>>> {
		self.communicator.read().observers()
	}

	/// Get the publishers
	#[must_use]
	pub fn publishers(&self) -> Arc<RwLock<HashMap<String, Box<dyn Publisher>>>> {
		self.communicator.read().publishers()
	}

	/// Get the queries
	#[must_use]
	pub fn queriers(&self) -> Arc<RwLock<HashMap<String, Box<dyn Querier>>>> {
		self.communicator.read().queriers()
	}

	/// Get the responders
	#[must_use]
	pub fn responders(&self) -> Arc<RwLock<HashMap<String, Box<dyn Responder>>>> {
		self.communicator.read().responders()
	}

	/// Get the timers
	#[must_use]
	pub fn timers(&self) -> Arc<RwLock<HashMap<String, IntervalTimer>>> {
		self.timers.clone()
	}

	/// Internal function for starting all registere)d tasks.
	///
	/// The tasks are started in the order
	/// - [`LivelinessSubscriber`]s
	/// - [`Queryable`]s
	/// - [`Observable`]s
	/// - [`Subscriber`]s  and last
	/// - [`Timer`]s
	///
	/// Beforehand of starting the [`Timer`]s there is the initialisation of the
	/// - [`Publisher`]s the
	/// - [`Observer`]s and the
	/// - [`Query`]s
	///
	/// # Errors
	/// Currently none
	#[instrument(level = Level::DEBUG, skip_all)]
	fn upgrade_registered_tasks(&self, new_state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "upgrade_registered_tasks");
		// start communication
		self.communicator
			.write()
			.state_transitions(new_state)?;

		// start all registered timers
		self.timers.write().iter_mut().for_each(|timer| {
			let _ = timer.1.state_transitions(new_state);
		});

		self.modify_state_property(new_state);
		Ok(())
	}

	/// Internal function for stopping all registered tasks.
	///
	/// The tasks are stopped in reverse order of their start in [`Context::start_registered_tasks()`]
	///
	/// # Errors
	/// Currently none
	#[instrument(level = Level::DEBUG, skip_all)]
	fn downgrade_registered_tasks(&self, new_state: OperationState) -> Result<()> {
		event!(Level::DEBUG, "downgrade_registered_tasks");
		// reverse order of start!
		// stop all registered timers
		self.timers.write().iter_mut().for_each(|timer| {
			let _ = timer.1.state_transitions(new_state);
		});

		// start communication
		self.communicator
			.write()
			.state_transitions(new_state)?;

		self.modify_state_property(new_state);
		Ok(())
	}
}
// endregion:	--- ContextImpl

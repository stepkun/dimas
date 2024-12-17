// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Zenoh based communication

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::sync::Arc;
use anyhow::Result;
use dimas_core::{
	message_types::Message, Activity, ActivityId, Agent, CommunicatorFactory, Component, ComponentData, ComponentId, ManageOperationState, OperationState, Operational, OperationalData, Transitions
};
use tracing::{error, event, info, instrument, warn, Level};
use uuid::Uuid;
#[cfg(feature = "std")]
use zenoh::{Config, Session, Wait};

use crate::traits_old::Communicator;

use super::subscriber::Subscriber;

/// Component providing zenoh based communication
#[derive(Debug)]
pub struct Zenoh {
	operational: OperationalData,
	component: ComponentData,
	session: Arc<Session>,
}

impl CommunicatorFactory for Zenoh {
    fn create_subscriber<CB, F>(&mut self, selector: &str, callback: CB) -> Result<()>
	    where
		    CB: FnMut(Agent, Message) -> F + Send + Sync + 'static,
		    F: std::future::Future<Output = Result<()>> + Send + Sync + 'static
	{
		let context = self.component.ctx.clone().expect("snh");
        let activity = Box::new(Subscriber::new(selector, context));
		self.add_activity(activity);
		Ok(())
    }
}

impl Component for Zenoh {
	fn uuid(&self) -> Uuid {
		self.component.uuid
	}

	fn id(&self) -> ComponentId {
		self.component.id.clone()
	}

	fn version(&self) -> u32 {
		self.component.version
	}

	fn add_activity(&mut self, mut activity: Box<dyn Activity>) {
		self.component.activities.push(activity);
	}

	fn remove_activity(&mut self, id: ActivityId) {
		todo!()
	}

	fn add_component(&mut self, component: Box<dyn Component>) {
		self.component.components.push(component);
	}

	fn remove_component(&mut self, id: ComponentId) {
		todo!()
	}
}

impl ManageOperationState for Zenoh {
	#[instrument(level = Level::TRACE, skip_all)]
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		event!(Level::TRACE, "manage_operation_state");
		let desired_state = self.desired_state(state);
		// step up?
		while self.state() < desired_state {
			assert!(self.state() < OperationState::Active);
			let next_state = self.state() + 1;
			// first handle sub elements
			for component in &mut self.component.components {
				component.state_transitions(next_state)?;
			}
			for activity in &mut self.component.activities {
				activity.state_transitions(next_state)?;
			}
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}
		// step down?
		while self.state() > desired_state {
			assert!(self.state() > OperationState::Created);
			let next_state = self.state() - 1;
			// first handle sub elements
			for activity in &mut self.component.activities {
				activity.state_transitions(next_state)?;
			}
			for component in &mut self.component.components {
				component.state_transitions(next_state)?;
			}
			// next do own transition
			self.state_transitions(next_state)?;
			self.set_state(next_state);
		}
		Ok(())
	}
}

impl Operational for Zenoh {
	fn activation_state(&self) -> OperationState {
		self.operational.activation
	}

	fn set_activation_state(&mut self, state: OperationState) {
		self.operational.activation = state;
	}

	fn state(&self) -> OperationState {
		self.operational.current
	}

	fn set_state(&mut self, state: OperationState) {
		self.operational.current = state;
	}
}

impl Transitions for Zenoh {}

impl Zenoh {
	/// Create a zenoh based communication component for the given domain
	/// # Panics
	/// - if the zenoh session cannot be created
	#[must_use]
	pub fn new(domain: &str, ctx: Agent) -> Self {
		let session = Arc::new(
			zenoh::open(Config::default())
				.wait()
				.expect("snh"),
		);
		Self {
			operational: OperationalData::default(),
			component: ComponentData::new(domain, 1, ctx),
			session,
		}
	}
}

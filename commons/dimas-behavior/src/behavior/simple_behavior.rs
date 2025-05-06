// Copyright © 2025 Stephan Kunz

//! `DiMAS` implementation for registering functions as behavior

// region:      --- modules
use alloc::{boxed::Box, sync::Arc};

use crate::{blackboard::BlackboardNodeRef, port::PortList, tree::BehaviorTreeComponentList};

use super::{
	BehaviorCreationFn, BehaviorInstanceMethods, BehaviorRedirectionMethods, BehaviorResult,
	BehaviorTickData, BehaviorTreeMethods,
};
// endregion:   --- modules

// region:      --- types
/// Signature of a simple registered behavior function called by `SimpleBehavior`'s tick
pub type SimpleBhvrTickFn = Arc<dyn Fn() -> BehaviorResult + Send + Sync + 'static>;

/// Signature of a registered behavior function called by `SimpleBehavior`'s tick
pub type ComplexBhvrTickFn =
	Arc<dyn Fn(&mut BlackboardNodeRef) -> BehaviorResult + Send + Sync + 'static>;
// endregion:   --- types

// region:      --- BehaviorFunction
/// A simple behavior
pub struct SimpleBehavior {
	/// the function to be called on tick
	pub(crate) simple_tick_fn: Option<SimpleBhvrTickFn>,
	/// the function to be called on tick if ports exist
	pub(crate) complex_tick_fn: Option<ComplexBhvrTickFn>,
	/// list of provided ports
	pub(crate) provided_ports: PortList,
}

impl core::fmt::Debug for SimpleBehavior {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SimpleBehavior")
			//.field("tick_fn", &self.tick_fn)
			.finish()
	}
}

impl BehaviorTreeMethods for SimpleBehavior {}

impl BehaviorInstanceMethods for SimpleBehavior {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut BlackboardNodeRef,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		if self.complex_tick_fn.is_some() {
			self.complex_tick_fn.as_ref().expect("snh")(blackboard)
		} else {
			(self.simple_tick_fn.as_ref().expect("snh"))()
		}
	}
}

impl BehaviorRedirectionMethods for SimpleBehavior {
	fn static_provided_ports(&self) -> PortList {
		PortList(self.provided_ports.clone())
	}
}

//impl BehaviorStaticMethods for SimpleBehavior {}

/// Implementation resembles the macro generated impl code
impl SimpleBehavior {
	/// Create a `SimpleBehavior` with the given function
	pub fn create(tick_fn: SimpleBhvrTickFn) -> Box<BehaviorCreationFn> {
		Box::new(move || {
			Box::new(Self {
				simple_tick_fn: Some(tick_fn.clone()),
				complex_tick_fn: None,
				provided_ports: PortList::default(),
			})
		})
	}

	/// Create a `SimpleBehavior` with the given function and list of ports
	pub fn new_create_with_ports(
		tick_fn: ComplexBhvrTickFn,
		port_list: PortList,
	) -> Box<BehaviorCreationFn> {
		Box::new(move || {
			Box::new(Self {
				simple_tick_fn: None,
				complex_tick_fn: Some(tick_fn.clone()),
				provided_ports: PortList(port_list.clone()),
			})
		})
	}
}
// endregion:   --- BehaviorFunction

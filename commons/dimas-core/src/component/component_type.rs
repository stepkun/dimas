// Copyright © 2024 Stephan Kunz

//! Component interface for `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use crate::{operational::Transitions, Activity, OperationState, Operational, OperationalType};
use alloc::{boxed::Box, sync::Arc, vec::Vec};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use super::{Component, ComponentId};
// endregion:	--- modules

// region:		--- ComponentType
/// Data necessary for a [`Component`].
#[derive(Clone, Debug, Default)]
pub struct ComponentType {
	id: ComponentId,
	operational: OperationalType,
	activities: Arc<RwLock<Vec<Box<dyn Activity>>>>,
	components: Arc<RwLock<Vec<Box<dyn Component>>>>,
}

impl Component for ComponentType {
	#[inline]
	fn id(&self) -> ComponentId {
		self.id.clone()
	}

	#[inline]
	fn add(&mut self, component: Box<dyn Component>) {
		self.components.write().push(component);
	}

	#[inline]
	fn remove(&mut self, _id: ComponentId) {
		todo!()
	}

	#[inline]
	fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>> {
		self.activities.read()
	}

	#[inline]
	fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>> {
		self.activities.write()
	}

	#[inline]
	fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>> {
		self.components.read()
	}

	#[inline]
	fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>> {
		self.components.write()
	}

	fn set_id(&mut self, id: alloc::string::String) {
		self.id = id;
	}
}

impl Transitions for ComponentType {}

impl Operational for ComponentType {
	#[inline]
	fn activation_state(&self) -> OperationState {
		self.operational.activation_state()
	}

	#[inline]
	fn set_activation_state(&mut self, state: OperationState) {
		self.operational.set_activation_state(state);
	}

	#[inline]
	fn desired_state(&self, state: OperationState) -> OperationState {
		self.operational.desired_state(state)
	}

	#[inline]
	fn state(&self) -> OperationState {
		self.operational.state()
	}

	#[inline]
	fn set_state(&mut self, state: OperationState) {
		self.operational.set_state(state);
	}
}

impl ComponentType {
	/// Create a [`ComponentId`] wirh default activation state [`OperationState::Active`].
	#[must_use]
	pub fn new(id: ComponentId) -> Self {
		Self::with_activation_state(id, OperationState::Active)
	}

	/// Create a [`ComponentId`] with given state.
	#[must_use]
	pub fn with_activation_state(id: ComponentId, activation_state: OperationState) -> Self {
		Self {
			id,
			operational: OperationalType::with_activation_state(activation_state),
			activities: Arc::new(RwLock::new(Vec::default())),
			components: Arc::new(RwLock::new(Vec::default())),
		}
	}
}
// endregion:	--- ComponentType

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<ComponentType>();
	}

	#[derive(Debug)]
	struct TestComponent {
		operational: OperationalType,
		component: ComponentType,
	}

	impl AsMut<ComponentType> for TestComponent {
		fn as_mut(&mut self) -> &mut ComponentType {
			&mut self.component
		}
	}

	impl AsRef<ComponentType> for TestComponent {
		fn as_ref(&self) -> &ComponentType {
			&self.component
		}
	}

	impl Transitions for TestComponent {}

	impl Component for TestComponent {
		#[inline]
		fn add(&mut self, component: Box<dyn Component>) {
			self.component.add(component);
		}

		#[inline]
		fn remove(&mut self, id: ComponentId) {
			self.component.remove(id);
		}

		#[inline]
		fn id(&self) -> ComponentId {
			self.component.id()
		}

		#[inline]
		fn activities(&self) -> RwLockReadGuard<Vec<Box<dyn Activity>>> {
			self.component.activities()
		}

		#[inline]
		fn activities_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Activity>>> {
			self.component.activities_mut()
		}

		#[inline]
		fn components(&self) -> RwLockReadGuard<Vec<Box<dyn Component>>> {
			self.component.components()
		}

		#[inline]
		fn components_mut(&mut self) -> RwLockWriteGuard<Vec<Box<dyn Component>>> {
			self.component.components_mut()
		}

		#[inline]
		fn set_id(&mut self, id: alloc::string::String) {
			self.component.set_id(id);
		}
	}

	impl Operational for TestComponent {
		#[inline]
		fn activation_state(&self) -> OperationState {
			self.operational.activation_state()
		}

		#[inline]
		fn set_activation_state(&mut self, state: OperationState) {
			self.operational.set_activation_state(state);
		}

		#[inline]
		fn desired_state(&self, state: OperationState) -> OperationState {
			self.operational.desired_state(state)
		}

		#[inline]
		fn state(&self) -> OperationState {
			self.operational.state()
		}

		#[inline]
		fn set_state(&mut self, state: OperationState) {
			self.operational.set_state(state);
		}
	}

	fn create_test_data() -> TestComponent {
		let mut component = TestComponent {
			operational: OperationalType::default(),
			component: ComponentType::new("component".into()),
		};

		let mut component1 = TestComponent {
			operational: OperationalType::with_activation_state(OperationState::Standby),
			component: ComponentType::new("component1".into()),
		};

		let mut component2 = TestComponent {
			operational: OperationalType::with_activation_state(OperationState::Inactive),
			component: ComponentType::new("component2".into()),
		};

		let component3 = TestComponent {
			operational: OperationalType::with_activation_state(OperationState::Created),
			component: ComponentType::new("component3".into()),
		};

		// create structure
		component2.component.add(Box::new(component3));
		component1.component.add(Box::new(component2));
		component.component.add(Box::new(component1));

		component
	}

	fn activate() {
		let mut component = create_test_data();
		assert_eq!(component.state(), OperationState::Undefined);

		// set parent to Active
		assert!(component
			.manage_operation_state(OperationState::Active)
			.is_ok());
		assert_eq!(component.state(), OperationState::Active);
		for sub in &*component.components() {
			// level 1 should be one step further than level 0 => Active
			assert_eq!(sub.state(), OperationState::Active);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Active
				assert_eq!(sub.state(), OperationState::Active);
				for sub in &*sub.components() {
					// level 3 should already be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}
	}

	fn up_stepping() {
		let mut component = create_test_data();

		// set parent to Created
		assert!(component
			.manage_operation_state(OperationState::Created)
			.is_ok());
		assert_eq!(component.state(), OperationState::Created);
		for sub in &*component.components() {
			// level 1 should be one step further than level 0 => Configured
			assert_eq!(sub.state(), OperationState::Configured);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Standby
				assert_eq!(sub.state(), OperationState::Standby);
				for sub in &*sub.components() {
					// level 3 should already be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}

		// set parent to Configured
		assert!(component
			.manage_operation_state(OperationState::Configured)
			.is_ok());
		assert_eq!(component.state(), OperationState::Configured);
		for sub in &*component.components() {
			// level 1 should be one step further than level 0 => Inactive
			assert_eq!(sub.state(), OperationState::Inactive);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Active
				assert_eq!(sub.state(), OperationState::Active);
				for sub in &*sub.components() {
					// level 3 should already be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}
	}

	fn down_stepping() {
		let mut component = create_test_data();
		assert!(component
			.manage_operation_state(OperationState::Active)
			.is_ok());

		// set parent to Standby
		assert!(component
			.manage_operation_state(OperationState::Standby)
			.is_ok());
		assert_eq!(component.state(), OperationState::Standby);
		for sub in &*component.components() {
			// level 1 should be one step further level 0 => Active
			assert_eq!(sub.state(), OperationState::Active);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Active
				assert_eq!(sub.state(), OperationState::Active);
				for sub in &*sub.components() {
					// level 3 should still be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}

		// set parent to Configured
		assert!(component
			.manage_operation_state(OperationState::Configured)
			.is_ok());
		assert_eq!(component.state(), OperationState::Configured);
		for sub in &*component.components() {
			// level 1 should be one step further than level 0 => Inactive
			assert_eq!(sub.state(), OperationState::Inactive);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Active
				assert_eq!(sub.state(), OperationState::Active);
				for sub in &*sub.components() {
					// level 3 should already be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}
	}

	fn up_and_down() {
		let mut component = create_test_data();
		assert!(component
			.manage_operation_state(OperationState::Active)
			.is_ok());

		// set parent to Created
		assert!(component
			.manage_operation_state(OperationState::Created)
			.is_ok());
		assert_eq!(component.state(), OperationState::Created);
		for sub in &*component.components() {
			// level 1 should be one step further than level 0 => Configured
			assert_eq!(sub.state(), OperationState::Configured);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Standby
				assert_eq!(sub.state(), OperationState::Standby);
				for sub in &*sub.components() {
					// level 3 should already be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}
	}

	fn no_stepping() {
		let mut component = create_test_data();
		assert!(component
			.manage_operation_state(OperationState::Configured)
			.is_ok());

		// set parent to Configured again
		assert!(component
			.manage_operation_state(OperationState::Configured)
			.is_ok());
		assert_eq!(component.state(), OperationState::Configured);
		for sub in &*component.components() {
			// level 1 should be one step further than level 0 => Inactive
			assert_eq!(sub.state(), OperationState::Inactive);

			for sub in &*sub.components() {
				// level 2 should be 2 steps further than level 1 => Active
				assert_eq!(sub.state(), OperationState::Active);
				for sub in &*sub.components() {
					// level 3 should already be active
					assert_eq!(sub.state(), OperationState::Active);
				}
			}
		}
	}

	#[test]
	fn all_tests() {
		let _ = ComponentType::new(ComponentId::from("test"));
		activate();
		up_stepping();
		down_stepping();
		up_and_down();
		no_stepping();
	}
}

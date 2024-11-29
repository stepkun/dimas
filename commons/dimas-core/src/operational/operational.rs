// Copyright © 2024 Stephan Kunz

//! Lifecycle interface for `DiMAS` entities
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::{boxed::Box, vec::Vec};
use anyhow::Result;
use core::fmt::Debug;

//use crate::traits::Component;

use super::{Error, OperationState};
// endregion:	--- modules

// region:		--- Operational
/// Contract for [`Operational`]
pub trait Operational: Debug + Send + Sync {
	/// A method to read the entities current [`OperationState`] must be provided
	fn state(&self) -> OperationState;

	/// A method to write the entities current [`OperationState`] must be provided
	fn set_state(&mut self, _state: OperationState);

	/// A method to access the entities sub [`Operational`]s must be provided
	fn operationals(&mut self) -> &mut Vec<Box<dyn Operational>>;

	/// # Errors
	/// @TODO: remove
	fn manage_operation_state_old(&self, _state: OperationState) -> Result<()> {
		Ok(())
	}

	/// Checks wether state of [`Operational`] is appropriate for the given [`OperationState`].
	/// If not, adjusts components state to needs considering its sub-components.
	/// # Errors
	fn manage_operation_state(&mut self, state: OperationState) -> Result<()> {
		// step up?
		while self.state() < state {
			match self.state() {
				OperationState::Error | OperationState::Active => {
					return Err(Error::ManageState.into())
				}
				OperationState::Created => {
					for component in self.operationals() {
						let state = component.configure()?;
						component.set_state(state);
					}
					let state = self.configure()?;
					self.set_state(state);
				}
				OperationState::Configured => {
					for component in self.operationals() {
						let state = component.commission()?;
						component.set_state(state);
					}
					let state = self.commission()?;
					self.set_state(state);
				}
				OperationState::Inactive => {
					for component in self.operationals() {
						let state = component.wakeup()?;
						component.set_state(state);
					}
					let state = self.wakeup()?;
					self.set_state(state);
				}
				OperationState::Standby => {
					for component in self.operationals() {
						let state = component.activate()?;
						component.set_state(state);
					}
					let state = self.activate()?;
					self.set_state(state);
				}
			}
		}

		// step down?
		while self.state() > state {
			match self.state() {
				OperationState::Error | OperationState::Created => {
					return Err(Error::ManageState.into())
				}
				OperationState::Active => {
					for component in self.operationals() {
						let state = component.deactivate()?;
						component.set_state(state);
					}
					let state = self.deactivate()?;
					self.set_state(state);
				}
				OperationState::Standby => {
					for component in self.operationals() {
						let state = component.suspend()?;
						component.set_state(state);
					}
					let state = self.suspend()?;
					self.set_state(state);
				}
				OperationState::Inactive => {
					for component in self.operationals() {
						let state = component.decommission()?;
						component.set_state(state);
					}
					let state = self.decommission()?;
					self.set_state(state);
				}
				OperationState::Configured => {
					for component in self.operationals() {
						let state = component.deconfigure()?;
						component.set_state(state);
					}
					let state = self.deconfigure()?;
					self.set_state(state);
				}
			}
		}

		Ok(())
	}

	/// configuration transition
	/// The default implementation just returns [`OperationState::Configured`]
	/// # Errors
	/// if something went wrong
	fn configure(&mut self) -> Result<OperationState> {
		Ok(OperationState::Configured)
	}

	/// comissioning transition
	/// The default implementation just returns [`OperationState::Inactive`]
	/// # Errors
	/// if something went wrong
	fn commission(&mut self) -> Result<OperationState> {
		Ok(OperationState::Inactive)
	}

	/// wake up transition
	/// The default implementation just returns [`OperationState::Standby`]
	/// # Errors
	/// if something went wrong
	fn wakeup(&mut self) -> Result<OperationState> {
		Ok(OperationState::Standby)
	}

	/// activate transition
	/// The default implementation just returns [`OperationState::Active`]
	/// # Errors
	/// if something went wrong
	fn activate(&mut self) -> Result<OperationState> {
		Ok(OperationState::Active)
	}

	/// deactivate transition
	/// The default implementation just returns [`OperationState::Standby`]
	/// # Errors
	/// if something went wrong
	fn deactivate(&mut self) -> Result<OperationState> {
		Ok(OperationState::Standby)
	}

	/// suspend transition
	/// The default implementation just returns [`OperationState::Inactive`]
	/// # Errors
	/// if something went wrong
	fn suspend(&mut self) -> Result<OperationState> {
		Ok(OperationState::Inactive)
	}

	/// decomission transition
	/// The default implementation just returns [`OperationState::Configured`]
	/// # Errors
	/// if something went wrong
	fn decommission(&mut self) -> Result<OperationState> {
		Ok(OperationState::Configured)
	}

	/// deconfigure transition
	/// The default implementation just returns [`OperationState::Created`]
	/// # Errors
	/// if something went wrong
	fn deconfigure(&mut self) -> Result<OperationState> {
		Ok(OperationState::Created)
	}
}
// endregion:	--- Operational

#[cfg(test)]
mod tests {
	use super::*;

	// check, that the auto traits are available
	const fn is_normal<T: Sized + Send + Sync>() {}

	#[test]
	const fn normal_types() {
		is_normal::<Box<dyn Operational>>();
	}

	#[derive(Debug)]
	struct TestOperational {
		value: i32,
		state: OperationState,
		subs: Vec<Box<dyn Operational>>,
	}

	impl Operational for TestOperational {
		fn state(&self) -> OperationState {
			self.state
		}

		fn set_state(&mut self, state: OperationState) {
			self.state = state;
		}

		fn operationals(&mut self) -> &mut Vec<Box<dyn Operational>> {
			&mut self.subs
		}

		fn manage_operation_state_old(&self, _state: OperationState) -> Result<()> {
			todo!()
		}

		fn configure(&mut self) -> Result<OperationState> {
			self.value += 1;
			Ok(OperationState::Configured)
		}

		fn commission(&mut self) -> Result<OperationState> {
			self.value += 2;
			Ok(OperationState::Inactive)
		}

		fn wakeup(&mut self) -> Result<OperationState> {
			self.value += 4;
			Ok(OperationState::Standby)
		}

		fn activate(&mut self) -> Result<OperationState> {
			self.value += 8;
			Ok(OperationState::Active)
		}

		fn deactivate(&mut self) -> Result<OperationState> {
			self.value -= 8;
			Ok(OperationState::Standby)
		}

		fn suspend(&mut self) -> Result<OperationState> {
			self.value -= 4;
			Ok(OperationState::Inactive)
		}

		fn decommission(&mut self) -> Result<OperationState> {
			self.value -= 2;
			Ok(OperationState::Configured)
		}

		fn deconfigure(&mut self) -> Result<OperationState> {
			self.value -= 1;
			Ok(OperationState::Created)
		}
	}

	#[test]
	#[allow(clippy::vec_init_then_push)]
	fn up_stepping() {
		let subs: Vec<Box<dyn Operational>> = Vec::new();
		let sub_operational = TestOperational {
			value: 0,
			state: OperationState::Created,
			subs,
		};

		let mut subs: Vec<Box<dyn Operational>> = Vec::new();
		subs.push(Box::new(sub_operational));

		let mut operational = TestOperational {
			value: 0,
			state: OperationState::Created,
			subs,
		};

		assert!(operational
			.manage_operation_state(OperationState::Active)
			.is_ok());
		assert_eq!(operational.value, 15);
		assert_eq!(operational.state, OperationState::Active);

		for sub in operational.subs {
			assert_eq!(sub.state(), OperationState::Active);
		}
	}

	#[test]
	#[allow(clippy::vec_init_then_push)]
	fn down_stepping() {
		let subs: Vec<Box<dyn Operational>> = Vec::new();
		let sub_operational = TestOperational {
			value: 0,
			state: OperationState::Active,
			subs,
		};

		let mut subs: Vec<Box<dyn Operational>> = Vec::new();
		subs.push(Box::new(sub_operational));

		let mut operational = TestOperational {
			value: 15,
			state: OperationState::Active,
			subs,
		};

		assert!(operational
			.manage_operation_state(OperationState::Created)
			.is_ok());
		assert_eq!(operational.value, 0);
		assert_eq!(operational.state, OperationState::Created);

		for sub in operational.subs {
			assert_eq!(sub.state(), OperationState::Created);
		}
	}

	#[test]
	#[allow(clippy::vec_init_then_push)]
	fn no_stepping() {
		let subs: Vec<Box<dyn Operational>> = Vec::new();
		let sub_operational = TestOperational {
			value: 0,
			state: OperationState::Standby,
			subs,
		};

		let mut subs: Vec<Box<dyn Operational>> = Vec::new();
		subs.push(Box::new(sub_operational));

		let mut operational = TestOperational {
			value: 7,
			state: OperationState::Standby,
			subs,
		};

		assert!(operational
			.manage_operation_state(OperationState::Standby)
			.is_ok());
		assert_eq!(operational.value, 7);
		assert_eq!(operational.state, OperationState::Standby);

		for sub in operational.subs {
			assert_eq!(sub.state(), OperationState::Standby);
		}
	}
}

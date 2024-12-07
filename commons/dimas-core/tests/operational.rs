//! Copyright © 2024 Stephan Kunz

use anyhow::Result;
use dimas_core::{OperationState, Operational, OperationalType, Transitions};

#[dimas_macros::operational]
struct TestOperational1<P>
where
	P: Send + Sync,
{
	dummy: P,
}

impl<P> Transitions for TestOperational1<P> where P: Send + Sync {}

#[dimas_macros::operational]
#[derive(Default)]
struct TestOperational2 {
	/// A value to test that all hooks have been processed
	value: i32,
}

impl Transitions for TestOperational2 {
	fn configure(&mut self) -> Result<()> {
		self.value += 1;
		Ok(())
	}

	fn commission(&mut self) -> Result<()> {
		self.value += 2;
		Ok(())
	}

	fn wakeup(&mut self) -> Result<()> {
		self.value += 4;
		Ok(())
	}

	fn activate(&mut self) -> Result<()> {
		self.value += 8;
		Ok(())
	}

	fn deactivate(&mut self) -> Result<()> {
		self.value -= 8;
		Ok(())
	}

	fn suspend(&mut self) -> Result<()> {
		self.value -= 4;
		Ok(())
	}

	fn decommission(&mut self) -> Result<()> {
		self.value -= 2;
		Ok(())
	}

	fn deconfigure(&mut self) -> Result<()> {
		self.value -= 1;
		Ok(())
	}
}

fn create_test_data() -> TestOperational2 {
	let operational = TestOperational2::default();
	assert_eq!(operational.state(), OperationState::Undefined);
	assert_eq!(operational.activation_state(), OperationState::Active);
	operational
}

#[test]
fn operational() {
	let mut operational = create_test_data();
	assert!(operational
		.state_transitions(OperationState::Created)
		.is_ok());
	assert_eq!(operational.value, 0);
	assert_eq!(operational.state(), OperationState::Created);

	assert!(operational
		.state_transitions(OperationState::Active)
		.is_ok());
	assert_eq!(operational.value, 15);
	assert_eq!(operational.state(), OperationState::Active);

	assert!(operational
		.state_transitions(OperationState::Inactive)
		.is_ok());
	assert_eq!(operational.value, 3);
	assert_eq!(operational.state(), OperationState::Inactive);

	assert!(operational
		.state_transitions(OperationState::Created)
		.is_ok());
	assert_eq!(operational.value, 0);
	assert_eq!(operational.state(), OperationState::Created);
}

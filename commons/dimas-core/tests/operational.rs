//! Copyright © 2024 Stephan Kunz

use anyhow::Result;
use dimas_core::{OperationState, Operational, OperationalType, Transitions};
use std::panic::catch_unwind;

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

fn operational_type() {
	let data = OperationalType::default();

	assert_eq!(
		data.desired_state(OperationState::Created),
		OperationState::Created
	);
	assert_eq!(
		data.desired_state(OperationState::Inactive),
		OperationState::Inactive
	);
	assert_eq!(
		data.desired_state(OperationState::Active),
		OperationState::Active
	);

	let mut data = OperationalType::new(OperationState::Created);

	assert_eq!(
		data.desired_state(OperationState::Created),
		OperationState::Active
	);
	assert_eq!(
		data.desired_state(OperationState::Inactive),
		OperationState::Active
	);
	assert_eq!(
		data.desired_state(OperationState::Active),
		OperationState::Active
	);

	data.set_activation_state(OperationState::Inactive);
	assert_eq!(
		data.desired_state(OperationState::Created),
		OperationState::Inactive
	);
	assert_eq!(
		data.desired_state(OperationState::Inactive),
		OperationState::Active
	);
	assert_eq!(
		data.desired_state(OperationState::Active),
		OperationState::Active
	);
}

fn operational_trait() {
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

fn add() {
	assert_eq!(OperationState::Created + 1, OperationState::Configured);
	assert_eq!(OperationState::Configured + 1, OperationState::Inactive);
	assert_eq!(OperationState::Inactive + 1, OperationState::Standby);
	assert_eq!(OperationState::Standby + 1, OperationState::Active);
	assert_eq!(OperationState::Created + 4, OperationState::Active);
}

fn add_assign() {
	let mut state = OperationState::Created;
	state += 1;
	assert_eq!(&state, &OperationState::Configured);
	state += 1;
	assert_eq!(&state, &OperationState::Inactive);
	state += 1;
	assert_eq!(&state, &OperationState::Standby);
	state += 1;
	assert_eq!(&state, &OperationState::Active);

	state = OperationState::Created;
	state += 4;
	assert_eq!(&state, &OperationState::Active);
}

fn failing_add() {
	assert!(catch_unwind(|| OperationState::Active + 1).is_err());
	assert!(catch_unwind(|| OperationState::Created + 5).is_err());
}

fn sub() {
	assert_eq!(OperationState::Active - 1, OperationState::Standby);
	assert_eq!(OperationState::Standby - 1, OperationState::Inactive);
	assert_eq!(OperationState::Inactive - 1, OperationState::Configured);
	assert_eq!(OperationState::Configured - 1, OperationState::Created);
	assert_eq!(OperationState::Active - 4, OperationState::Created);
}

fn sub_assign() {
	let mut state = OperationState::Active;
	state -= 1;
	assert_eq!(&state, &OperationState::Standby);
	state -= 1;
	assert_eq!(&state, &OperationState::Inactive);
	state -= 1;
	assert_eq!(&state, &OperationState::Configured);
	state -= 1;
	assert_eq!(&state, &OperationState::Created);

	state = OperationState::Active;
	state -= 4;
	assert_eq!(&state, &OperationState::Created);
}

fn failing_sub() {
	assert!(catch_unwind(|| OperationState::Created - 3).is_err());
	assert!(catch_unwind(|| OperationState::Active - 7).is_err());
}

fn operation_state() {
	add();
	add_assign();
	failing_add();
	sub();
	sub_assign();
	failing_sub();
}

#[test]
fn operational() {
	operation_state();
	operational_trait();
	operational_type();
}

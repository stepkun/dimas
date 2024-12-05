//! Copyright © 2024 Stephan Kunz

use dimas_core::{ActivityType, OperationState, Operational};

#[test]
fn activity_type() {
	let data = ActivityType::default();

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

	let mut data = ActivityType::new("test".into());

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

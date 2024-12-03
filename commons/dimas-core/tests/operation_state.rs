//! Copyright © 2024 Stephan Kunz

use dimas_core::OperationState;
use std::panic::catch_unwind;

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

#[test]
fn operation_state() {
	add();
	add_assign();
	failing_add();
	sub();
	sub_assign();
	failing_sub();
}

//! Copyright © 2024 Stephan Kunz

use dimas_core::{Activity, ActivityType};

#[test]
fn activity_type() {
	let data = ActivityType::default();
	assert!(data.id().is_empty());
}

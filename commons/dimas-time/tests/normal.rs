// Copyright © 2024 Stephan Kunz

//! Tests

use dimas_time::{IntervalTimer, IntervalTimerParameter, Timer, TimerVariant};

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<Box<dyn Timer>>();
	is_normal::<IntervalTimerParameter>();
	is_normal::<TimerVariant>();
	is_normal::<IntervalTimer>();
}

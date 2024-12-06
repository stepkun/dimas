// Copyright © 2024 Stephan Kunz

//! Tests

use dimas_time::{IntervalTimer, Timer, IntervalTimerParameter, TimerVariant};

#[derive(Debug)]
struct Props {}

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<Timer<Props>>();
	is_normal::<IntervalTimerParameter>();
	is_normal::<TimerVariant>();
	is_normal::<IntervalTimer<Props>>();
}

// Copyright © 2024 Stephan Kunz

//! Tests

use dimas_time::{IntervalTimer, Timer, TimerBuilder, TimerVariant};

#[derive(Debug)]
struct Props {}

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<Timer<Props>>();
	is_normal::<TimerBuilder<Props>>();
	is_normal::<TimerVariant>();
	is_normal::<IntervalTimer<Props>>();
}

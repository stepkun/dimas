// Copyright © 2024 Stephan Kunz

//! Tests

use dimas_config::Config;

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
    is_normal::<Config>();
}

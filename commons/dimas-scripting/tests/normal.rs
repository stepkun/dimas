// Copyright © 2024 Stephan Kunz
#![allow(unused)]
//! Tests

use dimas_scripting::{Lexer, Parser, VM};

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<Lexer>();
	is_normal::<Parser>();
	is_normal::<VM>();
}

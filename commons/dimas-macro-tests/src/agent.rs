// Copyright © 2024 Stephan Kunz
#![allow(dead_code)]

//! Test macro `agent`

#[doc(hidden)]
extern crate alloc;

use alloc::{boxed::Box, string::String};
use dimas_core::{Activity, ActivityId, Component, ComponentId};
use uuid::Uuid;

#[derive(Debug, Default)]
struct SomeStruct {
	s1: i32,
	s2: String,
}

//#[dimas_macros::agent]
//#[derive(Debug, Default)]
//struct TestProperties1(i32, String, SomeStruct);

#[dimas_macros::agent]
#[derive(Debug, Default)]
struct TestProperties2 {}

// #[dimas_macros::agent]
// #[derive(Debug, Default)]
// pub struct TestProperties3 {
// 	v1: i32,
// 	v2: String,
// 	v3: SomeStruct,
// }

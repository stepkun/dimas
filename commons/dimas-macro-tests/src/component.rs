// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! Test macro `agent`

#[doc(hidden)]
extern crate alloc;

use alloc::{boxed::Box, string::String};
use core::fmt::Debug;
use uuid::Uuid;

use dimas_core::{Activity, ActivityId, Agent, Component, ComponentId};

//#[derive(Debug, Default)]
struct SomeStruct {
	s1: i32,
	s2: String,
}

//#[dimas_macros::component]
//struct TestComponent1(i32, String);

#[dimas_macros::component]
#[derive(Debug)]
struct TestComponent2 {}

// #[dimas_macros::component]
// #[derive(Debug)]
// struct TestComponent3 {
// 	v1: i32,
// 	v2: String,
// 	v3: SomeStruct,
// }

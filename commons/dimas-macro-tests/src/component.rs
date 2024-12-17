// Copyright © 2024 Stephan Kunz
#![allow(unused)]

//! Test macro `agent`

#[doc(hidden)]
extern crate alloc;

use alloc::{boxed::Box, string::String};
use core::fmt::Debug;
use uuid::Uuid;

use dimas_core::{Activity, ActivityId, Agent, Component, ComponentId, ManageOperationState, Operational, Transitions};

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

impl Transitions for TestComponent2 {}

impl Operational for TestComponent2 {
    fn activation_state(&self) -> dimas_core::OperationState {
        todo!()
    }

    fn set_activation_state(&mut self, state: dimas_core::OperationState) {
        todo!()
    }

    fn state(&self) -> dimas_core::OperationState {
        todo!()
    }

    fn set_state(&mut self, state: dimas_core::OperationState) {
        todo!()
    }
}

impl ManageOperationState for TestComponent2 {
    fn manage_operation_state(&mut self, state: dimas_core::OperationState) -> anyhow::Result<()> {
        todo!()
    }
}

// #[dimas_macros::component]
// #[derive(Debug)]
// struct TestComponent3 {
// 	v1: i32,
// 	v2: String,
// 	v3: SomeStruct,
// }

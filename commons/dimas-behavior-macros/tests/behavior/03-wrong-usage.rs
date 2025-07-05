// Copyright © 2025 Stephan Kunz

//! Test wrong usage of behavior derive macro `Behavior` 

#[doc(hidden)]
extern crate alloc;

#[derive(dimas_behavior_macros::Behavior)]
struct TestBehavior;


// dummy main
fn main(){}

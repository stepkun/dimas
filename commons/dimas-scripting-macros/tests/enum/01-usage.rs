// Copyright © 2025 Stephan Kunz

//! Test correct usage of scripting enum derive macro `ScriptEnum` 

#[doc(hidden)]
extern crate alloc;

#[derive(dimas_scripting_macros::ScriptEnum)]
enum TestEnum {
    CaseA,
    CaseB,
}

// dummy main
fn main(){}

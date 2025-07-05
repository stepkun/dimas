// Copyright © 2025 Stephan Kunz

//! Test wrong usage of scripting enum derive macro `ScriptEnum` 

#[dimas_scripting_macros::ScriptEnum]
enum TestEnum {
    CaseA,
    CaseB,
}

// dummy main
fn main(){}

// Copyright © 2024 Stephan Kunz

//! Test wrong usage of main macro

#[dimas_macros::main]
pub struct Test {}

#[dimas_macros::main]
fn not_main() {}

#[dimas_macros::main]
fn main() {}
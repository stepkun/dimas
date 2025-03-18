// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Precedence definitions for `DiMAS` scripting Pratt-Parser
//!
//! Defines the different precedence levels used by the infix parsers.
//! These determine how a series of infix expressions will be grouped.
//! For example, "a + b * c - d" will be parsed as "(a + (b * c)) - d"
//! because "*" has higher precedence than "+" and "-".
//! Here a bigger numbers is higher precedence.
//!

/// @TODO
pub const NONE: i32 = 0;
/// @TODO
pub const ASSIGNMENT: i32 = NONE + 1; // = :=
/// @TODO
pub const OR: i32 = ASSIGNMENT + 1; // ||
/// @TODO
pub const AND: i32 = OR + 1; // &&
/// @TODO
pub const EQUALITY: i32 = AND + 1; // == !=
/// @TODO
pub const COMPARISON: i32 = EQUALITY + 1; // == !=
/// @TODO
pub const TERM: i32 = COMPARISON + 1; // + -
/// @TODO
pub const FACTOR: i32 = TERM + 1; // * /
/// @TODO
pub const UNARY: i32 = FACTOR + 1; // ! -
/// @TODO
pub const PRIMARY: i32 = UNARY + 1;

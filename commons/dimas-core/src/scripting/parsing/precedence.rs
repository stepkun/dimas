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

pub type Precedence = i8;
/// @TODO
pub const NONE: Precedence = 0;
/// @TODO
pub const ASSIGNMENT: Precedence = NONE + 1; // = :=
/// @TODO
pub const OR: Precedence = ASSIGNMENT + 1; // ||
/// @TODO
pub const AND: Precedence = OR + 1; // &&
/// @TODO
pub const EQUALITY: Precedence = AND + 1; // == !=
/// @TODO
pub const COMPARISON: Precedence = EQUALITY + 1; // == !=
/// @TODO
pub const TERM: Precedence = COMPARISON + 1; // + -
/// @TODO
pub const FACTOR: Precedence = TERM + 1; // * /
/// @TODO
pub const UNARY: Precedence = FACTOR + 1; // ! -
/// @TODO
pub const PRIMARY: Precedence = UNARY + 1;

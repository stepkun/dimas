// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Op-Code implementation for `DiMAS` scripting

/// The available operation codes
/// These cannot be an enum because an enum in Rust is not u8
/// @TODO
pub const OP_NONE: u8 = 0;
/// @TODO
pub const OP_CONSTANT: u8 = OP_NONE + 1;
/// @TODO
pub const OP_ADD: u8 = OP_CONSTANT + 1;
/// @TODO
pub const OP_SUBTRACT: u8 = OP_ADD + 1;
/// @TODO
pub const OP_MULTIPLY: u8 = OP_SUBTRACT + 1;
/// @TODO
pub const OP_DIVIDE: u8 = OP_MULTIPLY + 1;
/// @TODO
pub const OP_NEGATE: u8 = OP_DIVIDE + 1;
/// @TODO
pub const OP_RETURN: u8 = OP_NEGATE + 1;

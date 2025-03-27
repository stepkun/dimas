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
pub const OP_NIL: u8 = OP_CONSTANT + 1;
/// @TODO
pub const OP_TRUE: u8 = OP_NIL + 1;
/// @TODO
pub const OP_FALSE: u8 = OP_TRUE + 1;
/// @TODO
pub const OP_POP: u8 = OP_FALSE + 1;
/// @TODO
pub const OP_DEFINE_EXTERNAL: u8 = OP_POP + 1;
/// @TODO
pub const OP_GET_EXTERNAL: u8 = OP_DEFINE_EXTERNAL + 1;
/// @TODO
pub const OP_SET_EXTERNAL: u8 = OP_GET_EXTERNAL + 1;
/// @TODO
pub const OP_EQUAL: u8 = OP_SET_EXTERNAL + 1;
/// @TODO
pub const OP_GREATER: u8 = OP_EQUAL + 1;
/// @TODO
pub const OP_LESS: u8 = OP_GREATER + 1;
/// @TODO
pub const OP_ADD: u8 = OP_LESS + 1;
/// @TODO
pub const OP_SUBTRACT: u8 = OP_ADD + 1;
/// @TODO
pub const OP_MULTIPLY: u8 = OP_SUBTRACT + 1;
/// @TODO
pub const OP_DIVIDE: u8 = OP_MULTIPLY + 1;
/// @TODO
pub const OP_BINARY_NOT: u8 = OP_DIVIDE + 1;
/// @TODO
pub const OP_NOT: u8 = OP_BINARY_NOT + 1;
/// @TODO
pub const OP_NEGATE: u8 = OP_NOT + 1;
/// @TODO
pub const OP_RETURN: u8 = OP_NEGATE + 1;
/// @TODO
#[cfg(feature = "std")]
pub const OP_PRINT: u8 = OP_RETURN + 1;

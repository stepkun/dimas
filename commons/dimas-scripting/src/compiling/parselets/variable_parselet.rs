// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `VariableParselet` for `Dimas`scripting
//!

use alloc::{boxed::Box, string::ToString};

use crate::{
	Parser,
	compiling::{
		error::Error,
		precedence::Precedence,
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_CONSTANT, OP_DEFINE_EXTERNAL, OP_GET_EXTERNAL, OP_SET_EXTERNAL},
		values::Value,
	},
};

use super::{Expression, PrefixParselet};

pub struct VariableParselet;

impl PrefixParselet for VariableParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		match parser.next().kind {
			TokenKind::ColonEqual => {
				parser.advance()?;
				parser.expression(chunk);
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_CONSTANT, name, chunk);
				parser.emit_byte(OP_DEFINE_EXTERNAL, chunk);
			}
			TokenKind::Equal => {
				parser.advance()?;
				parser.expression(chunk);
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_CONSTANT, name, chunk);
				parser.emit_byte(OP_SET_EXTERNAL, chunk);
			}
			_ => {
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_CONSTANT, name, chunk);
				parser.emit_byte(OP_GET_EXTERNAL, chunk);
			}
		}
		Ok(())
	}
}

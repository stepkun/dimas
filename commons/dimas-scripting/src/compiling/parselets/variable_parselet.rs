// Copyright © 2025 Stephan Kunz

//! `VariableParselet` for `Dimas`scripting
//!

use crate::{
	Parser,
	compiling::{
		error::Error,
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{
			OP_ADD, OP_DEFINE_EXTERNAL, OP_DIVIDE, OP_GET_EXTERNAL, OP_MULTIPLY, OP_SET_EXTERNAL,
			OP_SUBTRACT,
		},
	},
};

use super::PrefixParselet;

pub struct VariableParselet;

impl PrefixParselet for VariableParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		match parser.next().kind {
			TokenKind::ColonEqual => {
				parser.advance()?;
				parser.expression(chunk)?;
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_DEFINE_EXTERNAL, name, chunk);
			}
			TokenKind::PlusEqual => {
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_GET_EXTERNAL, name, chunk);
				parser.advance()?;
				parser.expression(chunk)?;
				parser.emit_byte(OP_ADD, chunk);
				parser.emit_bytes(OP_SET_EXTERNAL, name, chunk);
			}
			TokenKind::MinusEqual => {
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_GET_EXTERNAL, name, chunk);
				parser.advance()?;
				parser.expression(chunk)?;
				parser.emit_byte(OP_SUBTRACT, chunk);
				parser.emit_bytes(OP_SET_EXTERNAL, name, chunk);
			}
			TokenKind::StarEqual => {
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_GET_EXTERNAL, name, chunk);
				parser.advance()?;
				parser.expression(chunk)?;
				parser.emit_byte(OP_MULTIPLY, chunk);
				parser.emit_bytes(OP_SET_EXTERNAL, name, chunk);
			}
			TokenKind::SlashEqual => {
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_GET_EXTERNAL, name, chunk);
				parser.advance()?;
				parser.expression(chunk)?;
				parser.emit_byte(OP_DIVIDE, chunk);
				parser.emit_bytes(OP_SET_EXTERNAL, name, chunk);
			}
			TokenKind::Equal => {
				parser.advance()?;
				parser.expression(chunk)?;
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_SET_EXTERNAL, name, chunk);
			}
			_ => {
				let name = chunk.add_string_constant(token.origin)?;
				parser.emit_bytes(OP_GET_EXTERNAL, name, chunk);
			}
		}
		Ok(())
	}
}

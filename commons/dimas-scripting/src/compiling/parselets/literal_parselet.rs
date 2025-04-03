// Copyright © 2025 Stephan Kunz

//! `LiteralParselet` for `Dimas`scripting
//!

use alloc::string::ToString;

use crate::{
	Parser,
	compiling::{
		error::Error,
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_FALSE, OP_NIL, OP_TRUE},
	},
};

use super::PrefixParselet;

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		let kind = parser.current().kind;

		match kind {
			TokenKind::False => {
				parser.emit_byte(OP_FALSE, chunk);
				Ok(())
			}
			TokenKind::Nil => {
				parser.emit_byte(OP_NIL, chunk);
				Ok(())
			}
			TokenKind::True => {
				parser.emit_byte(OP_TRUE, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().to_string(), line!())),
		}
	}
}

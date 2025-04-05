// Copyright © 2025 Stephan Kunz

//! `LiteralParselet` for `Dimas` scripting handles any literal like 'true' and 'false'
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
		op_code::OpCode,
	},
};

use super::PrefixParselet;

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		let kind = parser.current().kind;

		match kind {
			TokenKind::False => {
				parser.emit_byte(OpCode::False as u8, chunk);
				Ok(())
			}
			TokenKind::Nil => {
				parser.emit_byte(OpCode::Nil as u8, chunk);
				Ok(())
			}
			TokenKind::True => {
				parser.emit_byte(OpCode::True as u8, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().to_string(), line!())),
		}
	}
}

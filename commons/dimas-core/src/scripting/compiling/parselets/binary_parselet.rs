// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `GroupingParselet` for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{
	Parser,
	compiling::{
		error::Error,
		precedence::{FACTOR, Precedence, TERM},
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_ADD, OP_DIVIDE, OP_MULTIPLY, OP_SUBTRACT},
	},
};

use super::{Expression, InfixParselet};

pub struct BinaryParselet {
	precedence: Precedence,
	is_right: bool,
}

impl BinaryParselet {
	pub const fn new(precedence: Precedence, is_right: bool) -> Self {
		Self {
			precedence,
			is_right,
		}
	}
}

impl InfixParselet for BinaryParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		let kind = parser.previous().kind;
		parser.with_precedence(self.precedence + 1, chunk)?;
		match kind {
			TokenKind::Plus => {
				parser.emit_byte(OP_ADD, chunk);
				Ok(())
			}
			TokenKind::Minus => {
				parser.emit_byte(OP_SUBTRACT, chunk);
				Ok(())
			}
			TokenKind::Star => {
				parser.emit_byte(OP_MULTIPLY, chunk);
				Ok(())
			}
			TokenKind::Slash => {
				parser.emit_byte(OP_DIVIDE, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable),
		}
	}

	fn get_precedence(&self) -> Precedence {
		self.precedence
	}
}

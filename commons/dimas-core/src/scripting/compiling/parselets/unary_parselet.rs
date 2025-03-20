// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `UnaryParselet` for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{
	Parser,
	compiling::{
		error::Error,
		precedence::{Precedence, UNARY},
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_CONSTANT, OP_NEGATE},
	},
};

use super::{Expression, PrefixParselet};

pub struct UnaryParselet {
	precedence: Precedence,
}

impl UnaryParselet {
	pub const fn new(precedence: Precedence) -> Self {
		Self { precedence }
	}
}

impl PrefixParselet for UnaryParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		let token = parser.previous();
		// compile the operand
		parser.with_precedence(self.precedence, chunk)?;
		match token.kind {
			TokenKind::Minus => {
				// add the negation
				parser.emit_byte(OP_NEGATE, chunk);
				Ok(())
			}
			TokenKind::Plus => {
				// do nothing
				Ok(())
			}
			_ => Err(Error::Unreachable),
		}
	}
}

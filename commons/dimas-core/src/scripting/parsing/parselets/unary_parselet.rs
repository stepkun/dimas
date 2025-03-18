// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `UnaryParselet` for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{
	Chunk, Parser, TokenKind,
	error::Error,
	execution::opcodes::{OP_CONSTANT, OP_NEGATE},
	lexing::Token,
	parsing::precedence::UNARY,
};

use super::{Expression, PrefixParselet};

pub struct UnaryParselet;

impl PrefixParselet for UnaryParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		match parser.previous().kind {
			TokenKind::Minus => {
				// compile the operand
				parser.with_precedence(UNARY, chunk)?;
				// then add the negation
				parser.emit_byte(OP_NEGATE, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable),
		}
	}
}

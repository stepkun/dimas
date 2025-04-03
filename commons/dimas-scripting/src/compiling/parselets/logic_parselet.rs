// Copyright © 2025 Stephan Kunz

//! `GroupingParselet` for `Dimas`scripting
//!

use alloc::string::ToString;

use crate::{
	Parser,
	compiling::{
		error::Error,
		precedence::Precedence,
		token::{Token, TokenKind},
	},
	execution::{
		Chunk,
		opcodes::{OP_JMP_IF_FALSE, OP_JMP_IF_TRUE, OP_POP},
	},
};

use super::InfixParselet;

pub struct LogicParselet {
	precedence: Precedence,
}

impl LogicParselet {
	pub const fn new(precedence: Precedence) -> Self {
		Self { precedence }
	}
}

impl InfixParselet for LogicParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		let kind = parser.current().kind;
		match kind {
			TokenKind::And => {
				let target_pos = parser.emit_jump(OP_JMP_IF_FALSE, chunk);
				parser.emit_byte(OP_POP, chunk);
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.patch_jump(target_pos, chunk);
				Ok(())
			}
			TokenKind::Or => {
				let target_pos = parser.emit_jump(OP_JMP_IF_TRUE, chunk);
				parser.emit_byte(OP_POP, chunk);
				parser.with_precedence(self.precedence.next_higher(), chunk)?;
				parser.patch_jump(target_pos, chunk);
				Ok(())
			}
			_ => Err(Error::Unreachable(file!().to_string(), line!())),
		}
	}

	fn get_precedence(&self) -> Precedence {
		self.precedence
	}
}

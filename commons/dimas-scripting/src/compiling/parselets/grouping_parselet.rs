// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `GroupingParselet` for `Dimas`scripting
//!

use alloc::{boxed::Box, string::ToString};

use crate::{
	Parser,
	compiling::{
		error::Error,
		precedence::Precedence,
		token::{Token, TokenKind},
	},
	execution::{Chunk, opcodes::OP_CONSTANT},
};

use super::{Expression, PrefixParselet};

pub struct GroupingParselet;

impl PrefixParselet for GroupingParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		parser.expression(chunk)?;
		parser.consume(TokenKind::RightParen)
	}
}

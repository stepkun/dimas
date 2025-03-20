// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! `GroupingParselet` for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{
	Parser,
	compiling::{
		error::Error,
		token::{Token, TokenKind},
	},
	execution::{Chunk, opcodes::OP_CONSTANT},
};

use super::{Expression, PrefixParselet};

pub struct GroupingParselet;

impl PrefixParselet for GroupingParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error> {
		parser.expression(chunk);
		parser.advance_if(TokenKind::RightParen)?;
		Ok(())
	}
}

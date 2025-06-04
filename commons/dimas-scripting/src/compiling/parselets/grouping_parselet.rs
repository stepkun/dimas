// Copyright © 2025 Stephan Kunz

//! `GroupingParselet` for `Dimas` scripting handles parentheses
//!

use crate::{
	Error,
	compiling::{
		Lexer, Parser,
		token::{Token, TokenKind},
	},
	execution::Chunk,
};

use super::PrefixParselet;

pub struct GroupingParselet;

impl PrefixParselet for GroupingParselet {
	fn parse(
		&self,
		lexer: &mut Lexer,
		parser: &mut Parser,
		chunk: &mut Chunk,
		_token: Token,
	) -> Result<(), Error> {
		parser.expression(lexer, chunk)?;
		parser.consume(lexer, TokenKind::RightParen)
	}
}

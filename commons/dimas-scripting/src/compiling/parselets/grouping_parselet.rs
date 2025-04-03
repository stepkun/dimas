// Copyright © 2025 Stephan Kunz

//! `GroupingParselet` for `Dimas`scripting
//!

use crate::{
	Parser,
	compiling::{
		error::Error,
		token::{Token, TokenKind},
	},
	execution::Chunk,
};

use super::PrefixParselet;

pub struct GroupingParselet;

impl PrefixParselet for GroupingParselet {
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, _token: Token) -> Result<(), Error> {
		parser.expression(chunk)?;
		parser.consume(TokenKind::RightParen)
	}
}

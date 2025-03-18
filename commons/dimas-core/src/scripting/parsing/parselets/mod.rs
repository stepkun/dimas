// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//!Parselets for `Dimas`scripting
//!

use alloc::boxed::Box;

use crate::scripting::{error::Error, lexing::Token};

use super::Parser;

pub trait Expression {}

/// Interfaces used by the Pratt parser. A `PrefixParselet` is
/// associated with a token that appears at the beginning of an expression.
/// Its `parse()` method will be called with the consumed leading token, and the
/// parselet is responsible for parsing anything that comes after that token.
/// This interface is also used for single-token expressions like variables, in
/// which case `parse()` simply doesn't consume any more tokens.
pub trait PrefixParselet {
	/// Parse the token
	fn parse(&self, parser: &mut Parser, token: Token) -> Result<Box<dyn Expression>, Error>;
}

/// Interfaces used by the Pratt parser. An `InfixParselet` is
/// associated with a token that appears in the middle of the expression it parses.
/// Its `parse()` method will be called after the left-hand side has been parsed,
/// and it in turn is responsible for parsing everything that comes after the token.
/// This interface is also used for postfix expressions, in
/// which case `parse()` simply doesn't consume any more tokens.
pub trait InfixParselet {
	/// Parse the token together with the left hand expression
	fn parse(
		&self,
		parser: &mut Parser,
		left: Box<dyn Expression>,
		token: Token,
	) -> Result<Box<dyn Expression>, Error>;

	/// Get the precedence the parselet is executed with.
	fn get_precedence(&self) -> i32;
}

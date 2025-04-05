// Copyright © 2025 Stephan Kunz

//!Parselets for `Dimas`scripting
//!

mod assignment_parselet;
mod binary_parselet;
mod grouping_parselet;
mod literal_parselet;
mod logic_parselet;
mod unary_parselet;
mod value_parselet;

// flatten
pub use assignment_parselet::AssignmentParselet;
pub use binary_parselet::BinaryParselet;
pub use grouping_parselet::GroupingParselet;
pub use literal_parselet::LiteralParselet;
pub use logic_parselet::LogicParselet;
pub use unary_parselet::UnaryParselet;
pub use value_parselet::ValueParselet;

use crate::execution::Chunk;

use super::{Parser, error::Error, precedence::Precedence, token::Token};

/// Interfaces used by the Pratt parser. A `PrefixParselet` is
/// associated with a token that appears at the beginning of an expression.
/// Its `parse()` method will be called with the consumed leading token, and the
/// parselet is responsible for parsing anything that comes after that token.
/// This interface is also used for single-token expressions like variables, in
/// which case `parse()` simply doesn't consume any more tokens.
pub trait PrefixParselet: Send + Sync {
	/// Parse the token
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error>;
}

/// Interfaces used by the Pratt parser. An `InfixParselet` is
/// associated with a token that appears in the middle of the expression it parses.
/// Its `parse()` method will be called after the left-hand side has been parsed,
/// and it in turn is responsible for parsing everything that comes after the token.
/// This interface is also used for postfix expressions, in
/// which case `parse()` simply doesn't consume any more tokens.
pub trait InfixParselet: Send + Sync {
	/// Parse the token together with the left hand expression
	fn parse(&self, parser: &mut Parser, chunk: &mut Chunk, token: Token) -> Result<(), Error>;

	/// Get the precedence the parselet is executed with.
	fn get_precedence(&self) -> Precedence;
}

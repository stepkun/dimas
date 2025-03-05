// Copyright © 2025 Stephan Kunz
#![allow(unused)]

//! Token for `DiMAS` scripting

/// Token kind
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
	/// =
	Equal,
	/// :
	Colon,
	/// :=
	ColonEqual,
	/// +
	Plus,
	/// +=
	PlusEqual,
	/// -
	Minus,
	/// -=
	MinusEqual,
	/// *
	Star,
	/// *=
	StarEqual,
	/// /
	Slash,
	/// /=
	SlashEqual,
	/// ;
	Semicolon,
	/// &
	Ampersand,
	/// |
	Pipe,
	/// ^
	Circonflex,
	/// &&
	And,
	/// ||
	Or,
	/// !
	Not,
	/// !=
	NotEqual,
	/// ==
	EqualEqual,
	/// <
	Less,
	/// <=
	LessEqual,
	/// >
	Greater,
	/// >=
	GreaterEqual,
	/// ?
	QMark,
	/// (
	LeftParen,
	/// )
	RightParen,
	/// Keyword boolean 'true'
	True,
	/// Keyword boolean 'false'
	False,
	/// An Identifier
	Ident,
	/// Every Number is a f64
	Number(f64),
	/// Every hexadecimal Number is an i64
	HexNumber(i64),
	/// Any String
	String,
}

/// Token
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
	/// Reference to the underlying location
	pub origin: &'a str,
	/// @TODO
	pub offset: usize,
	/// Kind of token
	pub kind: TokenKind,
}

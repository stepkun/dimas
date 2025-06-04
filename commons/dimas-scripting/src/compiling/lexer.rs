// Copyright © 2025 Stephan Kunz

//! Lexer for `DiMAS` scripting
//!
//! Implementation is heavily inspired by
//! - Jon Gjengsets [video](https://www.youtube.com/watch?v=mNOLaw-_Buc) & [example](https://github.com/jonhoo/lox/blob/master/src/lex.rs)
//!

use core::cmp::min;

use alloc::string::ToString;

use crate::Error;

use super::token::{Token, TokenKind};

/// Enum to handle multi charakter tokens
enum Started {
	String,
	Number, // may be hex or not
	Ident,
	IfEqualElse(TokenKind, TokenKind),
	IfSameElse(TokenKind, TokenKind),
}

/// Lexer
pub struct Lexer<'a> {
	/// reference to the whole input 'code'
	whole: &'a str,
	/// reference to the start of the not yet lexed part
	rest: &'a str,
	/// current position in the input
	pos: usize,
	/// current line
	line: usize,
}

impl<'a> Lexer<'a> {
	/// Create a Lexer for a certain input str.
	#[must_use]
	pub const fn new(source_code: &'a str) -> Self {
		Self {
			whole: source_code,
			rest: source_code,
			pos: 0,
			line: 1,
		}
	}

	/// Set a new input str (source code).
	pub const fn set_input(&mut self, source_code: &'a str) {
		self.whole = source_code;
		self.rest = source_code;
		self.pos = 0;
		self.line = 1;
	}
}

impl Iterator for Lexer<'_> {
	type Item = Result<Token, Error>;

	#[allow(clippy::too_many_lines)]
	fn next(&mut self) -> Option<Self::Item> {
		// return a peeked token
		loop {
			// must be in the loop for the indices to match up with c_onwards
			let mut chars = self.rest.chars();
			let c = chars.next()?;
			let c_at = self.pos;
			let c_str = &self.rest[..c.len_utf8()];
			let c_onwards = self.rest;
			self.rest = chars.as_str();
			self.pos += c.len_utf8();

			// must be inside loop to capture variables
			let line = self.line;
			let create = move |kind: TokenKind| {
				Some(Ok(Token {
					kind,
					offset: c_at,
					line,
					origin: c_str.to_string(),
				}))
			};

			// this will return early for single character tokens
			let started = match c {
				// single character Tokens
				'(' => return create(TokenKind::LeftParen),
				')' => return create(TokenKind::RightParen),
				';' => return create(TokenKind::Semicolon),
				'^' => return create(TokenKind::Caret),
				'~' => return create(TokenKind::Tilde),
				'?' => return create(TokenKind::QMark),
				// possible double character Tokens containing an '='
				':' => Started::IfEqualElse(TokenKind::ColonEqual, TokenKind::Colon),
				'=' => Started::IfEqualElse(TokenKind::EqualEqual, TokenKind::Equal),
				'!' => Started::IfEqualElse(TokenKind::BangEqual, TokenKind::Bang),
				'+' => Started::IfEqualElse(TokenKind::PlusEqual, TokenKind::Plus),
				'-' => Started::IfEqualElse(TokenKind::MinusEqual, TokenKind::Minus),
				'*' => Started::IfEqualElse(TokenKind::StarEqual, TokenKind::Star),
				'/' => Started::IfEqualElse(TokenKind::SlashEqual, TokenKind::Slash),
				'<' => Started::IfEqualElse(TokenKind::LessEqual, TokenKind::Less),
				'>' => Started::IfEqualElse(TokenKind::GreaterEqual, TokenKind::Greater),
				// possible double character Tokens with twice the same character
				'&' => Started::IfSameElse(TokenKind::And, TokenKind::Ampersand),
				'|' => Started::IfSameElse(TokenKind::Or, TokenKind::Pipe),
				// multi character token
				'\'' => Started::String,
				'0'..='9' => Started::Number,
				'a'..='z' | 'A'..='Z' | '_' | '@' => Started::Ident,
				// count lines
				'\n' => {
					self.line += 1;
					continue;
				}
				// skip whitespaces
				c if c.is_whitespace() => continue,
				// something is wrong in the token stream
				c => return Some(Err(Error::UnexpectedChar(c.to_string().into(), self.line))),
			};

			// handling double & multi character token
			break match started {
				Started::IfEqualElse(yes, no) => {
					self.rest = self.rest.trim_start();
					let trimmed = c_onwards.len() - self.rest.len() - 1;
					self.pos += trimmed;
					if self.rest.starts_with('=') {
						let span = &c_onwards[..=c.len_utf8() + trimmed];
						self.rest = &self.rest[1..];
						self.pos += 1;
						Some(Ok(Token {
							origin: span.to_string(),
							offset: c_at,
							line: self.line,
							kind: yes,
						}))
					} else {
						Some(Ok(Token {
							origin: c_str.to_string(),
							offset: c_at,
							line: self.line,
							kind: no,
						}))
					}
				}
				Started::IfSameElse(yes, no) => {
					self.rest = self.rest.trim_start();
					let trimmed = c_onwards.len() - self.rest.len() - 1;
					self.pos += trimmed;
					if self.rest.starts_with(c) {
						let span = &c_onwards[..=c.len_utf8() + trimmed];
						self.rest = &self.rest[1..];
						self.pos += 1;
						Some(Ok(Token {
							origin: span.to_string(),
							offset: c_at,
							line: self.line,
							kind: yes,
						}))
					} else {
						Some(Ok(Token {
							origin: c_str.to_string(),
							offset: c_at,
							line: self.line,
							kind: no,
						}))
					}
				}
				Started::Ident => {
					// An @ may only be at the start, so we don't ignore it in the end-pattern search.
					// Therefor it is a little tricky  with the length detremination.
					let first_non_ident = c_onwards[1..]
						.find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
						.unwrap_or(c_onwards.len() - 1)
						+ 1;

					let literal = &c_onwards[..first_non_ident];
					let extra_bytes = literal.len() - c.len_utf8();
					self.pos += extra_bytes;
					self.rest = &self.rest[extra_bytes..];

					// distinguish keywords from idents
					let kind = match literal {
						"false" => TokenKind::False,
						"nil" => TokenKind::Nil,
						"print" => TokenKind::Print,
						"true" => TokenKind::True,
						_ => TokenKind::Ident,
					};

					return Some(Ok(Token {
						origin: literal.to_string(),
						offset: c_at,
						line: self.line,
						kind,
					}));
				}
				#[allow(clippy::redundant_guards)] // because that guard is not redundant!!
				Started::Number => {
					// check for hex number
					if self.rest.starts_with('x') {
						// skip the '0x' in c_onwards
						let number = &c_onwards[2..];
						let first_non_hex_digit = number
							.find(|c: char| !c.is_ascii_hexdigit())
							.unwrap_or(c_onwards.len());

						// remember the skipped '0x'
						let end = min(first_non_hex_digit + 2, c_onwards.len());
						let literal = &c_onwards[..end];

						let extra_bytes = literal.len() - c.len_utf8();
						self.pos += extra_bytes;
						self.rest = &self.rest[extra_bytes..];

						return Some(Ok(Token {
							origin: literal.to_string(),
							offset: c_at,
							line: self.line,
							kind: TokenKind::HexNumber,
						}));
					}

					let first_non_digit = c_onwards
						.find(|c| !matches!(c, '.' | '0'..='9'))
						.unwrap_or(c_onwards.len());

					let mut literal = &c_onwards[..first_non_digit];
					let mut dotted = literal.splitn(3, '.');
					match (dotted.next(), dotted.next(), dotted.next()) {
						(Some(one), Some(two), Some(_)) => {
							literal = &literal[..one.len() + 1 + two.len()];
						}
						(Some(one), Some(two), None) if two.is_empty() => {
							literal = &literal[..one.len()];
						}
						_ => {
							// leave literal as-is
						}
					}
					let extra_bytes = literal.len() - c.len_utf8();
					self.pos += extra_bytes;
					self.rest = &self.rest[extra_bytes..];

					if literal.contains('.') {
						return Some(Ok(Token {
							origin: literal.to_string(),
							offset: c_at,
							line: self.line,
							kind: TokenKind::FloatNumber,
						}));
					}
					return Some(Ok(Token {
						origin: literal.to_string(),
						offset: c_at,
						line: self.line,
						kind: TokenKind::IntNumber,
					}));
				}
				Started::String => {
					if let Some(end) = self.rest.find('\'') {
						// we do not want the leaing and trailing `'` included
						let literal = &c_onwards[1..=(end)];
						self.pos += end + 1;
						self.rest = &self.rest[end + 1..];
						Some(Ok(Token {
							origin: literal.to_string(),
							offset: c_at,
							line: self.line,
							kind: TokenKind::String,
						}))
					} else {
						return Some(Err(Error::UnterminatedString(
							self.whole[c_at..].into(),
							self.line,
						)));
					}
				}
			};
		}
	}
}

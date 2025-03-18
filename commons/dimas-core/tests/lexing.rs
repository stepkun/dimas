// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]

//! Tests of lexing functionality

use dimas_core::scripting::{Lexer, TokenKind};

enum Color {
	RED = 1,
	BLUE = 2,
	GREEN = 3,
}

#[allow(clippy::cognitive_complexity)]
fn lexing_tokens(tokens: &str) {
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::ColonEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Equal);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Plus);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Minus);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Star);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Slash);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::PlusEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::MinusEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::StarEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::SlashEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Semicolon);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Bang);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ampersand);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Pipe);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Circonflex);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::And);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Or);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::EqualEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::BangEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Less);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::LessEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Greater);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::GreaterEqual);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Colon);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::QMark);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::LeftParen);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::RightParen);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing() {
	let tokens = ":= = + - * / += -= *= /= ; ! & | ^ && || == != < <= > >= : ? ( )";
	lexing_tokens(tokens);
	let tokens2 = ":==+-*/+=-=*=/=;!&|^&&||==!=<<=>>=:?()";
	lexing_tokens(tokens2);
}

#[test]
fn lexing_keywords() {
	let tokens = "true false";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::True);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::False);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_idents() {
	let tokens = "a_name _another_name _aThirdName_";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ident);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ident);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Ident);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_numbers() {
	let tokens = "123 123.0 123.456 0.123";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Number);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Number);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Number);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Number);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_hex() {
	let tokens = "0x123 0xABC 0xabc 0xa1b2c3";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::HexNumber);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::HexNumber);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::HexNumber);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::HexNumber);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
fn lexing_strings() {
	let tokens = "'teststring' 'another_string'";
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::String);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::String);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

#[test]
#[ignore]
fn lexing_enums() {
	let tokens = "RED BLUE GREEN";
	// @TODO
	let mut lexer = Lexer::new(tokens);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Enum);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Enum);
	assert_eq!(lexer.next().unwrap().unwrap().kind, TokenKind::Enum);
	assert!(lexer.next().is_none());
	assert!(lexer.next().is_none());
}

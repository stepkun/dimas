// Copyright © 2025 Stephan Kunz

//! Scripting of `DiMAS`
//!
//! This scripting language follows the pattern of clox as described in Part III of [crafting interpreters](https://craftinginterpreters.com/)
//!
//! Definition of the grammer following this [notation](https://craftinginterpreters.com/representing-code.html#rules-for-grammars)
//!
//! ```no-test
//! script      → statement* EoF ;
//! statement   → expression ";" ;
//! expression  → assignment ;
//! assignment  → IDENTIFIER ":=" assignment | IDENTIFIER "=" assignment | logic_or ;
//! logic_or    → logic_and ( "||" logic_and )* ;
//! logic_and   → equality ( "&&" equality )* ;
//! equality    → comparison ( ( "!=" | "==" ) comparison )* ;
//! comparison  → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//! term        → factor ( ( "-" | "+" ) factor )* ;
//! factor      → unary ( ( "/" | "*" ) unary )* ;
//! unary       → ( "!" | "-" ) unary | primary ;
//! primary     → "true" | "false" | NUMBER | HEXNUMBER | STRING | IDENTIFIER | "(" expression ")" ;
//!
//! NUMBER      → DIGIT+ ( "." DIGIT+ )? ;
//! HEXNUMBER   → DIGIT+ | "a" ... "f"+ | "A" ... "F"+ ;
//! STRING      → "\'" <any char except "\'">* "\'" ;
//! IDENTIFIER  → ALPHA ( ALPHA | DIGIT )* ;
//! ALPHA       → "a" ... "z" | "A" ... "Z" | "_" ;
//! DIGIT       → "0" ... "9" ;
//!
//! TODO'S:
//! ternary     → condition "?" term ":" term ;
//! binary_xor  → binary_or ( "|" binary_or )* ;
//! binary_or   → binary_and ( "|" binary_and )* ;
//! binary_and  → equality ( "&" equality )* ;
//! c_assgnmnt  → IDENTIFIER ( "+=" | "-=" | "*="" | "/=" ) term ;
//! ```
//!

mod error;
pub mod execution;
mod lexing;
mod parsing;

// flatten
pub use execution::VM;
pub use lexing::{Lexer, TokenKind};
pub use parsing::Chunk;
pub use parsing::Parser;

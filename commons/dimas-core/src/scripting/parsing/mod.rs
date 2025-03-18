// Copyright © 2025 Stephan Kunz

//! Parser for `DiMAS` scripting implemented as a [Pratt-Parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
//! You should also read the articel by [Robert Nystrom](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
//!
//! Implementation is heavily inspired by
//! - Jon Gjengsets [video](https://www.youtube.com/watch?v=mNOLaw-_Buc) & [example](https://github.com/jonhoo/lox/blob/master/src/parse.rs)
//! - Jürgen Wurzers implementation of [Bantam](https://github.com/jwurzer/bantam-rust/blob/master/src/bantam/bantam_parser.rs)
//!

mod chunk;
mod parselets;
#[allow(clippy::module_inception)]
mod parser;
mod precedence;

// flatten
pub use chunk::Chunk;
pub use parser::Parser;

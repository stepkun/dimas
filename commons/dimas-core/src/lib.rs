// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core library of `DiMAS`

#[doc(hidden)]
extern crate alloc;

// modules
pub mod error;
pub mod utils;

// flatten:

// region:      --- modules
use alloc::{boxed::Box, rc::Rc, sync::Arc};
// endregion:   --- modules

// region:      --- types
/// An immutable `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs)
pub type ConstString = Box<str>;

/// An immutable reference counted `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs)
pub type RcConstString = Rc<str>;

/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs)
pub type ArcConstString = Arc<str>;
// endregion:   --- types

// Copyright © 2024 Stephan Kunz
#![no_std]

//! Core library of `DiMAS`.

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
/// An immutable thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
pub type ArcConstString = Arc<str>;

/// An immutable non thread safe `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
pub type BoxConstString = Box<str>;

/// An immutable reference counted `String` type
/// see: [Logan Smith](https://www.youtube.com/watch?v=A4cKi7PTJSs).
pub type RcConstString = Rc<str>;

/// As `DiMAS` uses multi threading the default constant string is the variant behind an [`Arc`].
pub type ConstString = Arc<str>;
// endregion:   --- types

// Copyright © 2024 Stephan Kunz
#![no_std]
#![allow(unused)]

//! Library for configuration
//!

#[doc(hidden)]
extern crate alloc;

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

pub mod builtin;
pub mod factory;

// flatten

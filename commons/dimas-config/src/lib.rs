// Copyright © 2024 Stephan Kunz
#![no_std]

//! Library for configuration
//!

mod config;
mod error;
mod utils;

// flatten
pub use config::Config;
pub use error::Error;

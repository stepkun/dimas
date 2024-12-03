// Copyright © 2024 Stephan Kunz
#![no_std]

//! Module `dimas-time` provides a set of `Timer` variants which can be created using the `TimerBuilder`.
//! When fired, a `Timer` calls his assigned `TimerCallback`.

// region:    --- modules
mod interval_timer;

// flatten
pub use interval_timer::*;
// endregion: --- modules

#[cfg(test)]
mod tests {}

// Copyright © 2024 Stephan Kunz

//! Core enums of `DiMAS`
//!

#[doc(hidden)]
extern crate alloc;

mod operation_state;
mod signal;
mod task_signal;

// flatten
pub use operation_state::OperationState;
pub use signal::Signal;
pub use task_signal::TaskSignal;

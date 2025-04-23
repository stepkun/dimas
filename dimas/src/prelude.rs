// Copyright © 2025 Stephan Kunz

//! Most commonly used interface of dimas.
//!
//! Typically it is sufficient to include the prelude with
//!
//! ```use dimas::prelude::*;```

// to avoid adding these crates to dependencies
pub extern crate anyhow;
pub extern crate tokio;

// reexports:
pub use anyhow::Result;

// DiMAS
pub use crate::Agent;
pub use crate::com::publisher::Publisher;
pub use crate::com::subscriber::Subscriber;
pub use crate::timer::interval_timer::IntervalTimer;
pub use dimas_core::utils::init_tracing;

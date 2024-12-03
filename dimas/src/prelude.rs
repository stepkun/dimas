// Copyright © 2023 Stephan Kunz

//! Most commonly used interface of dimas.
//!
//! Typically it is sufficient to include the prelude with
//!
//! ```use dimas::prelude::*;```

// to avoid adding these crates to dependencies
pub extern crate anyhow;
pub extern crate bitcode;
pub extern crate tokio;

// anyhow's Result
pub use anyhow::Result;

// bitcode encoding/decoding
pub use bitcode::{Decode, Encode};

// Duration from core
pub use core::time::Duration;

// stuff from parking_lot
pub use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

// zenoh stuff
pub use zenoh::qos::CongestionControl;
pub use zenoh::qos::Priority;
#[cfg(feature = "unstable")]
pub use zenoh::qos::Reliability;
pub use zenoh::query::ConsolidationMode;
pub use zenoh::query::QueryTarget;
#[cfg(feature = "unstable")]
pub use zenoh::sample::Locality;

// dimas stuff
pub use crate::agent::Agent;
pub use crate::utils::ComponentRegistry;
pub use crate::utils::LibManager;
pub use dimas_config::Config;
pub use dimas_core::message_types::{
	Message, ObservableControlResponse, ObservableResponse, QueryMsg, QueryableMsg,
};
pub use dimas_core::traits::{Context, System};
pub use dimas_core::utils::init_tracing;
pub use dimas_core::Activity;
pub use dimas_core::{Component, ComponentId};
pub use dimas_core::{OperationState, Operational, Transitions};
pub use dimas_macros::main;
pub use dimas_time::IntervalTimer;

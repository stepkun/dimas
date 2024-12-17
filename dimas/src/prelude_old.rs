// Copyright © 2023 Stephan Kunz

//! Most commonly used interface of dimas.
//!
//! Typically it is sufficient to include the prelude with
//!
//! ```use dimas::prelude::*;```

// to avoid adding these crates to dependencies
pub extern crate alloc;
pub extern crate anyhow;
pub extern crate bitcode;
pub extern crate parking_lot;
pub extern crate tokio;
pub extern crate uuid;

// anyhow's Result
pub use anyhow::Result;

// bitcode encoding/decoding
pub use bitcode::{Decode, Encode};

// Duration from core
pub use core::time::Duration;

// stuff from parking_lot
pub use parking_lot::RwLock;

// stuff from uuid
pub use uuid::Uuid;

// stuff from standard libraries (core, alloc, std)
// always using the deepest level
pub use alloc::sync::Arc;

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
pub use crate::agent_old::AgentOld;
pub use crate::utils_old::ComponentRegistry;
pub use crate::utils_old::LibManager;
pub use dimas_config::Config;
pub use dimas_core::message_types::{
	Message, ObservableControlResponse, ObservableResponse, QueryMsg, QueryableMsg,
};
pub use dimas_core::traits::Context;
pub use dimas_core::utils::init_tracing;
pub use dimas_core::System;
pub use dimas_core::{Activity, ActivityData, ActivityId};
pub use dimas_core::{Component, ComponentData, ComponentId};
pub use dimas_core::{OperationState, Operational, Transitions};

pub use dimas_core::Agent;
pub use dimas_core::ManageOperationState;
pub use dimas_time::IntervalTimerParameter;
pub use dimas_time::{TimerFactory, TimerLib, TimerVariant};
pub use dimas_core::CommunicatorFactory;

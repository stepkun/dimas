// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Zenoh based subscriber

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{boxed::Box, sync::Arc};
use anyhow::Result;
use dimas_core::{message_types::Message, Activity, ActivityData, ActivityId, Agent, OperationState, Operational, OperationalData, Transitions};
use futures::future::BoxFuture;
#[cfg(feature = "std")]
use tokio::sync::Mutex;
// endregion:   --- modules

// region:		--- types
/// Type definition for the functions called by a subscriber
#[allow(clippy::module_name_repetitions)]
pub type SubscriberCallback =
	Box<dyn FnMut(Agent, Message) -> BoxFuture<'static, Result<()>> + Send + Sync + 'static>;

/// Type definition for a subscribers atomic reference counted callback
/// @ TODO: remove pub if possible
pub type ArcSubscriberCallback = Arc<Mutex<SubscriberCallback>>;
// endregion:	--- types

// region:      --- Subscriber
/// [`Subscriber`] implementation.
#[derive(Debug)]
pub struct Subscriber {
    activity: ActivityData,
}

impl Activity for Subscriber {
    fn id(&self) -> ActivityId {
        self.activity.id.clone()
    }
}

impl Operational for Subscriber {
    fn activation_state(&self) -> OperationState {
        self.activity.operational.activation
    }

    fn set_activation_state(&mut self, state: OperationState) {
        self.activity.operational.activation = state;
    }

    fn state(&self) -> dimas_core::OperationState {
        self.activity.operational.current
    }

    fn set_state(&mut self, state: dimas_core::OperationState) {
        self.activity.operational.current = state;
    }
}

impl Transitions for Subscriber {}

impl Subscriber {
    pub fn new(id: &str, ctx: Agent) -> Self {
        Self {
            activity: ActivityData::new(id, ctx),
        }
    }
}
// endregion:   --- Subscriber

// Copyright © 2023 Stephan Kunz

//! Commonly used states for builders

// region:      --- modules
use core::time::Duration;
use dimas_core::ComponentType;
// endtregion:  --- modules

// region:		--- builder_states
/// State signaling that the builder has no storage value set
pub struct NoStorage;

/// State signaling that the builder has the storage value set
pub struct Storage<'a> {
	/// Mutable reference to context stored in [`ComponentType`]
	pub storage: &'a mut ComponentType,
}

/// State signaling that the builder has no selector set
pub struct NoSelector;
/// State signaling that the builder has the selector set
pub struct Selector {
	/// The selector
	pub selector: String,
}

/// State signaling that the builder has no interval set
pub struct NoInterval;
/// State signaling that the builder has the interval set
pub struct Interval {
	/// The [`Duration`] of the interval
	pub interval: Duration,
}

/// State signaling that the builder has a callback not set
pub struct NoCallback;
/// State signaling that the builder has a callback set
pub struct Callback<C>
where
	C: Send + Sync + 'static,
{
	/// The callback to use
	pub callback: C,
}
// endregion:	--- builder_states

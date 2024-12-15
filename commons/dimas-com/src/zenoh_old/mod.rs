// Copyright © 2024 Stephan Kunz

//! dimas-com implements the communication capabilities.
//!

/// zenoh communicator implementation
pub mod communicator_old;
/// the liveliness subscriber
#[cfg(feature = "unstable")]
pub mod liveliness_old;
/// the observable
pub mod observable_old;
/// the observer
pub mod observer_old;
/// the publisher
pub mod publisher_old;
/// the querier
pub mod querier_old;
/// the queryable
pub mod queryable_old;
/// the subscriber
pub mod subscriber_old;

// flatten
#[allow(clippy::module_name_repetitions)]
pub use communicator_old::CommunicatorOld;
#[cfg(feature = "unstable")]
pub use liveliness::LivelinessSubscriber;
pub use observable_old::Observable;
pub use observer_old::Observer;
pub use publisher_old::Publisher;
pub use querier_old::Querier;
pub use queryable_old::Queryable;
pub use subscriber_old::Subscriber;

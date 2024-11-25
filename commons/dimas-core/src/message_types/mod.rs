// Copyright © 2024 Stephan Kunz

//! Module `message_types` provides the different types of `Message`s used in callbacks.

#[doc(hidden)]
extern crate alloc;

mod message;
mod observable_msgs;
mod query_msg;
mod queryable_msg;

// flatten
pub use message::Message;
pub use observable_msgs::{ObservableControlResponse, ObservableResponse};
pub use query_msg::QueryMsg;
pub use queryable_msg::QueryableMsg;

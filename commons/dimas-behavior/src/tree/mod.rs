// Copyright © 2025 Stephan Kunz

//! A [`BehaviorTree`] library
//!

pub mod error;
pub mod observer;
#[allow(clippy::module_inception)]
mod tree;
mod tree_element;
mod tree_element_list;

// flatten
pub use tree::{BehaviorTree, print_tree};
pub use tree_element::BehaviorTreeElement;
pub use tree_element_list::BehaviorTreeElementList;

// region:      --- modules
use crate::behavior::BehaviorState;
// endregion:   --- modules

// region:      --- types
/// [`BehaviorTreeElement`] state change callback signature.
///
/// This callback can be used to observe a [`BehaviorTreeElement`] and manipulate the resulting [`BehaviorState`] of a tick.
/// In case of non std without a timestamp.
pub type BehaviorTreeElementTickCallback = dyn Fn(&BehaviorTreeElement, &mut BehaviorState) + Send + Sync + 'static;
// endregion:   --- types

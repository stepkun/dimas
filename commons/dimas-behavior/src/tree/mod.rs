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

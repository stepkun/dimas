// Copyright © 2025 Stephan Kunz
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::unnecessary_wraps)]
#![allow(unused)]

//! XML parser for the [`BehaviorTreeFactory`] of `DiMAS`

// region:      --- modules
use alloc::{boxed::Box, string::ToString, vec::Vec};
use roxmltree::{Node, NodeType};

use crate::{
	blackboard::Blackboard,
	new_behavior::{BehaviorMethods, BehaviorTickData, NewBehaviorType},
	tree::{BehaviorTree, BehaviorTreeComponent},
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- XmlParser
pub struct XmlParser {}

impl XmlParser {
	pub(crate) fn parse_root_element(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		root: Node,
	) -> Result<(), Error> {
		for element in root.children() {
			match element.node_type() {
				NodeType::Comment | NodeType::Text => {} // ignore
				NodeType::Root => {
					// this should not happen
					return Err(Error::Unexpected(
						"root element".into(),
						file!().into(),
						line!(),
					));
				}
				NodeType::Element => {
					// only 'BehaviorTree' or 'TreeNodesModel' are valid
					match element.tag_name().name() {
						"TreeNodesModel" => {} // ignore
						"BehaviorTree" => {
							// check for tree ID
							if let Some(id) = element.attribute("ID") {
								Self::build_subtree(blackboard, registry, tree, element, id)?;
							} else {
								return Err(Error::MissingId(element.tag_name().name().into()));
							}
						}
						_ => {
							return Err(Error::ElementNotSupported(
								element.tag_name().name().into(),
							));
						}
					}
				}
				NodeType::PI => {
					return Err(Error::UnsupportedProcessingInstruction(
						element.tag_name().name().into(),
					));
				}
			}
		}
		Ok(())
	}

	fn build_subtree(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
		id: &str,
	) -> Result<(), Error> {
		// A subtreee gets a new [`Blackboard`] with parent trees [`Blackboard`] as parent
		let blackboard = Blackboard::new(blackboard);
		let children = Self::build_children(&blackboard, registry, tree, element)?;
		let tick_data = BehaviorTickData::new(blackboard);
		let subtree = BehaviorTreeComponent::create_node(None, tick_data, children);
		tree.add(id, subtree);
		Ok(())
	}

	fn build_children(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
	) -> Result<Vec<BehaviorTreeComponent>, Error> {
		let mut children = Vec::new();

		for child in element.children() {
			match child.node_type() {
				NodeType::Comment | NodeType::Text => {} // ignore
				NodeType::Root => {
					// this should not happen
					return Err(Error::Unexpected(
						"root element".into(),
						file!().into(),
						line!(),
					));
				}
				NodeType::Element => {
					let behavior = Self::build_child(blackboard, registry, tree, child)?;
					children.push(behavior);
				}
				NodeType::PI => {
					return Err(Error::UnsupportedProcessingInstruction(
						element.tag_name().name().into(),
					));
				}
			}
		}

		children.shrink_to_fit();
		Ok(children)
	}

	#[allow(clippy::option_if_let_else)]
	fn build_child(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
	) -> Result<BehaviorTreeComponent, Error> {
		let bhvr_name = element.tag_name().name();
		let attributes = element.attributes();
		if bhvr_name == "SubTree" {
			if let Some(id) = element.attribute("ID") {
				todo!()
			} else {
				Err(Error::MissingId(element.tag_name().name().into()))
			}
		} else {
			// look for the behavior in the [`BehaviorRegisty`]
			let (bhvr_type, bhvr_creation_fn) = registry.find(bhvr_name)?;
			let bhvr = bhvr_creation_fn();
			let tree_node = match bhvr_type {
				NewBehaviorType::Action | NewBehaviorType::Condition => {
					if element.has_children() {
						return Err(Error::ChildrenNotAllowed(bhvr_type.to_string()));
					}
					let blackboard = blackboard.clone();
					let tick_data = BehaviorTickData::new(blackboard);
					BehaviorTreeComponent::create_leaf(bhvr, tick_data)
				}
				NewBehaviorType::Control | NewBehaviorType::Decorator => {
					let blackboard = blackboard.clone();
					let children = Self::build_children(&blackboard, registry, tree, element)?;
					if bhvr_type == NewBehaviorType::Decorator && children.len() > 1 {
						return Err(Error::DecoratorOnlyOneChild(
							element.tag_name().name().into(),
						));
					}
					let tick_data = BehaviorTickData::new(blackboard);
					BehaviorTreeComponent::create_node(Some(bhvr), tick_data, children)
				}
				NewBehaviorType::SubTree => {
					todo!()
				}
			};
			Ok(tree_node)
		}
	}
}
// endregion:   --- XmlParser

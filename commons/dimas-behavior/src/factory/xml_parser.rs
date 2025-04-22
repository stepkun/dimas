// Copyright © 2025 Stephan Kunz
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(unused)]

//! XML parser for the [`BehaviorTreeFactory`] of `DiMAS`

// region:      --- modules
use alloc::{
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec::{self, Vec},
};
use parking_lot::Mutex;
use roxmltree::{Node, NodeType};

use crate::{
	behavior::{
		BehaviorConfigurationData, BehaviorCreationMethods, BehaviorTickData, BehaviorTreeMethods,
		BehaviorType,
	},
	blackboard::Blackboard,
	tree::{
		BehaviorTree, BehaviorTreeComponent, BehaviorTreeComponentList, BehaviorTreeLeaf,
		BehaviorTreeNode, BehaviorTreeProxy,
	},
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
		element: Node,
		// A reference to the name of the main_tree_to_execute.
		main_tree_name: &str,
		// Signals whether to parse or to store the main_tree_to_execute.
		parse_main_tree: bool,
	) -> Result<(), Error> {
		for element in element.children() {
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
					let name = element.tag_name().name();
					match name {
						"TreeNodesModel" => {} // ignore
						"BehaviorTree" => {
							// check for tree ID
							if let Some(id) = element.attribute("ID") {
								// A subtreee gets a new [`Blackboard`] with parent trees [`Blackboard`] as parent
								let blackboard = if id == main_tree_name && !parse_main_tree {
									blackboard.clone()
								} else {
									Blackboard::new(blackboard)
								};
								let new_subtree = Self::handle_subtree(
									&blackboard,
									registry,
									tree,
									element,
									id,
									false,
								)?;
								// store subtree for later usage
								if id == main_tree_name {
									tree.add_root(new_subtree);
								} else {
									tree.add_subtree(new_subtree);
								}
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

	pub(crate) fn register_root_element(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
		// A reference to the name of the main_tree_to_execute.
		main_tree_name: &str,
	) -> Result<(), Error> {
		for element in element.children() {
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
					let name = element.tag_name().name();
					match name {
						"TreeNodesModel" => {} // ignore
						"BehaviorTree" => {
							// check for tree ID
							if let Some(id) = element.attribute("ID") {
								// A subtreee gets a new [`Blackboard`] with parent trees [`Blackboard`] as parent
								let blackboard = blackboard.clone();
								let new_subtree = Self::handle_subtree(
									&blackboard,
									registry,
									tree,
									element,
									id,
									true,
								)?;
								// store subtree for later usage
								if id == main_tree_name {
									tree.add_root(new_subtree);
								} else {
									tree.add_subtree(new_subtree);
								}
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

	fn build_children(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
		register_only: bool,
	) -> Result<BehaviorTreeComponentList, Error> {
		let mut children = BehaviorTreeComponentList::default();

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
					let behavior =
						Self::build_child(blackboard, registry, tree, child, register_only)?;
					children.push(behavior);
				}
				NodeType::PI => {
					return Err(Error::UnsupportedProcessingInstruction(
						element.tag_name().name().into(),
					));
				}
			}
		}

		// @TODO: children.shrink_to_fit();
		Ok(children)
	}

	#[allow(clippy::option_if_let_else)]
	fn build_child(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
		// if true, only registration of subtrees happens
		register_only: bool,
	) -> Result<Box<dyn BehaviorTreeComponent>, Error> {
		let element_name = element.tag_name().name();
		if element_name == "SubTree" {
			if let Some(id) = element.attribute("ID") {
				let tick_data = BehaviorTickData::default();
				let config_data = BehaviorConfigurationData::new(id);
				let behavior = BehaviorTreeProxy::create(id, BehaviorTickData::default());
				if register_only {
					Ok(behavior)
				} else {
					// use subtree from list
					todo!()
					// let subtree = tree.find_by_name(id)?;
					// Ok(behavior)
				}
			} else {
				Err(Error::MissingId(element.tag_name().name().into()))
			}
		} else {
			// look for the behavior in the [`BehaviorRegisty`]
			let (bhvr_type, bhvr_creation_fn) = registry.fetch(element_name)?;
			let mut bhvr = bhvr_creation_fn();
			let id = bhvr_type.to_string() + ": ";
			let tree_node = match bhvr_type {
				BehaviorType::Action | BehaviorType::Condition => {
					if element.has_children() {
						return Err(Error::ChildrenNotAllowed(bhvr_type.to_string()));
					}
					let mut tick_data = BehaviorTickData::new(blackboard.clone());
					let mut config_data = BehaviorConfigurationData::new(element_name);
					let mut tick_data = BehaviorTickData::new(blackboard.clone());
					Self::create_ports(
						&mut bhvr,
						&mut tick_data,
						&mut config_data,
						&element,
					)?;
					BehaviorTreeLeaf::create(id, tick_data, bhvr)
				}
				BehaviorType::Control | BehaviorType::Decorator => {
					let new_children =
						Self::build_children(blackboard, registry, tree, element, register_only)?;
					if bhvr_type == BehaviorType::Decorator && new_children.len() > 1 {
						return Err(Error::DecoratorOnlyOneChild(
							element.tag_name().name().into(),
						));
					}
					let mut tick_data = BehaviorTickData::new(blackboard.clone());
					let mut config_data = BehaviorConfigurationData::default();
					Self::create_ports(&mut bhvr, &mut tick_data, &mut config_data, &element)?;
					BehaviorTreeNode::create(
						id,
						new_children,
						BehaviorTickData::new(blackboard.clone()),
						bhvr,
					)
				}
				BehaviorType::SubTree => {
					todo!()
				}
			};
			Ok(tree_node)
		}
	}

	fn create_ports(
		bhvr: &mut Box<dyn BehaviorTreeMethods>,
		tick_data: &mut BehaviorTickData,
		config_data: &mut BehaviorConfigurationData,
		element: &Node,
	) -> Result<(), Error> {
		//let mut remappings = NewPortRemappings::new();
		for attribute in element.attributes() {
			let name = attribute.name().to_string();
			let value = attribute.value().to_string();
			// port "name" is always available
			if name == "name" {
				config_data.set_name(value);
			} else {
				// fetch found port name from list of provided ports
				let port_list = bhvr.static_provided_ports();
				match port_list.find(&name) {
					Ok(port_definition) => {
						tick_data.add_port(name, port_definition.direction, value);
						//todo!();
					}
					Err(_) => {
						return Err(Error::PortInvalid(
							name,
							config_data.name().into(),
							port_list.entries(),
						));
					}
				}
			}
		}
		Ok(())
	}

	#[allow(clippy::unnecessary_wraps)]
	pub(crate) fn handle_subtree(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
		id: &str,
		// if true, only registering happens
		register_only: bool,
	) -> Result<BehaviorTreeNode, Error> {
		let id_string = id.to_string();
		let blackboard = Blackboard::default();
		// look for the behavior in the [`BehaviorRegisty`]
		let (bhvr_type, bhvr_creation_fn) = registry.fetch("Subtree")?;
		let mut bhvr = bhvr_creation_fn();

		let children = Self::build_children(&blackboard, registry, tree, element, register_only)?;
		// let tick_data = BehaviorTickData::new(blackboard);
		let config_data = BehaviorConfigurationData::new(id);
		let subtree = BehaviorTreeNode::new(
			id,
			children,
			BehaviorTickData::new(Blackboard::default()),
			bhvr,
		);
		Ok(subtree)
	}
}
// endregion:   --- XmlParser

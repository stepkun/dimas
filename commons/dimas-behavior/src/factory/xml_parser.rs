// Copyright © 2025 Stephan Kunz

//! XML parser for the [`BehaviorTreeFactory`] of `DiMAS`

// region:      --- modules
use dimas_core::ConstString;
use hashbrown::HashMap;
use roxmltree::{Attributes, Node, NodeType};
use rustc_hash::FxBuildHasher;

use crate::{
	behavior::{BehaviorConfigurationData, BehaviorPtr, BehaviorTickData, BehaviorType},
	blackboard::Blackboard,
	tree::{
		BehaviorTreeComponentList, BehaviorTreeLeaf, BehaviorTreeNode, BehaviorTreeProxy,
		TreeElement,
	},
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:		--- helper
fn attrs_to_map(attrs: Attributes) -> HashMap<ConstString, ConstString, FxBuildHasher> {
	let mut map = HashMap::default();
	//dbg!(self);
	for attr in attrs {
		let name = attr.name().into();
		let value = attr.value().into();
		map.insert(name, value);
	}
	map
}
// endregion:	--- helper

// region:      --- XmlParser
pub struct XmlParser {}

impl XmlParser {
	pub(crate) fn register_root_element(
		blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		element: Node,
		// A reference to the name of the main_tree_to_execute.
		// main_tree_name: &str,
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
								let new_subtree =
									Self::handle_subtree(&blackboard, registry, element, id, true)?;
								// store subtree for later usage
								registry.add_subtree(new_subtree)?;
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
					let behavior = Self::build_child(blackboard, registry, child, register_only)?;
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
		element: Node,
		// if true, only registration of subtrees happens
		register_only: bool,
	) -> Result<TreeElement, Error> {
		let element_name = element.tag_name().name();
		let attrs = attrs_to_map(element.attributes());
		if element_name == "SubTree" {
			if let Some(id) = attrs.get("ID") {
				// let tick_data = BehaviorTickData::default();
				let behavior = BehaviorTreeProxy::create(id, BehaviorTickData::default());
				// let config_data = BehaviorConfigurationData::new(id);
				if register_only {
					Ok(behavior)
				} else {
					// use subtree from list
					todo!()
					// let subtree = registry.find_by_name(id)?;
					// Ok(behavior)
				}
			} else {
				Err(Error::MissingId(element.tag_name().name().into()))
			}
		} else {
			// look for the behavior in the [`BehaviorRegisty`]
			let (bhvr_type, bhvr_creation_fn) = registry.fetch(element_name)?;
			let bhvr = bhvr_creation_fn();
			let tree_node = match bhvr_type {
				BehaviorType::Action | BehaviorType::Condition => {
					if element.has_children() {
						return Err(Error::ChildrenNotAllowed(element_name.into()));
					}
					// let mut tick_data = BehaviorTickData::new(blackboard.clone());
					let mut config_data = BehaviorConfigurationData::new(element_name);
					let mut tick_data = BehaviorTickData::new(blackboard.clone());
					Self::create_ports(&bhvr, &mut tick_data, &mut config_data, &element)?;
					BehaviorTreeLeaf::create(element_name, tick_data, bhvr)
				}
				BehaviorType::Control | BehaviorType::Decorator => {
					let new_children =
						Self::build_children(blackboard, registry, element, register_only)?;
					if bhvr_type == BehaviorType::Decorator && new_children.len() > 1 {
						return Err(Error::DecoratorOnlyOneChild(
							element.tag_name().name().into(),
						));
					}
					let mut tick_data = BehaviorTickData::new(blackboard.clone());
					let mut config_data = BehaviorConfigurationData::default();
					Self::create_ports(&bhvr, &mut tick_data, &mut config_data, &element)?;
					BehaviorTreeNode::create(
						element_name,
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
		bhvr: &BehaviorPtr,
		tick_data: &mut BehaviorTickData,
		config_data: &mut BehaviorConfigurationData,
		element: &Node,
	) -> Result<(), Error> {
		//let mut remappings = NewPortRemappings::new();
		for attribute in element.attributes() {
			let name = attribute.name();
			let value = attribute.value();
			// port "name" is always available
			if name == "name" {
				config_data.set_name(value);
			} else {
				// fetch found port name from list of provided ports
				let port_list = bhvr.static_provided_ports();
				match port_list.find(name) {
					Some(port_definition) => {
						tick_data.add_port(name, port_definition.direction, value)?;
						//todo!();
					}
					None => {
						return Err(Error::PortInvalid(
							name.into(),
							config_data.name().into(),
							port_list.entries(),
						));
					}
				}
			}
		}
		Ok(())
	}

	pub(crate) fn handle_subtree(
		_blackboard: &Blackboard,
		registry: &mut BehaviorRegistry,
		element: Node,
		id: &str,
		// if true, only registering happens
		register_only: bool,
	) -> Result<TreeElement, Error> {
		let blackboard = Blackboard::default();
		// look for the behavior in the [`BehaviorRegisty`]
		let (_bhvr_type, bhvr_creation_fn) = registry.fetch("Subtree")?;
		let bhvr = bhvr_creation_fn();
		// let attrs = attrs_to_map(element.attributes());
		let children = Self::build_children(&blackboard, registry, element, register_only)?;
		// let tick_data = BehaviorTickData::new(blackboard);
		// let config_data = BehaviorConfigurationData::new(id);
		let subtree = BehaviorTreeNode::create(
			id,
			children,
			BehaviorTickData::new(Blackboard::default()),
			bhvr,
		);
		Ok(subtree)
	}
}
// endregion:   --- XmlParser

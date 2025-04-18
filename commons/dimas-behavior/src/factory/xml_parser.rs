// Copyright © 2025 Stephan Kunz
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::unnecessary_wraps)]
#![allow(unused)]

//! XML parser for the [`BehaviorTreeFactory`] of `DiMAS`

// region:      --- modules
use alloc::{
	boxed::Box,
	string::{String, ToString},
	vec::Vec,
};
use roxmltree::{Node, NodeType};

use crate::{
	new_behavior::{
		BehaviorConfigurationData, BehaviorTickData, BehaviorTreeMethods, NewBehaviorType,
	},
	new_blackboard::NewBlackboard,
	new_port::{NewPortRemappings, find_in_port_list, port_list_entries},
	tree::{BehaviorTree, BehaviorTreeComponent},
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:      --- XmlParser
pub struct XmlParser {}

impl XmlParser {
	pub(crate) fn parse_root_element(
		blackboard: &NewBlackboard,
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
		blackboard: &NewBlackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
		id: impl Into<String>,
	) -> Result<(), Error> {
		let id = id.into();
		// A subtreee gets a new [`Blackboard`] with parent trees [`Blackboard`] as parent
		let blackboard = if id == "MainTree" {
			blackboard.clone()
		} else {
			NewBlackboard::new(blackboard)
		};
		let children = Self::build_children(&blackboard, registry, tree, element)?;
		let tick_data = BehaviorTickData::new(blackboard);
		let config_data = BehaviorConfigurationData::default();
		let subtree = BehaviorTreeComponent::create_node(None, tick_data, children, config_data);
		tree.add(id, subtree);
		Ok(())
	}

	fn build_children(
		blackboard: &NewBlackboard,
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
		blackboard: &NewBlackboard,
		registry: &mut BehaviorRegistry,
		tree: &mut BehaviorTree,
		element: Node,
	) -> Result<BehaviorTreeComponent, Error> {
		let bhvr_name = element.tag_name().name();
		if bhvr_name == "SubTree" {
			if let Some(id) = element.attribute("ID") {
				todo!()
			} else {
				Err(Error::MissingId(element.tag_name().name().into()))
			}
		} else {
			// look for the behavior in the [`BehaviorRegisty`]
			let (bhvr_type, bhvr_creation_fn) = registry.fetch(bhvr_name)?;
			let mut bhvr = bhvr_creation_fn();
			let tree_node = match bhvr_type {
				NewBehaviorType::Action | NewBehaviorType::Condition => {
					if element.has_children() {
						return Err(Error::ChildrenNotAllowed(bhvr_type.to_string()));
					}
					let blackboard = blackboard.clone();
					let mut tick_data = BehaviorTickData::new(blackboard);
					let mut config_data = BehaviorConfigurationData::new(bhvr_name);
					Self::create_ports(&mut bhvr, &mut tick_data, &mut config_data, &element)?;
					BehaviorTreeComponent::create_leaf(bhvr, tick_data, config_data)
				}
				NewBehaviorType::Control | NewBehaviorType::Decorator => {
					let blackboard = blackboard.clone();
					let children = Self::build_children(&blackboard, registry, tree, element)?;
					if bhvr_type == NewBehaviorType::Decorator && children.len() > 1 {
						return Err(Error::DecoratorOnlyOneChild(
							element.tag_name().name().into(),
						));
					}
					let mut tick_data = BehaviorTickData::new(blackboard);
					let mut config_data = BehaviorConfigurationData::default();
					Self::create_ports(&mut bhvr, &mut tick_data, &mut config_data, &element)?;
					BehaviorTreeComponent::create_node(Some(bhvr), tick_data, children, config_data)
				}
				NewBehaviorType::SubTree => {
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
				config_data.set_name(name);
			} else {
				// fetch found port name from list of provided ports
				let port_list = bhvr.static_provided_ports();
				match find_in_port_list(&port_list, &name) {
					Ok(port_definition) => {
						tick_data.add_port(&port_definition.direction, name, value);
						//todo!();
					}
					Err(_) => {
						return Err(Error::PortInvalid(
							name,
							config_data.name().into(),
							port_list_entries(&port_list),
						));
					}
				}
			}
		}
		Ok(())
	}
}
// endregion:   --- XmlParser

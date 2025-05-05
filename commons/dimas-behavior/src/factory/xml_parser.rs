// Copyright © 2025 Stephan Kunz

//! XML parser for the [`BehaviorTreeFactory`] of `DiMAS`

extern crate std;

// region:      --- modules
use dimas_core::ConstString;
use hashbrown::HashMap;
use roxmltree::{Attributes, Node, NodeType};
use rustc_hash::FxBuildHasher;

use crate::{
	behavior::{BehaviorConfigurationData, BehaviorPtr, BehaviorTickData, BehaviorType},
	blackboard::BlackboardNodeRef,
	port::{PortRemappings, is_allowed_name},
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
		registry: &mut BehaviorRegistry,
		element: Node,
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
								let new_behavior_tree =
									Self::handle_behaviortree(registry, element, id)?;
								// store behavior tree for later usage
								registry.add_behavior_tree(new_behavior_tree)?;
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
		blackboard: &BlackboardNodeRef,
		registry: &mut BehaviorRegistry,
		element: Node,
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
					let behavior = Self::build_child(blackboard, registry, child)?;
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
	#[allow(clippy::needless_pass_by_value)]
	fn build_child(
		blackboard: &BlackboardNodeRef,
		registry: &mut BehaviorRegistry,
		element: Node,
	) -> Result<TreeElement, Error> {
		let element_name = element.tag_name().name();
		if element_name == "SubTree" {
			// look for the behavior in the `BehaviorRegistry`
			let (_bhvr_type, bhvr_creation_fn) = registry.fetch("Subtree")?;
			let bhvr = bhvr_creation_fn();
			let attrs = attrs_to_map(element.attributes());
			if let Some(id) = attrs.get("ID") {
				let (autoremap, remappings, values) =
					Self::create_remappings(element_name, &bhvr, &attrs)?;
				// A subtree gets a new Blackboard with the current Blackboard as parent and own remappings.
				let blackboard = BlackboardNodeRef::with(blackboard.clone(), remappings, autoremap);
				let _tick_data = BehaviorTickData::new(blackboard, values);
				let _config_data = BehaviorConfigurationData::new(id);
				let behavior = BehaviorTreeProxy::create(id, BehaviorTickData::default());
				Ok(behavior)
			} else {
				Err(Error::MissingId(element.tag_name().name().into()))
			}
		} else {
			// look for the behavior in the `BehaviorRegistry`
			let (bhvr_type, bhvr_creation_fn) = registry.fetch(element_name)?;
			let bhvr = bhvr_creation_fn();
			let attrs = attrs_to_map(element.attributes());
			let (autoremap, remappings, values) =
				Self::create_remappings(element_name, &bhvr, &attrs)?;
			// Within a subtree clone the current Blackboard with own remappings for each element.
			let blackboard = blackboard.cloned(remappings, autoremap);
			let tree_node = match bhvr_type {
				BehaviorType::Action | BehaviorType::Condition => {
					if element.has_children() {
						return Err(Error::ChildrenNotAllowed(element_name.into()));
					}
					let _config_data = BehaviorConfigurationData::new(element_name);
					let tick_data = BehaviorTickData::new(blackboard, values);
					BehaviorTreeLeaf::create(element_name, tick_data, bhvr)
				}
				BehaviorType::Control | BehaviorType::Decorator => {
					let children = Self::build_children(&blackboard, registry, element)?;

					if bhvr_type == BehaviorType::Decorator && children.len() > 1 {
						return Err(Error::DecoratorOnlyOneChild(
							element.tag_name().name().into(),
						));
					}
					let tick_data = BehaviorTickData::new(blackboard, values);
					let _config_data = BehaviorConfigurationData::default();
					BehaviorTreeNode::create(element_name, children, tick_data, bhvr)
				}
				BehaviorType::SubTree => {
					todo!("This should not happen, as Subtrees are handled earlier!")
				}
			};
			Ok(tree_node)
		}
	}

	pub(crate) fn handle_behaviortree(
		registry: &mut BehaviorRegistry,
		element: Node,
		id: &str,
	) -> Result<TreeElement, Error> {
		// look for the behavior in the `BehaviorRegistry`
		let (_bhvr_type, bhvr_creation_fn) = registry.fetch("Behaviortree")?;
		let bhvr = bhvr_creation_fn();
		let attrs = attrs_to_map(element.attributes());
		let (autoremap, remappings, values) = Self::create_remappings(id, &bhvr, &attrs)?;
		// Each behavior tree has a separate Blackboard.
		let blackboard = BlackboardNodeRef::new(remappings, autoremap);
		let children = Self::build_children(&blackboard, registry, element)?;
		let tick_data = BehaviorTickData::new(blackboard, values);
		let _config_data = BehaviorConfigurationData::new(id);
		let behaviortree = BehaviorTreeNode::create(id, children, tick_data, bhvr);
		Ok(behaviortree)
	}

	fn create_remappings(
		id: &str,
		bhvr: &BehaviorPtr,
		attrs: &HashMap<ConstString, ConstString, FxBuildHasher>,
	) -> Result<(bool, PortRemappings, PortRemappings), Error> {
		let autoremap = match attrs.get("_autoremap") {
			Some(s) => match s.parse::<bool>() {
				Ok(val) => val,
				Err(_) => return Err(Error::WrongAutoremap),
			},
			None => false,
		};

		let mut remappings = PortRemappings::default();
		let mut values = PortRemappings::default();
		for (key, value) in attrs {
			let key = key.as_ref();
			if key == "name" {
				// port "name" is always available
			} else if key == "ID" {
				// ignore as it is not a Port
			} else {
				// check found port name against list of provided ports
				let port_list = bhvr.static_provided_ports();
				match port_list.find(key) {
					Some(_) => {
						// check if it is a BB pointer
						if value.starts_with('{') && value.ends_with('}') {
							let stripped = value
								.strip_prefix('{')
								.unwrap_or_else(|| todo!())
								.strip_suffix('}')
								.unwrap_or_else(|| todo!());

							// check value for allowed names
							if is_allowed_name(stripped) {
								remappings.add(key, stripped)?;
							} else {
								return Err(crate::factory::error::Error::NameNotAllowed(
									key.into(),
								));
							}
						} else {
							// this is a normal string, representing a port value
							values.add(key, value)?;
						}
					}
					None => {
						return Err(Error::PortInvalid(
							key.into(),
							id.into(),
							port_list.entries(),
						));
					}
				}
			}
		}
		Ok((autoremap, remappings, values))
	}
}
// endregion:   --- XmlParser

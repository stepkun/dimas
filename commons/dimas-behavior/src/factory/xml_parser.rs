// Copyright © 2025 Stephan Kunz

//! XML parser for the [`BehaviorTreeFactory`] of `DiMAS`

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::string::String;
use dimas_core::BoxConstString;
use hashbrown::HashMap;
use roxmltree::{Attributes, Document, Node, NodeType};
use rustc_hash::FxBuildHasher;
#[cfg(feature = "std")]
use std::path::PathBuf;
use tracing::{Level, event, instrument};

use crate::{
	behavior::{BehaviorPtr, BehaviorTickData, BehaviorType},
	blackboard::SharedBlackboard,
	port::{PortRemappings, is_allowed_port_name},
	tree::{BehaviorTreeElement, BehaviorTreeElementList},
};

use super::{behavior_registry::BehaviorRegistry, error::Error};
// endregion:   --- modules

// region:		--- helper
fn attrs_to_map(attrs: Attributes) -> HashMap<BoxConstString, BoxConstString, FxBuildHasher> {
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
	#[instrument(level = Level::DEBUG, skip_all)]
	fn register_document(registry: &mut BehaviorRegistry, xml: &str) -> Result<(), Error> {
		// general checks
		let doc = Document::parse(xml)?;
		let root = doc.root_element();
		if root.tag_name().name() != "root" {
			return Err(Error::WrongRootName);
		}
		if let Some(format) = root.attribute("BTCPP_format") {
			if format != "4" {
				return Err(Error::BtCppFormat);
			}
		}

		Self::register_document_root(registry, root)?;
		Ok(())
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	pub(crate) fn register_document_root(
		registry: &mut BehaviorRegistry,
		element: Node,
	) -> Result<(), Error> {
		event!(Level::TRACE, "register_document_root");
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
								let source: BoxConstString =
									element.document().input_text()[element.range()].into();
								registry.add_tree_defintion(id, source)?;
							} else {
								return Err(Error::MissingId(element.tag_name().name().into()));
							}
						}
						#[cfg(feature = "std")]
						"include" => {
							let mut file_path: PathBuf;
							if let Some(path) = element.attribute("path") {
								file_path = PathBuf::from(path);
								if file_path.is_relative() {
									// get the "current" directory
									file_path = std::env::current_dir()?;
									file_path.push(path);
								}
							} else {
								return Err(Error::MissingPath(element.tag_name().name().into()));
							}
							let xml = std::fs::read_to_string(file_path)?;
							Self::register_document(registry, &xml)?;
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

	fn create_remappings(
		id: &str,
		is_subtree: bool,
		bhvr: &BehaviorPtr,
		attrs: &HashMap<BoxConstString, BoxConstString, FxBuildHasher>,
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
				// for a subtree we cannot check the ports
				if is_subtree {
					// check if it is a BB pointer
					if value.starts_with('{') && value.ends_with('}') {
						let stripped = value
							.strip_prefix('{')
							.unwrap_or_else(|| todo!())
							.strip_suffix('}')
							.unwrap_or_else(|| todo!());

						// check value for allowed names
						if is_allowed_port_name(stripped) {
							remappings.add(key, stripped)?;
						} else {
							return Err(crate::factory::error::Error::NameNotAllowed(key.into()));
						}
					} else {
						// this is a normal string, representing a port value
						values.add(key, value)?;
					}
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
								if is_allowed_port_name(stripped) {
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
		}
		Ok((autoremap, remappings, values))
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	pub(crate) fn create_tree_from_definition(
		id: &str,
		registry: &BehaviorRegistry,
	) -> Result<BehaviorTreeElement, Error> {
		event!(Level::TRACE, "create_tree_from_definition");
		registry.find_tree_definition(id).map_or_else(
			|| Err(Error::SubtreeNotFound(id.into())),
			|definition| {
				let doc = Document::parse(&definition)?;
				let node = doc.root_element();
				// look for the "SubTree" behavior in the `BehaviorRegistry` and create it.
				let (_bhvr_type, bhvr_creation_fn) = registry.fetch("SubTree")?;
				let bhvr = bhvr_creation_fn();
				// handle the nodes attributes
				let attrs = attrs_to_map(node.attributes());
				let (_, remappings, values) = Self::create_remappings(id, true, &bhvr, &attrs)?;
				let blackboard = SharedBlackboard::new(id.into(), remappings, values);
				let children = Self::build_children(id, node, registry, blackboard.clone())?;
				let tick_data = BehaviorTickData::default();
				let behaviortree =
					BehaviorTreeElement::create_node(id, id, children, tick_data, blackboard, bhvr);
				Ok(behaviortree)
			},
		)
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_children(
		path: &str,
		node: Node,
		registry: &BehaviorRegistry,
		blackboard: SharedBlackboard,
	) -> Result<BehaviorTreeElementList, Error> {
		event!(Level::TRACE, "build_children");
		let mut children = BehaviorTreeElementList::default();
		for child in node.children() {
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
					let element = Self::build_child(path, child, registry, blackboard.clone())?;
					children.push(element);
				}
				NodeType::PI => {
					return Err(Error::UnsupportedProcessingInstruction(
						node.tag_name().name().into(),
					));
				}
			}
		}

		children.shrink_to_fit();
		Ok(children)
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_child(
		path: &str,
		node: Node,
		registry: &BehaviorRegistry,
		blackboard: SharedBlackboard,
	) -> Result<BehaviorTreeElement, Error> {
		event!(Level::TRACE, "build_child");
		let mut node_name = node.tag_name().name();
		let is_subtree = node_name == "SubTree";
		// handle the nodes attributes
		let attrs = attrs_to_map(node.attributes());

		// if node is denoted with type of behavior, use ID attribute as name
		if node_name == "Action"
			|| node_name == "Condition"
			|| node_name == "Control"
			|| node_name == "Decorator"
			|| node_name == "SubTree"
		{
			if let Some(id) = attrs.get("ID") {
				node_name = id;
			} else {
				return Err(Error::MissingId(node.tag_name().name().into()));
			}
		}
		let path = String::from(path) + "/" + node_name;
		// look for the behavior in the `BehaviorRegistry`
		let (bhvr_type, bhvr_creation_fn) = if is_subtree {
			registry.fetch("SubTree")?
		} else {
			registry.fetch(node_name)?
		};
		let bhvr = bhvr_creation_fn();
		let (autoremap, remappings, values) =
			Self::create_remappings(node_name, is_subtree, &bhvr, &attrs)?;
		let tree_node = match bhvr_type {
			BehaviorType::Action | BehaviorType::Condition => {
				if node.has_children() {
					return Err(Error::ChildrenNotAllowed(node_name.into()));
				}
				// A leaf gets a cloned Blackboard with own remappings
				let blackboard = blackboard.cloned(remappings, values);
				let tick_data = BehaviorTickData::default();
				BehaviorTreeElement::create_leaf(node_name, &path, tick_data, blackboard, bhvr)
			}
			BehaviorType::Control | BehaviorType::Decorator => {
				// A node gets a cloned Blackboard with own remappings
				let blackboard = blackboard.cloned(remappings, values);
				let children = Self::build_children(&path, node, registry, blackboard.clone())?;

				if bhvr_type == BehaviorType::Decorator && children.len() > 1 {
					return Err(Error::DecoratorOnlyOneChild(node.tag_name().name().into()));
				}
				let tick_data = BehaviorTickData::default();
				BehaviorTreeElement::create_node(
					node_name, &path, children, tick_data, blackboard, bhvr,
				)
			}
			BehaviorType::SubTree => {
				let definition = registry.find_tree_definition(node_name);
				match definition {
					Some(definition) => {
						let doc = Document::parse(&definition)?;
						let node = doc.root_element();
						// A SubTree gets a new Blackboard with parent and remappings.
						let blackboard1 = SharedBlackboard::with(
							node_name.into(),
							blackboard,
							remappings,
							values,
							autoremap,
						);
						let children =
							Self::build_children(&path, node, registry, blackboard1.clone())?;
						let tick_data = BehaviorTickData::default();
						BehaviorTreeElement::create_node(
							node_name,
							&path,
							children,
							tick_data,
							blackboard1,
							bhvr,
						)
					}
					None => {
						return Err(Error::SubtreeNotFound(node_name.into()));
					}
				}
			}
		};
		Ok(tree_node)
	}
}
// endregion:   --- XmlParser

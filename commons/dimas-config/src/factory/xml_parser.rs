// Copyright © 2024 Stephan Kunz

//! XML parser for the [`BehaviorTree`] factory [`BTFactory`] of `DiMAS`

extern crate std;
use core::borrow::{Borrow, BorrowMut};
use std::dbg;

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	string::{String, ToString},
	vec::Vec,
};
use dimas_core::{
	behavior::{Behavior, BehaviorCategory, BehaviorConfig},
	blackboard::Blackboard,
	port::PortRemapping,
};
use hashbrown::HashMap;
use roxmltree::{Attributes, Document, Node, NodeType, ParsingOptions};
use tracing::{instrument, Level};

use super::{error::Error, factory::FactoryData};
// endregion:   --- modules

// region:      --- helper
#[derive(Debug)]
enum CreateBehaviorResult {
	Behavior(Behavior),
	Continue,
	End,
}

pub trait AttrsToMap {
	fn to_map(self) -> Result<HashMap<String, String>, Error>;
}

impl AttrsToMap for Attributes<'_, '_> {
	fn to_map(self) -> Result<HashMap<String, String>, Error> {
		let mut map = HashMap::new();
		//dbg!(self);
		for attr in self {
			let name = attr.name().into();
			let value = attr.value().to_string();
			map.insert(name, value);
		}

		Ok(map)
	}
}
// endregion:   --- helper

// region:      --- XmlParser
#[derive(Debug, Default)]
pub struct XmlParser {
	options: ParsingOptions,
}

impl XmlParser {
	/// @TODO:
	/// # Errors
	fn add_ports(
		bhvr_ptr: &mut Behavior,
		bhvr_name: &str,
		attributes: Attributes,
	) -> Result<(), Error> {
		let config = bhvr_ptr.config_mut();
		let manifest = config.manifest()?;

		let mut remap = PortRemapping::new();

		for (port_name, port_value) in attributes.to_map()? {
			remap.insert(port_name, port_value);
		}

		// Check if all ports from XML match ports in manifest
		for port_name in remap.keys() {
			if !manifest.ports.contains_key(port_name) {
				return Err(Error::PortInvalid(
					port_name.clone(),
					bhvr_name.to_owned(),
					manifest.ports.clone().into_keys().collect(),
				));
			}
		}

		// Add ports to BehaviorConfig
		for (remap_name, remap_val) in remap {
			if let Some(port) = manifest.ports.get(&remap_name) {
				// Validate that any expr-enabled ports contain valid expressions,
				// and the provided types for blackboard pointers are one of the valid ones
				if port.parse_expr() {
					let expr =
						evalexpr::build_operator_tree::<evalexpr::DefaultNumericTypes>(&remap_val)?;

					for key in expr.iter_variable_identifiers() {
						// Check if it's a blackboard pointer
						if key.starts_with('{') && key.ends_with('}') {
							// Remove the brackets
							let inner_key = &key[1..(key.len() - 1)];
							// Split the type from the name
							let (name, var_type) = inner_key.split_once(':').ok_or_else(|| {
								Error::PortExpressionMissingType(inner_key.to_owned())
							})?;

							// Check if the type is supported
							match var_type {
								"int" | "float" | "str" | "bool" => (),
								_ => {
									return Err(Error::PortExpressionInvalidType(
										var_type.to_owned(),
										name.to_owned(),
									))
								}
							};
						}
					}
				}

				config.add_port(port.direction(), remap_name, remap_val);
			}
		}

		Ok(())
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_child(
		&self,
		element: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		tree_name: &str,
		path: &str,
	) -> Result<Behavior, Error> {
		let bhvr_name = element.tag_name().name();
		let (bhvr_category, bhvr_fn) = data
			.bhvr_map
			.get(bhvr_name)
			.ok_or_else(|| Error::UnknownBehavior(bhvr_name.into()))?
			.clone();

		let attributes = element.attributes();
		let mut config = BehaviorConfig::new(blackboard.clone());
		config.path = path.to_owned() + bhvr_name;

		let node = match bhvr_category {
			BehaviorCategory::Action | BehaviorCategory::Condition => {
				if element.has_children() {
					return Err(Error::ChildrenNotAllowed(bhvr_category.to_string()));
				}
				let mut behavior = bhvr_fn(config, Vec::new());
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				behavior
			}
			BehaviorCategory::Control => {
				let children = self.build_children(element, data, blackboard, tree_name, path)?;
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				behavior
			}
			BehaviorCategory::Decorator => {
				let children = self.build_children(element, data, blackboard, tree_name, path)?;
				if children.len() != 1 {
					return Err(Error::DecoratorChildren(element.tag_name().name().into()));
				}
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				behavior
			}
			BehaviorCategory::SubTree => todo!(),
		};

		Ok(node)
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_children(
		&self,
		element: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		tree_name: &str,
		path: &str,
	) -> Result<Vec<Behavior>, Error> {
		let mut children: Vec<Behavior> = Vec::new();

		for child in element.children() {
			match child.node_type() {
				NodeType::Comment | NodeType::Text => {} // ignore
				NodeType::Root => todo!(),               // this should not happen
				NodeType::Element => {
					let behavior = self.build_child(child, data, blackboard, tree_name, path)?;
					children.push(behavior);
				}
				NodeType::PI => {
					return Err(Error::UnkownProcessingInstruction);
				}
			}
		}

		Ok(children)
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn parse_behavior(
		&self,
		element: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		tree_id: &str,
		path: &str,
	) -> Result<Behavior, Error> {
		// lookup behavior in registered behaviors
		// sub trees must have been parsed before their usage
		let bhvr_name = element.tag_name().name();
		let (bhvr_category, bhvr_fn) = data
			.bhvr_map
			.get(bhvr_name)
			.ok_or_else(|| Error::UnknownBehavior(bhvr_name.into()))?
			.clone();

		let attributes = element.attributes();
		let mut config = BehaviorConfig::new(blackboard.clone());
		config.path = path.to_owned() + bhvr_name;

		match bhvr_category {
			BehaviorCategory::Action | BehaviorCategory::Condition => {
				if element.has_children() {
					return Err(Error::ChildrenNotAllowed(bhvr_category.to_string()));
				}
				let mut behavior = bhvr_fn(config, Vec::new());
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				Ok(behavior)
			}
			BehaviorCategory::Control => {
				let children = self.build_children(element, data, blackboard, tree_id, path)?;
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				Ok(behavior)
			}
			BehaviorCategory::Decorator => {
				let children = self.build_children(element, data, blackboard, tree_id, path)?;
				if children.len() != 1 {
					return Err(Error::DecoratorChildren(element.tag_name().name().into()));
				}
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				Ok(behavior)
			}
			BehaviorCategory::SubTree => todo!(),
		}
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn parse_behavior_tree(
		&self,
		bt: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		tree_id: &str,
		path: String,
	) -> Result<Behavior, Error> {
		for element in bt.children() {
			match element.node_type() {
				NodeType::Comment | NodeType::Text => {} // ignore
				NodeType::Root => todo!(),               // this should not happen
				NodeType::Element => {
					return self.parse_behavior(element, data, blackboard, tree_id, &path);
				}
				NodeType::PI => {
					return Err(Error::UnkownProcessingInstruction);
				}
			}
		}
		todo!()
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn parse_document(
		&self,
		doc: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
	) -> Result<Option<Behavior>, Error> {
		let mut root_behavior: Option<Behavior> = None;

		for element in doc.children() {
			match element.node_type() {
				NodeType::Comment | NodeType::Text => {} // ignore
				NodeType::Root => todo!(),               // this should not happen
				NodeType::Element => {
					// only 'BehaviorTree' or 'TreeNodesModel' are valid
					match element.tag_name().name() {
						"TreeNodesModel" => {} // ignore
						"BehaviorTree" => {
							// check for root tree ID
							if let Some(id) = element.attribute("ID") {
								let behavior = self.parse_behavior_tree(
									element,
									data,
									blackboard,
									id,
									String::new(),
								)?;

								// root behavior?
                                if let Some(main_id) = &data.main_tree_id {
                                    if main_id == id {
                                        root_behavior = Some(behavior);
									} else {
										// SubTree definition
										dbg!(&element);
										todo!();
									}
								} else {
									// SubTree definition
									dbg!(&element);
									todo!();
								}
							} else {
								return Err(Error::MissingId(element.tag_name().name().into()));
							};
						}
						_ => {
							return Err(Error::ElementNotSupported(
								element.tag_name().name().into(),
							));
						}
					}
				}
				NodeType::PI => {
					return Err(Error::UnkownProcessingInstruction);
				}
			}
		}

		Ok(root_behavior)
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn parse_root(
		&self,
		blackboard: &Blackboard,
		data: &mut FactoryData,
		xml: &str,
	) -> Result<Behavior, Error> {
		let doc = Document::parse_with_options(xml, self.options)?;
		let root = doc.root_element();
		if root.tag_name().name() != "root" {
			return Err(Error::RootName);
		}

		if let Some(format) = root.attribute("BTCPP_format") {
			if format != "4" {
				return Err(Error::BtCppFormat);
			}
		};

		if let Some(id) = root.attribute("main_tree_to_execute") {
			data.main_tree_id = Some(id.into());
            let root_behavior = self.parse_document(root, data, blackboard)?;
			root_behavior.map_or_else(
				|| {
					Err(Error::Unexpected(
						"no tree created".into(),
						file!().into(),
						line!(),
					))
				},
				Ok,
			)
			} else {
            Err(Error::NoTreeToExecute)
        }
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn parse_subtree(
		&self,
		blackboard: &Blackboard,
		data: &mut FactoryData,
		xml: &str,
	) -> Result<(), Error> {
		let doc = Document::parse_with_options(xml, self.options)?;
		let root = doc.root_element();
		if root.tag_name().name() != "root" {
			return Err(Error::RootName);
		}

		if let Some(format) = root.attribute("BTCPP_format") {
			if format != "4" {
				return Err(Error::BtCppFormat);
			}
		};

		if let Some(id) = root.attribute("main_tree_to_execute") {
			return Err(Error::MainTreeNotAllowed);
		}

		let _ = self.parse_document(root, data, blackboard)?;

		Ok(())
	}
}
// endregion:   --- XmlParser

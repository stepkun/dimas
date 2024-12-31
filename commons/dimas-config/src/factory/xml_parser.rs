// Copyright © 2024 Stephan Kunz

//! XML parser for the [`BehaviorTree`] factory [`BTFactory`] of `DiMAS`

extern crate std;
use core::borrow::{Borrow, BorrowMut};
use std::dbg;

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	format,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use dimas_core::{
	behavior::{Behavior, BehaviorCategory, BehaviorConfig},
	blackboard::{Blackboard, BlackboardString},
	build_bhvr_ptr,
	port::{PortChecks, PortRemapping},
};
use hashbrown::HashMap;
use roxmltree::{Attributes, Document, Node, NodeType, ParsingOptions};
use tracing::{instrument, Level};

use super::{
	error::Error,
	factory::{BehaviorCreateFn, FactoryData},
};
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
pub struct XmlParser {}

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

	fn find_in_map(
		element: Node,
		data: &FactoryData,
	) -> Result<(BehaviorCategory, Arc<BehaviorCreateFn>), Error> {
		let bhvr_name = element.tag_name().name();
		if let Some(id) = element.attribute("ID") {
			Ok(data
				.bhvr_map
				.get(id)
				.ok_or_else(|| Error::UnknownBehavior(bhvr_name.into()))?
				.clone())
		} else {
			Ok(data
				.bhvr_map
				.get(bhvr_name)
				.ok_or_else(|| Error::UnknownBehavior(bhvr_name.into()))?
				.clone())
		}
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_child(
		element: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		tree_name: &str,
		path: &str,
	) -> Result<Behavior, Error> {
		// lookup behavior in registered behaviors and subtree definitions
		// sub trees must have been parsed before their usage
		let res = Self::find_in_map(element, data);
		let Ok((bhvr_category, bhvr_fn)) = res else {
			return Self::build_subtree(element, data, blackboard, path);
		};

		let bhvr_name = element.tag_name().name();
		let attributes = element.attributes();
		let mut config = BehaviorConfig::new(blackboard.clone());
		config.path = path.to_owned() + bhvr_name;

		let bhvr = match bhvr_category {
			BehaviorCategory::Action | BehaviorCategory::Condition => {
				if element.has_children() {
					return Err(Error::ChildrenNotAllowed(bhvr_category.to_string()));
				}
				let mut behavior = bhvr_fn(config, Vec::new());
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				behavior
			}
			BehaviorCategory::Control => {
				let children = Self::build_children(element, data, blackboard, tree_name, path)?;
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				behavior
			}
			BehaviorCategory::Decorator => {
				let children = Self::build_children(element, data, blackboard, tree_name, path)?;
				if children.len() != 1 {
					return Err(Error::DecoratorChildren(element.tag_name().name().into()));
				}
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				behavior
			}
		};

		Ok(bhvr)
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_children(
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
					let behavior = Self::build_child(child, data, blackboard, tree_name, path)?;
					children.push(behavior);
				}
				NodeType::PI => {
					return Err(Error::UnkownProcessingInstruction);
				}
			}
		}

		Ok(children)
	}

	#[instrument(level = Level::DEBUG, skip_all)]
	fn build_subtree(
		element: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		path: &str,
	) -> Result<Behavior, Error> {
		if let Some(id) = element.attribute("ID") {
			let definition = match data.tree_definitions.get(id) {
				Some(def) => def.to_owned(),
				None => return Err(Error::UnknownBehavior(id.into())),
			};
			let doc = Document::parse(&definition)?;
			let root = doc.root_element();

			let attributes = element.attributes();
			let path = path.to_owned() + "->" + id;

			let attributes = attributes.to_map()?;
			let mut subtree_blackboard = Blackboard::new(blackboard);

			// Process attributes (Ports, special fields, etc)
			for (attr, value) in attributes {
				// Set autoremapping to true or false
				if attr == "_autoremap" {
					let val = value.parse::<bool>()?;

					subtree_blackboard.enable_auto_remapping(val);
					continue;
				} else if !attr.is_allowed_port_name() {
					continue;
				}

				if let Some(port_name) = value.strip_bb_pointer() {
					// Add remapping if `value` is a Blackboard pointer
					subtree_blackboard.add_subtree_remapping(attr.clone(), port_name);
				} else {
					// Set string value into Blackboard
					subtree_blackboard.set(attr, value.clone());
				}
			}

			Self::build_child(root, data, &subtree_blackboard, id, &path)
		} else {
			let bhvr_name = element.tag_name().name();
			Err(Error::UnknownBehavior(bhvr_name.into()))
		}
	}

	/// @TODO:
	/// # Errors
	fn get_build_instructions(element: Node, id: &str) -> Result<String, Error> {
		let source = element.document().input_text();
		let pattern = format!("\"{id}\"");
		let start = pattern.len()
			+ 1 + source
			.find(&pattern)
			.ok_or_else(|| Error::MissingId(id.into()))?;
		let end = start
			+ source[start..]
				.find("</BehaviorTree")
				.ok_or_else(|| Error::MissingId(id.into()))?;
		Ok(source[start..end]
			.replace(['\n', '\t'], "")
			.trim()
			.replace("   ", " ")
			.replace("  ", " "))
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn parse_behavior(
		element: Node,
		data: &mut FactoryData,
		blackboard: &Blackboard,
		tree_id: &str,
		path: &str,
	) -> Result<Behavior, Error> {
		// lookup behavior in registered behaviors and subtree definitions
		// sub trees must have been parsed before their usage
		let res = Self::find_in_map(element, data);
		let Ok((bhvr_category, bhvr_fn)) = res else {
			return Self::build_subtree(element, data, blackboard, path);
		};

		let bhvr_name = element.tag_name().name();

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
				let children = Self::build_children(element, data, blackboard, tree_id, path)?;
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				Ok(behavior)
			}
			BehaviorCategory::Decorator => {
				let children = Self::build_children(element, data, blackboard, tree_id, path)?;
				if children.len() != 1 {
					return Err(Error::DecoratorChildren(element.tag_name().name().into()));
				}
				let mut behavior = bhvr_fn(config, children);
				Self::add_ports(&mut behavior, bhvr_name, attributes)?;
				Ok(behavior)
			}
		}
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	fn parse_behavior_tree(
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
					return Self::parse_behavior(element, data, blackboard, tree_id, &path);
				}
				NodeType::PI => {
					todo!(); //return Err(Error::UnkownProcessingInstruction);
				}
			}
		}
		Err(Error::NoTreeContent)
	}

	/// @TODO:
	/// # Errors
	//#[instrument(level = Level::DEBUG, skip_all)]
	fn parse_document(
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
								// 'main_behavior_to_execute' known?
								if let Some(main_id) = &data.main_tree_id {
									// is it 'main_behavior_to_execute'?
									if main_id == id {
										let behavior = Self::parse_behavior_tree(
											element,
											data,
											blackboard,
											id,
											String::from(id),
										)?;
										root_behavior = Some(behavior);
									} else {
										// SubTree definition
										let bi = Self::get_build_instructions(element, id)?;
										data.tree_definitions.insert(id.into(), bi);
									}
								} else {
									// SubTree definition
									let bi = Self::get_build_instructions(element, id)?;
									data.tree_definitions.insert(id.into(), bi);
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
	pub fn parse_main_xml(
		blackboard: &Blackboard,
		data: &mut FactoryData,
		xml: &str,
	) -> Result<Behavior, Error> {
		let doc = Document::parse(xml)?;
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
			let root_behavior = Self::parse_document(root, data, blackboard)?;
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
			todo!() // Err(Error::NoTreeToExecute)
		}
	}

	/// @TODO:
	/// # Errors
	#[instrument(level = Level::DEBUG, skip_all)]
	pub fn parse_sub_xml(
		blackboard: &Blackboard,
		data: &mut FactoryData,
		xml: &str,
	) -> Result<(), Error> {
		let doc = Document::parse(xml)?;
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
		};

		let _res = Self::parse_document(root, data, blackboard)?;

		Ok(())
	}
}
// endregion:   --- XmlParser

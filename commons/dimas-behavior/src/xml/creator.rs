// Copyright © 2025 Stephan Kunz

//! XML writer for `DiMAS`

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{
	collections::btree_map::BTreeMap,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use dimas_core::ConstString;
use parking_lot::Mutex;

use crate::{
	behavior::{
		BehaviorDescription,
		pre_post_conditions::{POST_CONDITIONS, PRE_CONDITIONS},
	},
	factory::BehaviorTreeFactory,
	tree::{BehaviorTree, BehaviorTreeElement, TreeElementKind},
};
use xml_writer::XmlWriter;

// endregion:   --- modules

// region:      --- XmlWriter
/// Write different kinds of XML from various sources.
#[derive(Default)]
pub struct XmlCreator;

impl XmlCreator {
	/// Create XML `TreeNodesModel` from factories registered nodes.
	/// # Errors
	pub fn write_tree_nodes_model(factory: &BehaviorTreeFactory, pretty: bool) -> Result<ConstString, std::io::Error> {
		let mut writer = if pretty {
			XmlWriter::very_pretty_mode(Vec::new())
		} else {
			XmlWriter::compact_mode(Vec::new())
		};

		writer.begin_elem("root")?;
		writer.attr("BTCPP_format", "4")?;
		writer.begin_elem("TreeNodesModel")?;

		// loop over factories behavior entries in registry
		for item in factory.registry().behaviors() {
			if !item.1.0.groot2() {
				writer.begin_elem(item.1.0.kind_str())?;
				writer.attr("ID", item.0)?;
				// look for a PortsList
				for port in &item.1.0.ports().0 {
					writer.begin_elem(port.direction().type_str())?;
					writer.attr("name", port.name())?;
					writer.attr("type", port.type_name())?;
					writer.end_elem()?;
				}
				writer.end_elem()?;
			}
		}

		writer.end_elem()?; // TreeNodesModel
		writer.end_elem()?; // root
		writer.flush()?;
		let raw = writer.into_inner();
		let mut output = String::with_capacity(raw.len());
		for c in raw {
			output.push(c as char);
		}
		Ok(output.into())
	}

	/// Create XML from tree including `TreeNodesModel`.
	/// # Errors
	pub fn write_tree(
		tree: &BehaviorTree,
		metadata: bool,
		builtin_models: bool,
		pretty: bool,
	) -> Result<ConstString, std::io::Error> {
		// storage for (non groot2 builtin) behaviors to mention in TreeNodesModel
		let mut behaviors: BTreeMap<ConstString, BehaviorDescription> = BTreeMap::new();
		let mut subtrees: BTreeMap<ConstString, &BehaviorTreeElement> = BTreeMap::new();

		let inner = if pretty {
			XmlWriter::very_pretty_mode(Vec::new())
		} else {
			XmlWriter::compact_mode(Vec::new())
		};
		let writer = Arc::new(Mutex::new(inner));
		{
			writer.lock().begin_elem("root")?;
			writer.lock().attr("BTCPP_format", "4")?;

			// scan the tree
			for item in tree.iter() {
				#[allow(clippy::match_same_arms)]
				match item.kind() {
					TreeElementKind::Leaf => {
						let desc = item.data().description();
						if builtin_models || !desc.groot2() {
							behaviors.insert(desc.name().clone(), desc.clone());
						}
					}
					TreeElementKind::Node => {
						let desc = item.data().description();
						if builtin_models || !desc.groot2() {
							behaviors.insert(desc.name().clone(), desc.clone());
						}
					}
					TreeElementKind::SubTree => {
						subtrees.insert(item.data().description().path().clone(), item);
					}
				}
			}

			// create the BehaviorTree's
			for (_path, subtree) in subtrees {
				writer.lock().begin_elem("BehaviorTree")?;
				writer
					.lock()
					.attr("ID", subtree.data().description().name())?;
				writer
					.lock()
					.attr("_fullpath", subtree.data().description().groot2_path())?;

				// recursive dive into children
				for element in subtree.children().iter() {
					Self::write_subtree(element, &writer, metadata)?;
				}
				writer.lock().end_elem()?; // BehaviorTree
			}

			// create the TreeNodesModel
			writer.lock().begin_elem("TreeNodesModel")?;
			// loop over collected behavior entries
			for (name, item) in &behaviors {
				if builtin_models || !item.groot2() {
					writer.lock().begin_elem(item.kind_str())?;
					writer.lock().attr("ID", name)?;
					// look for a PortsList
					for port in &item.ports().0 {
						writer
							.lock()
							.begin_elem(port.direction().type_str())?;
						writer.lock().attr("name", port.name())?;
						writer.lock().attr("type", port.type_name())?;
						writer.lock().end_elem()?;
					}
					writer.lock().end_elem()?;
				}
			}

			writer.lock().end_elem()?; // TreeNodesModel
			writer.lock().end_elem()?; // root
			writer.lock().flush()?;
		}

		Arc::into_inner(writer).map_or_else(
			|| todo!(),
			|inner| {
				let raw = inner.into_inner().into_inner();
				let mut output = String::with_capacity(raw.len());
				for c in raw {
					output.push(c as char);
				}
				Ok(output.into())
			},
		)
	}

	fn write_subtree<'a>(
		element: &'a BehaviorTreeElement,
		writer: &Arc<Mutex<XmlWriter<'a, Vec<u8>>>>,
		metadata: bool,
	) -> Result<(), std::io::Error> {
		let is_subtree = match element.kind() {
			TreeElementKind::Leaf | TreeElementKind::Node => {
				writer
					.lock()
					.begin_elem(element.data().description().id())?;
				writer
					.lock()
					.attr("name", element.data().description().name())?;
				false
			}
			TreeElementKind::SubTree => {
				writer.lock().begin_elem("SubTree")?;
				writer
					.lock()
					.attr("ID", element.data().description().name())?;
				if metadata {
					writer
						.lock()
						.attr("_fullpath", element.data().description().groot2_path())?;
				}
				true
			}
		};
		if metadata {
			writer
				.lock()
				.attr("_uid", &element.data().uid().to_string())?;
		}

		if is_subtree {
			// subtree port mappings/values are in blackboard
			if let Some(remappings) = element.data().blackboard().remappings() {
				for remapping in remappings.iter() {
					writer.lock().attr(&remapping.0, &remapping.1)?;
				}
			}
		} else {
			// behavior port mappings/values
			for remapping in element.data().remappings().iter() {
				writer.lock().attr(&remapping.0, &remapping.1)?;
			}
		}

		// Pre-conditions
		if let Some(conditions) = &element.pre_conditions().0 {
			for i in 0..PRE_CONDITIONS.len() {
				if let Some(cond) = &conditions[i] {
					writer.lock().attr(PRE_CONDITIONS[i], cond)?;
				}
			}
		};

		// Post-conditions
		if let Some(conditions) = &element.post_conditions().0 {
			for i in 0..POST_CONDITIONS.len() {
				if let Some(cond) = &conditions[i] {
					writer.lock().attr(POST_CONDITIONS[i], cond)?;
				}
			}
		};

		if !is_subtree {
			// recursive dive into children, ignoring subtrees
			for element in element.children().iter() {
				Self::write_subtree(element, writer, metadata)?;
			}
		}

		writer.lock().end_elem()?;

		Ok(())
	}
}
// endregion:   --- XmlWriter

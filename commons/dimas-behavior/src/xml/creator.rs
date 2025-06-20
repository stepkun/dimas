// Copyright © 2025 Stephan Kunz

//! XML writer for `DiMAS`

#[doc(hidden)]
#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::{collections::btree_map::BTreeMap, string::String, vec::Vec};
use dimas_core::ConstString;

use crate::{
	behavior::BehaviorDescription,
	factory::BehaviorTreeFactory,
	tree::{BehaviorTree, BehaviorTreeElement, TreeElementKind},
};
use xml_writer::XmlWriter;

// endregion:   --- modules

// region:		--- helper
// endregion:	--- helper

// region:      --- XmlWriter
/// Write different kinds of XML from various sources.
#[derive(Default)]
pub struct XmlCreator;

impl XmlCreator {
	/// Create XML `TreeNodesModel` from factories registered nodes.
	/// # Errors
	pub fn write_tree_nodes_model(factory: &BehaviorTreeFactory) -> Result<ConstString, std::io::Error> {
		let mut xml = XmlWriter::new(Vec::new());
		xml.pretty = true;
		xml.begin_elem("root")?;
		xml.attr("BTCPP_format", "4")?;
		xml.begin_elem("TreeNodesModel")?;

		// loop over factories behavior entries in registry
		for item in factory.registry().behaviors() {
			if !item.1.0.groot2() {
				xml.begin_elem(item.1.0.kind_str())?;
				xml.attr("ID", item.0)?;
				// look for a PortsList
				for port in &item.1.0.ports().0 {
					xml.begin_elem(port.direction().type_str())?;
					xml.attr("name", &port.name())?;
					xml.attr("type", port.type_name())?;
					xml.end_elem()?;
				}
				xml.end_elem()?;
			}
		}

		xml.end_elem()?; // TreeNodesModel
		xml.end_elem()?; // root
		xml.flush()?;
		let raw = xml.into_inner();
		let mut output = String::with_capacity(raw.len());
		for c in raw {
			output.push(c as char);
		}
		Ok(output.into())
	}

	/// Create XML from tree including `TreeNodesModel`.
	/// # Errors
	pub fn write_tree(tree: &BehaviorTree) -> Result<ConstString, std::io::Error> {
		/*
		<root BTCPP_format="4">
			<BehaviorTree ID="MainTree" _fullpath="">
				<Sequence name="Sequence">
					<Script name="Script" code="door_open:=false"/>
					<UpdatePosition name="UpdatePosition" pos="{pos_2D}"/>
					<Fallback name="Fallback">
						<Inverter name="Inverter">
							<IsDoorClosed name="IsDoorClosed"/>
						</Inverter>
						<SubTree ID="DoorClosed" door_open="{door_open}"/>
					</Fallback>
					<PassThroughDoor name="PassThroughDoor"/>
				</Sequence>
			</BehaviorTree>
			<BehaviorTree ID="DoorClosed" _fullpath="DoorClosed::7">
				<Fallback name="tryOpen" _onSuccess="door_open:=true">
					<OpenDoor name="OpenDoor"/>
					<RetryUntilSuccessful name="RetryUntilSuccessful" num_attempts="5">
						<PickLock name="PickLock"/>
					</RetryUntilSuccessful>
					<SmashDoor name="SmashDoor"/>
				</Fallback>
			</BehaviorTree>
		</root>
		 */

		// storage for (non groot2 builtin) behaviors to mention in TreeNodesModel
		let mut behaviors: BTreeMap<ConstString, BehaviorDescription> = BTreeMap::new();
		let mut subtrees: BTreeMap<ConstString, &BehaviorTreeElement> = BTreeMap::new();

		let mut xml: XmlWriter<'_, Vec<u8>> = XmlWriter::new(Vec::new());
		xml.pretty = true;
		xml.begin_elem("root")?;
		xml.attr("BTCPP_format", "4")?;

		// scan the tree
		for item in tree.iter() {
			#[allow(clippy::match_same_arms)]
			match item.kind() {
				TreeElementKind::Leaf => {
					let desc = item.description();
					if !desc.groot2() {
						behaviors.insert(desc.name(), desc.clone());
					}
				}
				TreeElementKind::Node => {
					let desc = item.description();
					if !desc.groot2() {
						behaviors.insert(desc.name(), desc.clone());
					}
				}
				TreeElementKind::SubTree => {
					subtrees.insert(item.path().clone() ,item);
				}
			}
		}

		// create the BehaviorTree's
		for (path, _subtree) in subtrees {
			std::dbg!(path);
		}

		// create the TreeNodesModel
		xml.begin_elem("TreeNodesModel")?;
		// loop over collected behavior entries
		for (name, item) in &behaviors {
			if !item.groot2() {
				xml.begin_elem(item.kind_str())?;
				xml.attr("ID", name)?;
				// look for a PortsList
				for port in &item.ports().0 {
					xml.begin_elem(port.direction().type_str())?;
					xml.attr("name", &port.name())?;
					xml.attr("type", port.type_name())?;
					xml.end_elem()?;
				}
				xml.end_elem()?;
			}
		}

		xml.end_elem()?; // TreeNodesModel

		xml.end_elem()?; // root
		xml.flush()?;

		let raw = xml.into_inner();
		let mut output = String::with_capacity(raw.len());
		for c in raw {
			output.push(c as char);
		}
		Ok(output.into())
	}

	// fn write_subtree(iter: &mut , writer: &mut XmlWriter<'_, Vec<u8>>) -> Result<(), std::io::Error> {

	// 	Ok(())
	// }
}
// endregion:   --- XmlWriter

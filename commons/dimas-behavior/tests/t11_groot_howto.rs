// Copyright © 2025 Stephan Kunz

//! This test implements the eleventh tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_11_groot2)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t11_groot_howto.cpp)
//!

extern crate alloc;
mod cross_door;
mod test_data;

use cross_door::CrossDoor;
use dimas_behavior::{
	Behavior, Groot2Publisher, SharedRuntime, XmlCreator,
	behavior::{BehaviorData, BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
	factory::BehaviorTreeFactory,
	output_port,
	port::PortList,
	port_list, register_behavior,
	tree::BehaviorTreeElementList,
};

use crate::test_data::Position2D;

const CYCLES: u8 = 0;

const XML: &str = r#"
<root BTCPP_format="4">
	<BehaviorTree ID="MainTree">
		<Sequence>
			<Script code="door_open:=false" />
			<UpdatePosition pos="{pos_2D}" />
			<Fallback>
				<Inverter>
					<IsDoorClosed/>
				</Inverter>
				<SubTree ID="DoorClosed" _autoremap="true" door_open="{door_open}"/>
			</Fallback>
			<PassThroughDoor/>
		</Sequence>
	</BehaviorTree>

	<BehaviorTree ID="DoorClosed">
		<Fallback name="tryOpen" _onSuccess="door_open:=true">
			<OpenDoor/>
			<RetryUntilSuccessful num_attempts="5">
				<PickLock/>
			</RetryUntilSuccessful>
			<SmashDoor/>
		</Fallback>
	</BehaviorTree>
</root>
"#;

/// Action `UpdatePosition`
#[derive(Behavior, Debug, Default)]
pub struct UpdatePosition {
	pos: Position2D,
}

#[async_trait::async_trait]
impl BehaviorInstance for UpdatePosition {
	async fn tick(
		&mut self,
		behavior: &mut BehaviorData,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.pos.x += 0.2;
		self.pos.y += 0.1;
		behavior.set("pos", self.pos.clone())?;
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for UpdatePosition {
	fn kind() -> BehaviorKind {
		BehaviorKind::Action
	}

	fn provided_ports() -> PortList {
		port_list![output_port!(Position2D, "pos")]
	}
}

#[tokio::test]
#[ignore = "groot publishing missing"]
async fn groot_howto() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	// Nodes registration, as usual
	CrossDoor::register_behaviors(&mut factory)?;
	register_behavior!(factory, UpdatePosition, "UpdatePosition")?;

	// Groot2 editor requires a model of your registered Nodes.
	// You don't need to write that by hand, it can be automatically
	// generated using the following command.
	let xml_model = XmlCreator::write_tree_nodes_model(&factory, true)?;
	println!("-------- TreeNodesModel --------");
	println!("{xml_model}");
	println!("--------------------------------");

	factory.register_behavior_tree_from_text(XML)?;

	// Add this to allow Groot2 to visualize your custom type
	// @TODO:
	//BT::RegisterJsonDefinition<Position2D>();

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	// Print the full tree with model
	let xml = XmlCreator::write_tree(&tree, true)?;
	println!("----------- XML file  ----------");
	println!("{}", &xml);
	println!("--------------------------------");

	// Connect the Groot2Publisher. This will allow Groot2 to
	// get the tree and poll status updates.
	let port: i16 = 1667;
	let _publisher = Groot2Publisher::new(&tree, port);

	#[allow(clippy::reversed_empty_ranges)]
	for _ in 0..CYCLES {
		tree.reset()?;
		let result = tree.tick_while_running().await?;
		assert_eq!(result, BehaviorState::Success);
	}

	assert_eq!(RESULT, xml.as_ref());
	Ok(())
}

const RESULT: &str = r#"<root BTCPP_format="4">
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
	<TreeNodesModel>
		<Condition ID="IsDoorClosed"/>
		<Action ID="OpenDoor"/>
		<Action ID="PassThroughDoor"/>
		<Action ID="PickLock"/>
		<Condition ID="SmashDoor"/>
		<Action ID="UpdatePosition">
			<output_port name="pos" type="Position2D"/>
		</Action>
	</TreeNodesModel>
</root>"#;

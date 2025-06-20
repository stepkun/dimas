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
	blackboard::{BlackboardInterface, SharedBlackboard},
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
		_behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		self.pos.x += 0.2;
		self.pos.y += 0.1;
		blackboard.set("pos", self.pos.clone())?;
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
	let _xml_model = XmlCreator::write_tree_nodes_model(&factory)?;
	// println!("-------- TreeNodesModel --------\n");
	// println!("{xml_model}");
	// println!("--------------------------------\n");

	factory.register_behavior_tree_from_text(XML)?;

	// Add this to allow Groot2 to visualize your custom type
	// @TODO:
	//BT::RegisterJsonDefinition<Position2D>();

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	// Print the full tree with model
	let xml = XmlCreator::write_tree(&tree)?;
	println!("----------- XML file  ----------\n");
	println!("{xml}");
	println!("--------------------------------\n");

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

	Ok(())
}

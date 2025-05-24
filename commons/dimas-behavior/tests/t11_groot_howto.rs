// Copyright © 2025 Stephan Kunz

//! This test implements the eleventh tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_11_groot2)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t11_groot_howto.cpp)
//!

extern crate alloc;

use std::{fmt::Display, num::ParseFloatError, str::FromStr};

use cross_door::cross_door::CrossDoor;
use dimas_behavior::{behavior::{BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData, BehaviorType}, blackboard::{BlackboardInterface, SharedBlackboard}, factory::BehaviorTreeFactory, output_port, port::PortList, port_list, tree::BehaviorTreeComponentList, Behavior};

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

impl BehaviorInstance for UpdatePosition {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		self.pos.x += 0.2;
		self.pos.y += 0.1;
		blackboard.set("pos".into(), self.pos.clone())?;
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for UpdatePosition {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![output_port!(Position2D, "pos")]
	}
}

/// `Position2D`
#[derive(Clone, Debug, Default)]
struct Position2D {
	x: f64,
	y: f64,
}

impl FromStr for Position2D {
	type Err = ParseFloatError;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		println!("Converting string: \"{value}\"");
		// remove redundant ' and &apos; from string
		let s = value
			.replace('\'', "")
			.trim()
			.replace("&apos;", "")
			.trim()
			.to_string();
		let v: Vec<&str> = s.split(';').collect();
		let x = f64::from_str(v[0])?;
		let y = f64::from_str(v[1])?;
		Ok(Self { x, y })
	}
}

impl Display for Position2D {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}, {}]", self.x, self.y)
	}
}


#[tokio::test]
#[ignore]
async fn groot_howto() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	let cross_door = CrossDoor::default();
	cross_door.register_nodes(&mut factory)?;
	factory.register_node_type::<UpdatePosition>("UpdatePosition")?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	Ok(())
}

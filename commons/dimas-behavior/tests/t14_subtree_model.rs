// Copyright © 2025 Stephan Kunz

//! This test implements the fourteenth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_14_subtree_model)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t14_subtree_model.cpp)
//!

extern crate alloc;
mod test_data;

use std::fmt::{Display, Formatter};

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::{BlackboardInterface, SharedBlackboard},
	factory::BehaviorTreeFactory,
	input_port,
	port::PortList,
	port_list, register_behavior,
	tree::BehaviorTreeElementList,
};
use test_data::SaySomething;

const XML: &str = r#"
<root BTCPP_format="4">
  	<BehaviorTree ID="MainTree">
        <Sequence>
            <Script code="target:='1;2;3'"/>
            <SubTree ID="MoveRobot"
                _autoremap="true"
                frame="world"/>
            <SaySomething message="{result}"/>
        </Sequence>
  	</BehaviorTree>

    <BehaviorTree ID="MoveRobot">
        <Fallback>
            <Sequence>
                <MoveBase goal="{target}"/>
                <Script code="result:=&apos;goal_reached&apos;"/>
            </Sequence>
            <ForceFailure>
                <Script code="result:=&apos;error&apos;"/>
            </ForceFailure>
        </Fallback>
    </BehaviorTree>
</root>
"#;

#[tokio::test]
async fn subtree_model() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_behavior!(factory, SaySomething, "SaySomething")?;
	// register subtrees nodes
	move_robot::register_behaviors(&mut factory)?;

	factory.register_behavior_tree_from_text(XML)?;

	let mut tree = factory.create_tree("MainTree")?;
	drop(factory);

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorState::Success);

	Ok(())
}

/// Implementation of `MoveRobot` tree
mod move_robot {
	use std::{num::ParseFloatError, str::FromStr};

	use dimas_behavior::{behavior::BehaviorData, factory::error::Error};

	use super::*;

	#[derive(Clone, Copy, Debug)]
	pub struct Position2D {
		x: f64,
		y: f64,
		theta: f64,
	}

	impl FromStr for Position2D {
		type Err = ParseFloatError;

		fn from_str(value: &str) -> Result<Self, Self::Err> {
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
			let theta = f64::from_str(v[2])?;
			Ok(Self { x, y, theta })
		}
	}

	impl Display for Position2D {
		fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
			todo!()
		}
	}

	/// Behavior `MoveBase`
	#[derive(Behavior, Debug, Default)]
	struct MoveBase {
		counter: usize,
	}

	#[async_trait::async_trait]
	impl BehaviorInstance for MoveBase {
		async fn start(
			&mut self,
			_behavior: &mut BehaviorData,
			blackboard: &mut SharedBlackboard,
			_children: &mut BehaviorTreeElementList,
			_runtime: &SharedRuntime,
		) -> BehaviorResult {
			let pos = blackboard.get::<Position2D>("goal".into())?;

			println!(
				"[ MoveBase: SEND REQUEST ]. goal: x={:2.1} y={:2.1} theta={:2.1}",
				pos.x, pos.y, pos.theta
			);

			Ok(BehaviorState::Running)
		}

		async fn tick(
			&mut self,
			_behavior: &mut BehaviorData,
			_blackboard: &mut SharedBlackboard,
			_children: &mut BehaviorTreeElementList,
			_runtime: &SharedRuntime,
		) -> BehaviorResult {
			if self.counter < 5 {
				self.counter += 1;
				println!("--- status: RUNNING");
				Ok(BehaviorState::Running)
			} else {
				println!("[ MoveBase: FINISHED ]");
				Ok(BehaviorState::Success)
			}
		}
	}

	impl BehaviorStatic for MoveBase {
		fn kind() -> BehaviorType {
			BehaviorType::Action
		}

		fn provided_ports() -> PortList {
			port_list!(input_port!(Position2D, "goal"),)
		}
	}

	pub fn register_behaviors(factory: &mut BehaviorTreeFactory) -> Result<(), Error> {
		register_behavior!(factory, MoveBase, "MoveBase")
	}
}

// Copyright © 2025 Stephan Kunz

//! This test implements the fourteenth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_14_subtree_model)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t14_subtree_model.cpp)
//!

extern crate alloc;
mod test_data;

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorKind, BehaviorResult, BehaviorState, BehaviorStatic},
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
	let mut factory = BehaviorTreeFactory::with_groot2_behaviors()?;

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
	use dimas_behavior::{behavior::BehaviorData, factory::error::Error};

	use crate::test_data::Pose2D;

	use super::*;

	/// Behavior `MoveBase`
	#[derive(Behavior, Debug, Default)]
	struct MoveBase {
		counter: usize,
	}

	#[async_trait::async_trait]
	impl BehaviorInstance for MoveBase {
		async fn start(
			&mut self,
			behavior: &mut BehaviorData,
			_children: &mut BehaviorTreeElementList,
			_runtime: &SharedRuntime,
		) -> BehaviorResult {
			let pos = behavior.get::<Pose2D>("goal")?;

			println!(
				"[ MoveBase: SEND REQUEST ]. goal: x={:2.1} y={:2.1} theta={:2.1}",
				pos.x, pos.y, pos.theta
			);

			Ok(BehaviorState::Running)
		}

		async fn tick(
			&mut self,
			_behavior: &mut BehaviorData,
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
		fn kind() -> BehaviorKind {
			BehaviorKind::Action
		}

		fn provided_ports() -> PortList {
			port_list!(input_port!(Pose2D, "goal"),)
		}
	}

	pub fn register_behaviors(factory: &mut BehaviorTreeFactory) -> Result<(), Error> {
		register_behavior!(factory, MoveBase, "MoveBase")
	}
}

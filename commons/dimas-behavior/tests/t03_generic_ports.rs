// Copyright © 2025 Stephan Kunz

//! This test implements the third tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_03_generic_ports)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t03_generic_ports.cpp)
//!

use alloc::str::FromStr;
use core::num::ParseFloatError;

use dimas_behavior::{
	factory::NewBehaviorTreeFactory,
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType,
	},
	new_port::{NewPortList, input_port, output_port},
	tree::BehaviorTreeComponent,
};
use dimas_behavior_derive::Behavior;

#[doc(hidden)]
extern crate alloc;

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

/// Behavior `CalculateGoal`
#[derive(Behavior, Debug)]
pub struct CalculateGoal {}

impl BehaviorCreationMethods for CalculateGoal {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorInstanceMethods for CalculateGoal {
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let mygoal = Position2D { x: 1.1, y: 2.3 };
		tree_node
			.tick_data
			.lock()
			.set_output("goal", mygoal)?;
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for CalculateGoal {
	fn provided_ports() -> NewPortList {
		vec![output_port::<Position2D>("goal", "", "").expect("snh")]
	}
}

/// Behavior `PrintTarget`
#[derive(Behavior, Debug)]
pub struct PrintTarget {}

impl BehaviorCreationMethods for PrintTarget {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorInstanceMethods for PrintTarget {
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let pos = tree_node
			.tick_data
			.lock()
			.get_input::<Position2D>("target")?;
		println!("Target positions: [ {}, {} ]", pos.x, pos.y);
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for PrintTarget {
	fn provided_ports() -> NewPortList {
		vec![input_port::<String>("target", "", "").expect("snh")]
	}
}

const XML: &str = r#"
<root BTCPP_format="4"
		main_tree_to_execute="MainTree">
	<BehaviorTree ID="MainTree">
		<Sequence name="root">
            <CalculateGoal   goal="{GoalPosition}" />
            <PrintTarget     target="{GoalPosition}" />
            <Script          code="OtherGoal:='-1;3'" />
            <PrintTarget     target="{OtherGoal}" />
		</Sequence>
	</BehaviorTree>
</root>
"#;

#[tokio::test]
async fn generic_ports() -> anyhow::Result<()> {
	let mut factory = NewBehaviorTreeFactory::with_core_behaviors()?;

	factory.register_node_type::<CalculateGoal>("CalculateGoal")?;
	factory.register_node_type::<PrintTarget>("PrintTarget")?;

	let mut tree = factory.create_from_text(XML)?;

	let result = tree.tick_while_running().await?;
	assert_eq!(result, NewBehaviorStatus::Success);
	Ok(())
}

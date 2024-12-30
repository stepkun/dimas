// Copyright © 2024 Stephan Kunz

//! This test implements the fourth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_04_sequence)
//!
//! Differences to BehaviorTree.CPP
//! - there is no `tree::sleep(...)` available, using sleep of async runtime instead,
//!   which is not interrupted, when tree state changes
//!

#[doc(hidden)]
extern crate alloc;

#[cfg(feature = "std")]
extern crate tokio;

use core::{num::ParseFloatError, str::FromStr, time::Duration};

use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{BehaviorResult, BehaviorStatus},
	blackboard::FromString,
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action, register_condition};

const XML: &str = r#"
<root BTCPP_format="4"
        main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
		<Sequence>
			<BatteryOK/>
			<SaySomething   message="mission started..." />
			<MoveBase          goal="1;2;3"/>
			<SaySomething   message="mission completed!" />
		</Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
		<Condition ID="BatteryOK"
				editable="true"/>
        <Action ID="SaySomething"
                editable="true">
            <input_port name="message"/>
        </Action>
        <Action ID="MoveBase"
                editable="true">
            <input_port name="goal"/>
        </Action>
    </TreeNodesModel>
</root>
"#;

#[derive(Clone, Copy, Debug)]
struct Pose2D {
	x: f64,
	y: f64,
	theta: f64,
}

impl FromString for Pose2D {
	type Err = ParseFloatError;

	fn from_string(value: impl AsRef<str>) -> Result<Self, Self::Err> {
		value.as_ref().parse()
	}
}

impl FromStr for Pose2D {
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
		let x = f64::from_string(v[0])?;
		let y = f64::from_string(v[1])?;
		let theta = f64::from_string(v[2])?;
		Ok(Self { x, y, theta })
	}
}

/// Condition "BatteryOK"
#[behavior(SyncCondition)]
struct BatteryOK {}

#[behavior(SyncCondition)]
impl BatteryOK {
	async fn tick(&mut self) -> BehaviorResult {
		println!("battery is ok");

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "SaySomething"
#[behavior(SyncAction)]
struct SaySomething {}

#[behavior(SyncAction)]
impl SaySomething {
	fn ports() -> PortList {
		define_ports!(input_port!("message", "hello"))
	}
	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config.get_input("message")?;

		println!("Robot says: {msg}");

		Ok(BehaviorStatus::Success)
	}
}

/// Action "MoveBase"
#[behavior(Action)]
struct MoveBase {
	#[bhvr(default)]
	counter: usize,
}

#[behavior(Action)]
impl MoveBase {
	fn ports() -> PortList {
		define_ports!(input_port!("goal"))
	}

	async fn on_start(&mut self) -> BehaviorResult {
		let pos = bhvr_.config.get_input::<Pose2D>("goal")?;

		println!(
			"[ MoveBase: SEND REQUEST ]. goal: x={:2.1} y={:2.1} theta={:2.1}",
			pos.x, pos.y, pos.theta
		);

		Ok(BehaviorStatus::Running)
	}

	async fn on_running(&mut self) -> BehaviorResult {
		if self.counter < 5 {
			self.counter += 1;
			println!("--- status: RUNNING");
			Ok(BehaviorStatus::Running)
		} else {
			println!("[ MoveBase: FINISHED ]");
			Ok(BehaviorStatus::Success)
		}
	}
}

#[tokio::test]
async fn main() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_condition!(factory, "BatteryOK", BatteryOK);
	register_action!(factory, "SaySomething", SaySomething);
	register_action!(factory, "MoveBase", MoveBase);

	// create the BT
	let mut tree = factory.create_tree(XML)?;

	// run the BT using own loop with sleep to avoid busy loop
	println!("--- ticking");
	let mut result = tree.tick_once().await?;
	while result == BehaviorStatus::Running {
		tokio::time::sleep(Duration::from_millis(100)).await;
		println!("--- ticking");
		result = tree.tick_once().await?;
	}

	println!("tree result is {result}");

	Ok(())
}

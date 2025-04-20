// Copyright © 2025 Stephan Kunz

//! Test behaviors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::str::FromStr;
use core::num::ParseFloatError;
use std::time::{Duration, Instant};

use dimas_behavior::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTreeMethods,
		NewBehaviorStatus, NewBehaviorType,
	},
	new_port::{NewPortList, input_port, output_port},
	tree::BehaviorTreeComponent,
};
use dimas_behavior_derive::Behavior;
// endregion:	--- modules

/// Behavior `ApproachObject`
/// Example of custom `SyncActionNode` (synchronous action) without ports.
#[derive(Behavior, Debug, Default)]
pub struct ApproachObject {}

impl BehaviorCreationMethods for ApproachObject {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorInstanceMethods for ApproachObject {
	fn tick(&mut self, _tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		println!("ApproachObject: approach_object");
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for ApproachObject {}

/// Function for behavior `CheckBattery`
/// # Errors
/// In this case never :-)
pub fn check_battery() -> BehaviorResult {
	println!("[ Battery: OK ]");
	Ok(NewBehaviorStatus::Success)
}

/// Struct for behaviors `OpenGripper` and `CloseGripper`
#[derive(Default)]
pub struct GripperInterface {}

impl GripperInterface {
	/// Open the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn open(&mut self) -> BehaviorResult {
		println!("GripperInterface::open");
		Ok(NewBehaviorStatus::Success)
	}
	/// Close the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn close(&mut self) -> BehaviorResult {
		println!("GripperInterface::close");
		Ok(NewBehaviorStatus::Success)
	}
}
/// Behavior `SaySomething`
/// Example of custom `SyncActionNode` (synchronous action) with an input port.
#[derive(Behavior, Debug, Default)]
pub struct SaySomething {}

impl BehaviorCreationMethods for SaySomething {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorInstanceMethods for SaySomething {
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		let msg = tree_node
			.tick_data
			.get_input::<String>("message")?;
		println!("Robot says: {msg}");
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for SaySomething {
	fn provided_ports() -> NewPortList {
		vec![input_port::<String>("message", "hello", "").expect("snh")]
	}
}

/// Behavior `ThinkWhatToSay`
#[derive(Behavior, Debug, Default)]
pub struct ThinkWhatToSay {}

impl BehaviorCreationMethods for ThinkWhatToSay {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self {}))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorInstanceMethods for ThinkWhatToSay {
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		tree_node
			.tick_data
			.set_output("text", "The answer is 42")?;
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for ThinkWhatToSay {
	fn provided_ports() -> NewPortList {
		vec![output_port::<String>("text", "", "").expect("snh")]
	}
}

/// Same as struct `SaySomething`, but to be registered with `SimpleBehavior`
/// # Errors
pub fn say_something_simple(tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
	let msg = tree_node
		.tick_data
		.get_input::<String>("message")?;
	println!("Robot says: {msg}");
	Ok(NewBehaviorStatus::Success)
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

/// Behavior `CalculateGoal`
#[derive(Behavior, Debug, Default)]
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
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		let mygoal = Position2D { x: 1.1, y: 2.3 };
		tree_node.tick_data.set_output("goal", mygoal)?;
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for CalculateGoal {
	fn provided_ports() -> NewPortList {
		vec![output_port::<Position2D>("goal", "", "").expect("snh")]
	}
}

/// Behavior `PrintTarget`
#[derive(Behavior, Debug, Default)]
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
	fn tick(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		let pos = tree_node
			.tick_data
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

/// `Position2D`
#[derive(Clone, Debug, Default)]
struct Pose2D {
	x: f64,
	y: f64,
	theta: f64,
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
		let x = f64::from_str(v[0])?;
		let y = f64::from_str(v[1])?;
		let theta = f64::from_str(v[2])?;
		Ok(Self { x, y, theta })
	}
}

/// Behavior `MoveBase`
#[derive(Behavior, Debug)]
pub struct MoveBaseAction {
	start_time: Instant,
	completion_time: Duration,
}

impl Default for MoveBaseAction {
	fn default() -> Self {
		Self {
			start_time: Instant::now(),
			completion_time: Duration::default(),
		}
	}
}

impl BehaviorCreationMethods for MoveBaseAction {
	fn create() -> Box<BehaviorCreationFn> {
		Box::new(|| Box::new(Self::default()))
	}

	fn kind() -> NewBehaviorType {
		NewBehaviorType::Action
	}
}

impl BehaviorInstanceMethods for MoveBaseAction {
	fn start(&mut self, tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		let pose = tree_node.tick_data.get_input::<Pose2D>("goal")?;
		println!(
			"[ MoveBase: SEND REQUEST ]. goal: x={} y={} theta={}",
			pose.x, pose.y, pose.theta
		);
		self.start_time = Instant::now();
		self.completion_time = Duration::from_millis(220);
		Ok(NewBehaviorStatus::Running)
	}

	fn tick(&mut self, _tree_node: &mut BehaviorTreeComponent) -> BehaviorResult {
		if Instant::now().duration_since(self.start_time) >= self.completion_time {
			println!("[ MoveBase: FINISHED ]");
			return Ok(NewBehaviorStatus::Success);
		}

		Ok(NewBehaviorStatus::Running)
	}
}

impl BehaviorStaticMethods for MoveBaseAction {
	fn provided_ports() -> NewPortList {
		vec![input_port::<Pose2D>("goal", "", "").expect("snh")]
	}
}

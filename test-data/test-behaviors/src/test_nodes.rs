// Copyright © 2025 Stephan Kunz

//! Test behaviors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use alloc::str::FromStr;
use core::num::ParseFloatError;
use std::{
	fmt::Display,
	time::{Duration, Instant},
};

use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port, output_port,
	port::PortList,
	port_list,
	tree::BehaviorTreeElementList,
};
// endregion:	--- modules

/// Behavior `ApproachObject`
/// Example of custom `ActionNode` (synchronous action) without ports.
#[derive(Behavior, Debug, Default)]
pub struct ApproachObject {}

#[async_trait::async_trait]
impl BehaviorInstance for ApproachObject {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("ApproachObject: approach_object");
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for ApproachObject {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}

/// Function for behavior `CheckBattery`
/// # Errors
/// In this case never :-)
pub fn check_battery() -> BehaviorResult {
	println!("[ Battery: OK ]");
	Ok(BehaviorState::Success)
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
		Ok(BehaviorState::Success)
	}
	/// Close the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn close(&mut self) -> BehaviorResult {
		println!("GripperInterface::close");
		Ok(BehaviorState::Success)
	}
}
/// Behavior `SaySomething`
/// Example of custom `ActionNode` (synchronous action) with an input port.
#[derive(Behavior, Debug, Default)]
pub struct SaySomething {}

#[async_trait::async_trait]
impl BehaviorInstance for SaySomething {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let msg = blackboard.get::<String>("message".into())?;
		println!("Robot says: {msg}");
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for SaySomething {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list! {input_port!(String, "message", "hello")}
	}
}

/// Behavior `ThinkWhatToSay`
#[derive(Behavior, Debug, Default)]
pub struct ThinkWhatToSay {}

#[async_trait::async_trait]
impl BehaviorInstance for ThinkWhatToSay {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		blackboard.set("text".into(), String::from("The answer is 42"))?;
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for ThinkWhatToSay {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![output_port!(String, "text")]
	}
}

/// Same as struct `SaySomething`, but to be registered with `SimpleBehavior`
/// # Errors
pub fn new_say_something_simple(blackboard: &mut SharedBlackboard) -> BehaviorResult {
	let msg = blackboard.get::<String>("message".into())?;
	println!("Robot2 says: {msg}");
	Ok(BehaviorState::Success)
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

/// Behavior `CalculateGoal`
#[derive(Behavior, Debug, Default)]
pub struct CalculateGoal {}

#[async_trait::async_trait]
impl BehaviorInstance for CalculateGoal {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let mygoal = Position2D { x: 1.1, y: 2.3 };
		blackboard.set("goal".into(), mygoal)?;
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for CalculateGoal {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![output_port!(Position2D, "goal")]
	}
}

/// Behavior `PrintTarget`
#[derive(Behavior, Debug, Default)]
pub struct PrintTarget {}

#[async_trait::async_trait]
impl BehaviorInstance for PrintTarget {
	async fn tick(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let pos = blackboard.get::<Position2D>("target".into())?;
		println!("Target positions: [ {}, {} ]", pos.x, pos.y);
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for PrintTarget {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(Position2D, "target")]
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

impl Display for Pose2D {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
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

#[async_trait::async_trait]
impl BehaviorInstance for MoveBaseAction {
	async fn start(
		&mut self,
		_state: BehaviorState,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		let pose = blackboard.get::<Pose2D>("goal".into())?;
		println!(
			"[ MoveBase: SEND REQUEST ]. goal: x={} y={} theta={}",
			pose.x, pose.y, pose.theta
		);
		self.start_time = Instant::now();
		self.completion_time = Duration::from_millis(220);
		Ok(BehaviorState::Running)
	}

	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		if Instant::now().duration_since(self.start_time) >= self.completion_time {
			println!("[ MoveBase: FINISHED ]");
			return Ok(BehaviorState::Success);
		}

		Ok(BehaviorState::Running)
	}
}

impl BehaviorStatic for MoveBaseAction {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port!(Pose2D, "goal")]
	}
}

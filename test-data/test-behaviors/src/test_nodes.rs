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
	behavior::{
		BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus, BehaviorTickData,
		BehaviorType,
	},
	blackboard::{BlackboardInterface, SharedBlackboard},
	input_port_macro, output_port_macro,
	port::PortList,
	port_list,
	tree::BehaviorTreeComponentList,
};
use dimas_behavior_derive::Behavior;
// endregion:	--- modules

/// Behavior `ApproachObject`
/// Example of custom `SyncActionNode` (synchronous action) without ports.
#[derive(Behavior, Debug, Default)]
pub struct ApproachObject {}

impl BehaviorInstance for ApproachObject {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		println!("ApproachObject: approach_object");
		Ok(BehaviorStatus::Success)
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
	Ok(BehaviorStatus::Success)
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
		Ok(BehaviorStatus::Success)
	}
	/// Close the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn close(&mut self) -> BehaviorResult {
		println!("GripperInterface::close");
		Ok(BehaviorStatus::Success)
	}
}
/// Behavior `SaySomething`
/// Example of custom `SyncActionNode` (synchronous action) with an input port.
#[derive(Behavior, Debug, Default)]
pub struct SaySomething {}

impl BehaviorInstance for SaySomething {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let msg = blackboard.get::<String>("message".into())?;
		println!("Robot says: {msg}");
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for SaySomething {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list! {input_port_macro!(String, "message", "hello")}
	}
}

/// Behavior `ThinkWhatToSay`
#[derive(Behavior, Debug, Default)]
pub struct ThinkWhatToSay {}

impl BehaviorInstance for ThinkWhatToSay {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		blackboard.set("text".into(), String::from("The answer is 42"))?;
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for ThinkWhatToSay {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![output_port_macro!(String, "text")]
	}
}

/// Same as struct `SaySomething`, but to be registered with `SimpleBehavior`
/// # Errors
pub fn new_say_something_simple(blackboard: &mut SharedBlackboard) -> BehaviorResult {
	let msg = blackboard.get::<String>("message".into())?;
	println!("Robot2 says: {msg}");
	Ok(BehaviorStatus::Success)
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
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

/// Behavior `CalculateGoal`
#[derive(Behavior, Debug, Default)]
pub struct CalculateGoal {}

impl BehaviorInstance for CalculateGoal {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let mygoal = Position2D { x: 1.1, y: 2.3 };
		blackboard.set("goal".into(), mygoal)?;
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for CalculateGoal {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![output_port_macro!(Position2D, "goal")]
	}
}

/// Behavior `PrintTarget`
#[derive(Behavior, Debug, Default)]
pub struct PrintTarget {}

impl BehaviorInstance for PrintTarget {
	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let pos = blackboard.get::<Position2D>("target".into())?;
		println!("Target positions: [ {}, {} ]", pos.x, pos.y);
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for PrintTarget {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port_macro!(Position2D, "target")]
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

impl BehaviorInstance for MoveBaseAction {
	fn start(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		let pose = blackboard.get::<Pose2D>("goal".into())?;
		println!(
			"[ MoveBase: SEND REQUEST ]. goal: x={} y={} theta={}",
			pose.x, pose.y, pose.theta
		);
		self.start_time = Instant::now();
		self.completion_time = Duration::from_millis(220);
		Ok(BehaviorStatus::Running)
	}

	fn tick(
		&mut self,
		_tick_data: &mut BehaviorTickData,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeComponentList,
	) -> BehaviorResult {
		if Instant::now().duration_since(self.start_time) >= self.completion_time {
			println!("[ MoveBase: FINISHED ]");
			return Ok(BehaviorStatus::Success);
		}

		Ok(BehaviorStatus::Running)
	}
}

impl BehaviorStatic for MoveBaseAction {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}

	fn provided_ports() -> PortList {
		port_list![input_port_macro!(Pose2D, "goal")]
	}
}

// Copyright © 2025 Stephan Kunz

//! Test behaviors
//!

#[doc(hidden)]
extern crate alloc;

// region:		--- modules
use dimas_behavior::{
	new_behavior::{
		BehaviorAllMethods, BehaviorCreationFn, BehaviorCreationMethods, BehaviorInstanceMethods,
		BehaviorRedirectionMethods, BehaviorResult, BehaviorStaticMethods, BehaviorTickData,
		BehaviorTreeMethods, NewBehaviorStatus, NewBehaviorType, SimpleBehavior,
	},
	new_port::{NewPortList, add_to_port_list, input_port, output_port},
	tree::BehaviorTreeComponent,
};
use dimas_behavior_derive::Behavior;
//  endregion:	--- modules

/// Behavior `ApproachObject`
/// Example of custom `SyncActionNode` (synchronous action) without ports.
#[derive(Behavior, Debug)]
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
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
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
	pub fn open(&self) -> BehaviorResult {
		println!("GripperInterface::open");
		Ok(NewBehaviorStatus::Success)
	}
	/// Close the gripper.
	/// # Errors
	/// In this case never :-)
	pub fn close(&self) -> BehaviorResult {
		println!("GripperInterface::close");
		Ok(NewBehaviorStatus::Success)
	}
}
/// Behavior `SaySomething`
/// Example of custom `SyncActionNode` (synchronous action) with an input port.
#[derive(Behavior, Debug)]
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
	fn tick(&mut self, tree_node: &BehaviorTreeComponent) -> BehaviorResult {
		let msg = tree_node
			.tick_data
			.lock()
			.get_input::<String>("message")?;
		println!("Robot says: {msg}");
		Ok(NewBehaviorStatus::Success)
	}
}

impl BehaviorStaticMethods for SaySomething {
	// 	define_ports!(input_port!("message", "hello"))
	fn provided_ports() -> NewPortList {
		// @TODO: list creation with variadic elements via macro
		let mut list = NewPortList::default();
		// @TODO: variadic attributes via macro
		let entry = input_port::<String>("message", "hello", "").expect("snh");
		match add_to_port_list(&mut list, entry) {
			Ok(entry) => {}
			Err(err) => panic!("{err}"),
		}
		list
	}
}

/// Same as struct `SaySomething`, but to be registered with `SimpleBehavior`
/// # Errors
pub fn say_something_simple(/* behavior: &SimpleBehavior, tree_node: &BehaviorTreeComponent */) -> BehaviorResult {
	println!("Robot says: this works not yet");
	Ok(NewBehaviorStatus::Success)
}

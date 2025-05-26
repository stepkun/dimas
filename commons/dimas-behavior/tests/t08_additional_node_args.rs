// Copyright © 2025 Stephan Kunz

//! This test implements the eigth tutorial/example from [BehaviorTree.CPP](https://www.behaviortree.dev)
//!
//! [tutorial:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_08_additional_args)
//! [cpp-source:](https://github.com/BehaviorTree/BehaviorTree.CPP/blob/master/examples/t08_additional_node_args.cpp)
//!

extern crate alloc;

use dimas_behavior::{
	Behavior,
	behavior::{
		BehaviorExecution, BehaviorInstance, BehaviorResult, BehaviorStatic, BehaviorStatus,
		BehaviorType,
	},
	blackboard::SharedBlackboard,
	factory::BehaviorTreeFactory,
	register_node,
	tree::BehaviorTreeElementList,
};

const XML: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <Action_A/>
            <Action_B/>
        </Sequence>
    </BehaviorTree>
</root>
"#;

/// Behavior `ActionA` has a different constructor than the default one.
#[derive(Behavior, Debug, Default)]
pub struct ActionA {
	arg1: i32,
	arg2: String,
}

impl BehaviorInstance for ActionA {
	fn tick(
		&mut self,
		_status: BehaviorStatus,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		assert_eq!(self.arg1, 42);

		assert_eq!(self.arg2, String::from("hello world"));
		println!("{}: {}, {}", String::from("?"), &self.arg1, &self.arg2);
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for ActionA {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}

impl ActionA {
	/// Constructor with arguments.
	#[must_use]
	pub const fn new(arg1: i32, arg2: String) -> Self {
		Self { arg1, arg2 }
	}
}

/// Behavior `ActionB` implements an initialize(...) method that must be called once at the beginning.
#[derive(Behavior, Debug, Default)]
pub struct ActionB {
	arg1: i32,
	arg2: String,
}

impl BehaviorInstance for ActionB {
	fn tick(
		&mut self,
		_status: BehaviorStatus,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
	) -> BehaviorResult {
		assert_eq!(self.arg1, 69);
		assert_eq!(self.arg2, String::from("interesting value"));
		println!("{}: {}, {}", String::from("?"), &self.arg1, &self.arg2);
		Ok(BehaviorStatus::Success)
	}
}

impl BehaviorStatic for ActionB {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}

impl ActionB {
	/// Initialization function.
	pub fn initialize(&mut self, arg1: i32, arg2: String) {
		self.arg1 = arg1;
		self.arg2 = arg2;
	}
}

#[tokio::test]
async fn additional_args() -> anyhow::Result<()> {
	let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

	register_node!(&mut factory, ActionA, "Action_A", 42, "hello world".into())?;
	factory.register_node_type::<ActionB>("Action_B")?;

	let mut tree = factory.create_from_text(XML)?;
	drop(factory);

	// initialize ActionB with the help of an iterator
	for node in tree.iter_mut() {
		if node.name() == ("Action_B") {
			let action = node
				.behavior_mut()
				.as_any_mut()
				.downcast_mut::<ActionB>()
				.expect("snh");
			action.initialize(69, "interesting value".into());
		}
	}

	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);

	// test the iterator
	let mut iter = tree.iter();
	assert_eq!(iter.next().expect("snh").name(), "MainTree");
	assert_eq!(iter.next().expect("snh").name(), "Sequence");
	assert_eq!(iter.next().expect("snh").name(), "Action_A");
	assert_eq!(iter.next().expect("snh").name(), "Action_B");
	assert!(iter.next().is_none());

	Ok(())
}

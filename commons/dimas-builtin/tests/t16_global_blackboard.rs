// Copyright © 2024 Stephan Kunz

//! This test implements the sixteenth tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-advanced/tutorial_16_global_blackboard)
//!

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	behavior::{BehaviorResult, BehaviorStatus},
	blackboard::{Blackboard, error::Error},
	define_ports, input_port,
	port::PortList,
};
use dimas_builtin::factory::BTFactory;
use dimas_macros::{behavior, register_action};

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <PrintNumber name="main_print" val="{@value}" />
            <SubTree ID="MySub"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="MySub">
        <Sequence>
            <PrintNumber name="sub_print" val="{@value}" />
            <Script code="@value_sqr := @value * @value" />
            <SubTree ID="MySubSub"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="MySubSub">
        <Sequence>
            <PrintNumber name="sub_sub_print" val="{@value}" />
            <Script code="@value_pow3 := @value * @value * @value" />
            <SubTree ID="MySubSubSub"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID="MySubSubSub">
        <Sequence>
            <PrintNumber name="sub_sub_sub_print" val="{@value}" />
            <Script code="@value_pow4 := @value * @value * @value * @value" />
        </Sequence>
    </BehaviorTree>
</root>"#;

/// ActionNode "PrintNumber"
#[behavior(SyncAction)]
struct PrintNumber {}

#[behavior(SyncAction)]
impl PrintNumber {
	async fn tick(&mut self) -> BehaviorResult {
		let value: i64 = bhvr_.config_mut().get_input("val")?;
		println!("PrintNumber [{}] has val: {value}", bhvr_.name());

		Ok(BehaviorStatus::Success)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("val"))
	}
}

#[tokio::test]
async fn global_blackboard() -> anyhow::Result<()> {
	// create an external blackboard which will survive the tree
	let global_blackboard = Blackboard::default();
	// BT-Trees blackboard has global blackboard as parent
	let blackboard = Blackboard::new(&global_blackboard);

	// create BT environment
	let mut factory = BTFactory::with_blackboard(blackboard);
	factory.add_extensions();

	// register all needed nodes
	register_action!(factory, "PrintNumber", PrintNumber);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;
	//dbg!(&tree);

	// direct interaction with the global blackboard
	for value in 1..=3 {
		global_blackboard.set("value", value);
		let result = tree.tick_once().await?;
		assert_eq!(result, BehaviorStatus::Success);

		let value_sqr = global_blackboard
			.get::<i64>("@value_sqr")
			.ok_or_else(|| Error::PortError("value_sqr".into()))?;
		let value_pow3 = global_blackboard
			.get::<i64>("@value_pow3")
			.ok_or_else(|| Error::PortError("value_pow3".into()))?;
		let value_pow4 = global_blackboard
			.get::<i64>("@value_pow4")
			.ok_or_else(|| Error::PortError("value_pow3".into()))?;

		assert_eq!(value_sqr, value * value);
		assert_eq!(value_pow3, value * value * value);
		assert_eq!(value_pow4, value * value * value * value);
	}

	Ok(())
}

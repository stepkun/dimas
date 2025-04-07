// Copyright © 2024 Stephan Kunz

//! This test implements the second tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_02_basic_ports)
//!

#[doc(hidden)]
extern crate alloc;

use dimas_behavior::{
	behavior::{BehaviorResult, BehaviorStatus},
	define_ports, inout_port, input_port, output_port,
	port::PortList,
};
use dimas_builtin::factory::BTFactory;
use dimas_macros::{behavior, register_action};

const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence  name="root_sequence">
            <SaySomething message="Hello."/>
            <ThinkWhatToSay text="{the_answer}"/>
            <SaySomething message="{the_answer}"/>
            <RefineWhatToSay text="{the_answer}"/>
            <SaySomething message="{the_answer}"/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
        <Action ID="SaySomething"
                editable="true">
            <input_port name="message"
                default="Hello"/>
        </Action>
        <Action ID="ThinkWhatToSay"
                editable="true">
            <output_port name="text"
                default="Nothing to say"/>
        </Action>
        <Action ID="RefineWhatToSay"
                editable="true">
            <inout_port name="text"/>
        </Action>
    </TreeNodesModel>
</root>
"#;

/// SyncAction "SaySomething"
#[behavior(SyncAction)]
struct SaySomething {}

#[behavior(SyncAction)]
impl SaySomething {
	fn ports() -> PortList {
		define_ports!(input_port!("message", "hello"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		let msg = bhvr_
			.config_mut()
			.get_input::<String>("message")?;

		println!("Robot says: {msg}");

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "ThinkWhatToSay"
#[behavior(SyncAction)]
struct ThinkWhatToSay {}

#[behavior(SyncAction)]
impl ThinkWhatToSay {
	fn ports() -> PortList {
		define_ports!(output_port!("text"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		bhvr_
			.config_mut()
			.set_output("text", "The answer is 42.")?;

		println!("Robot has thought");

		Ok(BehaviorStatus::Success)
	}
}

/// SyncAction "RefineWhatToSay"
#[behavior(SyncAction)]
struct RefineWhatToSay {}

#[behavior(SyncAction)]
impl RefineWhatToSay {
	fn ports() -> PortList {
		define_ports!(inout_port!("text"))
	}

	async fn tick(&mut self) -> BehaviorResult {
		let mut msg = bhvr_.config_mut().get_input::<String>("text")?;

		msg = msg.replace("42", "still 42");

		bhvr_.config_mut().set_output("text", msg)?;

		println!("Robot has refined his thoughts");

		Ok(BehaviorStatus::Success)
	}
}

#[tokio::test]
async fn basic_ports() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_action!(factory, "SaySomething", SaySomething);
	register_action!(factory, "ThinkWhatToSay", ThinkWhatToSay);
	register_action!(factory, "RefineWhatToSay", RefineWhatToSay);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);
	let answer: String = factory
		.blackboard()
		.get("the_answer")
		.expect("the_answer not found");
	assert_eq!(answer, "The answer is still 42.");
	Ok(())
}

const XML2: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence  name="root_sequence">
            <SaySomething message="Hello."/>
            <Script code=" the_answer:='The answer is 42' " />
            <SaySomething message="{the_answer}"/>
            <Script code="the_answer:='The answer is still 42'" />
            <SaySomething message="{the_answer}"/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
        <Action ID="SaySomething"
                editable="true">
            <input_port name="message"
                default="Hello"/>
        </Action>
    </TreeNodesModel>
</root>
"#;

#[tokio::test]
async fn basic_ports_with_script() -> anyhow::Result<()> {
	// create BT environment
	let mut factory = BTFactory::extended();

	// register all needed nodes
	register_action!(factory, "SaySomething", SaySomething);

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML2)?;

	// run the BT
	let result = tree.tick_while_running().await?;
	assert_eq!(result, BehaviorStatus::Success);
	let answer: String = factory
		.blackboard()
		.get("the_answer")
		.expect("the_answer not found");
	assert_eq!(answer, "The answer is still 42");
	Ok(())
}

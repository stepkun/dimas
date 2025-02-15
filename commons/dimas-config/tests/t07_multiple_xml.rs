// Copyright © 2024 Stephan Kunz

//! This test implements the seventh tutorial from [BehaviorTree.CPP](https://www.behaviortree.dev)
//! [see:](https://www.behaviortree.dev/docs/tutorial-basics/tutorial_07_multiple_xml)
//!
//! Via include gives an error.
//! Manual loading works.

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;
use dimas_core::{
	behavior::{BehaviorResult, BehaviorStatus},
	define_ports, input_port,
	port::PortList,
};
use dimas_macros::{behavior, register_action};

use std::{fs::File, io::Read, path::PathBuf};

const XML_MANUALLY: &str = r#"
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <BehaviorTree ID="MainTree">
        <Sequence>
            <SaySomething message="starting MainTree" />
            <SubTree ID="SubTreeA" />
            <SubTree ID="SubTreeB" />
        </Sequence>
    </BehaviorTree>
</root>
"#;

const XML_INCLUDE: &str = r#"
<root BTCPP_format="4"
      main_tree_to_execute="MainTree">
    <include path="./subtree_A.xml" />
    <include path="./subtree_B.xml" />
    <BehaviorTree ID="MainTree">
        <Sequence>
            <SaySomething message="starting MainTree" />
            <SubTree ID="SubTreeA" />
            <SubTree ID="SubTreeB" />
        </Sequence>
    </BehaviorTree>
</root>"#;

/// SyncAction "SaySomething"
#[behavior(SyncAction)]
struct SaySomething {}

#[behavior(SyncAction)]
impl SaySomething {
	async fn tick(&mut self) -> BehaviorResult {
		let msg: String = bhvr_.config_mut().get_input("message")?;

		println!("Robot says: {msg}");

		Ok(BehaviorStatus::Success)
	}

	fn ports() -> PortList {
		define_ports!(input_port!("message"))
	}
}

#[tokio::test]
#[ignore]
async fn via_include() -> anyhow::Result<()> {
	println!("subtrees via include");
	/*
	fails with
	Error: Errors like this shouldn't happen. Something bad has happened. Please report this. Empty(BytesStart { buf: Borrowed("include path=\"./subtree_A.xml\" "), name_len: 7 })
	*/
	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_action!(factory, "SaySomething", SaySomething);

	// create tree
	let tree = factory.create_tree_from_xml(XML_INCLUDE);
	match tree {
		Ok(mut tree) => {
			// run the BT
			let result = tree.tick_while_running().await?;
			println!("tree result is {result}");
		}
		Err(error) => println!("{error}"),
	}

	Ok(())
}

#[tokio::test]
async fn manually() -> anyhow::Result<()> {
	println!("subtrees manually");

	// create BT environment
	let mut factory = BTFactory::default();

	// register all needed nodes
	register_action!(factory, "SaySomething", SaySomething);

	// create tree
	// load files
	let directory = std::env::current_dir()?
		.to_str()
		.expect("path invalid")
		.to_string();
	let search_path = PathBuf::from(directory).join("tests");

	// iterate over directories xml files
	let files = std::fs::read_dir(search_path)?.flatten();
	for file in files {
		if file
			.file_name()
			.into_string()
			.expect("could not determine file type")
			.ends_with("xml")
		{
			// read xml from file
			let mut file = File::open(file.path())?;
			let mut xml = String::new();
			file.read_to_string(&mut xml)?;
			// register
			factory.register_subtree(&xml)?;
		}
	}

	//// register main tree
	//factory.register_bt_from_text(XML_MANUALLY.into())?;
	//// instantiate the BT
	//let tree = factory.instantiate_sync_tree(&blackboard, "MainTree");

	// create the BT
	let mut tree = factory.create_tree_from_xml(XML_MANUALLY)?;
	// run the BT
	let result = tree.tick_while_running().await?;
	println!("tree result is {result}");

	Ok(())
}

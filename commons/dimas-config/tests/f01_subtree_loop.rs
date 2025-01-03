// Copyright © 2024 Stephan Kunz

//! Test loops in the tree definition

#[doc(hidden)]
extern crate alloc;

use dimas_config::factory::BTFactory;

const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
        main_tree_to_execute="MainTree">
    <BehaviorTree ID = "MainTree">
        <Sequence>
            <SubTree ID = "Level1"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID =	"Level1">
        <Sequence>
            <SubTree ID= "Level2"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree   ID="Level2">
        <Sequence>
            <SubTree	ID="Level1"/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
    </TreeNodesModel>
</root>
"#;

const XML1: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
        main_tree_to_execute="MainTree">
    <BehaviorTree ID = "MainTree">
        <Sequence>
            <SubTree ID = "Level1"/>
        </Sequence>
    </BehaviorTree>

    <BehaviorTree ID =	"Level1">
        <Sequence>
            <SubTree ID= "Level2"/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
    </TreeNodesModel>
</root>
"#;

const XML2: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4">
    <BehaviorTree   ID="Level2">
        <Sequence>
            <SubTree	ID="Level1"/>
        </Sequence>
    </BehaviorTree>

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
    </TreeNodesModel>
</root>
"#;

#[test]
fn single_file() {
	let mut factory = BTFactory::extended();

	assert_eq!(
		factory
			.create_tree(XML)
			.expect_err("should error")
			.to_string(),
		"loop in tree detected: [MainTree->Level1->Level2] -> [Level1]"
	);
}

#[test]
fn multiple_files() {
	// create BT environment
	let mut factory = BTFactory::extended();

	// register subtree
	factory.register_subtree(XML2).expect("snh");

	assert_eq!(
		factory
			.create_tree(XML1)
			.expect_err("should error")
			.to_string(),
		"loop in tree detected: [MainTree->Level1->Level2] -> [Level1]"
	);
}

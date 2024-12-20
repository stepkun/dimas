// Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]
#![allow(clippy::unwrap_used)]

// region:      --- modules
use anyhow::Result;
use behaviortree_rs::{
	bt_node, macros::register_action_node, nodes::NodeStatus, tree::AsyncTree, Blackboard, Factory,
	NodeResult,
};
use std::{fs::File, io::Read, path::PathBuf, sync::Arc, time::Duration};
use tracing::{error, event, info, instrument, warn, Level};
// endregion:   --- modules

// region:      --- behavior
const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4"
        main_tree_to_execute="CoreTree">

    <BehaviorTree ID="AgentControl">
        <AlwaysRunning/>
    </BehaviorTree>

    <BehaviorTree ID="CoreTree">
        <WhileDoElse>
            <NotInterrupted/>
            <ParallelAll max_failures="-1">
                <SubTree ID="AgentControl"
                    _autoremap="true"/>
                <SubTree ID="AgentBehavior"
                    _autoremap="true"/>
            </ParallelAll>
            <Shutdown/>
        </WhileDoElse>
    </BehaviorTree>
</root>
"#;

/// ConditionNode "NotInterrupted"
// @TODO: NotInterrupted should be a condition node
#[bt_node(SyncActionNode)]
struct NotInterrupted {}

#[allow(clippy::use_self)]
#[bt_node(SyncActionNode)]
impl NotInterrupted {
	async fn tick(&mut self) -> NodeResult {
		//println!("ticking NotInterrupted");
		Ok(NodeStatus::Success)
	}
}

/// ActionNode "AlwaysSuccess"
#[bt_node(SyncActionNode)]
struct AlwaysSuccess {}

#[allow(clippy::use_self)]
#[bt_node(SyncActionNode)]
impl AlwaysSuccess {
	async fn tick(&mut self) -> NodeResult {
		println!("ticking AlwaysSuccess");
		Ok(NodeStatus::Success)
	}
}

/// ActionNode "AlwaysRunning"
#[bt_node(StatefulActionNode)]
struct AlwaysRunning {}

#[allow(clippy::use_self)]
#[bt_node(StatefulActionNode)]
impl AlwaysRunning {
	async fn on_start(&mut self) -> NodeResult {
		//println!("starting AlwaysRunning");
		Ok(NodeStatus::Running)
	}

	async fn on_running(&mut self) -> NodeResult {
		//println!("ticking AlwaysRunning");
		Ok(NodeStatus::Running)
	}
}

/// ActionNode "Shutdown"
#[bt_node(SyncActionNode)]
struct Shutdown {}

#[allow(clippy::use_self)]
#[bt_node(SyncActionNode)]
impl Shutdown {
	async fn tick(&mut self) -> NodeResult {
		println!("ticking Shutdown");
		Ok(NodeStatus::Success)
	}
}
// endregion:   --- behavior

// region:      --- Agent
/// Agent structure for std environment
pub struct Agent {
	/// A [`Blackboard`] to store information
	world: Blackboard,
	/// The factory to create & register behavior
	bt_factory: Factory,
	/// The behavior tree
	tree: Option<AsyncTree>,
}

impl Agent {
	/// create an [`Agent`] with a behavior tree environment
	/// # Errors
	/// - File `core_tree.xml` is not found
	/// # Panics
	/// - if detection of program directory fails
	pub fn create() -> Result<Self> {
		// install core behavior
		let world = Blackboard::create();
		let mut bt_factory = Factory::new();
		// register core nodes
		// @TODO: NotInterrupted should be a condition node
		register_action_node!(bt_factory, "NotInterrupted", NotInterrupted);
		register_action_node!(bt_factory, "AlwaysRunning", AlwaysRunning);
		register_action_node!(bt_factory, "AlwaysSuccess", AlwaysSuccess);
		register_action_node!(bt_factory, "Shutdown", Shutdown);

		Ok(Self {
			world,
			bt_factory,
			tree: None,
		})
	}

	/// Register nodes
	pub fn register_nodes(&mut self, reg_fn: impl Fn(&mut Factory)) {
		reg_fn(&mut self.bt_factory);
	}

	/// Set the [`Agent`]s behavior
	/// # Errors
	/// - ???
	/// # Panics
	/// - ???
	pub fn set_behavior(&mut self, xml: &str) {
        self.bt_factory.register_bt_from_text(xml.to_string());
	}

	/// Start the [`Agent`]
	/// # Errors
	/// - ???
	#[instrument(level = Level::INFO, skip_all)]
	pub async fn start(&mut self) -> Result<()> {
		event!(Level::INFO, "starting agent {}", "todo");

        // create the tree
		let mut tree = self
            .bt_factory
            .create_async_tree_from_text(XML.to_string(), &self.world)
            .await?;

        self.tree = Some(tree);

		// this will check the tree by running it once
		let mut result = self.tree.as_mut().unwrap().tick_once().await?;
		// run the BT using own loop with sleep to avoid busy loop
		while result == NodeStatus::Running {
			let () = tokio::time::sleep(Duration::from_millis(2000)).await;
			result = self.tree.as_mut().unwrap().tick_once().await?;
		}
		Ok(())
	}

	/// Stop the [`Agent`]
	/// # Errors
	/// - ???
	#[instrument(level = Level::INFO, skip_all)]
	pub async fn stop(&self) -> Result<()> {
		event!(Level::INFO, "stopping agent{}", "todo");
		Ok(())
	}
}
// endregion:   --- Agent

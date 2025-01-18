// Copyright © 2024 Stephan Kunz

// region:      --- modules
use anyhow::Result;
use dimas_config::factory::{BTFactory, Error};
use dimas_core::{
	behavior::tree::BehaviorTree,
	behavior::{BehaviorResult, BehaviorStatus},
};
use dimas_macros::{behavior, register_action, register_condition};
use std::time::Duration;
use tracing::{event, instrument, Level};
// endregion:   --- modules

// region:      --- behavior
const XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
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

    <!-- Description of Node Models (used by Groot) -->
    <TreeNodesModel>
		<Action ID="AlwaysRunning"
				editable="true"/>
		<Action ID="Shutdown"
				editable="true"/>
		<Condition ID="NotInterrupted"
				editable="true"/>
    </TreeNodesModel>
</root>
"#;

/// Condition "NotInterrupted"
#[behavior(SyncCondition)]
struct NotInterrupted {}

#[allow(clippy::use_self)]
#[behavior(SyncCondition)]
impl NotInterrupted {
	async fn tick(&self) -> BehaviorResult {
		//println!("ticking NotInterrupted");
		Ok(BehaviorStatus::Success)
	}
}

/// Action "AlwaysRunning"
#[behavior(Action)]
struct AlwaysRunning {}

#[allow(clippy::use_self)]
#[behavior(Action)]
impl AlwaysRunning {
	async fn on_start(&self) -> BehaviorResult {
		//println!("starting AlwaysRunning");
		Ok(BehaviorStatus::Running)
	}

	async fn on_running(&self) -> BehaviorResult {
		//println!("ticking AlwaysRunning");
		Ok(BehaviorStatus::Running)
	}
}

/// SyncAction "Shutdown"
#[behavior(SyncAction)]
struct Shutdown {}

#[allow(clippy::use_self)]
#[behavior(SyncAction)]
impl Shutdown {
	async fn tick(&self) -> BehaviorResult {
		println!("ticking Shutdown");
		Ok(BehaviorStatus::Success)
	}
}
// endregion:   --- behavior

// region:      --- Agent
/// Agent structure for std environment
pub struct Agent {
	/// The factory to create & register behavior
	bt_factory: BTFactory,
	/// The behavior tree
	tree: Option<BehaviorTree>,
}

impl Agent {
	/// create an [`Agent`] with a behavior tree environment
	/// # Errors
	/// - File `core_tree.xml` is not found
	/// # Panics
	/// - if detection of program directory fails
	pub fn create() -> Result<Self> {
		// install core behavior
		let mut bt_factory = BTFactory::default();
		bt_factory.add_extensions();

		// register core nodes
		register_condition!(bt_factory, "NotInterrupted", NotInterrupted,);
		register_action!(bt_factory, "AlwaysRunning", AlwaysRunning,);
		register_action!(bt_factory, "Shutdown", Shutdown,);

		Ok(Self {
			bt_factory,
			tree: None,
		})
	}

	/// Register behavior
	pub fn register_behavior(&mut self, reg_fn: impl Fn(&mut BTFactory)) {
		reg_fn(&mut self.bt_factory);
	}

	/// Set the [`Agent`]s behavior
	/// # Errors
	/// - ???
	/// # Panics
	/// - ???
	pub fn set_behavior(&mut self, xml: &str) -> Result<(), Error> {
		self.bt_factory.register_subtree(xml)
	}

	/// Start the [`Agent`]
	/// # Errors
	/// - ???
	#[instrument(level = Level::INFO, skip_all)]
	pub async fn start(&mut self) -> Result<()> {
		event!(Level::INFO, "starting agent {}", "todo");

		// create the tree
		let tree = self.bt_factory.create_tree_from_xml(XML)?;

		self.tree = Some(tree);

		// this will check the tree by running it once
		let mut result = self
			.tree
			.as_mut()
			.unwrap_or_else(|| todo!())
			.tick_once()
			.await?;
		// run the BT using own loop with sleep to avoid busy loop
		while result == BehaviorStatus::Running {
			let () = tokio::time::sleep(Duration::from_millis(2000)).await;
			result = self
				.tree
				.as_mut()
				.unwrap_or_else(|| todo!())
				.tick_once()
				.await?;
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

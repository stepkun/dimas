// Copyright © 2025 Stephan Kunz

// region:      --- modules
use anyhow::Result;
use dimas_behavior::{
	Behavior, SharedRuntime,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType},
	blackboard::SharedBlackboard,
	factory::{BehaviorTreeFactory, error::Error},
	tree::{BehaviorTree, BehaviorTreeElementList},
};
use std::time::Duration;
use tracing::{Level, event, instrument};
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

/// Condition `NotInterrupted`
#[derive(Behavior, Debug, Default)]
struct NotInterrupted;

#[async_trait::async_trait]
impl BehaviorInstance for NotInterrupted {
	/// @TODO:
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		// println!("ticking NotInterrupted");
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for NotInterrupted {
	fn kind() -> BehaviorType {
		BehaviorType::Condition
	}
}

/// Action `Shutdown`
#[derive(Behavior, Debug, Default)]
struct Shutdown {}

#[async_trait::async_trait]
impl BehaviorInstance for Shutdown {
	/// @TODO:
	async fn tick(
		&mut self,
		_state: BehaviorState,
		_blackboard: &mut SharedBlackboard,
		_children: &mut BehaviorTreeElementList,
		_runtime: &SharedRuntime,
	) -> BehaviorResult {
		println!("ticking Shutdown");
		Ok(BehaviorState::Success)
	}
}

impl BehaviorStatic for Shutdown {
	fn kind() -> BehaviorType {
		BehaviorType::Action
	}
}
// endregion:   --- behavior

// region:      --- Agent
/// Agent structure for std environment
pub struct Agent {
	/// The factory to create & register behavior
	factory: BehaviorTreeFactory,
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
		let mut factory = BehaviorTreeFactory::with_core_behaviors()?;

		// register core nodes
		factory.register_node_type::<NotInterrupted>("NotInterrupted")?;
		factory.register_node_type::<Shutdown>("Shutdown")?;

		Ok(Self { factory, tree: None })
	}

	/// Register a behavior within the [`Agent`].
	/// # Errors
	pub fn register_behavior<T>(&mut self, name: &str) -> Result<(), Error>
	where
		T: Behavior,
	{
		self.factory.register_node_type::<T>(name)
	}

	/// Set the [`Agent`]s behavior
	/// # Errors
	/// - ???
	/// # Panics
	/// - ???
	pub fn set_behavior(&mut self, xml: &str) -> Result<(), Error> {
		self.factory.register_behavior_tree_from_text(xml)
	}

	/// Start the [`Agent`]
	/// # Errors
	/// - ???
	#[instrument(level = Level::INFO, skip_all)]
	pub async fn start(&mut self) -> Result<()> {
		event!(Level::INFO, "starting agent {}", "todo");

		// create the tree
		let tree = self.factory.create_from_text(XML)?;

		self.tree = Some(tree);

		// this will check the tree by running it once
		let mut result = self
			.tree
			.as_mut()
			.unwrap_or_else(|| todo!())
			.tick_once()
			.await?;
		// run the BT using own loop with sleep to avoid busy loop
		while result == BehaviorState::Running {
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

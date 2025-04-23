//! `DiMAS` observation example
//! Copyright © 2024 Stephan Kunz

use dimas::prelude::*;

const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4">
    <BehaviorTree ID="AgentBehavior">
		<AlwaysRunning/>
    </BehaviorTree>
</root>
"#;

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	let mut agent = Agent::create()?;

	// nodes must be registered before they are addressed in a behavior tree

	agent.set_behavior(XML)?;

	agent.start().await?;
	Ok(())
}

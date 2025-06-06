//! `DiMAS` publisher example
//! Copyright © 2024 Stephan Kunz

use dimas::prelude::*;

const XML: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="AgentBehavior">
        <IntervalTimer>
			<Publisher/>
        </IntervalTimer>
    </BehaviorTree>
</root>
"#;

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	let mut agent = Agent::create()?;

	// behaviors must be registered before they are addressed in a behavior tree
	agent.register_behavior::<IntervalTimer>("IntervalTimer")?;
	agent.register_behavior::<Publisher>("Publisher")?;

	agent.set_behavior(XML)?;

	agent.start().await?;
	Ok(())
}

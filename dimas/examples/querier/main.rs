//! `DiMAS` query example
//! Copyright © 2024 Stephan Kunz

use dimas::prelude::*;

const XML: &str = r#"
<root BTCPP_format="4">
    <BehaviorTree ID="AgentBehavior">
        <IntervalTimer interval = "1000">
			<AlwaysSuccess/>
        </IntervalTimer>
    </BehaviorTree>
</root>
"#;

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	let mut agent = Agent::create()?;

	// nodes must be registered before they are addressed in a behavior tree
	agent.register_behavior::<IntervalTimer>("IntervalTimer")?;
	// agent.register_behavior::<Querier>("Querier")?;

	agent.set_behavior(XML)?;

	agent.start().await?;
	Ok(())
}

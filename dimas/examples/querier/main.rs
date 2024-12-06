//! `DiMAS` query example
//! Copyright © 2024 Stephan Kunz

use dimas::prelude::*;

#[derive(Debug)]
struct AgentProps {
	counter: u128,
}

async fn query_callback(ctx: Context<AgentProps>, response: QueryableMsg) -> Result<()> {
	let message: u128 = response.decode()?;
	println!("Response [{}] is '{message}'", ctx.read().counter);
	ctx.write().counter += 1;
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	// create & initialize agents properties
	let properties = AgentProps { counter: 0 };

	// create an agent with the properties and the prefix 'examples'
	let mut agent = Agent::new(properties)
		.prefix("examples")
		.name("querier")
		.config(&Config::default())?;

	// create querier for topic "query"
	agent
		.querier()
		.topic("query")
		.callback(query_callback)
		.add()?;

	// timer for regular querying
	let interval = Duration::from_secs(1);
	agent
		.timer()
		.name("timer")
		.interval(interval)
		.callback(move |ctx| -> Result<()> {
			let counter = ctx.read().counter;
			println!("Querying [{counter}]");
			let message = Message::encode(&counter);
			// querying with stored query
			ctx.get("query", Some(message), None)?;
			Ok(())
		})
		.add()?;

	// run agent
	agent.start().await?;

	Ok(())
}

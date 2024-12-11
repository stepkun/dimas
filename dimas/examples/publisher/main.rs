//! `DiMAS` publisher example
//! Copyright © 2024 Stephan Kunz
#![allow(unused)]

use dimas::prelude::*;
use dimas_time::TimerLib;

#[dimas::agent]
#[derive(Debug, Default)]
struct Publisher {
	count: u128,
}

#[derive(Debug)]
struct AgentProps {
	count: u128,
}

/// common structure for publisher and subscriber
#[derive(Debug, Encode, Decode)]
pub struct PubSubMessage {
	/// counter
	pub count: u128,
	/// text
	pub text: String,
}

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	// create an agent with the properties of `Publisher`
	let agent = Publisher::agent()
		.set_prefix("examples")
		.set_name("publisher");

	// how to access the properties
	let current = agent.read().count;
	agent.write().count += 1;
	agent.write().count = current;

	// add a timer component to the agent
	let timerlib = TimerLib::default();
	agent.add_component(Box::new(timerlib));

	// create an inteval timer using the timer component
	

	dbg!(&agent);

	// ==========================================================================
	// ---------------------------- old stuff -----------------------------------
	// ==========================================================================

	// create & initialize agents properties
	let properties = AgentProps { count: 0 };

	// create an agent with the properties and the prefix 'examples'
	let mut agent = Agent::new(properties)
		.prefix("examples")
		.name("publisher")
		.config(&Config::default())?;

	// create publisher for topic "hello"
	agent.publisher().topic("hello").add()?;

	// use timer for regular publishing
	agent
		.timer()
		.name("timer1")
		.interval(Duration::from_secs(1))
		.callback(|ctx| -> Result<()> {
			let count = ctx.read().count;
			// create structure to send
			let msg = PubSubMessage {
				count,
				text: String::from("hello world!"),
			};
			let message = Message::encode(&msg);
			println!("Sending {} [{}]", msg.text, msg.count);
			// publishing with stored publisher
			let _ = ctx.put("hello", message);
			ctx.write().count += 1;
			Ok(())
		})
		.add()?;

	// timer for regular deletion
	let duration = Duration::from_secs(3);
	agent
		.timer()
		.name("timer2")
		.interval(duration)
		.callback(move |ctx| -> Result<()> {
			println!("Deleting");
			// delete with stored publisher
			ctx.delete("hello")?;
			Ok(())
		})
		.add()?;

	// run the agent
	agent.start().await?;

	Ok(())
}

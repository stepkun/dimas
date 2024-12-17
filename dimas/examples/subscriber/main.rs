//! `DiMAS` subscriber example
//! Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]

use dimas::prelude::*;
use dimas_com::Zenoh;

#[dimas::agent]
#[derive(Debug, Default)]
struct Subscriber {
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

#[allow(clippy::unused_async)]
async fn subscriber_callback(ctx: Agent, message: Message) -> Result<()> {
	let message: PubSubMessage = message.decode()?;
	let count = ctx
		.read()
		.downcast_ref::<Subscriber>()
		.unwrap()
		.count;
	if message.count > count {
		println!("missed {} messages", message.count - count);
		ctx.write()
			.downcast_mut::<Subscriber>()
			.unwrap()
			.count = message.count;
	}
	println!("Received {} [{}]", message.text, message.count);
	ctx.write()
		.downcast_mut::<Subscriber>()
		.unwrap()
		.count += 1;
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	// create an agent with the properties of `Subscriber`
	let mut agent = Subscriber::default()
		.into_agent()
		.set_name("subscriber");

	// create communication component with subscriber activity
	let mut communicator = Box::new(Zenoh::new("examples", agent.clone()));
	communicator.create_subscriber("hello", subscriber_callback)?;

	agent.add_component(communicator);

	/// start agent in wanted operation state
	agent.manage_operation_state(OperationState::Active);
	dbg!(&agent);
	agent.start().await?;

	Ok(())
}

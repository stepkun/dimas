//! `DiMAS` publisher example
//! Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]

use dimas::prelude::*;

#[dimas::agent]
#[derive(Debug, Default)]
struct Publisher {
	count: u128,
}

/// common message structure for publisher and subscriber
#[derive(Debug, Encode, Decode)]
pub struct PubSubMessage {
	/// counter
	pub count: u128,
	/// text
	pub text: String,
}

async fn timer_callback(ctx: Agent) -> Result<()> {
	// read properties
	// @TODO: improve to easier access
	let count = ctx
		.read()
		.downcast_ref::<Publisher>()
		.unwrap()
		.count;
	// create structure to send
	let msg = PubSubMessage {
		count,
		text: String::from("hello world!"),
	};
	let message = Message::encode(&msg);
	println!("Sending {} [{}]", msg.text, msg.count);

	// publishing with stored publisher
	//let _ = ctx.put("hello", message);

	// update properties
	// @TODO: improve to easier access
	ctx.write()
		.downcast_mut::<Publisher>()
		.unwrap()
		.count += 1;
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	// create an agent with the properties of `Publisher`
	let mut agent = Publisher::default()
		.into_agent()
		.set_prefix("examples")
		.set_name("publisher");

	// add wanted components
	// @TODO: change to load library
	let timerlib = TimerLib::new(agent.clone());

	// create an interval timer using the timer library
	let parameter = IntervalTimerParameter::default();
	let timer = timerlib.create_timer(TimerVariant::Interval(parameter), timer_callback);
	agent.add_activity(timer);

	// drop factories to reduce memory footprint
	drop(timerlib);

	/// start agent in wanted operation state
	agent.manage_operation_state(OperationState::Active);
	agent.start().await;

	Ok(())
}

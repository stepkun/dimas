//! `DiMAS` query example
//! Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]

use dimas::prelude::*;

#[dimas::agent]
#[derive(Debug, Default)]
struct Querier {
	counter: u128,
}

#[derive(Debug)]
struct AgentProps {
	counter: u128,
}

async fn timer_callback(ctx: Agent) -> Result<()> {
	let counter = ctx
		.read()
		.downcast_ref::<Querier>()
		.unwrap()
		.counter;
	println!("Querying [{counter}]");
	let _message = Message::encode(&counter);
	// querying with stored query
	//ctx.get("query", Some(message), None)?;
	ctx.write()
		.downcast_mut::<Querier>()
		.unwrap()
		.counter = counter + 1;
	Ok(())
}

#[allow(clippy::unused_async)]
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

	// create an agent with the properties of `Querier`
	let mut agent = Querier::default()
		.into_agent()
		.set_name("querier");

	// add wanted components
	// @TODO: change to load library
	let timerlib = TimerLib::default();

	// create an interval timer using the timer library
	let activity = ActivityData::new("timer", agent.clone());
	let parameter = IntervalTimerParameter::new(Duration::from_secs(1), None, activity);
	let timer = timerlib.create_timer(TimerVariant::Interval(parameter), timer_callback);
	agent.add_activity(timer);

	// drop factories to reduce memory footprint
	drop(timerlib);

	/// start agent in wanted operation state
	agent.manage_operation_state(OperationState::Active);
	agent.start().await;

	Ok(())
}

//! `DiMAS` observation example
//! Copyright © 2024 Stephan Kunz
#![allow(unused)]
#![allow(clippy::unwrap_used)]

use dimas::prelude_old::*;

#[dimas::agent]
#[derive(Debug)]
struct Observer {
	counter: u128,
	limit: u128,
	new_limit: u128,
	occupied_counter: u8,
}

impl Default for Observer {
	fn default() -> Self {
		Self {
			counter: 0,
			limit: 5,
			new_limit: 5,
			occupied_counter: 3,
		}
	}
}

/// request structure for observer and observable
#[derive(Debug, Encode, Decode)]
pub struct FibonacciRequest {
	/// limit
	pub limit: u128,
}

async fn timer_callback(ctx: Agent) -> Result<()> {
	let counter = ctx
		.read()
		.downcast_ref::<Observer>()
		.unwrap()
		.counter;
	let limit = ctx
		.read()
		.downcast_ref::<Observer>()
		.unwrap()
		.new_limit;
	println!("Request [{counter}] for fibonacci up to {limit}");
	let msg = FibonacciRequest { limit };
	let message = Message::encode(&msg);
	//ctx.observe("fibonacci", Some(message))?;
	ctx.write()
		.downcast_mut::<Observer>()
		.unwrap()
		.counter += 1;
	Ok(())
}

#[derive(Debug)]
struct AgentProps {
	limit: u128,
	new_limit: u128,
	occupied_counter: u8,
}

#[allow(clippy::unused_async)]
async fn control_response(
	ctx: Context<AgentProps>,
	response: ObservableControlResponse,
) -> Result<()> {
	match response {
		ObservableControlResponse::Accepted => {
			let limit = ctx.read().new_limit;
			println!("Accepted fibonacci up to {limit}");
			ctx.write().limit = limit;

			ctx.write().new_limit += 1;
		}
		ObservableControlResponse::Declined => {
			println!("Declined fibonacci up to {}", ctx.read().new_limit);
			ctx.write().limit = 0;
			ctx.write().new_limit = 5;
		}
		ObservableControlResponse::Occupied => {
			println!("Service fibonacci is occupied");
			let occupied_counter = ctx.read().occupied_counter + 1;
			// cancel running request whenever 5 occupied messages arrived
			if occupied_counter % 5 == 0 {
				ctx.cancel_observe("fibonacci")?;
				ctx.write().occupied_counter = 0;
			} else {
				ctx.write().occupied_counter = occupied_counter;
			}
		}
		ObservableControlResponse::Canceled => {
			println!("Canceled fibonacci up to {}", ctx.read().limit);
		}
	};
	Ok(())
}

#[allow(clippy::unused_async)]
async fn response(ctx: Context<AgentProps>, response: ObservableResponse) -> Result<()> {
	match response {
		ObservableResponse::Canceled(value) => {
			let msg = Message::new(value);
			let result: Vec<u128> = msg.decode()?;

			println!("Canceled at {result:?}");
		}
		ObservableResponse::Feedback(value) => {
			let msg = Message::new(value);
			let result: Vec<u128> = msg.decode()?;
			let limit = ctx.read().limit;
			if result.len() <= limit as usize {
				println!("Received feedback {result:?}");
			} else {
				println!("Wrong feedback {result:?}");
			}
		}
		ObservableResponse::Finished(value) => {
			let msg = Message::new(value);
			let result: Vec<u128> = msg.decode()?;
			let limit = ctx.read().limit;
			if result.len() == limit as usize {
				println!("Received result {result:?}");
			} else {
				println!("Wrong result {result:?}");
			}
		}
	}
	Ok(())
}

#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	// create an agent with the properties of `Observer`
	let mut agent = Observer::default()
		.into_agent()
		.set_name("observer");

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

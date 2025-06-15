# dimas

[DiMAS](https://github.com/dimas-fw/dimas/tree/main/dimas) - A framework
for building **Di**stributed **M**ulti **A**gent **S**ystems

⚠️ WARNING ⚠️ : `DiMAS` is under active development,
so expect gaps between implementation and documentation.

A distributed multi agent system is a set of independant agents
that are widely distributed but somehow connected.
They are designed in a way that they can solve complex tasks by working together.

The system is characterised by

- a somewhat large and complex environment
- containing a set of (non agent) objects that can be perceived, created, moved,
modified or destroyed by the agents
- that changes over time due to external rules

with multiple agents operating in that environment which

- can perceive the environment to a limited extent
- have the possibility to communicate with some or all of the other agents
- have certain capabilities to influence the environment

This crate is available on [crates.io](https://crates.io/crates/dimas).

[DiMAS](https://github.com/dimas-fw/dimas/tree/main/dimas) follows the semantic
versioning principle with the enhancement, that until version 1.0.0
each new minor version has breaking changes, while patches are non breaking
changes but may include enhancements.

## Usage

`DiMAS` uses the `tokio` runtime, you have to define your `main` function as an
`async` function. The declaration of tokio crate is not necessary, unless you use
tokio functionality within your implementations.

So include `dimas` runtime in the dependencies section of
your `Cargo.toml`.

Your `Cargo.toml` should include:

```toml
[dependencies]
dimas = "0.5.0"
```

It makes sense to return a `Result` in `main`, as most `DiMAS` `Agent`s functions do.
`DiMAS` internally uses `anyhow::Result<T>` and re-exports it for convenience.

`DiMAS` also provides a `main` attribute macro to create the runtime environment
and a `prelude` to import most used declarations.

A suitable main program skeleton may look like:

```rust
use dimas::prelude::*;

#[dimas::main]
async fn main() -> Result<()> {

    // your code
    // ...

    Ok(())
}
```

## Example

A very simple example consist at least of two agents, a `publisher` publishing messages
and a `subscriber` that is listening to those messages.

The `Cargo.toml` for this publisher/subscriber example should include

```toml
[dependencies]
dimas = version = "0.4"
```

### Publisher

The `publisher.rs` should look like this:

```rust,no_run
use dimas::prelude::*;

const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
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

	// nodes must be registered before they are addressed in a behavior tree
	// agent.register_behavior(IntervalTimer::register);
	// agent.register_behavior(Publisher::register);

	agent.set_behavior(XML);

	agent.start().await?;
	Ok(())
}
```

### Subscriber

The `subscriber.rs` should look like this:

```rust,no_run
use dimas::prelude::*;

const XML: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<root BTCPP_format="4">
    <BehaviorTree ID="AgentBehavior">
        <Subscriber/>
    </BehaviorTree>
</root>
"#;


#[dimas::main]
async fn main() -> Result<()> {
	// initialize tracing/logging
	init_tracing();

	let mut agent = Agent::create()?;

	// nodes must be registered before they are addressed in a behavior tree
	//agent.register_behavior(Subscriber::register);

	agent.set_behavior(XML);

	agent.start().await?;
	Ok(())
}
```

## More examples

You can find some simple examples in [dimas-fw/dimas/examples](https://github.com/dimas-fw/dimas/blob/main/examples/README.md)
and more complex examples in [dimas-fw/examples](https://github.com/dimas-fw/examples/blob/main/README.md)

## Features

- unstable: Enables the unstable features.

## License

Licensed with the fair use "NGMC" license, see [license file](https://github.com/dimas-fw/dimas/blob/main/LICENSE)

## Contribution

Any contribution intentionally submitted for inclusion in the work by you,
shall be licensed with the same "NGMC" license, without any additional terms or conditions.

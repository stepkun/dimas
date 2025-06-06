# DiMAS Examples

Examples using DiMAS, the **Di**stributed **M**ulti **A**gent **S**ystem framework.

On Linux you can install `tmux` and in the `<workspace>` directory run
```shell
./run-examples.sh
```
or
```shell
./run-examples.sh --release
```
This will start all below examples in parallel.

## Publisher/Subscriber

Implements a simple "Hello World!" Publisher/Subscriber pair

Run the [Publisher](https://github.com/dimas-fw/dimas/blob/main/dimas/examples/publisher/main.rs)
in one terminal window with

```shell
cargo run --example publisher
```

and the [Subscriber](https://github.com/dimas-fw/dimas/blob/main/dimas/examples/subscriber/main.rs)
in another terminal window with

```shell
cargo run --example subscriber
```

## Queryable/Querier

Implements a simple Qeryable/Querier pair, where the Querier does not wait for
a started Queryable, but continues querying.

Run the [Querier](https://github.com/dimas-fw/dimas/blob/main/dimas/examples/querier/main.rs)
in one terminal window with

```shell
cargo run --example querier
```

and the [Queryable](https://github.com/dimas-fw/dimas/blob/main/dimas/examples/queryable/main.rs)
in another terminal window with
```shell
cargo run --example queryable
```

## Observable/Observer

Implements a simple Observable/Observer pair, where the Observer does not wait
for a started Observable, but continues requesting an Observation.

Run the [Observer](https://github.com/dimas-fw/dimas/blob/main/dimas/examples/observer/main.rs)
in one terminal window with

```shell
cargo run --example observer
```

and the [Observable](https://github.com/dimas-fw/dimas/blob/main/dimas/examples/observable/main.rs)
in another terminal window with

```shell
cargo run --example observable
```

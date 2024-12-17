// Copyright © 2024 Stephan Kunz

//! Tests

use dimas::agent_old::*;
use dimas::builder_old::builder_states::*;
use dimas::builder_old::*;
use dimas::context_old::ContextImpl;

#[derive(Debug)]
struct Props {}

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<AgentOld<Props>>();
	is_normal::<ContextImpl<Props>>();
	#[cfg(feature = "unstable")]
	is_normal::<LivelinessSubscriberBuilder<Props, NoCallback, NoStorage>>();
	is_normal::<ObservableBuilder<Props, NoSelector, NoCallback, NoCallback, NoCallback, NoStorage>>(
	);
	is_normal::<ObserverBuilder<Props, NoSelector, NoCallback, NoCallback, NoStorage>>();
	is_normal::<PublisherBuilder<Props, NoSelector, NoStorage>>();
	is_normal::<QuerierBuilder<Props, NoSelector, NoCallback, NoStorage>>();
	is_normal::<QueryableBuilder<Props, NoSelector, NoCallback, NoStorage>>();
	is_normal::<SubscriberBuilder<Props, NoSelector, NoCallback, NoStorage>>();
	is_normal::<UnconfiguredAgentOld<Props>>();
}

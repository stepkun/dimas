// Copyright © 2024 Stephan Kunz

//! Tests
#[cfg(feature = "unstable")]
use dimas_com::zenoh_old::LivelinessSubscriber;
use dimas_com::{
	enums_old::CommunicatorImplementation,
	zenoh_old::{CommunicatorOld, Observable, Observer, Publisher, Querier, Queryable, Subscriber},
	MultiCommunicator, SingleCommunicator,
};

#[derive(Debug)]
struct Props {}

// check, that the auto traits are available
const fn is_normal<T: Sized + Send + Sync>() {}

#[test]
const fn normal_types() {
	is_normal::<CommunicatorOld>();
	is_normal::<CommunicatorImplementation>();
	#[cfg(feature = "unstable")]
	is_normal::<LivelinessSubscriber<Props>>();
	is_normal::<MultiCommunicator>();
	is_normal::<Observable<Props>>();
	is_normal::<Observer<Props>>();
	is_normal::<Publisher>();
	is_normal::<Querier<Props>>();
	is_normal::<Queryable<Props>>();
	is_normal::<SingleCommunicator>();
	is_normal::<Subscriber<Props>>();
}

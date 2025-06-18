// Copyright © 2025 Stephan Kunz

//! `Loop` behavior implementation
//!


// region:      --- modules
use alloc::sync::Arc;
use alloc::{boxed::Box, string::ToString};
use alloc::collections::vec_deque::VecDeque;
use core::fmt::{Debug, Display, Formatter};
use core::str::FromStr;
use dimas_scripting::SharedRuntime;
use parking_lot::Mutex;

use crate::behavior::BehaviorData;
use crate::blackboard::BlackboardInterface;
use crate::port::PortList;
use crate::{self as dimas_behavior, inout_port, input_port, output_port, port_list};
use crate::{
	Behavior,
	behavior::{BehaviorInstance, BehaviorResult, BehaviorState, BehaviorStatic, BehaviorType, error::BehaviorError},
	blackboard::SharedBlackboard,
	tree::BehaviorTreeElementList,
};
// endregion:   --- modules

// region:		--- SharedQueue
/// Shared queue implementation for the [`Loop`] behavior
#[derive(Debug, Default)]
pub struct SharedQueue<T: FromStr + ToString>(pub Arc<Mutex<VecDeque<T>>>);

impl<T> Clone for SharedQueue<T>
	where T: FromStr + ToString
{
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> Display for SharedQueue<T>
	where T: FromStr + ToString
{
	fn fmt(&self, _f: &mut Formatter) -> core::fmt::Result {
		todo!()
	}
}

impl<T> FromStr for SharedQueue<T>
	where T: FromStr + ToString
{
    type Err = dimas_behavior::behavior::BehaviorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let queue: Self = Self::with_capacity(s.split(';').count());
		let vals = s.split(';');
		for val in vals {
			let item = match T::from_str(val) {
				Ok(item) => item,
				Err(_err) => return Err(BehaviorError::ParseError(val.into(), s.into())),
			};
			queue.push_back(item);
		}
		Ok(queue)
    }
}

impl<T> SharedQueue<T>
	where T: FromStr + ToString
{
	/// Create a shared queue with a given starting capacity.
	#[must_use]
	pub fn with_capacity(capacity: usize) -> Self {
		Self(Arc::new(Mutex::new(VecDeque::with_capacity(capacity))))
	}

	/// Removes the last element from the queue and returns it,
	/// or None if it is empty.
	#[must_use]
	pub fn pop_back(&self) -> Option<T> {
		self.0.lock().pop_back()
	}

	/// Removes the first element from the queue and returns it,
	/// or None if it is empty.
	#[must_use]
	pub fn pop_front(&self) -> Option<T> {
		self.0.lock().pop_front()
	}

	/// Appends an element to the back of the queue.
	pub fn push_back(&self, value: T) {
		self.0.lock().push_back(value);
	}

	/// Prepends an element to the queue.
	pub fn push_front(&mut self, value: T) {
		self.0.lock().push_front(value);
	}
}
// endregion:	--- SharedQueue

// region:      --- Loop
/// The [`Loop`] behavior is used to `pop_front` elements from a [`VecDeque`].
/// This element is copied into the port "value" and the child will be executed
/// as long as there are elements in the queue.
/// 
#[derive(Behavior, Debug, Default)]
pub struct Loop<T>
where
	T: Clone + Debug + Default + FromStr + ToString + Send + Sync + 'static,
{
	queue: Option<SharedQueue<T>>,
	state: BehaviorState,
}

#[async_trait::async_trait]
impl<T> BehaviorInstance for Loop<T>
where
	T: Clone + Debug + Default + FromStr + ToString + Send + Sync + 'static,
{
	async fn start(
		&mut self,
		behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		// only on first start
		if self.queue.is_none() {
			// check composition
			if children.len() != 1 {
				return Err(BehaviorError::Composition("Loop must have a single child!".into()));
			}
			// fetch if_empty value
			self.state = blackboard.get::<BehaviorState>("if_empty".into())?;
			// fetch the shared queue
			self.queue = Some(blackboard.get::<SharedQueue<T>>("queue".into())?);

		}
		self.tick(behavior, blackboard, children, runtime).await
	}

	async fn tick(
		&mut self,
		_behavior: &mut BehaviorData,
		blackboard: &mut SharedBlackboard,
		children: &mut BehaviorTreeElementList,
		runtime: &SharedRuntime,
	) -> BehaviorResult {
		if let Some(queue) = &self.queue {
			if let Some(value) = queue.pop_front() {
				blackboard.set::<T>("value".into(), value)?;
				let child_state = children[0].execute_tick(runtime).await?;
				if child_state.is_completed() {
					children[0].reset(runtime)?;
				};
				if child_state == BehaviorState::Failure {
					Ok(BehaviorState::Failure)
				} else {
					Ok(BehaviorState::Running)
				}
			} else {
				Ok(self.state)
			}
		} else {
			Err(BehaviorError::Composition("Queue was not initiialized properly!".into()))
		}
	}
}

impl<T> BehaviorStatic for Loop<T>
where
	T: Clone + Debug + Default + FromStr + ToString + Send + Sync + 'static,
{
	fn kind() -> BehaviorType {
		BehaviorType::Decorator
	}

	fn provided_ports() -> PortList {
		port_list![
			inout_port!(SharedQueue<T>, "queue"),
			input_port!(
				BehaviorState,
				"if_empty",
				BehaviorState::Success,
				"State to return if queue is empty: SUCCESS, FAILURE, SKIPPED"
			),
			output_port!(T, "value"),
		]
	}
}
// endregion:   --- Subtree

// Copyright © 2025 Stephan Kunz

//! [`BehaviorTreeObserver`] implementation.
//!

extern crate std;

// region:      --- modules
use alloc::{sync::Arc, vec::Vec};
use dimas_core::ConstString;
use parking_lot::Mutex;
#[cfg(feature = "std")]
use tokio::sync::mpsc;
#[cfg(feature = "std")]
use tokio::task::JoinHandle;
#[cfg(feature = "std")]
use tokio::time::Instant;

use crate::{
	behavior::BehaviorStatus,
	tree::{BehaviorTree, BehaviorTreeElement},
};
// endregion:   --- modules

// region:      --- Statistics
/// Structure to collect various statistic data.
#[derive(Clone)]
pub struct Statistics {
	/// count ticks.
	pub tick_count: usize,
	/// Last result of a tick, either Success or Failure.
	pub last_result: BehaviorStatus,
	/// Last status. Can be any status.
	pub current_status: BehaviorStatus,
	/// count status transitions, excluding transition to Idle.
	pub transitions_count: usize,
	/// count number of transitions to Success.
	pub success_count: usize,
	/// count number of transitions to Failure.
	pub failure_count: usize,
	/// count number of transitions to Skip.
	pub skip_count: usize,
	/// Duration of execution
	#[cfg(feature = "std")]
	pub timestamp: Instant,
}

impl Default for Statistics {
	fn default() -> Self {
		Self {
			tick_count: Default::default(),
			last_result: BehaviorStatus::default(),
			current_status: BehaviorStatus::default(),
			transitions_count: Default::default(),
			success_count: Default::default(),
			failure_count: Default::default(),
			skip_count: Default::default(),
			timestamp: Instant::now(),
		}
	}
}
// endregion:   --- Statistics

// region:      --- BehaviorTreeObserver
/// An observer collecting [`BehaviorTree`] statistics
pub struct BehaviorTreeObserver {
	_handle: JoinHandle<i32>,
	statistics: Arc<Mutex<Vec<Statistics>>>,
}

impl BehaviorTreeObserver {
	/// Construct a new [`BehaviorTreeObserver`].
	/// # Panics
	pub fn new(root: &mut BehaviorTree) -> Self {
		let id: ConstString = "statistics".into();
		let statistics: Arc<Mutex<Vec<Statistics>>> = Arc::new(Mutex::new(Vec::new()));
		let (tx, mut rx) = mpsc::channel::<(u16, Instant, BehaviorStatus, BehaviorStatus)>(10);
		// spawn receiver
		let statistics_clone = statistics.clone();
		let handle = tokio::spawn(async move {
			while let Some(val) = rx.recv().await {
				let mut stats = statistics_clone.lock();
				let entry = &mut stats[val.0 as usize];
				entry.tick_count += 1;
				if val.3 != val.2 {
					entry.transitions_count += 1;
					match val.3 {
						BehaviorStatus::Failure => {
							entry.failure_count += 1;
							entry.last_result = val.3;
						}
						BehaviorStatus::Idle | BehaviorStatus::Running => {}
						BehaviorStatus::Skipped => entry.skip_count += 1,
						BehaviorStatus::Success => {
							entry.success_count += 1;
							entry.last_result = val.3;
						}
					}
					entry.current_status = val.3;
					entry.timestamp = val.1;
				}
				drop(stats);
			}
			-1
		});

		// add a statistics entry and a callback for each tree element
		for element in root.iter_mut() {
			statistics.lock().push(Statistics::default());
			let tx_clone = tx.clone();
			let callback = move |element: &BehaviorTreeElement, new_status: &mut BehaviorStatus| {
				let timestamp = Instant::now();
				let tuple = (element.uid(), timestamp, element.status(), *new_status);
				// ignore any errors when sending
				let tx_clone_cloned = tx_clone.clone();
				tokio::spawn(async move {
					tx_clone_cloned.send(tuple).await.expect("snh");
				});
			};
			element.add_pre_status_change_callback(id.clone(), callback);
		}
		Self {
			_handle: handle,
			statistics,
		}
	}

	/// Get the [`Statistics`] for a [`BehaviorTreeElement`](crate::tree::BehaviorTreeElement) using its uid.
	#[must_use]
	pub fn get_statistics(&self, uid: u16) -> Option<Statistics> {
		if self.statistics.lock().len() >= uid as usize {
			return Some((self.statistics.lock()[uid as usize]).clone());
		}
		None
	}
}
// endregion:   --- BehaviorTreeObserver

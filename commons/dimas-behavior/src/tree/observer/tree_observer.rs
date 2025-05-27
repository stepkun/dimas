// Copyright © 2025 Stephan Kunz

//! [`BehaviorTreeObserver`] implementation.
//!

#[cfg(feature = "std")]
extern crate std;

// region:      --- modules
use alloc::vec::Vec;
use dimas_core::ConstString;
#[cfg(feature = "std")]
use std::thread::JoinHandle;
#[cfg(feature = "std")]
use std::sync::mpsc;
#[cfg(feature = "std")]
use std::time::{Duration, Instant};

use crate::{
	behavior::BehaviorStatus,
	tree::{BehaviorTree, BehaviorTreeElement},
};
// endregion:   --- modules

// region:      --- Statistics
/// Structure to collect various statistic data.
#[derive(Default)]
pub struct Statistics {
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
	pub duration: Duration,
}
// endregion:   --- Statistics

// region:      --- BehaviorTreeObserver
/// An observer collecting [`BehaviorTree`] statistics
pub struct BehaviorTreeObserver {
    _handle: JoinHandle<i32>,
	statistics: Vec<Statistics>,
}

impl BehaviorTreeObserver {
	/// Construct a new [`BehaviorTreeObserver`].
	pub fn new(root: &mut BehaviorTree) -> Self {
		let id: ConstString = "statistics".into();
        let (tx,rx) = mpsc::channel::<(Instant, BehaviorStatus, BehaviorStatus)>();
        // spawn receiver
        let handle = std::thread::spawn(move || {
            std::println!("observer is listening");
            loop {
                match rx.recv() {
                    Ok(value) => {
                        std::println!("{:?} {} {}", value.0, value.1, value.2);
                    },
                    Err(_) => {
                        std::println!("observer is done");
                        return -1
                    },
                };
            }
        });
		let mut observer = Self {
            _handle: handle,
			statistics: Vec::new(),
		};
		for node in root.iter_mut() {
			let statistic = Statistics::default();
            let tx_clone = tx.clone();
			let callback = move |
			       node: &BehaviorTreeElement,
			       new_status: &mut BehaviorStatus| {
                let timestamp = Instant::now();
                let tuple = (timestamp, node.status(), new_status.clone());
                // ignore any errors when sending
                tx_clone.send(tuple).ok();
                std::println!("sent status change");
			};
			observer
				.statistics
				.push(statistic);
			node.add_pre_status_change_callback(id.clone(), callback);
		}
		observer
	}

	/// Get the [`Statistics`] for a [`BehaviorTreeElement`](crate::tree::BehaviorTreeElement) using its uid.
	pub fn get_statistics(&self, uid: i16) -> Option<&Statistics> {
        if self.statistics.len() >= uid as usize {
            return Some(&(self.statistics[uid as usize]));
        }
		None
	}
}
// endregion:   --- BehaviorTreeObserver

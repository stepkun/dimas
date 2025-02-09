// Copyright © 2024 Stephan Kunz

//! `DiMAS` behavior implementation

#[doc(hidden)]
extern crate alloc;

// region:      --- modules
use alloc::{
	boxed::Box,
	format,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};
use core::{
	any::{Any, TypeId},
	fmt::{Debug, Display, Formatter},
};
use futures::future::BoxFuture;
use hashbrown::HashMap;
use tracing::{Level, debug, instrument};

use crate::{
	behavior::error::BehaviorError,
	blackboard::{Blackboard, BlackboardString, FromString, ParseStr},
	port::{PortDirection, PortList, PortRemapping, get_remapped_key},
};
// endregion:   --- modules

// region:      --- types
/// Return value of any behavior
#[allow(clippy::module_name_repetitions)]
pub type BehaviorResult<Output = BehaviorStatus> = Result<Output, BehaviorError>;

/// Signature of the function called when ticking a behavior
type BehaviorTickFn = for<'a> fn(
	&'a mut BehaviorData,
	&'a mut Box<dyn Any + Send + Sync>,
) -> BoxFuture<'a, Result<BehaviorStatus, BehaviorError>>;

/// Signature for the function to stop a behavior that is running
type BehaviorHaltFn =
	for<'a> fn(&'a mut BehaviorData, &'a mut Box<dyn Any + Send + Sync>) -> BoxFuture<'a, ()>;

/// Signature of the function that returns the list of available ports
type PortsFn = fn() -> PortList;
// endregion:   --- types

// region:      --- Behavior
/// A behavior node within the behavior tree
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Behavior {
	/// Holds behavior commons struct
	pub data: BehaviorData,
	/// Holds the original struct
	pub context: Box<dyn Any + Send + Sync>,
	/// Function pointer to start tick function
	/// Points to `tick()` for sync and `on_start()` for non sync
	pub start_fn: BehaviorTickFn,
	/// Function pointer to running tick function
	/// Points to `tick()` for sync and `on_running()` for non sync
	pub running_fn: BehaviorTickFn,
	/// Function pointer to halt function
	/// Poins to `halt()`
	pub halt_fn: BehaviorHaltFn,
}

impl Behavior {
	/// Returns behaviors current status
	#[must_use]
	pub const fn status(&self) -> BehaviorStatus {
		self.data.status
	}

	/// Resets the status back to [`BehaviorStatus::Idle`]
	pub const fn reset_status(&mut self) {
		self.data.status = BehaviorStatus::Idle;
	}

	/// Update the behavior's status
	pub const fn set_status(&mut self, status: BehaviorStatus) {
		self.data.status = status;
	}

	/// Tick the behavior
	/// # Errors
	/// - [`BehaviorError::Status`] if on_start() or on_running() return [`BehaviorStatus::Idle`]
	/// - [`BehaviorError`] of the called tick function
	#[instrument(level = Level::DEBUG, skip_all)]
	pub async fn execute_tick(&mut self) -> BehaviorResult {
		match self.data.bhvr_type {
			// The asnyc behaviors call different function when previous status was 'Running'
			BehaviorType::Action
			| BehaviorType::Condition
			| BehaviorType::Control
			| BehaviorType::Decorator => {
				let prev_status = self.data.status;

				let new_status = match prev_status {
					// start behavior when it is idle
					BehaviorStatus::Idle => {
						debug!("{}::on_start()", &self.data.config.path);
						let new_status = (self.start_fn)(&mut self.data, &mut self.context).await?;
						if matches!(new_status, BehaviorStatus::Idle) {
							return Err(BehaviorError::Status(
								format!("{}::on_start()", self.data.config.path),
								"Idle".to_string(),
							));
						}
						new_status
					}
					// check behavior status when it is running
					BehaviorStatus::Running => {
						debug!("{}::on_running()", &self.data.config.path);
						let new_status =
							(self.running_fn)(&mut self.data, &mut self.context).await?;
						if matches!(new_status, BehaviorStatus::Idle) {
							return Err(BehaviorError::Status(
								format!("{}::on_running()", self.data.config.path),
								"Idle".to_string(),
							));
						}
						new_status
					}
					// all other status return last status
					prev_status => prev_status,
				};

				self.set_status(new_status);

				Ok(new_status)
			}
			// SyncAction, SyncCondition may only return Success or Failure
			BehaviorType::SyncAction | BehaviorType::SyncCondition => {
				debug!("{}::tick()", &self.data.config.path);
				match (self.running_fn)(&mut self.data, &mut self.context).await? {
					status @ (BehaviorStatus::Running | BehaviorStatus::Idle) => Err(
						BehaviorError::Status(self.data.config.path.clone(), status.to_string()),
					),
					status => Ok(status),
				}
			}
			// SyncControl, SyncDecorator may return any status
			BehaviorType::SyncControl | BehaviorType::SyncDecorator => {
				debug!("{}::tick()", &self.data.config.path);
				(self.running_fn)(&mut self.data, &mut self.context).await
			}
		}
	}

	/// Halt the behavior
	pub async fn halt(&mut self) {
		(self.halt_fn)(&mut self.data, &mut self.context).await;
	}

	/// Get the name of the behavior
	#[must_use]
	pub fn name(&self) -> &str {
		&self.data.name
	}

	/// Get a mutable reference to the [`BehaviorConfig`]
	pub const fn config_mut(&mut self) -> &mut BehaviorConfig {
		&mut self.data.config
	}

	/// Get a reference to the [`BehaviorConfig`]
	#[must_use]
	pub const fn config(&self) -> &BehaviorConfig {
		&self.data.config
	}

	/// Get a mutable reference to the [`BehaviorData`]
	pub const fn data_mut(&mut self) -> &mut BehaviorData {
		&mut self.data
	}

	/// Get a reference to the [`BehaviorData`]
	#[must_use]
	pub const fn data(&self) -> &BehaviorData {
		&self.data
	}

	/// Get the [`BehaviorType`], which is one of:
	/// - [`BehaviorType::Action`],
	/// - [`BehaviorType::Condition`],
	/// - [`BehaviorType::Control`],
	/// - [`BehaviorType::Decorator`],
	/// - [`BehaviorType::SyncAction`].
	/// - [`BehaviorType::SyncCondition`],
	/// - [`BehaviorType::SyncControl`],
	/// - [`BehaviorType::SyncDecorator`],
	#[must_use]
	pub const fn bhvr_type(&self) -> BehaviorType {
		self.data.bhvr_type
	}

	/// Get the [`BehaviorCategory`], which is one of:
	/// - [`BehaviorCategory::Action`],
	/// - [`BehaviorCategory::Condition`],
	/// - [`BehaviorCategory::Control`],
	/// - [`BehaviorCategory::Decorator`],
	#[must_use]
	pub const fn bhvr_category(&self) -> BehaviorCategory {
		self.data.bhvr_category
	}

	/// Call the behaviors `ports()` function and return the [`PortList`]
	#[must_use]
	pub fn provided_ports(&self) -> PortList {
		(self.data.ports_fn)()
	}

	/// Return an iterator over the children or `None` if the behavior
	/// has no children
	#[must_use]
	pub fn children(&self) -> Option<&[Self]> {
		if self.data.children.is_empty() {
			None
		} else {
			Some(&self.data.children)
		}
	}

	/// Return a mutable iterator over the children or `None` if the behavior
	/// has no children
	pub fn children_mut(&mut self) -> Option<&mut [Self]> {
		if self.data.children.is_empty() {
			None
		} else {
			Some(&mut self.data.children)
		}
	}
}
// endregion:   --- Behavior

// region:      --- BehaviorCategory
/// Behavior category
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorCategory {
	/// Behavior without children that executes some action.
	Action,
	/// TODO: ????Behavior with children that executes a certain child based on a condition.
	Condition,
	/// Behavior with multiple children that executes them in some way.
	Control,
	/// Behavior with one child that modifies the execution or result of the child.
	Decorator,
}

impl Display for BehaviorCategory {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		let text = match self {
			Self::Action => "Action",
			Self::Condition => "Condition",
			Self::Control => "Control",
			Self::Decorator => "Decorator",
		};

		write!(f, "{text}")
	}
}
// endregion:   --- BehaviorCategory

// region:      --- BehaviorConfig
/// Contains configuration that all types of behaviors use.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct BehaviorConfig {
	/// The blackboard
	pub blackboard: Blackboard,
	/// Remapping for in ports
	pub input_ports: PortRemapping,
	/// remapping for out ports
	pub output_ports: PortRemapping,
	/// Behavior manifest
	pub manifest: Option<Arc<BehaviorManifest>>,
	/// Unique id of the behavior within the tree
	pub uid: u16,
	/// Full path to this behavior
	pub path: String,
}

impl BehaviorConfig {
	/// Aggregates the __instance__ dependent data of a behavior
	#[must_use]
	pub fn new(blackboard: Blackboard, path: String) -> Self {
		Self {
			blackboard,
			input_ports: HashMap::new(),
			output_ports: HashMap::new(),
			manifest: None,
			uid: 1, // TODO: set value dynamically
			path,
		}
	}

	/// Returns a reference to the blackboard.
	#[must_use]
	pub const fn blackboard(&self) -> &Blackboard {
		&self.blackboard
	}

	/// Adds a port to the config based on the direction
	pub fn add_port(&mut self, direction: &PortDirection, name: String, value: String) {
		match direction {
			PortDirection::Input => {
				self.input_ports.insert(name, value);
			}
			PortDirection::Output => {
				self.output_ports.insert(name, value);
			}
			PortDirection::InOut => {
				self.input_ports
					.insert(name.clone(), value.clone());
				self.output_ports.insert(name, value);
			}
		}
	}

	/// Check wether a certain port exists with a certain direction
	#[must_use]
	pub fn has_port(&self, direction: &PortDirection, name: &String) -> bool {
		match direction {
			PortDirection::Input => self.input_ports.contains_key(name),
			PortDirection::Output => self.output_ports.contains_key(name),
			PortDirection::InOut => {
				self.input_ports.contains_key(name) && self.output_ports.contains_key(name)
			}
		}
	}

	/// Returns a pointer to the [`BehaviorManifest`] for this behavior
	/// # Errors
	/// - if no manifest exists
	pub fn manifest(&self) -> Result<Arc<BehaviorManifest>, BehaviorError> {
		self.manifest.as_ref().map_or_else(
			|| {
				Err(BehaviorError::Unexpected(
					"Missing manifest, please report this with ".to_string(),
					file!().into(),
					line!(),
				))
			},
			|manifest| Ok(Arc::clone(manifest)),
		)
	}

	/// Replace the inner manifest.
	pub fn set_manifest(&mut self, manifest: Arc<BehaviorManifest>) {
		let _ = self.manifest.insert(manifest);
	}

	/// Returns the value of the input port at the `port` key as a `Result<T, BehaviorError>`
	/// # Errors
	/// The value is `Err` in the following situations:
	/// - The port wasn't found at that key
	/// - `T` doesn't match the type of the stored value
	/// - If a default value is needed (value is empty), couldn't parse default value
	/// - If a remapped key (e.g. a port value of `"{foo}"` references the blackboard
	///   key `"foo"`), blackboard entry wasn't found or couldn't be read as `T`
	/// - If port value is a string, couldn't convert it to `T` using `parse_str()`.
	pub fn get_input<T>(&mut self, port: &str) -> Result<T, BehaviorError>
	where
		T: FromString + Clone + Debug + Send + Sync + 'static,
	{
		match self.input_ports.get(port) {
			Some(val) => {
				// Check if default is needed
				if val.is_empty() {
					self.manifest().map_or_else(
						|_| Err(BehaviorError::Internal("no manifest found".into())),
						|manifest| {
							let port_info = manifest
								.ports
								.get(port)
								.ok_or_else(|| BehaviorError::FindPort(port.into()))?;

							port_info.default_value().map_or_else(
								|| Err(BehaviorError::FindPortDefault(port.into())),
								|default| {
									default.parse_str().map_or_else(
										|_| {
											Err(BehaviorError::ParsePortValue(
												port.into(),
												"String".into(),
											))
										},
										|value| Ok(value),
									)
								},
							)
						},
					)
				} else {
					match get_remapped_key(port, val) {
						// Value is a Blackboard pointer
						Some(key) => self
							.blackboard
							.get_stringy::<T>(&key)
							.map_or_else(
								|| Err(BehaviorError::NotInBlackboard(key)),
								|val| Ok(val),
							),
						// Value is just a normal string
						None => <T as FromString>::from_string(val).map_or_else(
							|_| {
								Err(BehaviorError::ParsePortValue(
									String::from(port),
									format!("{:?}", TypeId::of::<T>()),
								))
							},
							|val| Ok(val),
						),
					}
				}
			}
			// Port not found in behaviors port list
			None => Err(BehaviorError::PortNotDeclared(
				String::from(port),
				String::from(&self.path),
			)),
		}
	}

	/// Sets `value` into the blackboard. The key is based on the value provided
	/// to the port at `port`.
	///
	/// # Examples
	///
	/// - Port value: `"="`: uses the port name as the blackboard key
	/// - `"foo"` uses `"foo"` as the blackboard key
	/// - `"{foo}"` uses `"foo"` as the blackboard key
	/// # Errors
	/// - port is not defined at all or not defined as output port
	/// # Panics
	/// - if blackboard pointer cannot be stripped
	pub fn set_output<T>(&mut self, port: &str, value: T) -> Result<(), BehaviorError>
	where
		T: Clone + Debug + Send + Sync + 'static,
	{
		match self.output_ports.get(port) {
			Some(port_value) => {
				let blackboard_key = match port_value.as_str() {
					"=" => port.to_string(),
					value => {
						if value.is_bb_pointer() {
							value
								.strip_bb_pointer()
								.unwrap_or_else(|| 
									todo!()
								)
						} else {
							value.to_string()
						}
					}
				};

				self.blackboard.set(blackboard_key, value);

				Ok(())
			}
			None => Err(BehaviorError::Internal(
				port.to_string() + "could not set in Blackboard, possibly not defined as output",
			)),
		}
	}
}
// endregion:   --- BehaviorConfig

// region:      --- BehaviorData
/// @TODO: Restructure
#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct BehaviorData {
	/// Non unique behavior name. 
	/// Defaults to same as `type_str` if not set otherwise.
	pub name: String,
	/// Type name of the behavior
	pub type_str: String,
	/// @TODO: Restructure
	pub bhvr_type: BehaviorType,
	/// @TODO: Restructure
	pub bhvr_category: BehaviorCategory,
	/// @TODO:
	pub config: BehaviorConfig,
	/// @TODO:
	pub status: BehaviorStatus,
	/// Vector of child behaviors
	pub children: Vec<Behavior>,
	/// Function to get the list of ports
	pub ports_fn: PortsFn,
}

impl BehaviorData {
	/// Halt children from index `start` to the end.
	///
	/// # Errors
	/// - [`BehaviorError::IndexError`] if `start` is out of bounds.
	pub async fn halt_children(&mut self, start: usize) -> Result<(), BehaviorError> {
		if start >= self.children.len() {
			return Err(BehaviorError::Index(start));
		}

		let end = self.children.len();

		for i in start..end {
			self.halt_child_idx(i).await?;
		}

		Ok(())
	}

	/// Halts and resets all children
	/// # Panics
	/// if something really weird happens.
	pub async fn reset_children(&mut self) {
		self.halt_children(0)
			.await
			.expect("reset_children failed, shouldn't be possible. Report this");
	}

	/// Halt child at the `index`. Not to be confused with `halt_child()`, which is
	/// a helper that calls `halt_child_idx(0)`, primarily used for `Decorator`s.
	/// # Errors
	/// @TODO:
	pub async fn halt_child_idx(&mut self, index: usize) -> Result<(), BehaviorError> {
		let child = self
			.children
			.get_mut(index)
			.ok_or(BehaviorError::Index(index))?;
		if child.status() == BehaviorStatus::Running {
			child.halt().await;
		}
		child.reset_status();
		Ok(())
	}

	/// Sets the status of this behavior
	pub const fn set_status(&mut self, status: BehaviorStatus) {
		self.status = status;
	}

	/// Calls `halt_child_idx(0)`. This should only be used in
	/// `Decorator`s
	pub async fn halt_child(&mut self) {
		self.reset_child().await;
	}

	/// Halts and resets the first child. This should only be used in
	/// `Decorator`s
	pub async fn reset_child(&mut self) {
		if let Some(child) = self.children.get_mut(0) {
			if matches!(child.status(), BehaviorStatus::Running) {
				child.halt().await;
			}

			child.reset_status();
		}
	}

	/// Gets a mutable reference to the first child.
	/// Helper for `Decorator`s to get their child.
	pub fn child(&mut self) -> Option<&mut Behavior> {
		self.children.get_mut(0)
	}
}
// endregion:   --- BehaviorData

// region:      --- BehaviorManifest
/// @TODO:
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug)]
pub struct BehaviorManifest {
	/// @TODO:
	pub bhvr_type: BehaviorCategory,
	/// @TODO:
	pub registration_id: String,
	/// @TODO:
	pub ports: PortList,
	/// @TODO:
	pub description: String,
}

impl BehaviorManifest {
	/// Create the manifest
	pub fn new(
		bhvr_type: BehaviorCategory,
		registration_id: impl AsRef<str>,
		ports: PortList,
		description: impl AsRef<str>,
	) -> Self {
		Self {
			bhvr_type,
			registration_id: registration_id.as_ref().to_string(),
			ports,
			description: description.as_ref().to_string(),
		}
	}
}
// endregion:   --- BehaviorManifest

// region:      --- BehaviorStatus
/// Behavior status
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorStatus {
	/// Behavior execution failed.
	Failure,
	/// Behavior is not executing.
	Idle,
	/// Behavior is still executing.
	Running,
	/// Behavior has been skipped.
	Skipped,
	/// Behavior finished with success.
	Success,
}

impl BehaviorStatus {
	/// Create colourized output for modern terminals
	#[must_use]
	pub fn into_string_color(&self) -> String {
		let color_start = match self {
			Self::Failure => "\x1b[31m",
			Self::Idle => "\x1b[36m",
			Self::Running => "\x1b[33m",
			Self::Skipped => "\x1b[34m",
			Self::Success => "\x1b[32m",
		};

		color_start.to_string() + &self.to_string() + "\x1b[0m"
	}

	/// Check if status is signaling that the behavior is active
	#[must_use]
	pub const fn is_active(&self) -> bool {
		matches!(self, Self::Idle | Self::Skipped)
	}

	/// Check if status is signaling that the behavior is completed
	#[must_use]
	pub const fn is_completed(&self) -> bool {
		matches!(self, Self::Success | Self::Failure)
	}
}

impl Display for BehaviorStatus {
	fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
		let text = match self {
			Self::Failure => "Failure",
			Self::Idle => "Idle",
			Self::Running => "Running",
			Self::Skipped => "Skipped",
			Self::Success => "Success",
		};

		write!(f, "{text}")
	}
}
// endregion:   --- BehaviorStatus

// region:      --- BehaviorType
/// Behavior type
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorType {
	/// `Action`
	Action,
	/// `Condition`
	Condition,
	/// `Control`
	Control,
	/// `Decorator`
	Decorator,
	/// `SyncAction` will never return [`BehaviorStatus::Running`]
	SyncAction,
	/// `SyncCondition` will never return [`BehaviorStatus::Running`]
	SyncCondition,
	/// `SyncControl`
	SyncControl,
	/// `SyncDecorator`
	SyncDecorator,
}
// endregion:   --- BehaviorType

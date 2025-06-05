// Copyright © 2025 Stephan Kunz
#![allow(unused)]
#![allow(dead_code)]

//! Blackboard of `DiMAS`

// region:      --- modules
use alloc::{
	borrow::ToOwned,
	boxed::Box,
	collections::btree_map::BTreeMap,
	format,
	rc::Rc,
	string::{String, ToString},
	sync::Arc,
};
use core::{
	any::{Any, TypeId},
	cell::RefCell,
	fmt::Debug,
	ops::{Deref, DerefMut},
	str::FromStr,
};
use dimas_core::ConstString;
use dimas_scripting::{Environment, Error as ScriptingError, execution::ScriptingValue};
use parking_lot::RwLock;
use rustc_hash::FxBuildHasher;

use crate::behavior::BehaviorResult;

use super::{BlackboardInterface, error::Error};
// endregion:   --- modules

// region:      --- BlackboardData
/// `BlackboardData` are a key value store capable of storing any value.
#[derive(Debug, Default)]
pub struct BlackboardData {
	/// Using the [`FxBuildHasher`] to have same hash values for keys from different sources.
	storage: BTreeMap<ConstString, Entry>,
}

impl BlackboardInterface for BlackboardData {
	fn contains(&self, key: ConstString) -> bool {
		self.storage.contains_key(key.as_ref())
	}

	fn delete<T>(&mut self, key: ConstString) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		if let Some(old_entry) = self.storage.get(key.as_ref()) {
			let e = &*old_entry.0;
			let e = e as &dyn Any;
			let e = e.downcast_ref::<T>().cloned();
			if let Some(old) = e {
				self.storage.remove(key.as_ref());
				Ok(old)
			} else {
				Err(Error::WrongType(key))
			}
		} else {
			Err(Error::NotFound(key))
		}
	}

	fn get<T>(&self, key: ConstString) -> Result<T, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		let t_key = key.as_ref();
		self.storage.get(key.as_ref()).map_or_else(
			|| Err(Error::NotFound(key.clone())),
			|entry| {
				let en = &*entry.0;
				en.downcast_ref::<T>().map_or_else(
					|| {
						en.downcast_ref::<String>().map_or_else(
							|| {
								// maybe it is a value set by scripting
								self.get_env(key.clone()).map_or_else(
									|_| Err(Error::WrongType(t_key.into())),
									|val| {
										let s = match val {
											ScriptingValue::Nil() => todo!(),
											ScriptingValue::Boolean(b) => b.to_string(),
											ScriptingValue::Float64(f) => f.to_string(),
											ScriptingValue::Int64(i) => i.to_string(),
											ScriptingValue::String(s) => s,
										};
										T::from_str(&s).map_or_else(
											|_| {
												Err(Error::ParsePortValue(
													t_key.into(),
													format!("{:?}", TypeId::of::<T>()).into(),
												))
											},
											|val| Ok(val),
										)
									},
								)
							},
							|s| {
								T::from_str(s).map_or_else(
									|_| {
										Err(Error::ParsePortValue(
											t_key.into(),
											format!("{:?}", TypeId::of::<T>()).into(),
										))
									},
									|val| Ok(val),
								)
							},
						)
					},
					|value| Ok(value.clone()),
				)
			},
		)
	}

	fn get_entry(&self, key: ConstString) -> Option<Entry> {
		self.storage.get(key.as_ref()).map_or_else(
			|| None,
			|entry| {
				let e = entry.0.clone();
				Some(Entry(e))
			},
		)
	}

	fn set<T>(&mut self, key: ConstString, value: T) -> Result<Option<T>, Error>
	where
		T: Any + Clone + Debug + FromStr + ToString + Send + Sync + 'static,
	{
		if let Some(old_entry) = self.storage.get(key.as_ref()) {
			let e = &*old_entry.0;
			let e = e as &dyn Any;
			let e = e.downcast_ref::<T>().cloned();
			if e.is_some() {
				let entry = Entry(Arc::new(value));
				self.storage.insert(key, entry);
				Ok(e)
			} else {
				Err(Error::WrongType(key))
			}
		} else {
			let entry = Entry(Arc::new(value));
			self.storage.insert(key, entry);
			Ok(None)
		}
	}
}

impl Environment for BlackboardData {
	fn define_env(
		&mut self,
		key: ConstString,
		value: ScriptingValue,
	) -> Result<(), ScriptingError> {
		if self.contains(key.clone()) {
			self.set_env(key, value)
		} else {
			match value {
				ScriptingValue::Nil() => todo!(),
				ScriptingValue::Boolean(b) => {
					self.set(key, b);
				}
				ScriptingValue::Float64(f) => {
					self.set(key, f);
				}
				ScriptingValue::Int64(i) => {
					self.set(key, i);
				}
				ScriptingValue::String(s) => {
					self.set(key, s);
				}
			}
			Ok(())
		}
	}

	fn get_env(&self, name: ConstString) -> Result<ScriptingValue, ScriptingError> {
		self.get_entry(name.clone()).map_or_else(
			|| Err(ScriptingError::GlobalNotDefined(name.clone())),
			|entry| {
				// let entry = **(entry);
				let type_id = (**entry).type_id();
				if type_id == TypeId::of::<String>() {
					let s = entry.downcast_ref::<String>().expect("snh");
					Ok(ScriptingValue::String(s.to_owned()))
				} else if type_id == TypeId::of::<f64>() {
					let f = entry.downcast_ref::<f64>().expect("snh");
					Ok(ScriptingValue::Float64(f.to_owned()))
				} else if type_id == TypeId::of::<f32>() {
					let f = entry.downcast_ref::<f32>().expect("snh");
					Ok(ScriptingValue::Float64(f64::from(f.to_owned())))
				} else if type_id == TypeId::of::<i64>() {
					let i = entry.downcast_ref::<i64>().expect("snh");
					Ok(ScriptingValue::Int64(i.to_owned()))
				} else if type_id == TypeId::of::<i32>() {
					let i = entry.downcast_ref::<i32>().expect("snh");
					Ok(ScriptingValue::Int64(i64::from(i.to_owned())))
				} else if type_id == TypeId::of::<u32>() {
					let i = entry.downcast_ref::<u32>().expect("snh");
					Ok(ScriptingValue::Int64(i64::from(i.to_owned())))
				} else if type_id == TypeId::of::<i16>() {
					let i = entry.downcast_ref::<i16>().expect("snh");
					Ok(ScriptingValue::Int64(i64::from(i.to_owned())))
				} else if type_id == TypeId::of::<u16>() {
					let i = entry.downcast_ref::<u16>().expect("snh");
					Ok(ScriptingValue::Int64(i64::from(i.to_owned())))
				} else if type_id == TypeId::of::<u8>() {
					let i = entry.downcast_ref::<u8>().expect("snh");
					Ok(ScriptingValue::Int64(i64::from(i.to_owned())))
				} else if type_id == TypeId::of::<i8>() {
					let i = entry.downcast_ref::<i8>().expect("snh");
					Ok(ScriptingValue::Int64(i64::from(i.to_owned())))
				} else {
					Err(ScriptingError::GlobalHasUnknownType(name.clone()))
				}
			},
		)
	}

	#[allow(clippy::cast_possible_truncation)]
	#[allow(clippy::cast_sign_loss)]
	fn set_env(&mut self, name: ConstString, value: ScriptingValue) -> Result<(), ScriptingError> {
		let entry_type_id = if let Some(entry) = self.get_entry(name.clone()) {
			let inner_entry = &entry;
			(*(inner_entry.0)).type_id()
		} else {
			return Err(ScriptingError::GlobalNotDefined(name));
		};
		match value {
			ScriptingValue::Nil() => todo!(),
			ScriptingValue::Boolean(b) => {
				if TypeId::of::<bool>() == entry_type_id {
					self.set(name, b);
				} else {
					return Err(ScriptingError::GlobalWrongType(name));
				}
			}
			ScriptingValue::Float64(f) => {
				if TypeId::of::<f64>() == entry_type_id {
					self.set(name, f);
				} else if TypeId::of::<f32>() == entry_type_id {
					if f > f64::from(f32::MAX) || f < f64::from(f32::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, f as f32);
				} else {
					return Err(ScriptingError::GlobalWrongType(name));
				}
			}
			ScriptingValue::Int64(i) => {
				if TypeId::of::<i64>() == entry_type_id {
					self.set(name, i);
				} else if TypeId::of::<i32>() == entry_type_id {
					if i > i64::from(i32::MAX) || i < i64::from(i32::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, i as i32);
				} else if TypeId::of::<u32>() == entry_type_id {
					if i > i64::from(u32::MAX) || i < i64::from(u32::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, i as u32);
				} else if TypeId::of::<i16>() == entry_type_id {
					if i > i64::from(i16::MAX) || i < i64::from(i16::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, i as i16);
				} else if TypeId::of::<u16>() == entry_type_id {
					if i > i64::from(u16::MAX) || i < i64::from(u16::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, i as u16);
				} else if TypeId::of::<i8>() == entry_type_id {
					if i > i64::from(i8::MAX) || i < i64::from(i8::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, i as i8);
				} else if TypeId::of::<u8>() == entry_type_id {
					if i > i64::from(u8::MAX) || i < i64::from(u8::MIN) {
						return Err(ScriptingError::GlobalExceedsLimits(name));
					}
					self.set(name, i as u8);
				} else {
					return Err(ScriptingError::GlobalWrongType(name));
				}
			}
			ScriptingValue::String(s) => {
				if TypeId::of::<String>() == entry_type_id {
					self.set(name, s);
				} else {
					return Err(ScriptingError::GlobalWrongType(name));
				}
			}
		}
		Ok(())
	}
}
// endregion:   --- BlackboardData

// region:      --- Entry
pub struct Entry(Arc<dyn Any + Send + Sync + 'static>);

impl Debug for Entry {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:?}", self.0)?;
		Ok(())
	}
}

impl Deref for Entry {
	type Target = Arc<dyn Any + Send + Sync + 'static>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Entry {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
// endregion:   --- Entry

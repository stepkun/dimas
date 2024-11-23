// Copyright © 2024 Stephan Kunz

//! A loader for dynamic libraries
//!

extern crate std;

// region:      --- modules
use crate::error::Error;
use crate::ComponentRegistrar;
use anyhow::Result;
use libloading::Library;
use std::ffi::OsString;
use std::path::PathBuf;
use std::collections::HashMap;
// endregion:   --- modules

/// Library loader implementation
#[derive(Debug)]
pub struct LibManager {
	libs: HashMap<OsString, Library>,
}

impl Default for LibManager {
	/// Create a default [`LibLoader`]
	#[must_use]
	fn default() -> Self {
		Self::new()
	}
}

impl LibManager {
	/// Create a [`LibLoader`]
	#[must_use]
	pub fn new() -> Self {
		Self {
			libs: HashMap::new(),
		}
	}

	/// Load a library file from path and register contained components
	/// # Errors
	///
	#[allow(unsafe_code)]
	pub fn load_lib(&mut self, registrar: &mut dyn ComponentRegistrar, path: &str) -> Result<()> {
		std::dbg!(&path);
		let pathbuf = PathBuf::from(path);
		if !pathbuf.exists() || !pathbuf.is_file() {
			Err(Error::LibNotFound.into())
		} else {
			let filename = libloading::library_filename(&pathbuf);
			unsafe {
				let lib = libloading::Library::new(&filename);
				match lib {
					Ok(lib) => {
						let func: libloading::Symbol<unsafe extern fn(&mut dyn ComponentRegistrar) -> u32> =
						lib.get(b"register_components")?;
						let res = func(registrar);
						match res {
							0 => {
								self.libs.insert(filename, lib);
								Ok(())
							},
							_ => Err(Error::LibRegisterFailed.into())
						}
					},
					Err(_err) => Err(Error::LibLoadFailed.into()),
				}
			}
		}
	}

	/// Unload a library
	/// # Errors
	///
	#[allow(unsafe_code)]
	pub fn unload_lib(&mut self, registrar: &mut dyn ComponentRegistrar, path: &str) -> Result<()> {
		let filename = libloading::library_filename(path);
		if let Some(lib) = self.libs.remove(&filename) {
			unsafe {
				let func: libloading::Symbol<unsafe extern fn(&mut dyn ComponentRegistrar) -> u32> =
					lib.get(b"unregister_components")?;
				let res = func(registrar);
				match res {
					0 => return Ok(()),
					_ => return Err(Error::LibDeregisterFailed.into()),
				}
			}
		}
		Err(Error::LibUnloadFailed.into())
	}
}

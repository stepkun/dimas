// Copyright © 2024 Stephan Kunz

//! A loader for dynamic libraries
//!

extern crate std;

// region:      --- modules
use super::error::Error;
use crate::utils::ComponentRegistry;
use anyhow::Result;
use libloading::Library;
use std::collections::HashMap;
use std::ffi::OsString;
// endregion:   --- modules

/// Library loader implementation
#[derive(Debug)]
pub struct LibManager {
	libs: HashMap<OsString, Library>,
}

impl Default for LibManager {
	/// Create a default [`LibManager`]
	#[must_use]
	fn default() -> Self {
		Self::new()
	}
}

impl LibManager {
	/// Create a [`LibManager`]
	#[must_use]
	pub fn new() -> Self {
		Self {
			libs: HashMap::new(),
		}
	}

	/// Load a library with "libname" and register contained components.
	/// Currently works only, if lib is in same directory as executable
	/// # Errors
	///
	#[allow(unsafe_code)]
	pub fn load_lib(&mut self, registrar: &mut dyn ComponentRegistry, libname: &str) -> Result<()> {
		let filename = libloading::library_filename(libname);
		let pathbuf = std::env::current_exe()?
			.parent()
			.ok_or(Error::NotFound)?
			.join(&filename);
		if !pathbuf.exists() || !pathbuf.is_file() {
			Err(Error::NotFound.into())
		} else {
			unsafe {
				let lib = libloading::Library::new(&filename)?;
				let func: libloading::Symbol<
					unsafe extern "C" fn(&mut dyn ComponentRegistry) -> u32,
				> = lib.get(b"register_components")?;
				let res = func(registrar);
				match res {
					0 => {
						self.libs.insert(filename, lib);
						Ok(())
					}
					_ => Err(Error::RegisterFailed.into()),
				}
			}
		}
	}

	/// Unload a library
	/// # Errors
	///
	#[allow(unsafe_code)]
	pub fn unload_lib(
		&mut self,
		registrar: &mut dyn ComponentRegistry,
		libname: &str,
	) -> Result<()> {
		let filename = libloading::library_filename(libname);
		if let Some(lib) = self.libs.remove(&filename) {
			unsafe {
				let func: libloading::Symbol<
					unsafe extern "C" fn(&mut dyn ComponentRegistry) -> u32,
				> = lib.get(b"unregister_components")?;
				let res = func(registrar);
				lib.close()?;
				match res {
					0 => return Ok(()),
					_ => return Err(Error::DeregisterFailed.into()),
				}
			}
		}
		Err(Error::UnloadFailed.into())
	}
}

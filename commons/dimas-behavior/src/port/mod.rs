// Copyright © 2025 Stephan Kunz

//! `dimas-behavior` Port module

pub mod error;
#[allow(clippy::module_inception)]
mod port;

// flatten
pub use port::{
	NewPortDefinition, NewPortDirection, PortList, PortRemappings, create_port, get_remapped_key,
	is_bb_pointer, strip_bb_pointer,
};

// region:		---macros
/// macro for creation of an input port definition
#[macro_export]
macro_rules! input_port_macro {
	($tp:ty, $name:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::In, $name, "", "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::In, $name, $default, "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal, $desc:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::In, $name, $default, $desc)
			.expect("snh")
	};
}

/// macro for creation of an in/out port definition
#[macro_export]
macro_rules! inout_port_macro {
	($tp:ty, $name:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::InOut, $name, "", "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::InOut, $name, $default, "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal, $desc:literal) => {
		$crate::port::create_port::<$tp>(
			$crate::port::NewPortDirection::InOut,
			$name,
			$default,
			$desc,
		)
		.expect("snh")
	};
}

/// macro for creation of an output port definition
#[macro_export]
macro_rules! output_port_macro {
	($tp:ty, $name:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::Out, $name, "", "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal) => {
		$crate::port::create_port::<$tp>($crate::port::NewPortDirection::Out, $name, $default, "")
			.expect("snh")
	};
	($tp:ty, $name:literal, $default:literal, $desc:literal) => {
		$crate::port::create_port::<$tp>(
			$crate::port::NewPortDirection::Out,
			$name,
			$default,
			$desc,
		)
		.expect("snh")
	};
}

/// macro for creation of a [`PortList`]
#[macro_export]
macro_rules! port_list {
	($($e:expr),*) => {$crate::port::PortList(vec![$($e)*])};
}
// endregion:	--- macros

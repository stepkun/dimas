// Copyright © 2024 Stephan Kunz
#![no_std]

//! Behavior library of `DiMAS`.

#[doc(hidden)]
extern crate alloc;

// modules
pub mod behavior;
pub mod blackboard;
pub mod factory;
pub mod port;
pub mod tree;

// flatten:
pub use behavior::Behavior;

// re-export
pub use dimas_behavior_macros::Behavior;
pub use dimas_scripting::ScriptEnum;
pub use dimas_scripting::SharedRuntime;
pub use dimas_scripting_macros::ScriptEnum;

// region:		---macros
/// Macro to register a behavior with additional arguments.
///
/// # Usage:
///
/// ```no-test
/// register_node!(<mutable reference to behavior factory>, <struct to register>, <"identifying name">, <arg1>, <arg2>, ...>)
/// ```
///
/// # Example:
///
/// ```no-test
/// register_node!(&mut factory, ActionA, "Action_A", 42, "hello world".into())?;
/// ```
#[macro_export]
macro_rules! register_node {
	($factory:expr, $tp:ty, $name:expr, $($arg:expr),* $(,)?) => {{
		let bhvr_creation_fn = alloc::boxed::Box::new(move || -> alloc::boxed::Box<dyn $crate::behavior::BehaviorExecution> {
			alloc::boxed::Box::new(<$tp>::new($($arg),*))
		});
		$factory
			.registry()
			.add_behavior($name, bhvr_creation_fn, <$tp>::kind())
	}};
}

/// Macro to register enums for scripting.
/// Enum must derive [`ScriptEnum`].
/// It is also possible to register discrete value(s).
///
/// # Usage:
///
/// With an enum type:
/// ```no-test
/// register_scripting_enum!(<mutable reference to behavior factory>, <enum to register>)
/// ```
///
/// With discrete value(s)
/// ```no-test
/// register_scripting_enum!(<mutable reference to behavior factory>, <Identifier as str>, <Value as int>)
/// ```
///
/// # Examples:
///
/// ```no-test
/// #[derive(ScriptEnum)]
/// enum Color {
///     RED,
///     BLUE,
///     GREEN,
/// }
///
/// register_scripting_enum!(&mut factory, Color);
/// ```
///
/// ```no-test
/// register_scripting_enum!(&mut "THE_ANSWER", 42, "OTHER_ANSWER", 44);
/// ```
#[macro_export]
macro_rules! register_scripting_enum {
	($factory:ident, $tp:ty) => {
		for (key, value) in <$tp>::key_value_tuples() {
			$factory.register_enum_tuple(key, value)?;
		}
	};
	($factory:ident, $($key:literal, $value:literal),+ $(,)?) => {
		$( $factory.register_enum_tuple($key, $value)?; )+;
	};
}
// endregion:	---macros

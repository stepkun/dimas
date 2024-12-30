// Copyright © 2024 Stephan Kunz

//! `dimas-core` functional macros

#[doc(hidden)]
extern crate alloc;

#[macro_export]
/// Macro
macro_rules! build_bhvr_ptr {
    ($conf:expr, $n:expr, $t:ty $(,$x:expr),* $(,)?) => {
        {
            let mut behavior = <$t>::create_behavior($n, $conf, $($x),*);
            let manifest = $crate::behavior::BehaviorManifest::new(behavior.bhvr_category(), $n, behavior.provided_ports(), "");
            behavior.config_mut().set_manifest(::alloc::sync::Arc::new(manifest));

            behavior
        }
    };
}

/// Macro
#[macro_export]
macro_rules! define_ports {
    ( $($tu:expr),* ) => {
        {
            let mut ports = $crate::port::PortList::new();
            $(
                let (name, port_info) = $tu;
                ports.insert(::alloc::string::String::from(name), port_info);
            )*

            ports
        }
    };
}

/// Macro
#[macro_export]
macro_rules! input_port {
	($n:literal) => {{
		let port_info = $crate::port::Port::new($crate::port::PortDirection::Input);

		($n, port_info)
	}};
	($n:literal, expr) => {{
		let mut port_info = $crate::port::Port::new($crate::port::PortDirection::Input);

		port_info.set_expr(true);

		($n, port_info)
	}};
	($n:literal, $d:expr) => {{
		let mut port_info = $crate::port::Port::new($crate::port::PortDirection::Input);

		port_info.set_default($d);

		($n, port_info)
	}};
	($n:literal, $d:expr, expr) => {{
		let mut port_info = $crate::port::Port::new($crate::port::PortDirection::Input);

		port_info.set_default($d);
		port_info.set_expr(true);

		($n, port_info)
	}};
}

/// Macro
#[macro_export]
macro_rules! output_port {
	($n:tt) => {{
		let port_info = $crate::port::Port::new($crate::port::PortDirection::Output);

		($n, port_info)
	}};
}

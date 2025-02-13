// Copyright © 2024 Stephan Kunz

//! Macros for creating and managing ports

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
	(_) => { todo!(); };
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
	(_) => {
		todo!();
	};
}

/// Macro
#[macro_export]
macro_rules! output_port {
	($n:tt) => {{
		let port_info = $crate::port::Port::new($crate::port::PortDirection::Output);

		($n, port_info)
	}};
	(_) => {
		todo!();
	};
}

/// Macro
#[macro_export]
macro_rules! inout_port {
	($n:literal) => {{
		let port_info = $crate::port::Port::new($crate::port::PortDirection::InOut);

		($n, port_info)
	}};
	($n:literal, expr) => {{
		let mut port_info = $crate::port::Port::new($crate::port::PortDirection::InOut);

		port_info.set_expr(true);

		($n, port_info)
	}};
	($n:literal, $d:expr) => {{
		let mut port_info = $crate::port::Port::new($crate::port::PortDirection::InOut);

		port_info.set_default($d);

		($n, port_info)
	}};
	($n:literal, $d:expr, expr) => {{
		let mut port_info = $crate::port::Port::new($crate::port::PortDirection::InOut);

		port_info.set_default($d);
		port_info.set_expr(true);

		($n, port_info)
	}};
	(_) => {
		todo!();
	};
}

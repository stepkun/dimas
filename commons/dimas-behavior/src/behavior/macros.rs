// Copyright © 2024 Stephan Kunz

//! Macros used for creating and managing behavior

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
	(_) => { todo!(); };
}

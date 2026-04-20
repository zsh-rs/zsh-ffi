macro_rules! module {
    ($name:ident, use $module:path) => {
        pub mod $name {
            paste::paste! {
                pub use super::$module::[<boot_zshQs $name>] as boot;
                pub use super::$module::[<cleanup_zshQs $name>] as cleanup;
                pub use super::$module::[<enables_zshQs $name>] as enables;
                pub use super::$module::[<features_zshQs $name>] as features;
                pub use super::$module::[<finish_zshQs $name>] as finish;
                pub use super::$module::[<setup_zshQs $name>] as setup;
            }
        }
        hide_module_reexports!($name, $module);
    };
}

macro_rules! hide_module_reexports {
    ($name:ident, $module:path) => {
        paste::paste! {
            #[allow(unused_imports, hidden_glob_reexports)] use $module::[<boot_zshQs $name>];
            #[allow(unused_imports, hidden_glob_reexports)] use $module::[<cleanup_zshQs $name>];
            #[allow(unused_imports, hidden_glob_reexports)] use $module::[<enables_zshQs $name>];
            #[allow(unused_imports, hidden_glob_reexports)] use $module::[<features_zshQs $name>];
            #[allow(unused_imports, hidden_glob_reexports)] use $module::[<finish_zshQs $name>];
            #[allow(unused_imports, hidden_glob_reexports)] use $module::[<setup_zshQs $name>];
        }
    };
}

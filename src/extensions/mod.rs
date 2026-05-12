

// helpers
mod types;

// extensions
mod builtin;
mod conditions;
mod math;

unsafe impl Sync for crate::base::hashnode {}

unsafe impl Sync for crate::base::features {}
unsafe impl Send for crate::base::features {}
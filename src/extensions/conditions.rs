use std::ffi::CStr;

use crate::base::conddef;

unsafe impl Sync for conddef {}
unsafe impl Send for conddef {}

impl conddef {
    /// Creates a new `conddef` with the specified fields.
    ///
    /// Like `CONDDEF` in zsh's C code.
    /// 
    /// # Args
    /// - `name`: The name of the conddef command.
    /// - `handler`: The handler function for the conddef command.
    /// - `min`: The minimum number of arguments the conddef command accepts.
    /// - `max`: The maximum number of arguments the conddef command accepts.
    /// - `id`: The unique identifier for the conddef command. Can be used to distinguish between different conddefs using the same handler function.
    ///
    pub const fn new(
        name: &'static CStr,
        handler: super::types::CondHandlerInner,
        min: i32,
        max: i32,
        id: i32,
    ) -> Self {
        Self {
            next: std::ptr::null_mut(),
            name: name.as_ptr() as _,
            flags: 0,
            handler: Some(handler),
            min,
            max,
            condid: id,
            module: std::ptr::null_mut(),
        }
    }

    /// Creates a new `conddef` with the `CONDF_INFIX` flag set.
    ///
    /// # Args
    /// - `name`: The name of the conddef command.
    /// - `handler`: The handler function for the conddef command.
    /// - `id`: The unique identifier for the conddef command. Can be used to distinguish between different conddefs using the same handler function.
    ///
    pub const fn new_infix(
        name: &'static CStr,
        handler: super::types::CondHandlerInner,
        id: i32,
    ) -> Self {
        Self {
            next: std::ptr::null_mut(),
            name: name.as_ptr() as _,
            flags: crate::base::CONDF_INFIX as _,
            handler: Some(handler),
            min: 0,
            max: 0,
            condid: id,
            module: std::ptr::null_mut(),
        }
    }
}

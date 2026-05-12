use std::ffi::CStr;

use crate::base::builtin;

unsafe impl Sync for builtin {}
unsafe impl Send for builtin {}

impl builtin {
    /// Creates a new `builtin` with the specified fields.
    ///
    /// Like `BUILTIN` in zsh's C code.
    /// 
    /// # Args
    /// - `name`: The name of the builtin command.
    /// - `handler`: The handler function for the builtin command.
    /// - `minargs`: The minimum number of arguments the builtin command accepts.
    /// - `maxargs`: The maximum number of arguments the builtin command accepts.
    /// - `id`: The unique identifier for the builtin command. Can be used to distinguish between different builtins using the same handler function.
    /// - `optstr`: The option string for the builtin command.
    ///
    pub const fn new(
        name: &'static CStr,
        handler: super::types::HandlerFuncInner,
        minargs: i32,
        maxargs: i32,
        id: i32,
        optstr: &'static CStr,
    ) -> Self {
        Self {
            node: crate::base::hashnode {
                next: std::ptr::null_mut(),
                nam: name.as_ptr() as _,
                flags: 0,
            },
            handlerfunc: Some(handler),
            minargs,
            maxargs,
            funcid: id,
            optstr: optstr.as_ptr() as _,
            defopts: std::ptr::null_mut(),
        }
    }
}

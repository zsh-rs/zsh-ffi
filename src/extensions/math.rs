use std::ffi::CStr;

use crate::base::mathfunc;

unsafe impl Sync for mathfunc {}
unsafe impl Send for mathfunc {}

impl mathfunc {
    /// Creates a new `mathfunc` that gets an array of mnumbers (the zsh type for representing values in arithmetic expressions) taken from the string in parentheses at the function call.
    /// 
    /// Like `NUMMATHFUNC` in zsh's C code.
    ///
    /// # Args
    /// - `name`: The name of the mathfunc command.
    /// - `handler`: The handler function for the mathfunc command.
    /// - `minargs`: The minimum number of arguments the mathfunc command accepts.
    /// - `maxargs`: The maximum number of arguments the mathfunc command accepts.
    /// - `id`: The unique identifier for the mathfunc command. Can be used to distinguish between different mathfuncs using the same handler function.
    ///
    pub const fn new_num(
        name: &'static CStr,
        handler: super::types::NumMathFuncInner,
        minargs: i32,
        maxargs: i32,
        id: i32,
    ) -> Self {
        Self {
            next: std::ptr::null_mut(),
            name: name.as_ptr() as _,
            flags: 0,
            nfunc: Some(handler),
            sfunc: None,
            module: std::ptr::null_mut(),
            minargs,
            maxargs,
            funcid: id,
        }
    }
    
    /// Creates a new `mathfunc` that gets the string in parentheses at the call as one string argument (without the parentheses)
    /// 
    /// Like `STRMATHFUNC` in zsh's C code.
    /// 
    /// # Args
    /// - `name`: The name of the mathfunc command.
    /// - `handler`: The handler function for the mathfunc command.
    /// - `id`: The unique identifier for the mathfunc command. Can be used to distinguish between different mathfuncs using the same handler function.
    ///
    pub const fn new_str(
        name: &'static CStr,
        handler: super::types::StrMathFuncInner,
        id: i32,
    ) -> Self {
        Self {
            next: std::ptr::null_mut(),
            name: name.as_ptr() as _,
            flags: crate::base::MFF_STR as _,
            nfunc: None,
            sfunc: Some(handler),
            module: std::ptr::null_mut(),
            minargs: 0,
            maxargs: 0,
            funcid: id,
        }
    }
}

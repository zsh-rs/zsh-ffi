#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod bindings {
    use crate::base::*;
    include!(concat!(env!("OUT_DIR"), "/zle.rs"));
}

pub use bindings::*;

module!(zutil, use bindings);
module!(compctl, use bindings);
module!(complist, use bindings);
module!(computil, use bindings);
module!(zleparameter, use bindings);

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod bindings {
    use crate::base::Module;
    include!(concat!(env!("OUT_DIR"), "/modules.rs"));
}

module!(datetime, use bindings);
module!(langinfo, use bindings);
module!(parameter, use bindings);
module!(termcap, use bindings);
module!(terminfo, use bindings);
module!(zutil, use bindings);
